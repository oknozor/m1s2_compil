use crate::ast::expression::Expression;
use crate::ast::statement::ExpressionStatement;
use crate::ast::expression::BinaryExpression;
use crate::ast::node::Program;
use crate::ast::node::File;
use crate::ast::node::Node;
use crate::ast::statement::VariableDeclaration;
use crate::ast::statement::VariableDeclarator;
use crate::ast::expression::Literal;

pub trait Visitor<'ast>
{
    fn visit_expression_statement(&mut self, s: &ExpressionStatement);
    fn visit_expression(&mut self, s: &Expression);
    fn visit_node(&mut self, s: &Node);
    fn visit_binary_expression(&mut self, e: &BinaryExpression);
    fn visit_program_root(&mut self, e: &Program);
    fn visit_file(&mut self, e: &File);
    fn visit_variable_declaration(&mut self, e: &VariableDeclaration);
    fn visit_variable_declarator(&mut self, e: &VariableDeclarator);
    fn visit_literal<'t, T: Literal>(&mut self, lit: T);
}

