use std::fs;
use crate::ast::statement::RootStatement;


fn read_file(path: &str) -> String {
    let filename = path;
    fs::read_to_string(filename)
        .expect(format!("Failed to read file with path : {}", path).as_str())
        .clone()
}

pub fn deserialize_json_file(path: &str) -> RootStatement {
    let file = read_file(path);
    let program: RootStatement = serde_json::from_str(file.as_str())
        .expect("Unable to parse json file");
    program
}

pub fn deserialize_json(json: &str) -> RootStatement {
    let program: RootStatement = serde_json::from_str(json)
        .expect("Unable to parse json file");
    program
}
