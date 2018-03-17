extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate underscore_semant;
extern crate underscore_syntax;
extern crate underscore_util;

use underscore_util::emitter::Reporter;
use underscore_util::symbol::{FactoryMap, Table};
use underscore_syntax::lexer::Lexer;
use underscore_syntax::parser::Parser;
use std::rc::Rc;
use structopt::StructOpt;
use std::io::{self, Write};
use underscore_semant::{Infer, TypeEnv};
use underscore_syntax::ast::Ident;
use underscore_syntax::ast::{ItemName, Ty};
use underscore_util::pos::{Position, Span, Spanned, EMPTYSPAN};

// use underscore_semant::TypeEnv;

fn main() {
    let opts = Cli::from_args();

    if let Some(file) = opts.source {
        run(file, opts.file);
    } else {
        repl()
    }
}

fn repl() {
    loop {
        let _ = io::stdout().write(b"underscore>> ");
        let _ = io::stdout().flush();
        let reporter = Reporter::new();
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read input");

        let tokens = Lexer::new(&input, reporter.clone()).lex();

        let strings = Rc::new(FactoryMap::new());

        let mut table = Table::new(Rc::clone(&strings));

        let mut parser = Parser::new(tokens, reporter.clone(), &mut table);

        match parser.parse() {
            Ok(p) => println!("{:#?}", p),
            Err(_) => reporter.emit(&input),
        };
    }
}

fn run(path: String, dump_file: Option<String>) {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path).expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let input = contents.trim();

    if contents.is_empty() {
        ::std::process::exit(0)
    }

    let reporter = Reporter::new();

    let tokens = Lexer::new(&input, reporter.clone()).lex();

    let strings = Rc::new(FactoryMap::new());

    let mut table = Table::new(Rc::clone(&strings));

    let mut parser = Parser::new(tokens, reporter.clone(), &mut table);

    let ast = match parser.parse() {
        Ok(mut ast) => {
            if dump_file.is_some() {
                let mut file = File::create(dump_file.unwrap()).expect("Couldn't create file");
                file.write(ast.fmt().as_bytes())
                    .expect("Couldn't write to the file");
            }
            ast
        }
        Err(_) => {
            reporter.emit(&input);
            ::std::process::exit(65)
        }
    };

    let mut infer = Infer::new(reporter.clone());

    let mut type_env = TypeEnv::new(&Rc::clone(&strings));

    match infer.infer(ast, &mut type_env) {
        Ok(_) => (),
        Err(_) => {
            reporter.emit(&input);
            ::std::process::exit(65)
        }
    }

    // let mut env = TypeEnv::new(reporter.clone());

    // match env.ti(&ast) {
    //     Ok(()) => (),
    //     Err(_) => ::std::process::exit(65),
    // };
}

#[derive(StructOpt, Debug)]
#[structopt(name = "underscore")]
pub struct Cli {
    /// The source code file
    pub source: Option<String>,
    /// Dump the ast to a give file
    #[structopt(short = "d", long = "dump")]
    pub file: Option<String>,
}
