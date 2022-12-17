use crate::py2cpp::types::{Type, Argument, Value, Instruction, Library, Context};
use crate::py2cpp::constants::RE_AT;

pub fn py2code(context: &mut Context, content: &str) -> Result<Option<(Vec<Instruction>, Vec<Library>)>, String> {
    let cap_at = RE_AT.captures(content);

    match cap_at {
        Some(data) => {
            let vector = data.get(1).unwrap().as_str().to_string();
            let index = data.get(2).unwrap().as_str().to_string();
            let mut arguments = Vec::new();
            let result = context.get_type(&vector);

            match result {
                Ok(_) => {},
                Err(error) => return Err(error)
            }

            arguments.push(
                Argument {
                    type_: Type::Undefined,
                    value: Value::UseVar(vector)
                }
            );

            arguments.push(
                Argument {
                    type_: Type::Undefined,
                    value: Value::UseVar(index)
                }
            );

            let name = "at".to_string();
            let instruction = Instruction::CallFun { name, arguments };
            Ok(Some((vec![instruction], vec![])))
        },
        None => Ok(None)
    }
}

pub fn code2cpp(name: &String, arguments: &Vec<Argument>) -> String {
    match name.as_str() {
        "at" => {
            let vector = match &arguments[0].value {
                Value::UseVar(vector) => vector.to_string(),
                _ => String::new()
            };
            let index = match &arguments[1].value {
                Value::ConstValue(index) | Value::UseVar(index) => {
                    index.to_string()
                },
                _ => String::new()
            };
            format!("{}.at({})", vector, index)
        },
        _ => String::new()
    }
}
