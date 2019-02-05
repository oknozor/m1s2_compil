use crate::ast::expression::{Expression, AssignmentExpression, CallExpression};
use crate::ast::statement::{WhileStatement, VariableDeclarator};
use crate::runner::visitor::Visitor;
use crate::runner::handler::Handler;
use crate::ast::literal::JSLiteral;
use crate::ast::operator::BinaryOp;
use crate::ast::token::Token;
use crate::ast::literal::Literal;
use std::collections::HashMap;

pub struct Interpretter {
    pub global_scope: HashMap<String, Token>,
    pub local_scope: HashMap<String, Token>,
    pub stack: Stack,
}

pub struct Stack {
    pub out_queue: Vec<Token>,
    pub operator_stack: Vec<BinaryOp>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            out_queue: vec![],
            operator_stack: vec![],
        }
    }
}

impl Visitor for Interpretter {}

impl Handler for Interpretter {
    fn handle_program_root(&mut self) {
        self.global_scope.insert(String::from("print"), Token::Undefined);
    }

    fn handle_variable_declarator(&mut self, var: &VariableDeclarator) {
        self.visit_option_expression(&var.init);
        let value = self.pop_from_queue();
        self.local_scope.insert(var.id.name.clone(), value);
    }

    // Calculate the rest of the operation on the operator stack
    // Push the result back to the queue
    fn handle_expression_end(&mut self) {
        while !self.stack.operator_stack.is_empty() {
            let pop_op = self.pop_from_op_stack();
            let a = self.pop_from_queue();
            let b = self.pop_from_queue();
            // Check if we are dealing with literal
            if let (Token::Literal(l1), Token::Literal(l2)) = (a, b) {
                let r = Token::calc(l1, l2, pop_op);
                self.push_to_queue(r);
            }
        };
    }

    // Just pushing left parenthesis to the operator stack
    fn handle_start_extra(&mut self, parenthesized: bool) {
        if parenthesized {
            self.stack.operator_stack.push(BinaryOp::LeftParenthesis);
        }
    }

    // Search for a matching left parenthesis, calculate the expression in between
    // and push it back to the token queue
    fn handle_end_extra(&mut self, parenthesized: bool) {
        if parenthesized {
            while self.stack.operator_stack.last().unwrap() != &BinaryOp::LeftParenthesis {
                let pop_op = self.pop_from_op_stack();
                let a = self.pop_from_queue();
                let b = self.pop_from_queue();
                if let (Token::Literal(l1), Token::Literal(l2)) = (a, b) {
                    let r = Token::calc(l1, l2, pop_op);
                    self.push_to_queue(r);
                }
            };
            self.pop_from_op_stack(); // pop left parenthesis
        }
    }

    fn handle_assignement_expression(&mut self, e: &AssignmentExpression) {
        let identifier = match &e.left {
            box Expression::Identifier(id) => &id.name,
            _ => panic!("Invalid left member in assignement!")
        };
        self.visit_expression(&e.left);
        self.visit_expression(&e.right);
        let value = &mut self.pop_from_queue();
        if let Some(v) = self.local_scope.get_mut(identifier) {
            *v = value.clone();
        };
    }

    // Just push literals to the queue
    fn handle_string_literal(&mut self, literal: &JSLiteral<String>) {
        let literal = Literal::from(literal.value.clone());
        let string_token = Token::Literal(literal);
        self.push_to_queue(string_token);
    }

    fn handle_numeric_literal(&mut self, literal: &JSLiteral<i64>) {
        let literal = Literal::from(literal.value);
        self.push_to_queue(Token::Literal(literal));
    }

    #[allow(unused)]
    fn handle_identifier(&mut self, identifier: &String) {
        // First local scope
        if let Some(loc_token) = self.local_scope.get(identifier) {
            let value = self.local_scope.get(identifier).unwrap().clone();
            let m_value = &mut value.clone();
            self.push_to_queue(value);
            self.local_scope.get_mut(identifier).replace(m_value);
        };


        // Then global
        if let Some(glob_token) = self.global_scope.get(identifier) {
            let value: Token = self.global_scope.get(identifier).unwrap().clone();
            let m_value = &mut value.clone();
            self.stack.out_queue.push(value);
            self.local_scope.get_mut(identifier).replace(m_value);
        }
    }


    fn handle_num_operator(&mut self, operator: &str) {
        let operator = BinaryOp::from_str(operator);
        // while operator on the stack as precedence over the one being evaluated
        // and it's not a parenthesis, pop it and push it to queue
        while !self.stack.operator_stack.is_empty()
            && (BinaryOp::get_precedence(self.stack.operator_stack.last().unwrap(), &operator)
            && self.stack.operator_stack.last().unwrap() != &BinaryOp::LeftParenthesis) {
            let pop_op = self.stack.operator_stack.pop().unwrap();
            self.push_to_queue(Token::BinaryOp(pop_op));
        }
        self.stack.operator_stack.push(operator);
    }

    fn handle_call_expression(&mut self, func: &CallExpression) {
        self.visit_expression(&func.callee);
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
}


#[allow(unused)]
impl Interpretter {
    pub fn push_to_queue(&mut self, token: Token) {
        self.stack.out_queue.push(token);
    }

    pub fn pop_from_queue(&mut self) -> Token {
        let result = self.stack.out_queue.pop();
        if let Some(token) = result {
            token
        } else {
            panic!("Expected a token, found None")
        }
    }

    pub fn push_to_op_stack(&mut self, token: Token) {
        let token_as_bin_op: BinaryOp = token.into();
        self.stack.operator_stack.push(token_as_bin_op);
    }

    pub fn pop_from_op_stack(&mut self) -> Token {
        let result = self.stack.operator_stack.pop();
        if let Some(binop) = result {
            Token::BinaryOp(binop)
        } else {
            panic!("Expected a bin op, found None")
        }
    }

    fn print_out_queue(&mut self) {
        print!("Queue [ ");
        self.stack.out_queue.iter().for_each(|x| print!("{} ", x));
        print!(" ]");
        println!();
    }

    fn print_op_stack(&mut self) {
        print!("Op stack [ ");
        self.stack.operator_stack.iter().for_each(|x| print!("{} ", x));
        print!(" ]");
        println!();
    }

    fn print_local_scope(&mut self) {
        print!("\nCurrent scope holding: [");
        self.local_scope.iter().for_each(|reference| print!("{}:{}, ", reference.0, reference.1));
        print!("]");
        println!()
    }
    fn print_global_scope(&mut self) {
        print!("\nGlobal scope holding: [");
        self.global_scope.iter().for_each(|reference| print!("{}:{}, ", reference.0, reference.1));
        print!("]");
    }
}
