use std::collections::HashMap;
use crate::py2cpp::types::{Type, Argument, Value, Instruction, Library};
use crate::py2cpp::constants::{NATIVE_FUNS, RE_FUN, RE_ARGS, RE_INT, RE_STR, RE_VAR};
use crate::py2cpp::instructions::{int, len};
use crate::py2cpp::infer::{get_var_type, get_fun_type};

pub fn py2code(body: &mut Vec<Instruction>, fun_types: &HashMap<String, Type>, content: &str) -> Option<(Vec<Instruction>, Vec<Library>)> {
    let cap_fun = RE_FUN.captures(content);

    match cap_fun {
        Some(data) => {
            let mut libraries = Vec::new();
            let fun = data.get(1).unwrap().as_str();
            if NATIVE_FUNS.contains(&fun) {
                return None;
            }
            let arguments = data.get(2).unwrap().as_str();
            let caps_args = RE_ARGS.captures_iter(arguments);
            let name = fun.to_string();
            let mut arguments = Vec::new();

            for cap in caps_args {
                let content = cap.get(1).unwrap().as_str().to_string();
                let (type_, value) = match content.as_str() {
                    text if RE_INT.is_match(text) => (Type::Int, Value::ConstValue(content)),
                    text if RE_STR.is_match(text) => (Type::String, Value::ConstValue(content)),
                    text if RE_VAR.is_match(text) => (get_var_type(text, body), Value::UseVar(content)),
                    text if RE_FUN.is_match(text) => {
                        let cap = RE_FUN.captures(text).unwrap();
                        let fun = cap.get(0).unwrap().as_str();
                        let fun_name = cap.get(1).unwrap().as_str();
                        let (arg_type, (instructions, mut fun_libraries)) = match fun_name {
                            "int" => (Type::Int, int::py2code(body, fun_types, text).unwrap()),
                            "len" => (Type::Int, len::py2code(fun).unwrap()),
                            _ => (get_fun_type(fun_types, fun_name), py2code(body, fun_types, text).unwrap())
                        };
                        libraries.append(&mut fun_libraries);
                        (arg_type, Instruction::instruc2value(&instructions[0]))
                    },
                    _ => (Type::Undefined, Value::None)
                };

                arguments.push(
                    Argument { type_, value }
                );
            }

            let instruction = Instruction::CallFun { name, arguments };
            Some((vec![instruction], libraries))
        },
        None => None
    }
}

pub fn code2cpp(name: &String, arguments: &Vec<Argument>, semicolon: bool) -> String {
    if NATIVE_FUNS.contains(&name.as_str()) {
        return String::new();
    }
    let mut result = format!("{}(", name);
    for (index, argument) in arguments.iter().enumerate()  {
        result = match &argument.value {
            Value::ConstValue(value) | Value::UseVar(value) => {
                format!("{}{}", result, value)
            },
            Value::CallFun { name, arguments } => {
                match name.as_str() {
                    "int" => format!("{}{}", result, int::code2cpp(&arguments[0])),
                    "len" => format!("{}{}", result, len::code2cpp(&arguments[0])),
                    _ => format!("{}{}", result, code2cpp(name, arguments, false))
                }
            }
            _ => result
        };
        if index < arguments.len() - 1 {
            result = format!("{}, ", result);
        }
    }
    match semicolon {
        true  => format!("{});", result),
        false => format!("{})", result),
    }
}
