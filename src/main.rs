#![feature(box_patterns)]
#![feature(box_syntax)]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::process::Command;
use std::rc::Rc;

use crate::ast::statement::RootNode;
use crate::ast::statement::Statement;
use crate::pretty_printer::PrettyPrinter;

pub mod ast;
pub mod pretty_printer;
pub mod to_c;
pub mod file_writer;

fn main() {
    let args: Vec<_> = env::args().collect();
    let json_estree = if args.len() > 1 {
        &args[1]
    } else {
        panic!("Usage parse {file name}");
    };

    let root_node: RootNode = ast::file_util::deserialize_json(json_estree);
    let program_root = root_node.get_program_root();
    let program_root = program_root.expect("Error parsing Json AST");


    let mut pretty = PrettyPrinter {
        out: &mut "".to_string(),
    };

    pretty.visit_program_root(program_root);

    println!("{}", pretty.out);
    write_to_file(pretty).expect("Error writing file");
    compile();
}

fn write_to_file(pretty: PrettyPrinter) -> Result<(), io::Error> {
    let mut file = File::create("out.c")?;
    file.write_all(pretty.out.as_bytes())?;
    Ok(())
}

fn compile() {
    let mut gcc_cmd = Command::new("gcc");

    gcc_cmd.arg("out.c");
    gcc_cmd.arg("-g");
    gcc_cmd.arg("-Wall");
    gcc_cmd.arg("-o");
    gcc_cmd.arg("out.o");

    gcc_cmd.status().expect("Failed to compile source");
}
