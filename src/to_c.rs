

pub fn to_c(js_string: &str) -> &str {
    match js_string {
        ">" => "gt(",
        "<" => "lt",
        "==" => "eq",
        "+" => "add",
        "*" => "mul",
        "/" => "div",
        "-" => "sub",
        _ => panic!("Unkown javascript token")
    }
}

