use crate::ast::expression::{Expression, BinaryExpression, AssignmentExpression, CallExpression, Identifier,
                             LogicalExpression, UnaryExpression, MemberExpression};
use crate::ast::statement::{Statement, ExpressionStatement, BlockStatement, WhileStatement,
                            VariableDeclaration, VariableDeclarator, IfStatement, ForStatement, BreakStatement,
                            ContinueStatement, FunctionDeclaration, ReturnStatement, SwitchStatement};
use super::ast::expression::UpdateExpression;
use crate::runner::visitor::Visitor;
use crate::runner::handler::Handler;
use crate::ast::literal::Literal;
use crate::ast::expression::Extra;
use std::collections::HashMap;

pub struct Interpretter ;

impl Visitor for Interpretter {}
enum Token {
    BooleanLiteral(bool),
    Identifier(String),
    NullLiteral,
    NumericLiteral(f64),
    StringLiteral(String),
    RegularExpression(String),
}

impl Handler for Interpretter {
    fn handle_expression(&mut self, s: &Expression) {
        unimplemented!()
    }

    fn handle_expression_statement(&mut self, s: &ExpressionStatement) {
        unimplemented!()
    }

    fn handle_binary_expression(&mut self, binary_exp: &BinaryExpression) {

    }

    fn handle_unary_expression(&mut self, u: &UnaryExpression) {
        unimplemented!()
    }

    fn handle_assignement_expression(&mut self, e: &AssignmentExpression) {
        unimplemented!()
    }

    fn handle_member_expression(&mut self, m: &MemberExpression) {
        unimplemented!()
    }

    fn handle_logical_expression(&mut self, l: &LogicalExpression) {
        unimplemented!()
    }

    fn handle_call_expression(&mut self, e: &CallExpression) {
        unimplemented!()
    }

    fn handle_update_expression(&mut self, e: &UpdateExpression) {
        unimplemented!()
    }

    fn handle_identifier(&mut self, id: &Identifier) {
        unimplemented!()
    }

    fn handle_num_operator(&mut self, operator: &str) {
        println!("{}", operator);

    }

    fn handle_string_literal(&mut self, literal: &Literal<String>) {
        println!("{}", literal.value);
    }

    fn handle_numeric_literal(&mut self, literal: &Literal<i64>) {
        println!("{}", literal.value)
    }

    fn handle_start_extra(&mut self, ex: &Option<Extra>) {
        unimplemented!()
    }

    fn handle_end_extra(&mut self, ex: &Option<Extra>) {
        unimplemented!()
    }


    fn handle_statement(&mut self, s: &Statement) {
        unimplemented!()
    }

    fn handle_block_statement(&mut self, s: &BlockStatement) {
        unimplemented!()
    }

    fn handle_while_statement(&mut self, w: &WhileStatement) {
        unimplemented!()
    }

    fn handle_variable_declaration(&mut self, v: &VariableDeclaration) {
        unimplemented!()
    }

    fn handle_variable_declarator(&mut self, v: &VariableDeclarator) {
        unimplemented!()
    }

    fn handle_if_statement(&mut self, i: &IfStatement) {
        unimplemented!()
    }

    fn handle_switch_statement(&mut self, s: &SwitchStatement) {
        unimplemented!()
    }

    fn handle_for_statement(&mut self, f: &ForStatement) {
        unimplemented!()
    }

    fn handle_break_statement(&mut self, f: &BreakStatement) {
        unimplemented!()
    }

    fn handle_continue_statement(&mut self, c: &ContinueStatement) {
        unimplemented!()
    }

    fn handle_return_statement(&mut self, r: &ReturnStatement) {
        unimplemented!()
    }

    fn handle_function_declaration(&mut self, f: &FunctionDeclaration) {
        unimplemented!()
    }
}