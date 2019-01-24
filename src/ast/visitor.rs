use crate::ast::statement::{Statement, ExpressionStatement, BlockStatement, WhileStatement,
                            VariableDeclarator, VariableDeclaration, IfStatement, ForStatement,
                            BreakStatement, ContinueStatement};

use crate::ast::expression::{Expression, BinaryExpression, AssignmentExpression, CallExpression,
                             UpdateExpression, Literal, Identifier};
use crate::ast::node::{File, Program, Node};

pub trait Visitor<'ast> {
    fn visit_node(&mut self, s: &Node);
    fn visit_program_root(&mut self, e: &Program);
    fn visit_file(&mut self, e: &File);

    // statement
    fn visit_statement(&mut self, s: &Statement);
    fn visit_block_statement(&mut self, b: &BlockStatement);
    fn visit_while_statement(&mut self, w: &WhileStatement);
    fn visit_variable_declaration(&mut self, v: &VariableDeclaration);
    fn visit_variable_declarator(&mut self, v: &VariableDeclarator);
    fn visit_if_statement(&mut self, i: &IfStatement);
    fn visit_for_statement(&mut self, f: &ForStatement);
    fn visit_break_statement(&mut self, f: &BreakStatement);
    fn visit_continue_statement(&mut self, f: &ContinueStatement);

    // expression
    fn visit_expression(&mut self, s: &Expression);
    fn visit_expression_statement(&mut self, s: &ExpressionStatement);
    fn visit_binary_expression(&mut self, e: &BinaryExpression);
    fn visit_assignement_expression(&mut self, e: &AssignmentExpression);
    fn visit_call_expression(&mut self, e: &CallExpression);
    fn visit_update_expression(&mut self, e: &UpdateExpression);
    fn visit_identifier(&mut self, id: &Identifier);
    fn visit_literal<T: Literal>(&mut self, lit: T);
}

