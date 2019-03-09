use crate::ast::statement::*;
use crate::ast::expression::*;
use crate::ast::statement::Statement::*;

pub trait Visitor {
    // statement
    fn visit_statement(&mut self, s: &Statement) {
        match s {
            BlockStatement(b) => self.visit_block_statement(b),
            ExpressionStatement(e) => self.visit_expression_statement(e),
            WhileStatement(v) => self.visit_while_statment(v),
            IfStatement(i) => self.visit_if_statement(i),
            SwitchStatement(s) => self.visit_switch_statement(s),
            ForStatement(f) => self.visit_for_statement(f),
            BreakStatement(b) => self.visit_break_statement(b),
            ContinueStatement(c) => self.visit_continue_statement(c),
            ReturnStatement(r) => self.visit_return_statement(r),
            SwitchCase(case) => self.visit_case(case),
            _ => (),
        };
    }
    fn visit_while_statment(&mut self, w: &WhileStmt);
    fn visit_block_statement(&mut self, s: &BlockStmt);
    fn visit_variable_declarator(&mut self, v: &Variable);
    fn visit_while_statement(&mut self, w: &WhileStmt);
    fn visit_variable_declaration(&mut self, v: &VariableDec);
    fn visit_if_statement(&mut self, i: &IfStmt);
    fn visit_switch_statement(&mut self, s: &SwitchStmt);
    fn visit_case(&mut self, case: &CaseStmt);
    fn visit_for_statement(&mut self, f: &ForStmt);
    fn visit_break_statement(&mut self, f: &BreakStmt);
    fn visit_return_statement(&mut self, r: &ReturnStmt);
    fn visit_continue_statement(&mut self, c: &ContinueStmt);
    fn visit_function_declaration(&mut self, f: &FunctionDec);

    //expression
    fn visit_option_expression(&mut self, exp: &Option<Box<Expression>>);
    fn visit_expression(&mut self, exp: &Expression);
    fn visit_expression_statement(&mut self, s: &ExpressionStmt);
    fn visit_binary_expression(&mut self, b: &BinaryExp);
    fn visit_assign(&mut self, a: &AssignmentExp);
    fn visit_unary_expression(&mut self, u: &UnaryExp);
    fn visit_update_expression(&mut self, u: &UpdateExp);
    fn visit_member_expression(&mut self, m: &MemberExp);
    fn visit_logical_expression(&mut self, l: &LogicalExp);
    fn visit_call_expression(&mut self, e: &CallExp);
    fn visit_object_expression(&mut self, o: &ObjectExp, id: String);
    fn visit_property_expression(&mut self, id: &str, p: &Property);
}
