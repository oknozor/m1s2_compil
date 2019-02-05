use crate::ast::statement::{Statement, ExpressionStatement, BlockStatement, WhileStatement,
                            VariableDeclarator, VariableDeclaration, IfStatement, ForStatement,
                            BreakStatement, ContinueStatement, FunctionDeclaration, ReturnStatement,
                            SwitchStatement};

use crate::ast::expression::{Expression, BinaryExpression, AssignmentExpression, CallExpression,
                             UpdateExpression, LogicalExpression, UnaryExpression,
                             MemberExpression};
use crate::ast::node::{File, Program, RootNode};
use crate::runner::handler::Handler;

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
        Handler::handle_program_root(self);
        e.body.iter()
            .for_each(|statement| {
                self.visit_statement(statement);
            });
    }

    fn visit_file(&mut self, e: &File) {
        self.visit_program_root(&e.program);
    }

    // statement
    fn visit_statement(&mut self, s: &Statement) {
        match s {
            Statement::BlockStatement(b) => self.visit_block_statement(b),
            Statement::VariableDeclaration(v) => self.visit_variable_declaration(v),
            Statement::VariableDeclarator(v) => self.visit_variable_declarator(v),
            Statement::ExpressionStatement(e) => self.visit_expression_statement(e),
            Statement::WhileStatement(v) => Handler::handle_while_statement(self, v),
            Statement::IfStatement(i) => self.visit_if_statement(i),
            Statement::SwitchStatement(s) => self.visit_switch_statement(s),
            Statement::ForStatement(f) => self.visit_for_statement(f),
            Statement::BreakStatement(b) => self.visit_break_statement(b),
            Statement::ContinueStatement(c) => self.visit_continue_statement(c),
            Statement::ReturnStatement(r) => self.visit_return_statement(r),
            Statement::FunctionDeclaration(f) => self.visit_function_declaration(f),
            Statement::EmptyStatement => ()
        }
    }

    fn visit_block_statement(&mut self, s: &BlockStatement) {
        s.body.iter().for_each(|statement| self.visit_statement(statement))
    }

    fn visit_while_statement(&mut self, w: &WhileStatement) {
        Handler::handle_while_statement( self, &w);
    }

    fn visit_variable_declaration(&mut self, v: &VariableDeclaration) {
        Handler::handle_variable_declaration(self, &v);
        v.declarations.iter()
            .for_each(|declaration| {
                self.visit_variable_declarator(&declaration);
            });
    }

    fn visit_variable_declarator(&mut self, v: &VariableDeclarator) {
        Handler::handle_variable_declarator(self, &v);
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
            Handler::handle_identifier(self, &s.name);
        }
    }

    fn visit_continue_statement(&mut self, c: &ContinueStatement) {
        match &c.label {
            Some(s) => Handler::handle_identifier(self,&s.name),
            None => ()
        }
    }

    fn visit_return_statement(&mut self, r: &ReturnStatement) {
        self.visit_option_expression(&r.argument);
    }


    fn visit_function_declaration(&mut self, f: &FunctionDeclaration) {
        f.params.iter()
            .for_each(|param| Handler::handle_identifier(self, &param.name));
        self.visit_block_statement(&f.body);
    }

    fn visit_expression(&mut self, exp: &Expression) {
        Handler::handle_expression_start( self, exp);
        match exp {
            Expression::NumericLiteral(ref n) => Handler::handle_numeric_literal(self, n),
            Expression::UpdateExpression(ref u) => self.visit_update_expression(u),
            Expression::BinaryExpression(ref b) => self.visit_binary_expression(b),
            Expression::UnaryExpression(ref u) => self.visit_unary_expression(u),
            Expression::MemberExpression(ref m) => self.visit_member_expression(m),
            Expression::StringLiteral(ref s) => Handler::handle_string_literal(self, s),
            Expression::Identifier(ref id) => Handler::handle_identifier(self, &id.name),
            Expression::CallExpression(ref c) => self.visit_call_expression(c),
            Expression::AssignmentExpression(ref e) => self.visit_assignement_expression(e),
            Expression::LogicalExpression(ref l) => self.visit_logical_expression(l),
        }
        Handler::handle_expression_end(self);
    }

    fn visit_option_expression(&mut self, e: &Option<Expression>) {
        if let Some(s) = &e {
            self.visit_expression(&s)
        }
    }

    fn visit_expression_statement(&mut self, s: &ExpressionStatement) {
        self.visit_expression(&s.expression);
    }

    fn visit_binary_expression(&mut self, e: &BinaryExpression) {
        if let Some(ex) = &e.extra {
            Handler::handle_start_extra(self, ex.parenthesized);
        }
        self.visit_expression(&e.right);
        Handler::handle_num_operator(self, &e.operator);
        self.visit_expression(&e.left);

        if let Some(ex) = &e.extra {
            Handler::handle_end_extra(self, ex.parenthesized);
        }
    }

    fn visit_unary_expression(&mut self, u: &UnaryExpression) {
        /* TODO OPERATOR
                self.out.push_str(&u.operator.as_str());
        */
        self.visit_expression(&u.argument);
    }

    fn visit_assignement_expression(&mut self, e: &AssignmentExpression) {
        Handler::handle_assignement_expression(self, &e);
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
}

