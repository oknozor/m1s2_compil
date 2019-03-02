use crate::visitor::Visitor;
use crate::ast::statement::Statement::*;
use crate::ast::statement::*;
use crate::ast::expression::Expression::*;
use crate::ast::expression::*;
use crate::asm_compile::asm_writer::ASMWriter;

impl <'pr> Visitor for ASMWriter<'pr> {

    fn visit_while_statment(&mut self, w: &WhileStmt) {
        unimplemented!()
    }

    fn visit_block_statement(&mut self, s: &BlockStmt) {
        unimplemented!()
    }

    fn visit_variable_declarator(&mut self, v: &Variable) {
        unimplemented!()
    }

    fn visit_while_statement(&mut self, w: &WhileStmt) {
        unimplemented!()
    }

    fn visit_variable_declaration(&mut self, v: &VariableDec) {
        unimplemented!()
    }

    fn visit_if_statement(&mut self, i: &IfStmt) {
        unimplemented!()
    }

    fn visit_switch_statement(&mut self, s: &SwitchStmt) {
        unimplemented!()
    }

    fn visit_case(&mut self, case: &CaseStmt) {
        unimplemented!()
    }

    fn visit_for_statement(&mut self, f: &ForStmt) {
        unimplemented!()
    }

    fn visit_break_statement(&mut self, f: &BreakStmt) {
        unimplemented!()
    }

    fn visit_return_statement(&mut self, r: &ReturnStmt) {
        unimplemented!()
    }

    fn visit_continue_statement(&mut self, c: &ContinueStmt) {
        unimplemented!()
    }

    fn visit_function_declaration(&mut self, f: &FunctionDec) {
        unimplemented!()
    }

    fn visit_option_expression(&mut self, exp: &Option<Box<Expression>>) {
        unimplemented!()
    }

    fn visit_expression(&mut self, exp: &Expression) {
        unimplemented!()
    }

    fn visit_expression_statement(&mut self, s: &ExpressionStmt) {
        unimplemented!()
    }

    fn visit_binary_expression(&mut self, b: &BinaryExp) {
        unimplemented!()
    }

    fn visit_assign(&mut self, a: &AssignmentExp) {
        unimplemented!()
    }

    fn visit_unary_expression(&mut self, u: &UnaryExp) {
        unimplemented!()
    }

    fn visit_update_expression(&mut self, u: &UpdateExp) {
        unimplemented!()
    }

    fn visit_member_expression(&mut self, m: &MemberExp) {
        unimplemented!()
    }

    fn visit_logical_expression(&mut self, l: &LogicalExp) {
        unimplemented!()
    }

    fn visit_call_expression(&mut self, e: &CallExp) {
        unimplemented!()
    }
}