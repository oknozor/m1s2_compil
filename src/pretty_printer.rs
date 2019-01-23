use crate::ast::node::File;
use crate::ast::visitor::Visitor;
use crate::ast::statement::ExpressionStatement;
use crate::ast::expression::Expression;
use crate::ast::expression::BinaryExpression;
use crate::ast::node::Node;
use crate::ast::node::Program;
use crate::ast::statement::VariableDeclaration;
use crate::ast::statement::VariableDeclarator;
use crate::ast::expression::Literal;

pub struct PrettyPrinter<'printer> {
    pub out: &'printer mut String,
}
/* Todo : make the visitor self mutate a TokenStream simplify future interpreter and compiler implementation */

impl<'printer> Visitor<'printer> for PrettyPrinter<'printer> {
    fn visit_expression_statement(&mut self, s: &ExpressionStatement) {
        match s.expression {
            Expression::NumericLiteral(ref n) => self.visit_literal(n),
            Expression::BinaryExpression(ref b) => self.visit_binary_expression(b),
            Expression::StringLiteral(ref s) => self.visit_literal(s),
        }
        self.new_line();
    }

    fn visit_expression(&mut self, s: &Expression) {
        match s {
            Expression::BinaryExpression(ref b) => self.visit_binary_expression(b),
            Expression::NumericLiteral(ref n) => self.visit_literal(n),
            Expression::StringLiteral(ref s) => self.visit_literal(s),
        }
    }

    fn visit_node(&mut self, s: &Node) {
        match s {
            Node::Program(ref p) => self.visit_program_root(p),
            Node::File(ref f) => self.visit_file(f),
            Node::ExpressionStatement(ref e) => self.visit_expression_statement(e),
            Node::VariableDeclaration(ref v) => self.visit_variable_declaration(v),
        }
    }

    fn visit_binary_expression(&mut self, e: &BinaryExpression) {
        match *e.left {
            Expression::NumericLiteral(ref n) => self.visit_literal(n),
            Expression::BinaryExpression(ref b) => self.visit_binary_expression(b),
            Expression::StringLiteral(ref s) => self.visit_literal(s),
        };
        self.out.push_str(e.operator.to_string().as_str());
        match *e.right {
            Expression::NumericLiteral(ref n) => self.visit_literal(n),
            Expression::BinaryExpression(ref b) => self.visit_binary_expression(b),
            Expression::StringLiteral(ref s) => self.visit_literal(s),
        };
    }

    fn visit_program_root(&mut self, e: &Program) {
        e.body.iter()
            .for_each(|node| {
                self.visit_node(node)
            });
    }

    fn visit_file(&mut self, e: &File) {
        self.visit_program_root(&e.program);
    }

    fn visit_variable_declaration(&mut self, v: &VariableDeclaration) {
        self.out.push_str(v.kind.as_str());
        self.backspace();
        v.declarations.iter()
            .for_each(|declaration| {
                self.visit_variable_declarator(declaration);
            });
        self.new_line();
    }

    fn visit_variable_declarator(&mut self, v: &VariableDeclarator) {
        self.out.push_str(v.id.name.as_str());

        match &v.init {
            Some(s) => {
                self.out.push_str("=");
                self.visit_expression(s)
            },
            None => ()
        };
    }

    fn visit_literal<T: Literal>(&mut self, lit: T) {
        self.out.push_str(lit.get_as_string().as_str());
    }
}

impl <'printer>PrettyPrinter<'printer> {
    fn new_line(&mut self) {
        self.out.push_str(";\n");
    }
    fn backspace(&mut self){
        self.out.push_str(" ");
    }
}

