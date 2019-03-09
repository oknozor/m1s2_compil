use std::fmt::Display;

use crate::ast::expression::*;
use crate::ast::expression::Expression::*;
use crate::ast::statement::*;
use crate::ast::statement::Statement::*;
use crate::ast::statement::Statement;
use crate::c_compile::c_write_utils::*;
use crate::visitor::Visitor;
use crate::c_compile::*;

pub struct CWriter<'printer> {
    pub out: &'printer mut String,
}

impl <'pr> CWriter <'pr> {
    fn with_loc() -> bool {
        true
    }

    pub fn visit_program_root(&mut self, root_nodes: Vec<Box<Statement>>) {
        self.append(INCLUDES);
        let vars_gen = self.visit_global_vars(root_nodes);
        let func_gen = self.visit_function_declarations(vars_gen);
        let main_gen = self.visit_calls(func_gen);
        self.append(NEW_LINE);
        self.append(BRACKET_RIGHT);
    }

    fn visit_calls(&mut self, nodes: Vec<Box<Statement>>) -> Vec<Box<Statement>> {
        self.append(MAIN);
        let mut new_root = nodes.clone();
        nodes.iter().enumerate().for_each(|(i, statement)| {
            self.visit_statement(statement);
        });
        self.append(END);
        new_root
    }

    fn visit_global_vars(&mut self, nodes: Vec<Box<Statement>>) -> Vec<Box<Statement>> {
        let mut new_root = nodes.clone();
        new_root.iter().enumerate().for_each(|(i, statement)| {
            if let box VariableDeclaration(var) = statement {
                self.visit_variable_declaration(var);
            }
        });
        new_root
    }

    fn visit_function_declarations(&mut self, nodes: Vec<Box<Statement>>) -> Vec<Box<Statement>> {
        let mut new_root = nodes.clone();
        nodes.iter().enumerate().for_each(|(i, statement)| {
            if let box FunctionDeclaration(function) = statement {
                new_root.remove(i);
                self.append(DATABOX);
                self.visit_function_declaration(function);
            };
        });
        new_root
    }

    pub fn append_ref_as_databox(&mut self, init: &Expression, id: String) {
        match init {
            StringLiteral(ref s) => {
                let as_databox = format!("new(\"{}\")", &s.to_string());
                self.append(&as_databox);
            },
            NumericLiteral(n) => {
                let as_databox = format!("new({})", &n.to_string());
                self.append(&as_databox);
            },
            ObjectExpression(ref o) => {
                self.append(NEW_DICT);
                self.append(SEMI_COL);
                self.append(NEW_LINE);
                self.visit_object_expression(&o, id);
            }
            _ => {
                self.visit_expression(init);
            }
        }
    }

    pub fn append_option_identifier_or_visit_expression(&mut self, identifier: &Option<&Id>, expression: &Expression) {
        if let Some(id) = identifier {
            self.append(&id.to_string());
        } else {
            self.visit_expression(expression);
        };
    }

    pub fn append(&mut self, word: &str) {
        self.out.push_str(word)
    }
}


impl Expression {
    pub fn try_as_identifier(&self) -> Option<&Id> {
        if let Identifier(id) = &self {
            Some(id)
        } else {
            None
        }
    }
    pub fn try_as_string_from_lit(&self) -> Option<String> {
        if let StringLiteral(lit) = &self {
            Some(lit.value.clone())
        } else {
            None
        }
    }
    pub fn try_as_string_from_identifier(&self) -> Option<String> {
        if let Identifier(id) = &self {
            Some(id.name.clone())
        } else {
            None
        }
    }
}

trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
