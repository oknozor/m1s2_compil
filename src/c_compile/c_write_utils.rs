use std::fmt::Display;
use std::io;

use crate::ast::expression::Expression;
use crate::ast::expression::Expression::*;
use crate::c_compile::c_write_utils::FunctionReserved::*;

pub const STD_LIB: &[&'static str] = &["add", "mull", "div", "eq", "print", "sub"];
pub const INCLUDES: &'static str = "   \n#include \"print.h\
                                        \"\n#include \"databox.h\"\n";
pub const MAIN: &'static str = "\nvoid main() {\n";

pub const PARENTHESIS_LEFT: &'static str = "(";
pub const PARENTHESIS_RIGHT: &'static str = ")";
pub const BRACKET_LEFT: &'static str = "{";
pub const BRACKET_RIGHT: &'static str = "}";
pub const EQ: &'static str = "=";
pub const NEW_LINE: &'static str = "\n";
pub const SEMI_COL: &'static str = ";";
pub const COMA: &'static str = ",";

pub const WHILE: &'static str = "while";
pub const FOR: &'static str = "for";
pub const IF: &'static str = "if";
pub const ELSE: &'static str = "else";

pub const NEW: &'static str = "new";
pub const SWITCH: &'static str = "switch ";
pub const CASE: &'static str = "case ";
pub const BREAK: &'static str = "break ";
pub const CONTINUE: &'static str = "continue ";
pub const DEFAULT: &'static str = "default ";
pub const RETURN: &'static str = "return ";
pub const DATABOX: &'static str = "databox ";

pub enum FunctionReserved {
    Print,
    Main,
    Add,
    Mull,
    GreaterThan,
    LessThan,
    Eq,
}

impl From<String> for FunctionReserved {
    fn from(function_id: String) -> Self {
        let id_as_str = function_id.as_str();
        match id_as_str {
            "print" => Print,
            "main" => Main,
            "add" => Add,
            "mull" => Mull,
            "gt" => GreaterThan,
            "lt" => LessThan,
            "eq" => Eq,
            _ => panic!("error")
        }
    }
}

pub fn bin_op_to_c(js_string: &str) -> &str {
    match js_string {
        ">" => "gt",
        "<" => "lt",
        "==" => "eq",
        "+" => "add",
        "*" => "mul",
        "/" => "div",
        "-" => "sub",
        "||" => "or",
        _ => panic!("Unkown javascript token")
    }
}

pub fn update_to_c(js_string: &str, var: &str) -> Result<String, io::Error> {
    match js_string {
        "++" => Ok(format!("increment({})", var)),
        "--" => Ok(format!("decrement({})", var)),
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