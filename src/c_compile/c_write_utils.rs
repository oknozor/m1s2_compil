use std::fmt::Display;
use std::io;

use crate::ast::expression::Expression;
use crate::ast::expression::Expression::*;
use crate::c_compile::c_write_utils::FunctionReserved::*;
use crate::ast::expression::BinaryExp;
use crate::ast::expression::Id;
use crate::ast::expression::Loc;
use super::*;

pub enum FunctionReserved {
    Print,
    Main,
    Add,
    Mull,
    GreaterThan,
    LessThan,
    Eq,
    NotEq,
}

impl From<String> for FunctionReserved {
    fn from(function_id: String) -> Self {
        let id_as_str = function_id.as_str();
        match id_as_str {
            STD_PRINT => Print,
            MAIN => Main,
            STD_ADD => Add,
            STD_MUL => Mull,
            STD_GT => GreaterThan,
            STD_LT => LessThan,
            STD_EQ => Eq,
            STD_NEQ => NotEq,
            _ => panic!("error")
        }
    }
}

pub fn bin_op_to_c(js_string: &str) -> &str {
    match js_string {
        ">" => STD_GT,
        "<" => STD_LT,
        "==" => STD_EQ,
        "!=" => STD_NEQ,
        "+" => STD_ADD,
        "*" => STD_MUL,
        "/" => STD_DIV,
        "-" => STD_SUB,
        "||" => "||",
        _ => {
            print!("{}", js_string);

            panic!("Unkown javascript token")
        }
    }
}

/// Generate a binary expression from special assign operators
pub fn assign_to_c(identifier: Id, js_op: &str, right: Box<Expression>, loc: Loc) -> BinaryExp {
    let operator = match js_op {
        "-=" => "-",
        "+=" => "+",
        "*=" => "*",
        "/=" => "/",
        "%=" => "%",
        _ => panic!("Unknown assignment operator")
    };

    BinaryExp {
        left: box Identifier(identifier),
        operator: operator.to_string(),
        extra: None,
        right,
        loc
    }
}

pub fn update_to_c(js_string: &str, var: &str) -> Result<String, io::Error> {
    match js_string {
        "++" => Ok(format!("increment(&{})", var)),
        "--" => Ok(format!("decrement(&{})", var)),
        _ => unimplemented!()
    }
}

trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}