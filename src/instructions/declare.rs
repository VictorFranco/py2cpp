use regex::Regex;
use crate::py2cpp::{Type, type2cpp, Value, Instruction, Library, get_libraries, NATIVE_FUNS, INTEGER, STRING, VECTOR, CUSTOM_FUN};
use crate::instructions::{custom_fun, input, int};

const DECLARE: &str = r##"(?m)^([a-zA-Z][a-zA-Z0-9]*)\s*=\s*(\d+|"[a-zA-Z0-9: ]*"|\[\]|([a-zA-Z][a-zA-Z0-9]*)\(.*\)?)$"##;

pub fn py2code(body: &mut Vec<Instruction>, content: &str) -> Option<(Vec<Instruction>, Vec<Library>)> {
    let re_dec = Regex::new(DECLARE).unwrap();
    let re_int = Regex::new(INTEGER).unwrap();
    let re_str = Regex::new(STRING).unwrap();
    let re_vec = Regex::new(VECTOR).unwrap();
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
                let fun = data.get(3).unwrap().as_str();
                if NATIVE_FUNS.contains(&fun) {
                    match fun {
                        "input" => {
                            let (mut input_instructions, mut input_libraries) = input::py2code(var_name, content, false).unwrap();
                            libraries.append(&mut input_libraries);
                            instructions.append(&mut input_instructions);
                            type_ = Type::String;
                        },
                        "int" => {
                            let (int_instructions, mut int_libraries) = int::py2code(content).unwrap();
                            libraries.append(&mut int_libraries);
                            match &int_instructions[0] {
                                Instruction::CallFun { name, arguments } => {
                                    let name = name.to_string();
                                    let arguments = arguments.to_vec();
                                    value = Value::CallFun { name, arguments };
                                },
                                _ => {}
                            }
                            type_ = Type::Int;
                        },
                        _ => {}
                    };
                }
                let option = custom_fun::py2code(body, content);
                match option {
                    Some((custom_instructions, mut custom_libraries)) => {
                        libraries.append(&mut custom_libraries);
                        let instruction = &custom_instructions[0];
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
            }
            else {
                if re_int.is_match(content) {
                    type_ = Type::Int;
                }
                if re_str.is_match(content) {
                    type_ = Type::String;
                }
                value = Value::ConstValue(content.to_string());
                if re_vec.is_match(content) {
                    type_ = Type::Vector(Box::new(Type::Undefined));
                    value = Value::None;
                    libraries = get_libraries(&["vector"]);
                }
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
    let var_name = name;
    match value {
        Value::ConstValue(value) | Value::UseVar(value) => {
            format!("{} {} = {};", type2cpp(type_), name, value)
        },
        Value::CallFun { name, arguments } => {
            let value = match name.as_str() {
                "int" => int::code2cpp(&arguments[0]),
                _ => custom_fun::code2cpp(name, arguments, false)
            };
            format!("{} {} = {};", type2cpp(type_), var_name, value)
        },
        Value::None => {
            format!("{} {};", type2cpp(type_), name)
        }
    }
}
