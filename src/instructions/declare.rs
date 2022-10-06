use regex::Regex;
use crate::py2cpp::{Instruction, Type, Library};

const INTEGER: &str = r"(?m)^([a-zA-Z][a-zA-Z0-9]*)\s*=\s*[+-]?\s*(\d+)$";

pub fn py2code(content: &str) -> Option<(Vec<Instruction>, Vec<Library>)> {
    let re_int = Regex::new(INTEGER).unwrap();
    let cap_int = re_int.captures(content);

    match cap_int {
        Some(data) => {
            let type_ = Type::Int;
            let name = data.get(1).unwrap().as_str().to_string();
            let value = Some(data.get(2).unwrap().as_str().to_string());
            let instruction = Instruction::CreateVar { type_, name, value };
            Some((vec![instruction], vec![]))
        },
        None => None
    }
}

pub fn code2cpp(type_: &Type, name: &String, option: &Option<String>) -> String {
    match option {
        Some(value) => {
            match type_ {
                Type::Int => format!("int {} = {};", name, value),
                Type::String => format!("string {} = {};", name, value),
                _ => String::new()
            }
        },
        None => {
            match type_ {
                Type::Int => format!("int {};", name),
                Type::String => format!("string {};", name),
                _ => String::new()
            }
        }
    }
}
