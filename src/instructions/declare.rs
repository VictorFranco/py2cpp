use regex::Regex;
use crate::py2cpp::{Instruction, Type, Library, INTEGER, STRING, Value};

const DECLARE: &str = r##"(?m)^([a-zA-Z][a-zA-Z0-9]*)\s*=\s*(\d+|"[a-zA-Z0-9: ]*")$"##;

pub fn py2code(content: &str) -> Option<(Vec<Instruction>, Vec<Library>)> {
    let re_dec = Regex::new(DECLARE).unwrap();
    let re_int = Regex::new(INTEGER).unwrap();
    let re_str = Regex::new(STRING).unwrap();
    let cap_dec = re_dec.captures(content);

    match cap_dec {
        Some(data) => {
            let mut type_ = Type::Undefined;
            let name = data.get(1).unwrap().as_str().to_string();
            let value = data.get(2).unwrap().as_str();
            if re_int.is_match(value) {
                type_ = Type::Int;
            }
            if re_str.is_match(value) {
                type_ = Type::String;
            }
            let value = Value::ConstValue(value.to_string());
            let instruction = Instruction::CreateVar { type_, name, value };
            Some((vec![instruction], vec![]))
        },
        None => None
    }
}

pub fn code2cpp(type_: &Type, name: &String, value: &Value) -> String {
    match value {
        Value::ConstValue(value) => {
            match type_ {
                Type::Int => format!("int {} = {};", name, value),
                Type::String => format!("string {} = {};", name, value),
                _ => String::new()
            }
        },
        Value::CallFun { name: _, arguments: _ } => {
            match type_ {
                _ => String::new()
            }
        },
        Value::None => {
            match type_ {
                Type::Int => format!("int {};", name),
                Type::String => format!("string {};", name),
                _ => String::new()
            }
        }
    }
}
