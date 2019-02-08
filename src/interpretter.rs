use std::collections::HashMap;

use crate::ast::expression::*;
use crate::ast::statement::*;
use crate::runner::visitor::Visitor;
use crate::runner::handler::Handler;
use crate::token::token::Token;
use crate::token::binary_operator::BinaryOp;
use crate::token::assignement_operator::AssignOp;
use crate::token::update_operator::UpdateOp;
use crate::token::literal::Literal;
use crate::token::operator::*;
use std::rc::Rc;
use crate::runner::scope::Scope;
use crate::runner::scope::Parent;

pub struct Interpreter {
    pub global_scope: Parent,
}

impl Visitor for Interpreter {}

impl Handler for Interpreter {
}


pub struct Stack {
    pub queue: Vec<Token>,
    pub operators: Vec<BinaryOp>,
}


impl Stack {
    pub fn new() -> Self {
        Stack {
            queue: vec![],
            operators: vec![],
        }
    }
}
  /*  fn handle_statement(&mut self, s: &Statement){}

    // Calculate the rest of the operation on the operator stack
    // Push the result back to the queue
    fn handle_expression_end(&mut self) {
        while !self.current.get_operators().is_empty() {
            let pop_op = self.pop_from_op_stack();
            let a = self.pop_from_queue();
            let b = self.pop_from_queue();
            // Check if we are dealing with literal
            if let (Token::Literal(l1), Token::Literal(l2)) = (a, b) {
                let r = Token::calc(l1, l2, pop_op);
                self.push_to_queue(r);
            };
        }
    }

    fn handle_while_statement(&mut self, w: &WhileStatement) {
        self.visit_expression(&w.test);
        let mut test_token = self.pop_from_queue();
        while test_token.as_literal().as_bool() {
            self.visit_statement(&w.body);
            self.visit_expression(&w.test);
            test_token = self.pop_from_queue();
        }
    }

    fn handle_binary_expression(&mut self, e: &BinaryExpression) {
        if let Some(ex) = &e.extra {
            Handler::handle_start_extra(self, ex.parenthesized);
        };
        self.visit_expression(&e.right);
        Handler::handle_binary_operator(self, &e.operator);
        self.visit_expression(&e.left);

        if let Some(ex) = &e.extra {
            Handler::handle_end_extra(self, ex.parenthesized);
        };
        Handler::handle_expression_end(self);
    }

    fn handle_assignment_expression(&mut self, e: &AssignmentExpression) {
        let identifier = match &e.left {
            box Expression::Identifier(id) => &id.name,
            _ => panic!("Invalid left member in assignement!"),
        };

        self.handle_identifier(identifier);
        let left = self.pop_from_queue();

        self.visit_expression(&e.right);
        let right = self.pop_from_queue();

        let operator = AssignOp::from_str(&e.operator);
        let operator = Token::AssingOp(operator);

        if let (
            Token::Literal(left),
            Token::Literal(right)
        ) = (left, right) {
            let assigned_value = &mut Token::assign(left, right, operator);
            self.update_local_var(identifier, assigned_value);
        }
        self.print_local_scope();
        self.print_out_queue();
    }

    fn handle_update_expression(&mut self, e: &UpdateExpression) {
        if let box Expression::Identifier(identifier) = &e.argument {
            let name = &identifier.name;
            self.handle_identifier(name);
            let operator = UpdateOp::from_str(&e.operator);
            let operator = Token::UpdateOp(operator);
            let value = self.pop_from_queue();
            if let Token::Literal(value) = value {
                let updated = &mut Token::update(value, operator);
                self.update_local_var(name, updated);
            }
            self.pop_from_queue();
        };
    }

    fn handle_call_expression(&mut self, func: &CallExpression) {
        self.visit_expression(&func.callee);
    }

    #[allow(unused)]
    fn handle_identifier(&mut self, identifier: &String) {
        // First local scope
        if let loc_token = self.current.get_child_by_name(identifier) {
            let value = self.current.get_child_by_name(identifier);
            self.push_to_queue(value.clone());
        };

        // Then global
        if let Some(glob_token) = self.current.parent.get(identifier) {
            let value: Token = self.current.parent.get(identifier).unwrap().clone();
            self.push_to_queue(value);
        }
    }

    // while operator on the stack as precedence over the one being evaluated
    // and it's not a parenthesis, pop it and push it to queue
    fn handle_binary_operator(&mut self, operator: &str) {
        self.print_local_scope();
        let operator = BinaryOp::from_str(operator);

        while !self.current.get_operators().is_empty()
            && (BinaryOp::get_precedence(self.current.get_operators().last().unwrap(), &operator)
            && self.current.get_operators().last().unwrap() != &BinaryOp::LeftParenthesis) {
            let pop_op = self.current.get_operators().pop().unwrap();
            self.push_to_queue(Token::BinaryOp(pop_op));
        };

        self.current.get_operators().push(operator);
    }

    // push literals to the queue
    fn handle_string_literal(&mut self, literal: &JSLiteral<String>) {
        let literal = Literal::from(literal.value.clone());
        let string_token = Token::Literal(literal);
        self.push_to_queue(string_token);
    }

    fn handle_numeric_literal(&mut self, literal: &JSLiteral<i64>) {
        let literal = Literal::from(literal.value);
        self.push_to_queue(Token::Literal(literal));
    }

    // push left parenthesis to the operator stack
    fn handle_start_extra(&mut self, parenthesized: bool) {
        if parenthesized {
            self.current.get_operators().push(BinaryOp::LeftParenthesis);
        }
    }

    // Search for a matching left parenthesis, calculate the expression in between
    // and push it back to the token queue
    fn handle_end_extra(&mut self, parenthesized: bool) {
        if parenthesized {
            while self.current.get_operators().last().unwrap() != &BinaryOp::LeftParenthesis {
                let pop_op = self.pop_from_op_stack();
                let a = self.pop_from_queue();
                let b = self.pop_from_queue();
                if let (Token::Literal(l1), Token::Literal(l2)) = (a, b) {
                    let r = Token::calc(l1, l2, pop_op);
                    self.push_to_queue(r);
                }
            }
            self.pop_from_op_stack(); // pop left parenthesis
        }
    }

    fn handle_if_statement(&mut self, i: &IfStatement) {
        self.visit_expression(&i.test);
        let test_result = self.pop_from_queue();
        if Token::as_literal(&test_result).as_bool() {
            self.visit_statement(&i.consequent);
        } else {
            self.visit_statement(&i.alternate.clone().unwrap());
        }
    }


    fn handle_variable_declarator(&mut self, var: &VariableDeclarator) {
        if let Some(exp) = &var.init {
            self.visit_expression(&exp);
            let value = self.pop_from_queue();
            self.local_scope.insert(var.id.name.clone(), value);
        } else {
            self.local_scope
                .insert(var.id.name.clone(), Token::Undefined);
        };
        self.print_global_scope();
        self.print_local_scope();
    }*/


// A hanful of utility function to manipulate the interpreter struct
/*#[allow(unused)]
impl Interpreter {
    pub fn get_identifier_value(&mut self, identifier: &String) -> Option<&Token> {
        // First local scope
        if self.local_scope.get(identifier).is_some() {
            self.local_scope.get(identifier)
        }

        // Then global
        else if self.global_scope.get(identifier).is_some() {
            self.global_scope.get(identifier)
        } else {
            None
        }
    }

    pub fn push_to_queue(&mut self, token: Token) {
        println!("push {}", token.to_string());
        self.stack.out_queue.push(token);
    }

    pub fn pop_from_queue(&mut self) -> Token {
        println!("pop");
        let result = self.stack.out_queue.pop();
        if let Some(token) = result {
            token
        } else {
            panic!("Expected); a token, found None")
        }
    }

    pub fn push_to_op_stack(&mut self, token: Token) {
        let token_as_bin_op: BinaryOp = token.into();
        println!("{}", token_as_bin_op.to_string());
        self.current.get_operators().push(token_as_bin_op);
    }

    pub fn pop_from_op_stack(&mut self) -> Token {
        let result = self.current.get_operators().pop();
        if let Some(binop) = result {
            println!("{}", binop.to_string());
            Token::BinaryOp(binop)
        } else {
            panic!("Expected a bin op, found None")
        }
    }

    fn update_local_var(&mut self, identifier: &String, value: &mut Token) {
        if let Some(v) = self.local_scope.get_mut(identifier) {
            *v = value.clone();
        };
    }

    fn print_out_queue(&mut self) {
        print!("Queue [ ");
        self.stack.out_queue.iter().for_each(|x| print!("{} ", x));
        print!(" ]");
        println!();
    }

    fn print_op_stack(&mut self) {
        print!("Op stack [ ");
        self.stack
            .operator_stack
            .iter()
            .for_each(|x| print!("{} ", x));
        print!(" ]");
        println!();
    }

    fn print_local_scope(&mut self) {
        print!("\nCurrent scope holding: [");
        self.local_scope
            .iter()
            .for_each(|reference| print!("{}:{}, ", reference.0, reference.1));
        print!("]");
        println!()
    }

    fn print_global_scope(&mut self) {
        print!("\nGlobal scope holding: [");
        self.global_scope
            .iter()
            .for_each(|reference| print!("{}:{}, ", reference.0, reference.1));
        print!("]");
    }

}*/

