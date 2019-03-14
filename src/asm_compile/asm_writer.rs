use crate::ast::statement::Statement;
use crate::asm_compile::ASM_INIT;
use crate::visitor::Visitor;
use crate::asm_compile::RAX;
use crate::asm_compile::RBX;
use crate::asm_compile::Register;
use crate::asm_compile::*;
use crate::writer::TAB;
use crate::writer::NEW_LINE;
use crate::token::token::Token;
use crate::token::token::Literal;
use crate::token::token::Operator;
use crate::token::token::Token::LiteralToken;
use std::collections::HashMap;
use crate::token::token::Node;
use crate::ast::statement::Statement::*;
use crate::token::token::Token::OperatorToken;

pub struct ASMWriter<'printer> {
    pub out: &'printer mut String,
    pub reg: Register,
    pub functions: HashMap<String, Vec<Node>>,
    pub vars: HashMap<String, Vec<Token>>,
    pub main: Vec<Token>,
}

impl<'printer> ASMWriter<'printer> {
    pub fn build(&mut self, ast: Vec<Box<Statement>>) {
        let mut ast_mut = ast.clone();

        let mut indexes_to_remove = vec![];
        ast_mut.iter_mut().enumerate().for_each(|(i, statement)| {
            if let box VariableDeclaration(var) = statement {
                self.visit_variable_declaration(var);
                indexes_to_remove.push(i);
            };
        });

        indexes_to_remove.iter().for_each(|i| { ast_mut.remove(*i); });
        let mut indexes_to_remove = vec![];

        ast.iter().enumerate().for_each(|(i, statement)| {
            if let box FunctionDeclaration(function) = statement {
                self.visit_function_declaration(function);
                indexes_to_remove.push(i);
            };
        });

        indexes_to_remove.iter().for_each(|i| { ast_mut.remove(*i); });

        ast.iter().enumerate().for_each(|(i, statement)| {
            self.visit_statement(statement);
        });
    }

    pub fn append(&mut self, word: &str) {
        self.out.push_str(word)
    }

    pub fn change_register(&mut self) {
        if let Register::RAX = self.reg {
            self.reg = Register::RBX;
        } else {
            self.reg = Register::RAX;
        }
    }

    pub fn write_asm_op(&mut self, str_op: &str) {
        self.append(TAB);
        match str_op {
            "+" => {
                self.append(ASM_ADD);
                self.append(&format!("{}%{}, %{}", TAB, RAX, RBX));
            }
            "-" => {
                self.append(ASM_SUB);
                self.append(&format!("{}%{}, %{}", TAB, RBX, RAX));
            }
            "/" => {
                self.append(ASM_DIV);
                self.append("cqto");
                self.append(&format!("{} %{}", TAB, RBX));
                self.append(NEW_LINE)
            }
            "*" => {
                self.append(ASM_MUL);
                self.append(&format!("{} %{}", TAB, RBX));
                self.append(NEW_LINE)
            }
            _ => println!("{} not found", str_op)
        }
    }

    pub fn append_lit(&mut self, lit: &str) {
        self.change_register();
        self.append(&format!("\t{}\t${}, %{}", ASM_MOVE, lit, self.reg.to_string()))
    }

    pub fn postfix_to_asm(&mut self, postfix_expression: &mut Vec<Token> ) {
        let mut postfix_expression = postfix_expression.clone();
        postfix_expression.reverse();


        while !postfix_expression.is_empty() {
            if let Some(LiteralToken(lit)) = postfix_expression.pop() {
                self.append_lit(&lit.to_string());
                self.append(NEW_LINE);
            };

            if let Some(OperatorToken(op)) = postfix_expression.pop() {
                self.write_asm_op(op.as_str());
                self.append(NEW_LINE);
            };
        };
    }

}
