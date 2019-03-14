use crate::ast::statement::Statement;
use std::collections::HashMap;
use crate::token::token::Node;
use crate::token::token::Token;
use crate::ast::statement::Statement::*;
use crate::ast::expression::Expression::*;
use crate::visitor::Visitor;

pub struct Interpreter {
    pub functions: HashMap<String, Vec<Node>>,
    pub vars: HashMap<String, Vec<Token>>,
    pub main: Vec<Token>,
}

impl Interpreter {
    pub fn run(ast: Vec<Box<Statement>>) -> Interpreter {
        let mut ast_mut = ast.clone();
        let mut token_root = Interpreter {
            functions: HashMap::new(),
            vars: HashMap::new(),
            main: vec![],
        };

        let mut indexes_to_remove = vec![];
        ast_mut.iter_mut().enumerate().for_each(|(i, statement)| {
            if let box VariableDeclaration(var) = statement {
                token_root.visit_variable_declaration(var);
                indexes_to_remove.push(i);
            };
        });

        indexes_to_remove.iter().for_each(|i| { ast_mut.remove(*i); });
        let mut indexes_to_remove = vec![];

        ast.iter().enumerate().for_each(|(i, statement)| {
            if let box FunctionDeclaration(function) = statement {
                token_root.visit_function_declaration(function);
                indexes_to_remove.push(i);
            };
        });

        indexes_to_remove.iter().for_each(|i| { ast_mut.remove(*i); });

        ast.iter().enumerate().for_each(|(i, statement)| {
            token_root.visit_statement(statement);
        });

        token_root
    }
}