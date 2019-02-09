#![feature(box_patterns)]
#![feature(box_syntax)]
extern crate serde;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::env;

use crate::ast::node::RootNode;
use crate::ast::statement::Statement;
use crate::runner::scope::Scope;
use crate::interpretter::Interpreter;
use crate::runner::visitor::Visitor;

pub mod ast;
pub mod token;
pub mod runner;
pub mod interpretter;

fn main() {
    let args: Vec<_> = env::args().collect();
    let json_estree =
        if args.len() > 1 {
            &args[1]
        } else {
            panic!("Usage parse {file name}");
        };

    let root_node: RootNode = ast::node::deserialize_json(json_estree);
    let program_root = root_node.get_program_root();
    let program_root = program_root.expect("Error parsing Json AST");

    let mut interpretter = Interpreter {
        scope: Scope::new(Statement::Root(program_root)),
    };

    interpretter.visit_root();
}




#[test]
fn root_node_shall_have_some_childs() {
    let json_estree = "/home/okno/LILLE1/S2/COMPIL/pretty_printer/exemples/01-expressions.js";
    let root_node: RootNode = ast::node::deserialize_json(json_estree);
    let program_root = root_node.get_program_root();
    let program_root = program_root.expect("Error parsing Json AST");

    let interpretter = Interpreter {
        scope: Scope::new(Statement::Root(program_root)),
    };

    let child_size = interpretter.scope.children.borrow_mut().len();

    assert_eq!(12, child_size);


}




