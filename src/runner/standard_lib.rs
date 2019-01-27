use crate::ast::token::Token;
use std::ffi::CString;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

#[derive(Clone)]
pub enum Function {
    Print
}

impl Function {
    pub fn exec(&self, args: Vec<Token>) {
        match self {
            Function::Print => {
                args.iter().for_each(|arg| print!("{}" ,arg.to_string()))
            }

            _ => eprintln!("No implemented!")
        }
    }
}
