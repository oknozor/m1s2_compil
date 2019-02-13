#![feature(box_patterns)]
#![feature(box_syntax)]
extern crate serde;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::env;

use crate::ast::node::RootNode;
use crate::scope::scope::Scope;
use crate::ast::statement::Statement;

pub mod ast;
pub mod token;
pub mod scope;

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

    println!("I am Root");
    println!("And i have {} childs", scope.childs.len());
    scope.childs.iter().for_each(|ch| {
        println!("{:?}:{:?}", ch.name, ch.scope_type);
    });
    println!("______________________________________");


    tail(&scope, 0);
}

fn tail(scope: &Scope, depth: i32) -> () {
    scope.childs.iter().for_each(| ch| {
        for n in 0..depth {
            print!("| \t\t");
        }
        let depth = depth + 1;
        println!(" {:?}, {:?} -> {:?}" , ch.name, ch.scope_type, ch.token_stream);
        tail( ch, depth);
    })
}




