extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate underscore_semant;
extern crate underscore_syntax;
extern crate underscore_util;
extern crate underscore_x86;
extern crate underscore_ir;
#[macro_use]
extern crate underscore_vm;

use std::io::{self, Write};
use std::rc::Rc;
use structopt::StructOpt;
use underscore_semant::{Codegen, Infer, TypeEnv,build_program};
use underscore_syntax::lexer::Lexer;
use underscore_syntax::parser::Parser;
use underscore_util::emitter::Reporter;
use underscore_util::symbol::{SymbolMap, Symbols};
use underscore_vm::{Chunk, VM};
use underscore_ir::printer::Printer;

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

        let tokens = match Lexer::new(&input, reporter.clone()).lex() {
            Ok(tokens) => tokens,
            Err(_) => {
                reporter.emit(&input);
                ::std::process::exit(65)
            }
        };

        let strings = Rc::new(SymbolMap::new());

        let mut table = Symbols::new(Rc::clone(&strings));

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

    let mut file = File::open(path.clone()).expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let input = contents.trim();

    if contents.is_empty() {
        ::std::process::exit(0)
    }

    let mut reporter = Reporter::new();

    let tokens = match Lexer::new(&input, reporter.clone()).lex() {
        Ok(tokens) => tokens,
        Err(_) => {
            reporter.emit(&input);
            ::std::process::exit(65)
        }
    };

    let strings = Rc::new(SymbolMap::new());

    let mut table = Symbols::new(Rc::clone(&strings));

    let mut parser = Parser::new(tokens, reporter.clone(), &mut table);

    let mut ast = match parser.parse() {
        Ok(mut ast) => {
            if dump_file.is_some() {
                let mut file =
                    File::create(dump_file.clone().unwrap()).expect("Couldn't create file");
                file.write(format!("../{:#?}", ast).as_bytes())
                    .expect("Couldn't write to the file");
            }
            ast
        }
        Err(_) => {
            reporter.emit(&input);
            ::std::process::exit(65)
        }
    };

    let mut infer = Infer::new();

    let mut type_env = TypeEnv::new(&Rc::clone(&strings));

    let symbols = Symbols::new(Rc::clone(&strings));

    let ast = match infer.infer(&mut ast, &mut type_env, &mut reporter) {
        Ok(ast) => {
            if dump_file.is_some() {
                let mut file =
                    File::create(dump_file.clone().unwrap()).expect("Couldn't create file");
                file.write(format!("../{:#?}.", ast).as_bytes())
                    .expect("Couldn't write to the file");
            }

            ast
        }
        Err(_) => {
            reporter.emit(&input);
            ::std::process::exit(65)
        }
    };


    let lowered = build_program(&symbols,ast);
    let s2 = Symbols::new(Rc::clone(&strings));
    let mut printer = Printer::new(&s2);

    let mut file = File::create("lowered.ir").expect("Couldn't create file");
    file.write(&printer.print_program(&lowered).unwrap())
        .expect("Couldn't write to the file");
}

#[derive(StructOpt, Debug)]
#[structopt(name = "underscore")]
pub struct Cli {
    /// The source code file
    pub source: Option<String>,
    /// Dump the ast to a give file
    #[structopt(short = "d", long = "dump")]
    pub file: Option<String>,
    #[structopt(short = "ir", long = "emit-ir")]
    pub emit_ir: bool,
}
