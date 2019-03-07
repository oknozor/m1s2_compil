pub mod c_write_utils;
pub mod c_writer;
pub mod c_visitor;

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
pub const INCLUDES: &'static str = "   \n#include \"print.h\"\n
                                         #include \"databox.h\"\n";
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
pub const NEW_DICT: &'static str = "new_object()";
pub const SWITCH: &'static str = "switch ";
pub const CASE: &'static str = "case ";
pub const BREAK: &'static str = "break ";
pub const CONTINUE: &'static str = "continue ";
pub const DEFAULT: &'static str = "default ";
pub const RETURN: &'static str = "return ";
pub const DATABOX: &'static str = "databox ";