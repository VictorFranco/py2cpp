use crate::py2cpp::types::{Type, Argument, Value, Instruction, Library, Context};
use crate::py2cpp::constants::{RE_APPEND, RE_INT, RE_STR, RE_VAR};

pub fn py2code(context: &mut Context, fun_body: &mut Vec<Instruction>, content: &str) -> Result<Option<(Vec<Instruction>, Vec<Library>)>, String> {
    let cap_append = RE_APPEND.captures(content);

    match cap_append {
        Some(data) => {
            let vector = data.get(1).unwrap().as_str();
            let element = data.get(2).unwrap().as_str();
            let mut arguments = Vec::new();

            let result = match element {
                text if RE_INT.is_match(text) => Ok(Type::Int),
                text if RE_STR.is_match(text) => Ok(Type::String),
                text if RE_VAR.is_match(text) => context.get_type(text),
                _ => Ok(Type::Undefined)
            };

            let vec_type;

            match result {
                Ok(type_) => vec_type = type_,
                Err(error) => return Err(error)
            }

            arguments.push(
                Argument {
                    type_: Type::Undefined,
                    value: Value::UseVar(vector.to_string())
                }
            );

            arguments.push(
                Argument {
                    type_: Type::Undefined,
                    value: Value::UseVar(element.to_string())
                }
            );

            for instruction in fun_body.iter_mut() {
                match instruction {
                    Instruction::CreateVar { type_, name, value: _ } => {
                        if name.as_str() == vector {
                            match type_ {
                                Type::Vector(data) => *data = Box::new(vec_type.clone()),
                                _ => {}
                            }
                        };
                    },
                    _ => {}
                }
            }

            let name = "append".to_string();
            let instruction = Instruction::CallFun { name, arguments };
            Ok(Some((vec![instruction], vec![])))
        },
        None => Ok(None)
    }
}

pub fn code2cpp(name: &String, arguments: &Vec<Argument>) -> String {
    match name.as_str() {
        "append" => {
            let vector = match &arguments[0].value {
                Value::UseVar(vector) => vector.to_string(),
                _ => String::new()
            };
            let element = match &arguments[1].value {
                Value::UseVar(element) => element.to_string(),
                _ => String::new()
            };
            format!("{}.push_back({});", vector, element)
        },
        _ => String::new()
    }
}
