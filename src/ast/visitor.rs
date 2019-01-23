use crate::ast::expression::Expression;
use crate::ast::statement::ExpressionStatement;
use crate::ast::expression::BinaryExpression;
use crate::ast::node::Program;
use crate::ast::node::File;
use crate::ast::node::Node;

pub trait Visitor
{
    fn visit_expression_statement(&mut self, s: &ExpressionStatement);
    fn visit_expression(&mut self, s: &Expression);
    fn visit_node(&mut self, s: &Node);
    fn visit_binary_expression(&mut self, e: &BinaryExpression);
    fn visit_program_root(&mut self, e: &Program);
    fn visit_file(&mut self, e: &File);
}

