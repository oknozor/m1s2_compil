use crate::ast::expression::Expression;
use crate::ast::node::{Node, File, Program};
use crate::ast::expression::{Literal, BinaryExpression, AssignmentExpression, CallExpression, Identifier};
use crate::ast::statement::{Statement, ExpressionStatement, BlockStatement, WhileStatement,
                            VariableDeclaration, VariableDeclarator, IfStatement, ForStatement, BreakStatement,
                            ContinueStatement};
use crate::ast::visitor::Visitor;
use crate::ast::expression::UpdateExpression;

pub struct PrettyPrinter<'printer> {
    pub out: &'printer mut String,
}
/* Todo : separate de visitor logic from code generation logic  */
/* Todo : manage indentation */

impl<'printer> Visitor<'printer> for PrettyPrinter<'printer> {
    fn visit_expression(&mut self, s: &Expression) {
        match s {
            Expression::NumericLiteral(ref n) => self.visit_literal(n),
            Expression::UpdateExpression(ref u) => self.visit_update_expression(u),
            Expression::BinaryExpression(ref b) => self.visit_binary_expression(b),
            Expression::StringLiteral(ref s) => self.visit_literal(s),
            Expression::Identifier(ref i) => self.visit_identifier(i),
            Expression::CallExpression(ref c) => self.visit_call_expression(c),
            Expression::AssignmentExpression(ref e) => self.visit_assignement_expression(e),
            Expression::EmptyStatement => (),
        }
    }

    fn visit_expression_statement(&mut self, s: &ExpressionStatement) {
        self.visit_expression(&s.expression);
    }

    fn visit_node(&mut self, s: &Node) {
        match s {
            Node::Program(ref p) => self.visit_program_root(p),
            Node::File(ref f) => self.visit_file(f),
        }
    }

    fn visit_binary_expression(&mut self, e: &BinaryExpression) {
        self.visit_expression(&e.left);
        self.out.push_str(e.operator.to_string().as_str());
        self.visit_expression(&e.right);
    }

    fn visit_program_root(&mut self, e: &Program) {
        e.body.iter()
            .for_each(|statement| {
                self.visit_statement(statement)
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
                self.visit_variable_declarator(&declaration);
            });
    }

    fn visit_variable_declarator(&mut self, v: &VariableDeclarator) {
        self.out.push_str(v.id.name.as_str());
        if v.init.is_some() {
            self.append_equal();
        }
        self.visit_option_expression(&v.init);
    }

    fn visit_literal<T: Literal>(&mut self, lit: T) {
        self.out.push_str(lit.get_as_string().as_str());
    }

    fn visit_statement(&mut self, s: &Statement) {
        match s {
            Statement::BlockStatement(b) => {
                self.visit_block_statement(b);
            }
            Statement::VariableDeclaration(v) => {
                self.visit_variable_declaration(v);
                self.semi_col();
                self.new_line();
            }
            Statement::ExpressionStatement(e) => {
                self.visit_expression_statement(e);
                self.semi_col();
                self.new_line();
            }
            Statement::VariableDeclarator(v) => {
                self.visit_variable_declarator(v);
                self.semi_col();
                self.new_line();
            }
            Statement::WhileStatement(v) => {
                self.visit_while_statement(v);
                self.new_line();
            }
            Statement::IfStatement(i) => {
                self.visit_if_statement(i);
                self.new_line();
            },
            Statement::ForStatement(f) => {
                self.visit_for_statement(f);
            }
            Statement::BreakStatement(b) => {
                self.visit_break_statement(b);
            }
            Statement::ContinueStatement(c) => {
                self.visit_continue_statement(c);
            }
        }
    }

    fn visit_block_statement(&mut self, s: &BlockStatement) {
        s.body.iter().for_each(|statement| self.visit_statement(statement))
    }

    fn visit_while_statement(&mut self, w: &WhileStatement) {
        self.append_while();
        self.parentesis_left();
        self.visit_expression(&w.test);
        self.parentesis_right();
        self.backspace();
        self.bracket_left();
        self.new_line();
        self.visit_statement(&w.body);
        self.bracket_right();
    }

    fn visit_identifier(&mut self, id: &Identifier) {
        self.out.push_str(id.name.as_str());
    }

    fn visit_update_expression(&mut self, e: &UpdateExpression) {
        self.visit_expression(&e.argument);
        self.out.push_str(e.operator.as_str());
    }

    fn visit_call_expression(&mut self, e: &CallExpression) {
        self.visit_expression(&e.callee);
        let last = e.arguments.len() - 1;
        self.parentesis_left();
        e.arguments.iter().enumerate()
            .for_each(|(i, expression)| {
                self.visit_expression(expression);
                if i != last {
                    self.coma();
                }
            });
        self.parentesis_right();
    }

    fn visit_if_statement(&mut self, i: &IfStatement) {
        self.out.push_str("if");
        self.parentesis_left();
        self.visit_expression(&i.test);
        self.parentesis_right();
        //fixme: lifetime of copy trait impl needed here
        /*match *i.alternate {
            Some(e) => self.visit_statement(&e),
            None => ()
        }*/
        self.bracket_left();
        self.new_line();

        self.visit_statement(&i.consequent);
        self.bracket_right()
    }

    fn visit_assignement_expression(&mut self, e: &AssignmentExpression) {
        self.visit_expression(&e.left);
        self.out.push_str(e.operator.as_str());
        self.visit_expression(&e.right)
    }

    fn visit_for_statement(&mut self, f: &ForStatement) {
        self.append_for();
        self.parentesis_left();
        // todo : need some syntactic sugar here
        self.visit_option_expression(&f.init);
        self.semi_col();
        self.visit_option_expression(&f.test);
        self.semi_col();
        self.visit_option_expression(&f.update);
        self.parentesis_right();
        self.bracket_left();
        self.new_line();
        self.new_line();
        self.visit_statement(&f.body);
        self.bracket_right();
        self.new_line();
    }

    // todo: make a macro for option unwraping
    fn visit_break_statement(&mut self, f: &BreakStatement) {
        self.append_break();
        self.semi_col();
        self.new_line();
        match &f.label {
            Some(s) => self.visit_identifier(s),
            None => ()
        }

    }

    fn visit_continue_statement(&mut self, c: &ContinueStatement) {
        self.append_continue();
        self.semi_col();
        self.new_line();
        match &c.label {
            Some(s) => self.visit_identifier(s),
            None => ()
        }
    }
}

impl <'printer> PrettyPrinter<'printer>{
    fn visit_option_expression(&mut self, e: &Option<Expression>) {
        match &e {
            Some(s) => {
                self.visit_expression(&s)
            }
            None => ()
        };
    }
}

// todo: use an enum ?
impl<'printer> PrettyPrinter<'printer> {
    fn append_while(&mut self) {
        self.out.push_str("while");
    }
    fn append_for(&mut self) {
        self.out.push_str("for");
    }
    fn append_break(&mut self) {
        self.out.push_str("break");
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
    fn indent(&mut self) {
        self.out.push_str("\t");
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

