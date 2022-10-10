use regex::Regex;
use crate::py2cpp::{Instruction, Type, Library, CUSTOM_FUN, INTEGER, STRING, Value};
use crate::instructions::custom_fun;

const DECLARE: &str = r##"(?m)^([a-zA-Z][a-zA-Z0-9]*)\s*=\s*(\d+|"[a-zA-Z0-9: ]*"|[a-zA-Z][a-zA-Z0-9]*\(.*\))$"##;

pub fn py2code(body: &mut Vec<Instruction>, content: &str) -> Option<(Vec<Instruction>, Vec<Library>)> {
    let re_dec = Regex::new(DECLARE).unwrap();
    let re_int = Regex::new(INTEGER).unwrap();
    let re_str = Regex::new(STRING).unwrap();
    let re_fun = Regex::new(CUSTOM_FUN).unwrap();
    let cap_dec = re_dec.captures(content);

    match cap_dec {
        Some(data) => {
            let mut type_ = Type::Undefined;
            let name = data.get(1).unwrap().as_str().to_string();
            let value = data.get(2).unwrap().as_str();
            let value = if re_fun.is_match(value) {
                type_ = Type::Undefined;
                let info = custom_fun::py2code(body, value);
                match info {
                    Some((instructions, _libraries)) => {
                        let instruction = &instructions[0];
                        match instruction {
                            Instruction::CallFun { name, arguments } => {
                                let name = name.to_string();
                                let arguments = arguments.to_vec();
                                Value::CallFun { name, arguments }
                            },
                           _ => Value::None
                        }
                    },
                    None => Value::None
                }
            }
            else {
                if re_int.is_match(value) {
                    type_ = Type::Int;
                }
                if re_str.is_match(value) {
                    type_ = Type::String;
                }
                Value::ConstValue(value.to_string())
            };
            let instruction = Instruction::CreateVar { type_, name, value };
            Some((vec![instruction], vec![]))
        },
        None => None
    }
}

pub fn code2cpp(type_: &Type, name: &String, value: &Value) -> String {
    let name_var = name;
    match value {
        Value::ConstValue(value) => {
            match type_ {
                Type::Int => format!("int {} = {};", name, value),
                Type::String => format!("string {} = {};", name, value),
                _ => String::new()
            }
        },
        Value::CallFun { name, arguments } => {
            let fun = custom_fun::code2cpp(name, arguments);
            match type_ {
                Type::Int => format!("int {} = {}", name_var, fun),
                Type::String => format!("string {} = {}", name_var, fun),
                Type::Undefined => format!("undefined {} = {}", name_var, fun),
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
