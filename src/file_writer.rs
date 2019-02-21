use crate::pretty_printer::PrettyPrinter;
use crate::file_writer::CReserved::*;
use crate::file_writer::FunctionReserved::*;

pub static INCLUDES: &'static str = "\n#include \"c_datalib/print.c\
                                \"\n#include \"c_datalib/print.c\"\n";


pub enum CReserved {
    Databox,
    While,
    For,
    Switch,
    Case,
    Continue,
    Break,
    Default,
    Return,
}


impl CReserved {
    pub fn as_str(&self) -> &str {
        match self {
            Databox => "databox",
            While => "while",
            For => "for",
            Switch => "switch",
            Case => "case",
            Continue => "continue",
            Break => "break",
            Default => "default",
            Return => "return",
        }
    }
}


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
        match function_id {
          String::from("print") => Print,
          String::from("main") => Main,
          String::from("add") => Add,
          String::from("mull") => Mull,
          String::from("gt") => GreaterThan,
          String::from("lt") => LessThan,
          String::from("eq") => Eq,
        }
    }
}