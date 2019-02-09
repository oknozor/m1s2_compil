use std::fs;
use crate::ast::statement::Statement;

/* Todo: explore lifetimes capabilities to use ref instead of cloned data and Box ref */
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RootNode {
    Program(Program),
    File(File),
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub program: Program,
}

#[derive(Serialize, Deserialize)]
pub struct Program {
    pub body: Vec<Box<Statement>>
}

impl RootNode {
    pub fn get_program_root(&self) -> Option<Vec<Box<Statement>>> {
        match self {
            RootNode::Program(p) => Some( p.body.to_owned()),
            RootNode::File(ref f) => Some(f.program.body.to_owned()),
            _ => None
        }
    }
}

// todo: Move this into a dedicated file module
fn read_file(path: &str) -> String {
    let filename = path;
    fs::read_to_string(filename)
        .expect(format!("Failed to read file with path : {}", path).as_str())
        .clone()
}

pub fn deserialize_json(path: &str) -> RootNode {
    let file = read_file(path);
    let program: RootNode = serde_json::from_str(file.as_str())
        .expect("Unable to parse json file");
    program
}
