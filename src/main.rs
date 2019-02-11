#![feature(box_patterns)]
#![feature(box_syntax)]
extern crate serde;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::env;

use crate::ast::node::RootNode;
use crate::runner::scope::Scope;
use crate::ast::statement::Statement;

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

    let mut scope = Scope::init_root(None);
    scope.build(&program_root);

    tail(&mut scope)
}

fn tail(scope: &mut Scope) -> () {
    scope.childs.iter_mut().for_each(| ch| {
        println!("{} : {:?}", ch.0, ch.1);
        tail( ch.1)
    })
}




