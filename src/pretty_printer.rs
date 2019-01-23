use crate::ast::node::File;
use crate::ast::visitor::Visitor;
use crate::ast::statement::ExpressionStatement;
use crate::ast::expression::Expression;
use crate::ast::expression::BinaryExpression;
use crate::ast::node::Node;
use crate::ast::node::Program;

pub struct PrettyPrinter<'printer> {
    pub out: &'printer mut String,
}

impl <'printer> Visitor for PrettyPrinter <'printer> {
    fn visit_expression_statement(&mut self, s: &ExpressionStatement) {
        match s.expression {
            Expression::NumericLiteral(ref n) => self.out.push_str(n.value.to_string().as_str()),
            Expression::BinaryExpression(ref b) => self.visit_binary_expression(b),
        };
        self.out.push_str(";\n");
    }

    fn visit_expression(&mut self, s: &Expression) {
        match s {
            Expression::BinaryExpression(ref b) => self.visit_binary_expression(b),
            Expression::NumericLiteral(ref n) => self.out.push_str(n.value.to_string().as_str())
        }
    }

    fn visit_node(&mut self, s: &Node) {
        match s {
            Node::Program(ref p) => {
                println!("traversing program");
                self.visit_program_root(p)
            },
            Node::File(ref f) => self.visit_file(f),
            Node::ExpressionStatement(ref e) => self.visit_expression_statement(e)
        }
    }

    fn visit_binary_expression(&mut self, e: &BinaryExpression) {

        match *e.left {
            Expression::NumericLiteral(ref n) => self.out.push_str(n.value.to_string().as_str()),
            Expression::BinaryExpression(ref b) => self.visit_binary_expression(b),
        };
        self.out.push_str(e.operator.to_string().as_str());
        match *e.right {
            Expression::NumericLiteral(ref n) => self.out.push_str(n.value.to_string().as_str()),
            Expression::BinaryExpression(ref b) => self.visit_binary_expression(b),
        };
    }

    fn visit_program_root(&mut self, e: &Program) {
        e.body.iter().for_each(|node| self.visit_node(node));
    }

    fn visit_file(&mut self, e: &File) {
        self.visit_program_root(&e.program);
    }
}