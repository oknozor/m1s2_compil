use crate::token::binary_operator::BinaryOp;
use crate::token::token::Token;
use crate::ast::statement::Statement;
use crate::runner::handler::Handler;
use crate::ast::statement::BlockStatement;
use std::any::Any;
use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Scope {
    pub name: Option<Rc<String>>,
    pub parents: Vec<Rc<String>>,
    pub childs: HashMap<String, Scope>,
    pub content: Vec<Box<Statement>>,
}

impl Scope {

    pub fn new_named(name: String, parent: Option<&Scope>) -> Self {
        let mut parents;
        let parent_name: String;
        if let Some(parent) = parent {
            parents = parent.parents.clone();

            if let Some(parent_name) = &parent.name {
                parents.push(Rc::clone(&parent_name));
            }
        } else {
            parents = vec![];
        }
        let name = Rc::new(name);

        Scope {
            name: Some(name),
            parents,
            childs: HashMap::new(),
            content: vec![],
        }
    }

    pub fn new_anon(parent: Option<Scope>) -> Self {
        let mut parents;
        let parent_name: String;
        if let Some(parent) = parent {
            parents = parent.parents.clone();

            if let Some(parent_name) = parent.name {
                parents.push(Rc::clone(&parent_name));
            }
        } else {
            parents = vec![];
        }

        Scope {
            name: None,
            parents,
            childs: HashMap::new(),
            content: vec![],
        }
    }

    pub fn init_root(&mut self, statements: Vec<Box<Statement>>) {
        println!("my name is {:?}", self.name);
        statements.iter().for_each(|statement| {
            if let Some(name) = statement.get_name().clone() {
                match statement {
                    box Statement::FunctionDeclaration(f) => {
                        let mut function_scope = Scope::new_named(name, Some(self));
                        let body = f.body.clone();
                        let body = body.body;
                        function_scope.init_root(body);
                        let name = f.id.name.clone();
                        self.childs.insert(name, function_scope.clone());
                    }
                    box Statement::VariableDeclarator(f) => {
                        let mut function_scope = Scope::new_named(name, Some(self));
                        let body = vec![box Statement::EmptyStatement];
                        function_scope.init_root(body);
                        let name = f.id.name.clone();
                        self.childs.insert(name, function_scope.clone());
                    }
                    _ => ()
                }
            } else {
                match statement {
                    box Statement::VariableDeclaration(v) => {
                        self.init_root(v.declarations.clone())
                    }
                    _ => {
                        self.content.push(statement.to_owned())
                    }
                }
            }
        });
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


