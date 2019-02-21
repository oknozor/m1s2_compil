use std::fs;
use crate::ast::statement::Statement;
use crate::ast::statement::RootNode;


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
