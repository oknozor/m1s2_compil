use crate::ast::statement::ExpressionStatement;
use std::fs;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Node {
    Program(Program),
    File(File),
    ExpressionStatement(ExpressionStatement),
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub program: Program,
}

#[derive(Serialize, Deserialize)]
pub struct Program {
    pub body: Vec<Node>
}

fn read_file(path: &str) -> String {
    let filename = path;
    fs::read_to_string(filename)
        .expect("failed to read file").clone()
}

pub fn deserialize_json(path: &str) -> Node {
    let file = read_file(path);
    let program: Node = serde_json::from_str(file.as_str())
        .expect("Unable to parse json file");
    program
}