use crate::ast::expression::*;
use crate::ast::statement::*;

#[allow(unused)]
pub trait Handler {

    fn handle_program_root(&mut self){}

    //expression
    fn handle_expression_start(&mut self, s: &Expression){}
    fn handle_expression(&mut self, s: &Expression){}
    fn handle_expression_end(&mut self){}
    fn handle_expression_statement(&mut self, s: &ExpressionStatement){}
    fn handle_binary_expression(&mut self, binary_exp: &BinaryExpression){}
    fn handle_unary_expression(&mut self, u: &UnaryExpression){}
    fn handle_assignment_expression(&mut self, e: &AssignmentExpression){}
    fn handle_member_expression(&mut self, m: &MemberExpression){}
    fn handle_logical_expression(&mut self, l: &LogicalExpression){}
    fn handle_call_expression(&mut self, e: &CallExpression){}
    fn handle_update_expression(&mut self, e: &UpdateExpression){}
    fn handle_identifier(&mut self, id: &String){}
    fn handle_binary_operator(&mut self, operator: &str){}
    fn handle_string_literal(&mut self, literal: &JSLiteral<String>){}
    fn handle_numeric_literal(&mut self, literal: &JSLiteral<i64>){}
    fn handle_start_extra(&mut self, par: bool){}
    fn handle_end_extra(&mut self, par: bool){}
    fn on_statement_end(&mut self){}

    //statement
    fn handle_statement(&mut self, s: &Statement){}
    fn handle_statement_end(&mut self) {}
    fn handle_block_statement(&mut self, s: &BlockStatement){}
    fn handle_while_statement(&mut self, w: &WhileStatement){}
    fn handle_variable_declaration(&mut self, v: &VariableDeclaration){}
    fn handle_variable_declarator(&mut self, v: &VariableDeclarator){}
    fn handle_if_statement(&mut self, i: &IfStatement){}
    fn handle_switch_statement(&mut self, s: &SwitchStatement){}
    fn handle_for_statement(&mut self, f: &ForStatement){}
    fn handle_break_statement(&mut self, f: &BreakStatement){}
    fn handle_continue_statement(&mut self, c: &ContinueStatement){}
    fn handle_return_statement(&mut self, r: &ReturnStatement){}
    fn handle_function_declaration(&mut self, f: &FunctionDeclaration){}

}