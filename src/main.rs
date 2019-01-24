extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use crate::ast::node::RootNode;
use std::env;
use crate::runner::visitor::Visitor;
use crate::interpretter::Interpretter;

pub mod ast;
pub mod runner;
pub mod interpretter;
pub mod pretty_printer;


fn main() {
    let args: Vec<_> = env::args().collect();
    let json_estree =
        if args.len() > 1 {
            &args[1]
        } else {
            panic!("Usage parse {file name}");
        };

    let root_node: RootNode = ast::node::deserialize_json(json_estree);
    let mut pretty_printer = pretty_printer::PrettyPrinter {
        out: &mut String::new(),
    };
    let interpretter = Interpretter;
    pretty_printer.visit_node(&root_node);
    pretty_printer.print();
}

