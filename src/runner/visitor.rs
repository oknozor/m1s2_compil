use crate::ast::statement::{Statement, ExpressionStatement, BlockStatement, WhileStatement,
                            VariableDeclarator, VariableDeclaration, IfStatement, ForStatement,
                            BreakStatement, ContinueStatement, FunctionDeclaration, ReturnStatement,
                            SwitchStatement};

use crate::ast::expression::{Expression, BinaryExpression, AssignmentExpression, CallExpression,
                             UpdateExpression, Identifier, LogicalExpression, UnaryExpression,
                             MemberExpression};
use crate::ast::node::{File, Program, RootNode};
use crate::runner::handler::Handler;
use crate::ast::literal::Literal;

/*pub struct Visitor<'ast, V, T: Handler> {
    pub handler: &'ast  T,
}*/

pub trait Visitor: Handler
{
    // module root
    fn visit_node(&mut self, s: &RootNode) {
        match s {
            RootNode::Program(ref p) => {
                self.visit_program_root(p);
            }
            RootNode::File(ref f) => {
                self.visit_file(f);
            }
        }
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

    // statement
    fn visit_statement(&mut self, s: &Statement) {
        Handler::handle_statement(self, s);
        match s {
            Statement::BlockStatement(b) => {
                self.visit_block_statement(b);
            }
            Statement::VariableDeclaration(v) => {
                self.visit_variable_declaration(v);
            }
            Statement::ExpressionStatement(e) => {
                self.visit_expression_statement(e);
            }
            Statement::VariableDeclarator(v) => {
                self.visit_variable_declarator(v);
            }
            Statement::WhileStatement(v) => {
                self.visit_while_statement(v);
            }
            Statement::IfStatement(i) => {
                self.visit_if_statement(i);
            }
            Statement::SwitchStatement(s) => {
                self.visit_switch_statement(s);
            }
            Statement::ForStatement(f) => {
                self.visit_for_statement(f);
            }
            Statement::BreakStatement(b) => {
                self.visit_break_statement(b);
            }
            Statement::ContinueStatement(c) => {
                self.visit_continue_statement(c);
            }
            Statement::ReturnStatement(r) => {
                self.visit_return_statement(r);
            }
            Statement::FunctionDeclaration(f) => {
                self.visit_function_declaration(f);
            }
            Statement::EmptyStatement => ()
        }
    }

    fn visit_block_statement(&mut self, s: &BlockStatement) {
        s.body.iter().for_each(|statement| self.visit_statement(statement))
    }

    fn visit_while_statement(&mut self, w: &WhileStatement) {
        self.visit_expression(&w.test);
        self.visit_statement(&w.body);
    }

    fn visit_variable_declaration(&mut self, v: &VariableDeclaration) {
        v.declarations.iter()
            .for_each(|declaration| {
                self.visit_variable_declarator(&declaration);
            });
    }

    fn visit_variable_declarator(&mut self, v: &VariableDeclarator) {
        if v.init.is_some() {}
        self.visit_option_expression(&v.init);
    }

    fn visit_if_statement(&mut self, i: &IfStatement) {
        self.visit_expression(&i.test);
        //fixme: lifetime of copy trait impl needed here
        /*match *i.alternate {
            Some(e) => self.visit_statement(&e),
            None => ()
        }*/
        self.visit_statement(&i.consequent);
    }

    fn visit_switch_statement(&mut self, s: &SwitchStatement) {
        self.visit_expression(&s.discriminant);
        s.cases.iter().for_each(|case| {
            match &case.test {
                Some(s) => {
                    self.visit_expression(s);
                }
                None => ()
            };
            case.consequent.iter()
                .for_each(|cons| self.visit_statement(&cons));
        });
    }

    fn visit_for_statement(&mut self, f: &ForStatement) {
        self.visit_option_expression(&f.init);
        self.visit_option_expression(&f.test);
        self.visit_option_expression(&f.update);
        self.visit_statement(&f.body);
    }

    fn visit_break_statement(&mut self, f: &BreakStatement) {
        if let Some(s) = &f.label {
            self.visit_identifier(s);
        }
    }

    fn visit_continue_statement(&mut self, c: &ContinueStatement) {
        match &c.label {
            Some(s) => self.visit_identifier(s),
            None => ()
        }
    }

    fn visit_return_statement(&mut self, r: &ReturnStatement) {
        self.visit_option_expression(&r.argument);
    }


    fn visit_function_declaration(&mut self, f: &FunctionDeclaration) {
        f.params.iter()
            .for_each(|param| self.visit_identifier(param));
        self.visit_block_statement(&f.body);
    }

    // expression
    fn visit_expression(&mut self, s: &Expression) {
        Handler::handle_expression(self, s);

        match s {
            Expression::NumericLiteral(ref n) => Handler::handle_numeric_literal(self, n),
            Expression::UpdateExpression(ref u) => self.visit_update_expression(u),
            Expression::BinaryExpression(ref b) => self.visit_binary_expression(b),
            Expression::UnaryExpression(ref u) => self.visit_unary_expression(u),
            Expression::MemberExpression(ref m) => self.visit_member_expression(m),
            Expression::StringLiteral(ref s) => Handler::handle_string_literal(self, s),
            Expression::Identifier(ref i) => self.visit_identifier(i),
            Expression::CallExpression(ref c) => self.visit_call_expression(c),
            Expression::AssignmentExpression(ref e) => self.visit_assignement_expression(e),
            Expression::LogicalExpression(ref l) => self.visit_logical_expression(l),
        }
    }

    fn visit_option_expression(&mut self, e: &Option<Expression>) {
        match &e {
            Some(s) => {
                self.visit_expression(&s)
            }
            None => ()
        };
    }

    fn visit_expression_statement(&mut self, s: &ExpressionStatement) {
        Handler::handle_expression_statement(self, &s);
        self.visit_expression(&s.expression);
    }

    fn visit_binary_expression(&mut self, e: &BinaryExpression) {
        Handler::handle_start_extra(self, &e.extra);

        Handler::handle_binary_expression(self, e);

        self.visit_expression(&e.left);
        Handler::handle_num_operator(self, &e.operator);
        self.visit_expression(&e.right);
        Handler::handle_end_extra(self, &e.extra);

    }

    fn visit_unary_expression(&mut self, u: &UnaryExpression) {
        /* TODO OPERATOR
                self.out.push_str(&u.operator.as_str());
        */
        self.visit_expression(&u.argument);
    }

    fn visit_assignement_expression(&mut self, e: &AssignmentExpression) {
        self.visit_expression(&e.left);
        /* TODO OPERATOR
                self.out.push_str(&u.operator.as_str());
        */
        self.visit_expression(&e.right)
    }

    fn visit_member_expression(&mut self, m: &MemberExpression) {
        self.visit_expression(&m.object);
        self.visit_expression(&m.property);
    }

    fn visit_logical_expression(&mut self, l: &LogicalExpression) {
        self.visit_expression(&l.left);
        /* TODO OPERATOR
                self.out.push_str(&u.operator.as_str());
        */
        self.visit_expression(&l.right);
    }

    fn visit_call_expression(&mut self, e: &CallExpression) {
        self.visit_expression(&e.callee);
        let last = e.arguments.len() - 1;
        e.arguments.iter().enumerate()
            .for_each(|(i, expression)| {
                self.visit_expression(expression);
                if i != last {}
            });
    }

    fn visit_update_expression(&mut self, e: &UpdateExpression) {
        self.visit_expression(&e.argument);
        /* TODO OPERATOR
                self.out.push_str(&u.operator.as_str());
        */
    }

    fn visit_identifier(&mut self, id: &Identifier) {
        /*self.out.push_str(id.name.as_str())*/;
    }

    fn visit_literal<L>(&mut self, lit: &Literal<L>) {}
}

