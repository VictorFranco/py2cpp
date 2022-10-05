use regex::Regex;
use crate::py2cpp::Instruction;
use crate::py2cpp::Type;

const INTEGER: &str = r"(?m)^([a-zA-Z][a-zA-Z0-9]*)\s*=\s*[+-]?\s*(\d+)$";

pub fn py2code(content: &str) -> Option<Instruction> {
    let re_int = Regex::new(INTEGER).unwrap();
    let cap_int = re_int.captures(content);

    match cap_int {
        Some(data) => {
            let type_ = Type::Int;
            let name = data.get(1).unwrap().as_str().to_string();
            let value = data.get(2).unwrap().as_str().to_string();
            Some(Instruction::CreateVar { type_, name, value })
        },
        None => None
    }
}

pub fn code2cpp(type_: &Type, name: &String, value: &String) -> String {
    match type_ {
        Type::Int => format!("int {} = {};", name, value),
        _ => String::new()
    }
}
