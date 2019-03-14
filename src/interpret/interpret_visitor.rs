use crate::visitor::Visitor;
use crate::interpret::interpreter::Interpreter;
use crate::ast::statement::Statement::*;
use crate::ast::statement::*;
use crate::ast::expression::Expression::*;
use crate::ast::expression::*;
use crate::token::token::Token;
use crate::token::to_token::ToToken;
use crate::token::to_token::postfix;
use crate::interpret::solve_postfix;

impl <'ast> Visitor for Interpreter {
    fn visit_block_statement(&mut self, s: &BlockStmt) {

    }

    fn visit_variable_declarator(&mut self, v: &Variable) {
        unimplemented!("Declarator TODO");
    }

    fn visit_while_statement(&mut self, w: &WhileStmt) {
        unimplemented!("WHILE TODO");
    }

    fn visit_variable_declaration(&mut self, v: &VariableDec) {
        unimplemented!("Declaration TODO");
    }

    fn visit_if_statement(&mut self, i: &IfStmt) {
        unimplemented!("If TODO");
    }

    fn visit_switch_statement(&mut self, s: &SwitchStmt) {
        unimplemented!("Swithc TODO");
    }

    fn visit_case(&mut self, case: &CaseStmt) {
        unimplemented!("Case TODO");
    }

    fn visit_for_statement(&mut self, f: &ForStmt) {
        unimplemented!("For TODO");
    }

    fn visit_break_statement(&mut self, f: &BreakStmt) {
        unimplemented!("Break TODO");
    }

    fn visit_return_statement(&mut self, r: &ReturnStmt) {
        unimplemented!("Return TODO");
    }

    fn visit_continue_statement(&mut self, c: &ContinueStmt) {
        unimplemented!("Continue TODO");
    }

    fn visit_function_declaration(&mut self, f: &FunctionDec) {
        unimplemented!("Function TODO");
    }

    fn visit_option_expression(&mut self, exp: &Option<Box<Expression>>) {
        unimplemented!("Opt exp TODO");
    }

    fn visit_expression(&mut self, exp: &Expression) {
        match exp {
            NumericLiteral(ref n) => self.main.extend(n.to_token()),
            StringLiteral(ref s) => self.main.extend(s.to_token()),
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
        let tokens = &mut b.to_token();
        unimplemented!("evaluate expression {:?}",tokens);
        let mut postfix_exp  = postfix(tokens);
        let result = solve_postfix(&mut postfix_exp);
        unimplemented!("result : {}", result);
    }

    fn visit_assign(&mut self, a: &AssignmentExp) {
        unimplemented!("Assign TODO");
    }

    fn visit_unary_expression(&mut self, u: &UnaryExp) {
        unimplemented!("UnaryExp TODO");
    }

    fn visit_update_expression(&mut self, u: &UpdateExp) {
        unimplemented!("UpdateExp TODO");
    }

    fn visit_member_expression(&mut self, m: &MemberExp) {
        unimplemented!("MemberExp TODO");
    }

    fn visit_logical_expression(&mut self, l: &LogicalExp) {
        unimplemented!("LogExp TODO");
    }

    fn visit_call_expression(&mut self, e: &CallExp) {
        unimplemented!("Call TODO");
    }

    fn visit_object_expression(&mut self, o: &ObjectExp, id: String) {
        unimplemented!("Object TODO");
    }

    fn visit_property_expression(&mut self, id: &str, p: &Property) {
        unimplemented!("Prop TODO");
    }
}