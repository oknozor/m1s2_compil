/*
use crate::ast::statement::{Statement, ExpressionStatement, BlockStatement, WhileStatement,
                            VariableDeclarator, VariableDeclaration, IfStatement, ForStatement,
                            BreakStatement, ContinueStatement, FunctionDeclaration, ReturnStatement,
                            SwitchStatement};

use crate::ast::expression::{Expression, CallExpression,
                             LogicalExpression, UnaryExpression,
                             MemberExpression};
use crate::runner::handler::Handler;
use crate::runner::scope::Scoped;
use crate::runner::scope::Scope;

pub fn with_loc() -> bool {
    true
}

// statement
pub fn visit_statement(scope: &mut Scope, s: &Statement) {
    match s {
        Statement::BlockStatement(b) => visit_block_statement(scope, b),
        Statement::VariableDeclaration(v) => visit_variable_declaration(scope, v),
        Statement::ExpressionStatement(e) => (),
        Statement::WhileStatement(w) => visit_while_statement(scope, w),
        Statement::IfStatement(i) => visit_if_statement(scope, i),
        Statement::SwitchStatement(s) => visit_switch_statement(scope, s),
        Statement::ForStatement(f) => visit_for_statement(scope, f),
        Statement::BreakStatement(b) => visit_break_statement(scope, b),
        Statement::ContinueStatement(c) => visit_continue_statement(scope, c),
        Statement::ReturnStatement(r) => visit_return_statement(scope, r),
        Statement::FunctionDeclaration(f) => visit_function_declaration(scope, f),
        _ => ()
    }
}

pub fn visit_block_statement(scope: &mut Scope, s: &BlockStatement) {
    s.body.iter().for_each(|statement| visit_statement(scope, statement));
}

pub fn visit_while_statement(scope: &mut Scope, w: &WhileStatement) {
    scope.childs.push(Scope::<WhileStatement>::new_anon(&Some(scope), w));
}

pub fn visit_variable_declaration(scope: &mut Scope, v: &VariableDeclaration) {
    let mut scopes = vec![];
    v.declarations.iter()
     .for_each(|declaration| {
         let name = declaration.id.name.clone();
         let child = Scope::<Statement>::new_named(name, &Some(*scope), Statement::VariableDeclarator(declaration.clone()));
         scope.childs.push(child)
     });
}

pub fn visit_if_statement(scope: &mut Scope, i: &IfStatement) {
    scope.childs.push(Scope::new_anon(scope, i));
}

pub fn visit_switch_statement(scope: &mut Scope, s: &SwitchStatement) {
    scope.visit_expression(&s.discriminant);
    s.cases.iter().for_each(|case| {
        match &case.test {
            Some(s) => {
                scope.visit_expression(s);
            }
            None => ()
        };
        case.consequent.iter()
            .for_each(|cons| scope.visit_statement(&cons));
    });
}

pub fn visit_for_statement(scope: &mut Scope, f: &ForStatement) {
    scope.visit_option_expression(&f.init);
    scope.visit_option_expression(&f.test);
    scope.visit_option_expression(&f.update);
    scope.visit_statement(&f.body);
}

pub fn visit_break_statement(scope: &mut Scope, f: &BreakStatement) {
    if let Some(s) = &f.label {}
}

pub fn visit_continue_statement(scope: &mut Scope, c: &ContinueStatement) {
    match &c.label {
        Some(s) => (),
        None => ()
    }
}

pub fn visit_return_statement(scope: &mut Scope, r: &ReturnStatement) {
    scope.visit_option_expression(&r.argument);
}


pub fn visit_function_declaration(scope: &mut Scope, f: &FunctionDeclaration) {
    let new_child = Scope::new_named(f.id.name.clone(), scope, &f.params);
    scope.visit_block_statement(&f.body);
}

pub fn visit_expression(scope: &mut Scope, exp: &Expression) {
    match exp {
        Expression::NumericLiteral(ref n) => (),
        Expression::UpdateExpression(ref u) => (),
        Expression::BinaryExpression(ref b) => (),
        Expression::UnaryExpression(ref u) => scope.visit_unary_expression(u),
        Expression::MemberExpression(ref m) => scope.visit_member_expression(m),
        Expression::StringLiteral(ref s) => (),
        Expression::CallExpression(ref c) => scope.visit_call_expression(c),
        Expression::AssignmentExpression(ref e) => (),
        Expression::LogicalExpression(ref l) => scope.visit_logical_expression(l),
        _ => (),
    };
}

pub fn visit_option_expression(scope: &mut Scope, e: &Option<Expression>) {}

pub fn visit_expression_statement(scope: &mut Scope, s: &ExpressionStatement) {
    scope.visit_expression(&s.expression);
}


pub fn visit_unary_expression(scope: &mut Scope, u: &UnaryExpression) {
    *//* TODO OPERATOR
            scope.out.push_str(&u.operator.as_str());
    *//*
    scope.visit_expression(&u.argument);
}

pub fn visit_member_expression(scope: &mut Scope, m: &MemberExpression) {
    scope.visit_expression(&m.object);
    scope.visit_expression(&m.property);
}

pub fn visit_logical_expression(scope: &mut Scope, l: &LogicalExpression) {
    scope.visit_expression(&l.left);
    *//* TODO OPERATOR
            scope.out.push_str(&u.operator.as_str());
    *//*
    scope.visit_expression(&l.right);
}

pub fn visit_call_expression(scope: &mut Scope, e: &CallExpression) {
    scope.visit_expression(&e.callee);
    let last = e.arguments.len() - 1;
    e.arguments.iter().enumerate()
     .for_each(|(i, expression)| {
         scope.visit_expression(expression);
         if i != last {}
     });
}

*/
