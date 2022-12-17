use crate::py2cpp::types::{Type, Argument, Value, Instruction, Library, Context};
use crate::py2cpp::constants::{NATIVE_FUNS, RE_FUN, RE_ARGS, RE_INT, RE_STR, RE_VAR};
use crate::py2cpp::instructions::{int, len};

pub fn py2code(context: &mut Context, content: &str) -> Result<Option<(Vec<Instruction>, Vec<Library>)>, String> {
    let cap_fun = RE_FUN.captures(content);

    match cap_fun {
        Some(data) => {
            let mut libraries = Vec::new();
            let fun = data.get(1).unwrap().as_str();
            if NATIVE_FUNS.contains(&fun) {
                return Ok(None);
            }
            let arguments = data.get(2).unwrap().as_str();
            let caps_args = RE_ARGS.captures_iter(arguments);
            let name = fun.to_string();
            let mut arguments = Vec::new();

            for cap in caps_args {
                let content = cap.get(1).unwrap().as_str().to_string();
                let result = match content.as_str() {
                    text if RE_INT.is_match(text) => Ok((Type::Int, Value::ConstValue(content))),
                    text if RE_STR.is_match(text) => Ok((Type::String, Value::ConstValue(content))),
                    text if RE_VAR.is_match(text) => {
                        let mut name = String::new();
                        match context.0.get(text) {
                            Some(vec) => {
                                let last = vec.last().unwrap().clone();
                                name = last.name;
                            },
                            None => {}
                        }
                        Ok((context.get_type(text), Value::UseVar(name)))
                    },
                    text if RE_FUN.is_match(text) => {
                        let cap = RE_FUN.captures(text).unwrap();
                        let fun = cap.get(0).unwrap().as_str();
                        let fun_name = cap.get(1).unwrap().as_str();
                        let result = match fun_name {
                            "int" => Ok((Type::Int, int::py2code(context, text).unwrap())),
                            "len" => Ok((Type::Int, len::py2code(fun).unwrap())),
                            _ => Ok((context.get_type(fun_name), py2code(context, text).unwrap()))
                        };
                        match result {
                            Ok((arg_type, option)) => {
                                match option {
                                    Some((instructions, mut fun_libraries)) => {
                                        libraries.append(&mut fun_libraries);
                                        Ok((arg_type, instructions[0].inst2value()))
                                    },
                                    None => Err(String::new())
                                }
                            },
                            Err(error) => Err(error)
                        }
                    },
                    _ => Ok((Type::Undefined, Value::None))
                };

                match result {
                    Ok((type_, value)) => {
                        arguments.push(
                            Argument { type_, value }
                        );
                    },
                    Err(error) => return Err(error)
                }
            }

            let instruction = Instruction::CallFun { name, arguments };
            Ok(Some((vec![instruction], libraries)))
        },
        None => Ok(None)
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
