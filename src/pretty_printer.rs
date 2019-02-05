use crate::ast::expression::{BinaryExpression};
use crate::ast::expression::Expression;
use crate::ast::literal::JSLiteral;
use crate::ast::statement::{ExpressionStatement, Statement, VariableDeclaration, VariableDeclarator};
use crate::runner::handler::Handler;
use crate::runner::visitor::Visitor;

pub struct PrettyPrinter<'printer> {
    pub out: &'printer mut String,
}

impl<'pr> Visitor for PrettyPrinter<'pr> {}

#[allow(unused)]
impl<'pr> Handler for PrettyPrinter<'pr> {
    fn handle_program_root(&mut self) {}

    fn handle_expression(&mut self, s: &Expression) {
    }

    fn handle_expression_statement<>(&mut self, s: &ExpressionStatement) {
    }

    fn handle_binary_expression(&mut self, binary_exp: &BinaryExpression) {
        self.out.push_str(&binary_exp.operator);
    }

    fn handle_num_operator(&mut self, operator: &str) {
        self.out.push_str(operator)
    }

    fn handle_string_literal(&mut self, literal: &JSLiteral<String>) {
        self.out.push_str(format!("\"{}\"", &literal.value).as_str());
    }

    fn handle_numeric_literal(&mut self, literal: &JSLiteral<i64>) {
        self.out.push_str(&literal.value.to_string());
    }

    fn handle_start_extra(&mut self, parenthesized: bool) {
        if parenthesized {
            self.parentesis_left();
        }
    }

    fn handle_end_extra(&mut self, parenthesized: bool) {
        if parenthesized {
            self.parentesis_right();
        }
    }

    fn on_statement_end(&mut self) {
    }

    fn handle_statement(&mut self, s: &Statement) {
        self.new_line();
    }

    fn handle_variable_declaration(&mut self, v: &VariableDeclaration) {
        self.out.push_str(&v.kind);
    }

    fn handle_variable_declarator(&mut self, v: &VariableDeclarator) {
        self.backspace();
        self.out.push_str(&v.id.name);
        self.append_equal();
    }
}

impl<'printer> PrettyPrinter<'printer> {}

// todo: refacto with tokens
#[allow(unused)]
impl<'printer> PrettyPrinter<'printer> {
    pub fn print(&self) {
        println!("{}", self.out)
    }
    fn append_while(&mut self) {
        self.out.push_str("while");
    }
    fn append_switch(&mut self) {
        self.out.push_str("switch");
    }
    fn append_case(&mut self) {
        self.out.push_str("case");
    }
    fn append_function(&mut self) {
        self.out.push_str("function")
    }
    fn append_return(&mut self) {
        self.out.push_str("return")
    }
    fn append_for(&mut self) {
        self.out.push_str("for");
    }
    fn append_break(&mut self) {
        self.out.push_str("break");
    }
    fn append_default(&mut self) {
        self.out.push_str("default");
    }

    fn append_continue(&mut self) {
        self.out.push_str("continue");
    }
    fn new_line(&mut self) {
        self.out.push_str("\n");
    }
    fn semi_col(&mut self) {
        self.out.push_str(";");
    }
    fn append_col(&mut self) {
        self.out.push_str(":");
    }
    fn backspace(&mut self) {
        self.out.push_str(" ");
    }
    fn parentesis_left(&mut self) {
        self.out.push_str("(");
    }
    fn parentesis_right(&mut self) {
        self.out.push_str(")");
    }
    fn bracket_left(&mut self) {
        self.out.push_str("{");
    }
    fn bracket_right(&mut self) {
        self.out.push_str("}");
    }
    fn coma(&mut self) {
        self.out.push_str(",");
    }
    fn append_equal(&mut self) {
        self.out.push_str("=");
    }
}

