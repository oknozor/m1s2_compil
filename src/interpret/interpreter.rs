use crate::ast::statement::Statement;

pub struct Interpreter {
    statements: Vec<Box<Statement>>
}