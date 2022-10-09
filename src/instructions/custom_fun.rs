use regex::Regex;
use crate::py2cpp::{Argument, Instruction, Type, Library, get_libraries, NATIVE_FUNS, INTEGER, STRING, VARIABLE};

const CUSTOM_FUN: &str = r##"^([a-zA-Z0-9]*)\((.*)\)[^"]*$"##;
const ARGUMENTS: &str = r##"(\d+|"[ a-zA-Z0-9: ]+"|[a-zA-Z][a-zA-Z0-9]*),?"##;

pub fn py2code(body: &mut Vec<Instruction>, content: &str) -> Option<(Vec<Instruction>, Vec<Library>)> {
    let re_fun = Regex::new(CUSTOM_FUN).unwrap();
    let re_args = Regex::new(ARGUMENTS).unwrap();
    let re_int = Regex::new(INTEGER).unwrap();
    let re_str = Regex::new(STRING).unwrap();
    let re_var = Regex::new(VARIABLE).unwrap();
    let cap_fun = re_fun.captures(content);

    match cap_fun {
        Some(data) => {
            let fun = data.get(1).unwrap().as_str();
            if NATIVE_FUNS.contains(&fun) {
                return None;
            }
            let arguments = data.get(2).unwrap().as_str();
            let caps_args = re_args.captures_iter(arguments);
            let name = fun.to_string();
            let mut arguments = Vec::new();

            for cap in caps_args {
                let content = cap.get(1).unwrap().as_str();
                let mut arg_type = Type::Undefined;
                if re_int.is_match(content) {
                    arg_type = Type::Int;
                }
                if re_str.is_match(content) {
                    arg_type = Type::String;
                }
                if re_var.is_match(content) {
                    for instruction in body.iter() {
                        match instruction {
                            Instruction::CreateVar { type_, name, value: _ } => {
                                if content == name {
                                    arg_type = type_.clone();
                                }
                            },
                            _ => {}
                        }
                    }
                }
                let type_ = arg_type;
                let content = content.to_string();
                arguments.push(
                    Argument { type_, content }
                );

            }

            let instruction = Instruction::CallFun { name, arguments };
            let libraries = get_libraries(&["iostream"]);
            Some((vec![instruction], libraries))
        },
        None => None
    }
}

pub fn code2cpp(name: &String, arguments: &Vec<Argument>) -> String {
    if NATIVE_FUNS.contains(&name.as_str()) {
        return String::new();
    }
    let mut result = format!("{}(", name);
    for (index, argument) in arguments.iter().enumerate()  {
        result = format!("{}{}", result, argument.content );
        if index < arguments.len() - 1 {
            result = format!("{}, ", result);
        }
    }
    format!("{});", result)
}