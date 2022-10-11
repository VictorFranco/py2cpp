use regex::Regex;
use crate::py2cpp::{Instruction, Type, Library, CUSTOM_FUN, INTEGER, STRING, Value};
use crate::instructions::{custom_fun, input};

const DECLARE: &str = r##"(?m)^([a-zA-Z][a-zA-Z0-9]*)\s*=\s*(\d+|"[a-zA-Z0-9: ]*"|([a-zA-Z][a-zA-Z0-9]*)\(.*\))$"##;

pub fn py2code(body: &mut Vec<Instruction>, content: &str) -> Option<(Vec<Instruction>, Vec<Library>)> {
    let re_dec = Regex::new(DECLARE).unwrap();
    let re_int = Regex::new(INTEGER).unwrap();
    let re_str = Regex::new(STRING).unwrap();
    let re_fun = Regex::new(CUSTOM_FUN).unwrap();
    let cap_dec = re_dec.captures(content);

    match cap_dec {
        Some(data) => {
            let mut libraries = Vec::new();
            let mut instructions = Vec::new();
            let var_name = data.get(1).unwrap().as_str();
            let mut type_ = Type::Undefined;
            let mut value = Value::None;
            let content = data.get(2).unwrap().as_str();
            if re_fun.is_match(content) {
                let option = custom_fun::py2code(body, content);
                match option {
                    Some((instructions, _libraries)) => {
                        let instruction = &instructions[0];
                        match instruction {
                            Instruction::CallFun { name, arguments } => {
                                let name = name.to_string();
                                let arguments = arguments.to_vec();
                                value = Value::CallFun { name, arguments };
                            },
                           _ => {}
                        }
                    },
                    None => {}
                }
                let fun_name = data.get(3).unwrap().as_str();
                if fun_name == "input" {
                    type_ = Type::String;
                    value = Value::None;
                    let (mut input_instructions, mut input_libraries) = input::py2code(var_name, content, "true").unwrap();
                    libraries.append(&mut input_libraries);
                    instructions.append(&mut input_instructions);
                }
            }
            else {
                if re_int.is_match(content) {
                    type_ = Type::Int;
                }
                if re_str.is_match(content) {
                    type_ = Type::String;
                }
                value = Value::ConstValue(content.to_string());
            }
            let name = var_name.to_string();
            let instruction = Instruction::CreateVar { type_, name, value };
            instructions.insert(0, instruction);
            Some((instructions, libraries))
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
