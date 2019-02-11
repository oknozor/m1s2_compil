use crate::token::binary_operator::BinaryOp;
use crate::token::token::Token;
use crate::ast::statement::Statement;
use crate::runner::handler::Handler;
use crate::ast::statement::BlockStatement;
use std::any::Any;
use std::rc::Rc;
use std::collections::HashMap;
use crate::ast::expression::Expression;
use crate::ast::expression::TokenStream;
use crate::ast::expression::ToToken;

#[derive(Debug)]
pub struct Scope {
    pub name: Option<Rc<String>>,
    pub parents: Vec<Rc<String>>,
    pub childs: HashMap<String, Scope>,
    pub token_stream: TokenStream,
}

impl Scope {
    // Create a new named scope and increase the reference conter on all ancestors
    pub fn new_named(name: String, parent: Option<&Scope>) -> Self {
        let mut parents;
        let parent_name: String;
        if let Some(parent) = parent {
            // Cloning the entire vec produce a new vec with updated reference count
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
            token_stream: vec![],
        }
    }

    // Used only once to init the root scope
    pub fn init_root(parent: Option<Scope>) -> Self {
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
            parents, // No parent, like Batman
            childs: HashMap::new(),
            token_stream: vec![],
        }
    }

    // Explore the AST to generate new scopes
    pub fn build(&mut self, statements: &Vec<Box<Statement>>) {
        statements.iter().for_each(|statement| {
            self.explore_statement(statement)
        });
    }

    // 1. Extract named reference into scope child
    // 2. Tokenize expression on the fly
    fn explore_statement(&mut self, statement: &Box<Statement>) -> () {
        if let Some(named_scope) = &self.name {
            println!("my name is {:?} currently exploring a new statement", named_scope);
            println!(" my ref count is {}", Rc::strong_count(named_scope));
            println!("and i have {} childs", self.childs.len());
            println!("i contain {:?}", self.token_stream);
        }

        // Todo make name condition inner so we can handle error on unkown statement
        if let Some(name) = statement.get_name().clone() {
            match statement {
                box Statement::FunctionDeclaration(f) =>
                    {
                        let mut function_scope = Scope::new_named(name, Some(self));
                        let body = f.body.clone();
                        let body = body.body;
                        function_scope.build(&body);
                        let name = f.id.name.clone();
                        self.childs.insert(name, function_scope);
                    }
                box Statement::VariableDeclarator(f) =>
                    {
                        let mut function_scope = Scope::new_named(name, Some(self));
                        let body = vec![box Statement::EmptyStatement];
                        function_scope.build(&body);
                        let name = f.id.name.clone();
                        self.childs.insert(name, function_scope);
                    }
                _ => ()
            };
        } else {
            match statement {
                box Statement::VariableDeclaration(v) => self.build(&v.declarations),
                box Statement::BlockStatement(block) => self.build(&block.body),
                box Statement::ExpressionStatement(exp_stmt) => self.tokenize(&exp_stmt.expression),
                box Statement::SwitchStatement(switch) => self.tokenize(&switch.discriminant),
                box Statement::SwitchCase(case) =>
                    {
                        if let Some(test) = &case.test { self.tokenize(test); }
                        self.build(&case.consequent);
                    }
                box Statement::IfStatement(if_stmt) =>
                    {
                        self.tokenize(&if_stmt.test);
                        self.explore_statement(&if_stmt.consequent);
                        if let Some(alt) = &if_stmt.alternate {
                            self.explore_statement(alt);
                        };
                    }
                box Statement::ForStatement(for_stmt) =>
                    {
                        if let Some(init) = &for_stmt.init { self.tokenize(init) }
                        if let Some(test) = &for_stmt.test { self.tokenize(test) }
                        if let Some(update) = &for_stmt.update { self.tokenize(update) }
                        self.explore_statement(&for_stmt.body);
                    }
                box Statement::WhileStatement(while_stmt) =>
                    {
                        self.tokenize(&while_stmt.test);
                        self.explore_statement(&while_stmt.body);
                    }

                // Todo : need to think on this
                box Statement::ContinueStatement(cn) => (),
                box Statement::BreakStatement(cn) => (),
                box Statement::ReturnStatement(cn) => (),
                _ => ()
            };
        }
    }

    // Dispatch expression arrays
    fn explore_expression(&mut self, expression: &Vec<Box<Expression>>) {}

    // Push expressions as token on the scope token_stream
    fn tokenize(&mut self, expression: &Box<Expression>) {
        self.token_stream.extend(expression.to_token());
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


