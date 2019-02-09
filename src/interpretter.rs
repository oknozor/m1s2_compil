/*
use crate::ast::statement::*;
use crate::runner::handler::Handler;
use crate::ast::expression::CallExpression;
use crate::runner::scope::Scope;

pub struct Interpreter<'ast> {
    pub root: Vec<Scope<'ast, Statement>>,
}

impl<'ast> Handler for Interpreter<'ast> {
    fn handle_function_declaration(&mut self, func: &FunctionDeclaration) {}

    fn handle_call_expression(&mut self, func: &CallExpression) {}

    fn handle_identifier(&mut self, id: &String) {
        *//*    let current_childs = self.root.children.borrow_mut();
            let calle = &current_childs.get(id.as_str());
            match calle {
                Some(s) => println!("found a statment for {}", id),
                None => eprintln!("{} not found in this scope", id)
            };*//*
    }
}

impl<'ast> Interpreter<'ast> {
    pub fn print_scopes(&self) {
        self.root.iter().for_each(|stmt| {
            println!("{:?}", stmt);
        })
    }
}*/
