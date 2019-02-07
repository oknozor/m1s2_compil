#![feature(box_patterns)]
#![feature(box_syntax)]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::collections::HashMap;
use std::env;

use crate::ast::node::RootNode;
use crate::interpretter::Interpreter;
use crate::interpretter::Stack;
use crate::runner::visitor::Visitor;

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

    let interpretter = &mut Interpreter {
        global_scope: HashMap::new(),
        local_scope: HashMap::new(),
        stack: Stack::new(),
    };

    interpretter.visit_node(&root_node);


    /*pretty_printer.visit_node(&root_node);
    pretty_printer.print();*/
}

