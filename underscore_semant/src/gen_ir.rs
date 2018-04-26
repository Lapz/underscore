use codegen::{ir::*, temp::{new_label, new_named_label, Temp}};
use std::mem;
use ast;
use util::symbol::Symbols;
use syntax::ast::{Literal, Op, Sign, Size, UnaryOp};
use types::{TyCon, Type};
use std::u64;
#[derive(Debug)]
pub struct Codegen {
    instructions: Vec<Instruction>,
    symbols: Symbols<Temp>,
}

impl Codegen {
    pub fn new(symbols: Symbols<Temp>) -> Self {
        Self {
            symbols,
            instructions: vec![],
        }
    }

    pub fn dump_to_file(&mut self, path: String) {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(path).expect("Couldn't create file");

        for instruction in &self.instructions {
            file.write(instruction.fmt(&mut self.symbols).as_bytes())
                .expect("Couldn't write to the file");
        }

        file.write(format!("\n{:?}", self.instructions).as_bytes())
            .expect("Couldn't write to the file");
    }

    pub fn gen_program(&mut self, program: &ast::Program) {
        for function in &program.functions {
            self.gen_statement(&function.body)
        }
    }

    fn gen_statement(&mut self, statement: &ast::Statement) {
        match *statement {
            ast::Statement::Block(ref statements) => for statement in statements {
                self.gen_statement(statement)
            },
            ast::Statement::Expr(ref expr) => self.gen_expression(expr, Temp::new()),
            _ => unimplemented!(),
        }
    }

    fn gen_expression(&mut self, expr: &ast::TypedExpression, temp: Temp) {
        match *expr.expr {
            ast::Expression::Assign(ref name, ref value) => {
                //  let temp = self.symbols.look(symbol)
            }
            ast::Expression::Binary(ref lhs, ref op, ref rhs) => {
                let lhs_temp = Temp::new();

                let rhs_temp = Temp::new();

             

                match *op {
                    Op::And => (),
                    Op::Or => {
                        self.gen_expression(lhs, lhs_temp);
                        let end = new_named_label("end", &mut self.symbols);

                        self.instructions.push(Instruction::TJump(lhs_temp, end));
                        self.gen_expression(rhs, rhs_temp);

                        self.instructions.push(Instruction::Label(end));
                    }
                    rest => {
                        self.gen_expression(lhs, lhs_temp);
                        self.gen_expression(rhs, rhs_temp);
                        let op = gen_bin_op(&rest);
                        self.instructions
                            .push(Instruction::BinOp(op, lhs_temp, rhs_temp, temp));
                    }
                }
            }

            ast::Expression::Call(ref name, ref exprs) => {
                let label = new_label(*name, &mut self.symbols);

                let mut params = vec![];

                for expr in exprs {
                    let temp = Temp::new();
                    self.gen_expression(expr, temp);
                    params.push(temp)
                }

                self.instructions
                    .push(Instruction::Call(temp, label, params))
            }

            ast::Expression::Cast(ref from, ref to) => {
                let temp = Temp::new();
                self.gen_expression(from, temp);

                match expr.ty {
                    Type::App(TyCon::Int(sign, size), _) => {
                        self.instructions.push(Instruction::Cast(temp, sign, size))
                    }

                    _ => panic!("Can only cast to ints"),
                }
            }
            ast::Expression::Grouping { ref expr } => self.gen_expression(expr, temp),
            ast::Expression::Literal(ref literal) => {
                let value = match *literal {
                    Literal::Char(ref ch) => Value::Const(*ch as u64, Sign::Unsigned, Size::Bit8),

                    Literal::True(ref b) | Literal::False(ref b) => {
                        Value::Const(*b as u64, Sign::Unsigned, Size::Bit8)
                    }

                    Literal::Nil => Value::Mem(vec![]),

                    Literal::Number(ref number) => match number.ty {
                        Some((sign, size)) => Value::Const(number.value, sign, size),
                        None => match expr.ty {
                            Type::App(TyCon::Int(sign, size), _) => {
                                Value::Const(number.value, sign, size)
                            }
                            Type::Var(_) => Value::Const(number.value, Sign::Signed, Size::Bit32),
                            _ => unreachable!(),
                        },
                    },
                    Literal::Str(ref string) => {
                        let mut bytes = vec![];
                        bytes.push(string.len() as u8);
                        bytes.extend(string.as_bytes());

                        Value::Mem(bytes)
                    }
                };

                self.instructions.push(Instruction::Store(temp, value))
            }

            ast::Expression::Unary(ref op, ref expr) => {
                let new_temp = Temp::new();
                self.gen_expression(expr, new_temp);
                let op = gen_un_op(op);

                self.instructions
                    .push(Instruction::UnOp(op, temp, new_temp))
            }

            ast::Expression::Var(ref var, _) => {
                let var = self.symbols.look(*var).unwrap().clone();
                self.instructions.push(Instruction::Copy(temp, var))
            }

            _ => unimplemented!(),
        }
    }
}

fn gen_bin_op(op: &Op) -> BinOp {
    match *op {
        Op::Plus => BinOp::Plus,
        Op::Minus => BinOp::Minus,
        Op::Star => BinOp::Minus,
        Op::Slash => BinOp::Div,
        Op::And => BinOp::And,
        Op::Or => BinOp::Or,
        _ => unreachable!(),
    }
}

fn gen_un_op(op: &UnaryOp) -> UnOp {
    match *op {
        UnaryOp::Minus => UnOp::Minus,
        UnaryOp::Bang => UnOp::Bang,
    }
}
