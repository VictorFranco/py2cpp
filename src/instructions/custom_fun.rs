use regex::Regex;
use crate::py2cpp::{Type, Argument, Value, Instruction, instruc2value, Library, NATIVE_FUNS, INTEGER, STRING, VARIABLE, CUSTOM_FUN};
use crate::instructions::int;

const ARGUMENTS: &str = r##"([+-]?\s*\d+|"[ a-zA-Z0-9: ]+"|[a-zA-Z][a-zA-Z0-9]*(\(.*\))?),?"##;

pub fn py2code(body: &mut Vec<Instruction>, content: &str) -> Option<(Vec<Instruction>, Vec<Library>)> {
    let re_fun = Regex::new(CUSTOM_FUN).unwrap();
    let re_args = Regex::new(ARGUMENTS).unwrap();
    let re_int = Regex::new(INTEGER).unwrap();
    let re_str = Regex::new(STRING).unwrap();
    let re_var = Regex::new(VARIABLE).unwrap();
    let cap_fun = re_fun.captures(content);

    match cap_fun {
        Some(data) => {
            let mut libraries = Vec::new();
            let fun = data.get(1).unwrap().as_str();
            if NATIVE_FUNS.contains(&fun) {
                return None;
            }
            let arguments = data.get(2).unwrap().as_str();
            let caps_args = re_args.captures_iter(arguments);
            let name = fun.to_string();
            let mut arguments = Vec::new();

            for cap in caps_args {
                let content = cap.get(1).unwrap().as_str().to_string();
                let (type_, value) = match content.as_str() {
                    text if re_int.is_match(text) => (Type::Int, Value::ConstValue(content)),
                    text if re_str.is_match(text) => (Type::String, Value::ConstValue(content)),
                    text if re_var.is_match(text) => {
                        let mut arg_type = Type::Undefined;
                        for instruction in body.iter() {
                            match instruction {
                                Instruction::CreateVar { type_, name, value: _ } => {
                                    if text == name {
                                        arg_type = type_.clone();
                                    }
                                },
                                _ => {}
                            }
                        }
                        (arg_type, Value::UseVar(content))
                    },
                    text if re_fun.is_match(text) => {
                        let cap = re_fun.captures(text).unwrap();
                        let fun = cap.get(0).unwrap().as_str();
                        let fun_name = cap.get(1).unwrap().as_str();
                        let (arg_type, (instructions, mut fun_libraries)) = match fun_name {
                            "int" => (Type::Int, int::py2code(fun).unwrap()),
                            _ => (Type::Undefined, py2code(body, text).unwrap())
                        };
                        libraries.append(&mut fun_libraries);
                        (arg_type, instruc2value(&instructions[0]))
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
