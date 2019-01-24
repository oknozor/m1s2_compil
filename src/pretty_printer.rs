use crate::ast::expression::Expression;
use crate::ast::expression::{BinaryExpression, AssignmentExpression, CallExpression, Identifier,
                             LogicalExpression, UnaryExpression, MemberExpression};
use crate::ast::statement::{Statement, ExpressionStatement, BlockStatement, WhileStatement,
                            VariableDeclaration, VariableDeclarator, IfStatement, ForStatement, BreakStatement,
                            ContinueStatement, FunctionDeclaration, ReturnStatement, SwitchStatement};
use super::ast::expression::UpdateExpression;
use crate::runner::visitor::Visitor;
use crate::runner::handler::Handler;
use crate::ast::literal::Literal;
use crate::ast::expression::Extra;

pub struct PrettyPrinter<'printer> {
    pub out: &'printer mut String,
}

impl<'pr> Visitor for PrettyPrinter<'pr> {}

impl<'pr> Handler for PrettyPrinter<'pr> {
    fn handle_expression(&mut self, s: &Expression) {
    }

    fn handle_expression_statement<>(&mut self, s: &ExpressionStatement) {
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
        self.out.push_str(operator)
    }

    fn handle_string_literal(&mut self, literal: &Literal<String>) {
        unimplemented!()
    }

    fn handle_numeric_literal(&mut self, literal: &Literal<i64>) {
        self.out.push_str(&literal.value.to_string())
    }

    fn handle_start_extra(&mut self, ex: &Option<Extra>) {
        if let Some(ex) = ex {
            self.parentesis_left();
        }
    }

    fn handle_end_extra(&mut self, ex: &Option<Extra>) {
        if let Some(ex) = ex {
            self.parentesis_right();
        }
    }

    fn handle_statement(&mut self, s: &Statement) {
        self.new_line();
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

impl<'printer> PrettyPrinter<'printer> {}

// todo: refacto with enum
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

