use std::collections::HashMap;
use crate::py2cpp::types::{Type, Value, Instruction, Library};
use crate::py2cpp::constants::{RE_DEC, RE_INT, RE_STR, RE_VEC, RE_FUN};
use crate::py2cpp::instructions::{input, custom_fun, int, len};
use crate::py2cpp::infer::get_fun_type;

pub fn py2code(body: &mut Vec<Instruction>, fun_types: &HashMap<String, Type>, content: &str) -> Option<(Vec<Instruction>, Vec<Library>)> {
    let cap_dec = RE_DEC.captures(content);

    match cap_dec {
        Some(data) => {
            let mut libraries = Vec::new();
            let mut instructions = Vec::new();
            let name = data.get(1).unwrap().as_str().to_string();
            let content = data.get(2).unwrap().as_str().to_string();
            let (type_, value) = match content.as_str() {
                text if RE_INT.is_match(text) => (Type::Int, Value::ConstValue(content)),
                text if RE_STR.is_match(text) => (Type::String, Value::ConstValue(content)),
                text if RE_VEC.is_match(text) => {
                    libraries = Library::get_libraries(&["vector"]);
                    (Type::Vector(Box::new(Type::Undefined)), Value::None)
                },
                text if RE_FUN.is_match(text) => {
                    let cap_fun = RE_FUN.captures(text).unwrap();
                    let fun_name = cap_fun.get(1).unwrap().as_str();
                    let (fun_type, fun_value, mut fun_libraries) = match fun_name {
                        "input" => {
                            let (mut input_instructions, input_libraries) = input::py2code(&name, text, false).unwrap();
                            instructions.append(&mut input_instructions);
                            (Type::String, Value::None, input_libraries)
                        },
                        "int" => {
                            let (int_instructions, int_libraries) = int::py2code(text).unwrap();
                            (Type::Int, Instruction::instruc2value(&int_instructions[0]), int_libraries)
                        },
                        "len" => {
                            let (len_instructions, len_libraries) = len::py2code(text).unwrap();
                            (Type::Int, Instruction::instruc2value(&len_instructions[0]), len_libraries)
                        },
                        _ => {
                            let (custom_instructions, custom_libraries) = custom_fun::py2code(body, fun_types, text).unwrap();
                            (get_fun_type(fun_types, fun_name), Instruction::instruc2value(&custom_instructions[0]), custom_libraries)
                        }
                    };
                    libraries.append(&mut fun_libraries);
                    (fun_type, fun_value)
                },
                _ => (Type::Undefined, Value::ConstValue(content))
            };
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
            format!("{} {} = {};", Type::type2cpp(type_), name, value)
        },
        Value::CallFun { name, arguments } => {
            let value = match name.as_str() {
                "int" => int::code2cpp(&arguments[0]),
                "len" => len::code2cpp(&arguments[0]),
                _ => custom_fun::code2cpp(name, arguments, false)
            };
            format!("{} {} = {};", Type::type2cpp(type_), var_name, value)
        },
        Value::None => {
            format!("{} {};", Type::type2cpp(type_), name)
        }
    }
}
