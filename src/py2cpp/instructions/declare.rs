use std::collections::HashMap;
use crate::py2cpp::types::{Type, Value, Instruction, Library};
use crate::py2cpp::constants::{RE_FUN, RE_DEC, RE_EXP, RE_AT, RE_INT, RE_STR, RE_VEC, RE_VAR};
use crate::py2cpp::instructions::{input, custom_fun, int, len, at};
use crate::py2cpp::infer::{get_var_type, get_fun_type};

pub fn py2code(body: &mut Vec<Instruction>, context: &mut Vec<Instruction>, fun_types: &HashMap<String, Type>, content: &str) -> Option<(Vec<Instruction>, Vec<Library>)> {
    let cap_dec = RE_DEC.captures(content);

    match cap_dec {
        Some(data) => {
            let mut libraries = Vec::new();
            let mut instructions = Vec::new();
            let name = data.get(1).unwrap().as_str().to_string();
            let content = data.get(2).unwrap().as_str().to_string();
            let mut first_declare = false;
            let (type_, value) = match content.as_str() {
                text if RE_EXP.is_match(text) => (Type::Int, Value::exp2value(text)),
                text if RE_INT.is_match(text) => (Type::Int, Value::ConstValue(content)),
                text if RE_STR.is_match(text) => (Type::String, Value::ConstValue(content)),
                text if RE_VAR.is_match(text) => (get_var_type(text, body), Value::UseVar(content)),
                text if RE_VEC.is_match(text) => {
                    libraries = Library::get_libraries(&["vector"]);
                    (Type::Vector(Box::new(Type::Undefined)), Value::None)
                },
                text if RE_AT.is_match(text) => {
                    let (at_instructions, _at_libraries) = at::py2code(body, text).unwrap();
                    (Type::Int, at_instructions[0].inst2value())
                },
                text if RE_FUN.is_match(text) => {
                    let cap_fun = RE_FUN.captures(text).unwrap();
                    let fun_name = cap_fun.get(1).unwrap().as_str();
                    let (fun_type, fun_value, mut fun_libraries) = match fun_name {
                        "input" => {
                            first_declare = true;
                            let (mut input_instructions, input_libraries) = input::py2code(&name, text, false).unwrap();
                            instructions.append(&mut input_instructions);
                            (Type::String, Value::None, input_libraries)
                        },
                        "int" => {
                            let (mut int_instructions, int_libraries) = int::py2code(body, fun_types, text).unwrap();
                            instructions.append(&mut int_instructions);
                            let call_instr = instructions.pop().unwrap();
                            let value = call_instr.inst2value();
                            (Type::Int, value, int_libraries)
                        },
                        "len" => {
                            let (len_instructions, len_libraries) = len::py2code(text).unwrap();
                            (Type::Int, len_instructions[0].inst2value(), len_libraries)
                        },
                        _ => {
                            let (custom_instructions, custom_libraries) = custom_fun::py2code(body, fun_types, text).unwrap();
                            (get_fun_type(fun_types, fun_name), custom_instructions[0].inst2value(), custom_libraries)
                        }
                    };
                    libraries.append(&mut fun_libraries);
                    (fun_type, fun_value)
                },
                _ => (Type::Undefined, Value::ConstValue(content))
            };
            let var_type = type_.clone();
            let var_name = name.to_string();
            let var_value = value.clone();
            let mut declare = Instruction::CreateVar { type_, name, value };
            for instruction in body.iter() {
                match instruction {
                    Instruction::CreateVar { type_, name, value: _ } => {
                        if &var_name == name && &var_type == type_ {
                            let type_ = type_.clone();
                            let name = name.to_string();
                            let value = var_value.clone();
                            declare = Instruction::ReassignVar { type_, name, value };
                        }
                    },
                    _ => {}
                }
            }
            for instruction in context.iter() {
                match instruction {
                    Instruction::CreateVar { type_, name, value: _ } => {
                        if &var_name == name && &var_type == type_ {
                            let type_ = type_.clone();
                            let name = name.to_string();
                            let value = var_value.clone();
                            declare = Instruction::ReassignVar { type_, name, value };
                        }
                    },
                    _ => {}
                }
            }
            match first_declare {
                true  => instructions.insert(0, declare),
                false => instructions.push(declare)
            }
            Some((instructions, libraries))
        },
        None => None
    }
}

pub fn code2cpp(type_: &Type, name: &String, value: &Value, declare: bool) -> String {
    let result = match declare {
        true => format!("{} {}", type_.type2cpp(), name),
        false => format!("{}", name)
    };
    match value {
        Value::ConstValue(value) | Value::UseVar(value) => {
            format!("{} = {};", result, value)
        },
        Value::CallFun { name, arguments } => {
            let value = match name.as_str() {
                "int" => int::code2cpp(&arguments[0]),
                "len" => len::code2cpp(&arguments[0]),
                "at"  => at::code2cpp(name, arguments),
                _ => custom_fun::code2cpp(name, arguments, false)
            };
            format!("{} = {};", result, value)
        },
        Value::Expression { operators, values } => {
            format!("{} = {};", result, Value::exp2cpp(operators, values))
        },
        Value::None => {
            format!("{};", result)
        }
    }
}
