
use crate::ast::statement::*;
use crate::runner::visitor::Visitor;
use crate::runner::handler::Handler;
use crate::token::token::Token;
use crate::token::binary_operator::BinaryOp;
use crate::runner::scope::Scope;
use std::rc::Rc;
use core::borrow::BorrowMut;
use core::borrow::Borrow;

pub struct Interpreter {
    pub scope: Rc<Scope<Statement>>,
}

impl Visitor for Interpreter {}

impl Handler for Interpreter {

    fn handle_function_declaration(&mut self, func: &FunctionDeclaration) {
        let ref_count = Rc::clone(&self.scope);
        self.scope.add_child(ref_count, Statement::FunctionDeclaration(func.to_owned()));
        println!("{:?}", self.scope.children);
    }
}

impl Interpreter {
    pub fn visit_root(&mut self) {
        match self.scope.get_content().borrow() {
            Statement::Root(statements) => {
                for statement in statements {
                    self.visit_statement(&statement);
                }
            },
            _ => ()
        }
    }
}


pub struct Stack {
    pub queue: Vec<Token>,
    pub operators: Vec<BinaryOp>,
}


impl Stack {
    pub fn new() -> Self {
        Stack {
            queue: vec![],
            operators: vec![],
        }
    }
}
