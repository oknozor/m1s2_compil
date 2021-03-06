use crate::visitor::Visitor;
use crate::ast::statement::Statement::*;
use crate::ast::statement::*;
use crate::ast::expression::Expression::*;
use crate::ast::expression::*;
use crate::asm_compile::asm_writer::ASMWriter;
use crate::writer::*;
use crate::asm_compile::Register;
use crate::asm_compile::*;
use crate::token::to_token::ToToken;
use crate::token::to_token::postfix;

impl<'pr> Visitor for ASMWriter<'pr> {
    fn visit_block_statement(&mut self, s: &BlockStmt) {
        unimplemented!();
    }

    fn visit_variable_declarator(&mut self, v: &Variable) {
        unimplemented!();
    }

    fn visit_while_statement(&mut self, w: &WhileStmt) {
        unimplemented!();
    }

    fn visit_variable_declaration(&mut self, v: &VariableDec) {
        unimplemented!();
    }

    fn visit_if_statement(&mut self, i: &IfStmt) {
        unimplemented!();
    }

    fn visit_switch_statement(&mut self, s: &SwitchStmt) {
        unimplemented!();
    }

    fn visit_case(&mut self, case: &CaseStmt) {
        unimplemented!();
    }

    fn visit_for_statement(&mut self, f: &ForStmt) {
        unimplemented!();
    }

    fn visit_break_statement(&mut self, f: &BreakStmt) {
        unimplemented!();
    }

    fn visit_return_statement(&mut self, r: &ReturnStmt) {
        unimplemented!();
    }

    fn visit_continue_statement(&mut self, c: &ContinueStmt) {
        unimplemented!();
    }

    fn visit_function_declaration(&mut self, f: &FunctionDec) {
        unimplemented!();
    }

    fn visit_option_expression(&mut self, exp: &Option<Box<Expression>>) {
        unimplemented!();
    }

    fn visit_expression(&mut self, exp: &Expression) {
        match exp {
            UpdateExpression(ref u) => self.visit_update_expression(u),
            BinaryExpression(ref b) => self.visit_binary_expression(b),
            UnaryExpression(ref u) => self.visit_unary_expression(u),
            MemberExpression(ref m) => self.visit_member_expression(m),
            CallExpression(ref c) => self.visit_call_expression(c),
            AssignmentExpression(ref e) => self.visit_assign(e),
            LogicalExpression(ref l) => self.visit_logical_expression(l),
            _ => (),
        };
    }

    fn visit_expression_statement(&mut self, s: &ExpressionStmt) {
        self.visit_expression(&s.expression);
    }

    fn visit_binary_expression(&mut self, b: &BinaryExp) {
        let tokens = &b.to_token();
        let mut postfix_expression = postfix(&mut tokens.clone());
        self.postfix_to_asm(&mut postfix_expression);
        self.append(TAB);
        self.append(NEW_LINE);

    }

    fn visit_assign(&mut self, a: &AssignmentExp) { unimplemented!(); }

    fn visit_unary_expression(&mut self, u: &UnaryExp) {
        unimplemented!();
    }

    fn visit_update_expression(&mut self, u: &UpdateExp) {
        unimplemented!();
    }

    fn visit_member_expression(&mut self, m: &MemberExp) {
        unimplemented!();
    }

    fn visit_logical_expression(&mut self, l: &LogicalExp) {
        unimplemented!();
    }

    fn visit_call_expression(&mut self, e: &CallExp) {
        unimplemented!();
    }

    fn visit_object_expression(&mut self, o: &ObjectExp, id: String) {
        unimplemented!();
    }

    fn visit_property_expression(&mut self, id: &str, p: &Property) {
        unimplemented!();
    }
}