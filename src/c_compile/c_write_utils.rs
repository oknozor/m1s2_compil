use std::fmt::Display;
use std::io;

use crate::ast::expression::Expression;
use crate::ast::expression::Expression::*;
use crate::c_compile::c_write_utils::FunctionReserved::*;
use crate::ast::expression::BinaryExp;
use crate::ast::expression::Id;
use crate::ast::expression::Loc;

pub const STD_ADD: &'static str = "add";
pub const STD_SUB: &'static str = "sub";
pub const STD_MUL: &'static str = "mul";
pub const STD_DIV: &'static str = "div";
pub const STD_EQ: &'static str = "eq";
pub const STD_NEQ: &'static str = "neq";
pub const STD_GT: &'static str = "gt";
pub const STD_LT: &'static str = "lt";
pub const STD_PRINT: &'static str = "print";

pub const STD_LIB: &[&'static str] = &["add", "mull", "div", "eq", "print", "sub"];
pub const INCLUDES: &'static str = "   \n#include \"print.c\
                                        \"\n#include \"databox.c\"\n";
pub const MAIN: &'static str = "\nint main() {\n";
pub const END: &'static str = "\nreturn 0;";

pub const PARENTHESIS_LEFT: &'static str = "(";
pub const PARENTHESIS_RIGHT: &'static str = ")";
pub const BRACKET_LEFT: &'static str = "{";
pub const BRACKET_RIGHT: &'static str = "}";
pub const EQ: &'static str = "=";
pub const NEW_LINE: &'static str = "\n";
pub const SEMI_COL: &'static str = ";";
pub const COL: &'static str = ":";
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