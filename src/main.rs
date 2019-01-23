extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use crate::ast::node::Node;
use crate::ast::visitor::Visitor;
use std::env;

pub mod ast;
pub mod pretty_printer;

fn main() {
    let args: Vec<_> = env::args().collect();
    let json_estree =
        if args.len() > 1 {
            &args[1]
        } else {
            panic!("Usage parse {file name}");
        };

    let root_node: Node = ast::node::deserialize_json(json_estree);
    let mut pretty_printer = pretty_printer::PrettyPrinter {
        out: &mut String::new(),
    };
    pretty_printer.visit_node(&root_node);

    println!("{}", pretty_printer.out);
}

