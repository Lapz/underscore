use std::collections::{HashMap, HashSet};
use util::emitter::Reporter;
use syntax::ast::{Expression, Function, Ident, Literal, Op, Program, Statement, Ty, TyAlias};
use util::pos::{Span, Spanned};
use util::symbol::Table;
use syntax::ast::Ty as astType;

use std::ops::{Deref, DerefMut};

type InferResult<T> = Result<T, ()>;

trait Types {
    fn ftv(&self) -> HashSet<TypeVar>;
    fn apply(&self, &Subst) -> Self;
}

trait GetIdent {
    fn ident(&mut self, name: &str) -> Ident;
}

#[derive(Debug, Clone)]
pub struct Subst(HashMap<TypeVar, Type>);

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Unique(pub u64);

#[derive(Clone, Debug)]
pub struct Infer {
    reporter: Reporter,
    gen: TypeVarGen,
}

#[derive(Clone, Debug)]
pub struct Env {
    env: Table<Ident, Scheme>,
    tyenv: Table<Ident, Type>,
}

#[derive(Debug, Clone)]
pub struct TypeEnv(HashMap<Ident, Scheme>, Reporter, TypeVarGen);

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct TypeVar(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    name: TypeVar,
    pub ty: Type,
}

static mut UNIQUE_COUNT: u64 = 0;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Nil,
    Int,
    String,
    Char,
    Bool,
    Struct(Vec<Type>, Vec<Field>),
    Fun(Vec<Type>, Box<Type>),
    Var(TypeVar),
    Unique(Box<Type>, Unique),
}

#[derive(Debug, Clone)]
pub struct Scheme {
    pub vars: Vec<TypeVar>,
    pub ty: Type,
}

impl Scheme {
    fn new(ty: Type) -> Self {
        Scheme {
            vars: Vec::new(),
            ty,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeVarGen {
    supply: u32,
}

impl Unique {
    pub fn new() -> Self {
        let value = unsafe { UNIQUE_COUNT };
        unsafe { UNIQUE_COUNT += 1 };
        Unique(value)
    }
}

impl TypeVarGen {
    pub fn new() -> TypeVarGen {
        TypeVarGen { supply: 0 }
    }
    pub fn next(&mut self) -> TypeVar {
        let v = TypeVar(self.supply);
        self.supply += 1;
        v
    }
}

impl<'a, T> Types for Vec<T>
where
    T: Types,
{
    fn ftv(&self) -> HashSet<TypeVar> {
        self.iter()
            .map(|x| x.ftv())
            .fold(HashSet::new(), |set, x| set.union(&x).cloned().collect())
    }

    fn apply(&self, s: &Subst) -> Vec<T> {
        self.iter().map(|x| x.apply(s)).collect()
    }
}

impl Types for Type {
    fn ftv(&self) -> HashSet<TypeVar> {
        match *self {
            Type::Var(ref n) => {
                let mut set = HashSet::new();
                set.insert(*n);
                set
            }
            Type::Unique(ref ty, _) => ty.ftv(),
            Type::Nil | Type::Int | Type::String | Type::Char | Type::Bool => HashSet::new(),
            Type::Fun(ref params, ref returns) => {
                let mut set = HashSet::new();

                for param in params {
                    set.union(&param.ftv());
                }

                set.union(&returns.ftv());

                set
            }

            Type::Struct(ref items, ref fields) => {
                let mut set = HashSet::new();

                for item in items {
                    set.union(&item.ftv());
                }

                for field in fields {
                    set.union(&field.ty.ftv());
                }

                set
            }
        }
    }

    fn apply(&self, subst: &Subst) -> Type {
        match *self {
            Type::Var(ref n) => subst.0.get(n).unwrap_or(self).clone(),
            Type::Fun(ref types, ref returns) => {
                let mut param_tys: Vec<Type> = vec![];

                for param in types {
                    param_tys.push(param.apply(subst))
                }

                Type::Fun(param_tys, Box::new(returns.apply(subst)))
            }
            _ => self.clone(),
        }
    }
}

impl Subst {
    fn new() -> Self {
        Subst(HashMap::new())
    }

    fn union(&self, other: &Subst) -> Subst {
        let mut new_context = HashMap::new();
        for (key, value) in self.0.iter() {
            new_context.insert(*key, value.clone());
        }
        for (key, value) in other.0.iter() {
            new_context.insert(*key, value.clone());
        }
        Subst(new_context)
    }

    fn compose(&self, other: &Subst) -> Subst {
        self.union(other)
    }
}

impl Types for Scheme {
    fn ftv(&self) -> HashSet<TypeVar> {
        self.ty
            .ftv()
            .union(&self.vars.iter().cloned().collect())
            .cloned()
            .collect()
    }
    fn apply(&self, subst: &Subst) -> Scheme {
        Scheme {
            vars: self.vars.clone(),
            ty: {
                let mut sub = subst.clone();
                for var in &self.vars {
                    sub.remove(var);
                }
                self.ty.apply(&sub)
            },
        }
    }
}

impl Deref for Subst {
    type Target = HashMap<TypeVar, Type>;
    fn deref(&self) -> &HashMap<TypeVar, Type> {
        &self.0
    }
}
impl DerefMut for Subst {
    fn deref_mut(&mut self) -> &mut HashMap<TypeVar, Type> {
        &mut self.0
    }
}

impl Scheme {
    fn instantiate(&self, tvg: &mut TypeVarGen) -> Type {
        let newvars = self.vars.iter().map(|_| Type::Var(tvg.next()));
        self.ty
            .apply(&Subst(self.vars.iter().cloned().zip(newvars).collect()))
    }
}

impl TypeVar {
    fn bind(&self, ty: &Type) -> InferResult<Subst> {
        if let &Type::Var(ref u) = ty {
            if u == self {
                return Ok(Subst::new());
            }
        }

        // The occurs check prevents illegal recursive types.
        if ty.ftv().contains(self) {
            return Err(());
            // return Err(format!("occur check fails: {:?} vs {:?}", self, ty));
        }

        let mut s = Subst::new();
        s.insert(self.clone(), ty.clone());
        Ok(s)
    }
}

impl Type {
    fn mgu(&self, other: &Type, span: Span, reporter: &mut Reporter) -> InferResult<Subst> {
        match (self, other) {
            (&Type::Nil, &Type::Nil)
            | (&Type::Int, &Type::Int)
            | (&Type::Bool, &Type::Bool)
            | (&Type::String, &Type::String)
            | (&Type::Char, &Type::Char) => Ok(Subst::new()),
            (&Type::Nil, &Type::Struct(_, _)) => Ok(Subst::new()),
            (&Type::Struct(_, _), &Type::Nil) => Ok(Subst::new()),
            (&Type::Var(ref v), t) => v.bind(t),
            (t, &Type::Var(ref v)) => v.bind(t),
            (&Type::Unique(ref ty1, ref unique1), &Type::Unique(ref ty2, ref unique2)) => {
                if unique1 != unique2 {
                    reporter.error(
                        format!("types do not unify: {:?} vs {:?}", self, other),
                        span,
                    );
                    return Err(());
                }
                ty1.mgu(ty2, span, reporter)
            }
            (&Type::Fun(ref t1p, ref t1r), &Type::Fun(ref t2p, ref t2r)) => {
                if t1p.len() != t2p.len() {
                    reporter.error(
                        format!("types do not unify: {:?} vs {:?}", self, other),
                        span,
                    );
                    return Err(());
                }

                for (a, b) in t1p.iter().zip(t2p.iter()) {
                    a.mgu(b, span, reporter)?;
                }

                t1r.mgu(t2r, span, reporter)
            }
            (t1, t2) => {
                reporter.error(format!("types do not unify: {:?} vs {:?}", t1, t2), span);
                Err(())
            }
        }
    }
}

impl Types for TypeEnv {
    fn ftv(&self) -> HashSet<TypeVar> {
        self.values()
            .map(|x| x.clone())
            .collect::<Vec<Scheme>>()
            .ftv()
    }
    fn apply(&self, subst: &Subst) -> TypeEnv {
        TypeEnv(
            self.iter()
                .map(|(k, v)| (k.clone(), v.apply(subst)))
                .collect(),
            self.1.clone(),
            self.2.clone(),
        )
    }
}

use std::rc::Rc;
use util::symbol::FactoryMap;

impl GetIdent for Table<Ident, Type> {
    fn ident(&mut self, name: &str) -> Ident {
        for (key, value) in self.strings.mappings.borrow().iter() {
            if value == name {
                return *key;
            }
        }
        let symbol = Ident(*self.strings.next.borrow());
        self.strings
            .mappings
            .borrow_mut()
            .insert(symbol, name.to_owned());
        *self.strings.next.borrow_mut() += 1;
        symbol
    }
}
impl Env {
    pub fn new(strings: &Rc<FactoryMap<Ident>>) -> Self {
        let mut tyenv = Table::new(strings.clone());
        let string_ident = tyenv.ident("str");
        let int_ident = tyenv.ident("int");
        let nil_ident = tyenv.ident("nil");
        let bool_ident = tyenv.ident("bool");

        tyenv.enter(int_ident, Type::Int);
        tyenv.enter(bool_ident, Type::Bool);
        tyenv.enter(nil_ident, Type::Nil);
        tyenv.enter(string_ident, Type::String);

        Env {
            tyenv,
            env: Table::new(Rc::clone(&strings)),
        }
    }

    pub fn look_scheme(&mut self, ident: Ident) -> Option<&Scheme> {
        self.env.look(ident)
    }

    pub fn begin_scope(&mut self) {
        self.env.begin_scope();
        self.tyenv.begin_scope();
    }

    pub fn end_scope(&mut self) {
        self.env.end_scope();
        self.tyenv.end_scope();
    }

    pub fn add_type(&mut self, ident: Ident, data: Type) {
        self.tyenv.enter(ident, data);
    }

    pub fn add_var(&mut self, ident: Ident, data: Scheme) {
        self.env.enter(ident, data);
    }

    pub fn look_type(&mut self, ident: Ident) -> Option<&Type> {
        self.tyenv.look(ident)
    }

    pub fn name(&self, ident: Ident) -> String {
        self.tyenv.name(ident)
    }

    fn ftv(&self) -> HashSet<TypeVar> {
      self.env.table.values().map(|x| x.last().unwrap().clone()).collect::<Vec<Scheme>>().ftv()
    }
}

impl Deref for TypeEnv {
    type Target = HashMap<Ident, Scheme>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for TypeEnv {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


impl Infer {
    pub fn new(reporter: Reporter) -> Self {
        Infer {
            reporter,
            gen: TypeVarGen::new(),
        }
    }

    pub fn infer(&mut self, program: Program, env: &mut Env) -> InferResult<()> {
        for alias in &program.type_alias {
            self.type_alias(alias, env)?
        }

        for function in &program.functions {
            self.function(function, env)?
        }

        Ok(())
    }

    

    fn generalize(&self, ty: &Type,env:&mut Env) -> Scheme {
        Scheme {
            vars: ty.ftv().difference(&env.ftv()).cloned().collect(),
            ty: ty.clone(),
        }
    }

    

    fn trans_ty(&mut self, ty: &Spanned<astType>, env: &mut Env) -> InferResult<Type> {
        match ty.value {
            astType::Bool => Ok(Type::Bool),
            astType::Str => Ok(Type::String),
            astType::Nil => Ok(Type::Nil),
            astType::U8
            | astType::I8
            | astType::U32
            | astType::I32
            | astType::U64
            | astType::I64 => Ok(Type::Int),
            astType::Simple(ref ident) => {
                if let Some(ty) = env.look_type(ident.value).cloned() {
                    Ok(ty)
                } else if let Some(ty) = env.look_scheme(ident.value).cloned() {
                    Ok(ty.instantiate(&mut self.gen))
                } else {
                    let msg = format!("Undefined Type '{}'", env.name(ident.value));
                    self.error(msg, ident.span);
                    Err(())
                }
            }
            astType::Poly(ref ident, ref types) => {
                //Concrete generics i.e List<i32>. List<bool>
                let ty = if let Some(ty) = env.look_scheme(ident.value).cloned() {
                    ty.clone()
                } else {
                    let msg = format!("Undefined Type '{}'", env.name(ident.value));
                    self.error(msg, ident.span);
                    return Err(());
                };

                let mut ty = ty.instantiate(&mut self.gen);

                match ty {
                    Type::Fun(ref mut paramty, _) => for ty in types {
                        paramty.push(self.trans_ty(ty, env)?)
                    },
                    _ => unreachable!(),
                }

                Ok(ty)
            }
        }
    }

    fn type_alias(&mut self, alias: &Spanned<TyAlias>, env: &mut Env) -> InferResult<()> {
        let ty = self.trans_ty(&alias.value.ty, env)?;

        env.add_var(alias.value.alias.value, Scheme::new(ty));

        Ok(())
    }

    fn function(&mut self, function: &Spanned<Function>, env: &mut Env) -> InferResult<()> {
        let returns = if let Some(ref return_ty) = function.value.returns {
            self.trans_ty(return_ty, env)?
        } else {
            Type::Nil
        };

        let mut scheme = Scheme::new(returns.clone());

        let mut ftys = Vec::new();

        for tv in &function.value.name.value.type_params {
            scheme.vars.push(self.gen.next());
        }

        env.add_var(function.value.name.value.name.value, scheme);

        env.begin_scope();

        for paramdef in &function.value.params.value {
            ftys.push(self.trans_ty(&paramdef.value.ty, env)?);

            let ty =self.trans_ty(&paramdef.value.ty, env)?;
            env.add_var(
                paramdef.value.name.value,
                Scheme::new(ty),
            )
        }

        env.add_type(
            function.value.name.value.name.value,
            Type::Fun(ftys, Box::new(returns.clone())),
        );

        let body = self.statement(&function.value.body,env)?;

        body.mgu(&returns, function.value.body.span, &mut self.reporter)?;

        Ok(())
    }

    fn statement(&mut self, statement: &Spanned<Statement>,env:&mut Env) -> InferResult<Type> {
       match statement.value {
            Statement::Block(ref statements) => {
                let mut result = Type::Nil;

                for statement in statements {
                    result = self.statement(statement,env)?
                }

                Ok(result)
            }
            Statement::Break | Statement::Continue => Ok(Type::Nil),
            Statement::Expr(ref expr) => self.expr(expr,env),
            Statement::For {
                ref init,
                ref cond,
                ref incr,
                ref body,
            } => {
                if init.is_none() && cond.is_none() && incr.is_none() {
                    let body = self.statement(body,env)?;

                    return Ok(body);
                }

                if let Some(ref init) = *init {
                    self.statement(init,env)?;
                }

                if let Some(ref incr) = *incr {
                    let ty = self.expr(incr,env)?;

                    Type::Int.mgu(&ty, incr.span, &mut self.reporter)?;
                }

                if let Some(ref cond) = *cond {
                    let ty = self.expr(cond,env)?;

                    Type::Bool.mgu(&ty, cond.span, &mut self.reporter)?;
                }

                let body = self.statement(body,env)?;

                Ok(body)
            }

            Statement::If {
                ref cond,
                ref then,
                ref otherwise,
            } => {
                Type::Bool.mgu(&self.expr(cond,env)?, cond.span, &mut self.reporter)?;

                let then_ty = self.statement(then,env)?;

                if let Some(ref otherwise) = *otherwise {
                    then_ty.mgu(&self.statement(otherwise,env)?, otherwise.span,  &mut self.reporter)?;
                    Ok(then_ty)
                } else {
                    Ok(then_ty)
                }
            }

            Statement::Let {
                ref ident,
                ref ty,
                ref expr,
            } => {
                if let Some(ref expr) = *expr {
                    let expr_ty = self.expr(expr,env)?;

                    if let Some(ref ty) = *ty {
                        let t = self.trans_ty(ty, env)?;

                        expr_ty.mgu(&t, ty.span, &mut self.reporter)?;

                        let scheme = self.generalize(&t,env);

                        env.add_var(ident.value, scheme);

                        return Ok(t);
                    }

                    let scheme = self.generalize(&expr_ty,env);

                    env.add_var(ident.value, scheme);

                    Ok(Type::Nil)
                } else {
                    if let Some(ref ty) = *ty {
                        let ty = self.trans_ty(ty, env)?;

                        let scheme = self.generalize(&ty,env);

                        env.add_var(ident.value, scheme);
                        return Ok(ty);
                    }

                    Ok(Type::Nil)
                }
            }

            Statement::Return(ref expr) => self.expr(expr,env),
            Statement::While { ref cond, ref body } => {
                Type::Bool.mgu(&self.expr(cond,env)?, cond.span, &mut self.reporter)?;

                self.statement(body,env)?;

                Ok(Type::Nil)
            }
        }
    }

    fn expr(&mut self, expr: &Spanned<Expression>,env:&mut Env) -> InferResult<Type> {
         match expr.value {
            Expression::Assign {
                ref name,
                ref value,
            } => unimplemented!(),
            Expression::Binary {
                ref lhs,
                ref op,
                ref rhs,
            } => {
                let lhs = self.expr(lhs,env)?;
                let rhs = self.expr(rhs,env)?;

                match op.value {
                    Op::NEq | Op::Equal => Ok(Type::Bool),
                    Op::LT | Op::LTE | Op::GT | Op::GTE => {
                        lhs.mgu(&rhs, expr.span, &mut self.reporter)?;
                        Ok(Type::Bool)
                    }

                    Op::Plus | Op::Slash | Op::Star | Op::Minus => {
                        if let Err(_) = Type::Int.mgu(&lhs, expr.span, &mut self.reporter) {
                            Type::String.mgu(&lhs, expr.span, &mut self.reporter)?;
                        }

                        lhs.mgu(&rhs, expr.span, &mut self.reporter)?;
                        Ok(lhs)
                    }

                    Op::And | Op::Or => {
                        lhs.mgu(&rhs, expr.span, &mut self.reporter)?;
                        Ok(Type::Bool)
                    }
                }
            }

            Expression::Cast { ref expr, ref to } => unimplemented!(),
            Expression::Call {
                ref callee,
                ref args,
            } => unimplemented!(),
            Expression::Grouping { ref expr } => self.expr(expr,env),
            Expression::Literal(ref literal) => match *literal {
                Literal::Char(_) => Ok(Type::Char),
                Literal::False(_) => Ok(Type::Bool),
                Literal::True(_) => Ok(Type::Bool),
                Literal::Str(_) => Ok(Type::String),
                Literal::Number(_) => Ok(Type::Int),
                Literal::Nil => Ok(Type::Nil),
            },
            Expression::StructLiteral {
                ref ident,
                ref fields,
            } => unimplemented!(),
            Expression::Unary { ref op, ref expr } => unimplemented!(),
            Expression::Var(ref var) => {
                //    match self.0.get(var) {
                //        Some(s) =>s.instantiate()
                //    }

                unimplemented!()
            }
        }
    }

    fn error<T: Into<String>>(&mut self, msg: T, span: Span) {
        self.reporter.error(msg, span)
    }
}