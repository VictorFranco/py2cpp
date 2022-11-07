use crate::py2cpp::types::{Type, Argument, Value, Instruction, Library};
use crate::py2cpp::constants::RE_APPEND;

pub fn py2code(_body: &mut Vec<Instruction>, content: &str) -> Option<(Vec<Instruction>, Vec<Library>)> {
    let cap_append = RE_APPEND.captures(content);

    match cap_append {
        Some(data) => {
            let vector = data.get(1).unwrap().as_str().to_string();
            let element = data.get(2).unwrap().as_str().to_string();
            let mut arguments = Vec::new();

            arguments.push(
                Argument {
                    type_: Type::Undefined,
                    value: Value::UseVar(vector)
                }
            );

            arguments.push(
                Argument {
                    type_: Type::Undefined,
                    value: Value::UseVar(element)
                }
            );

            let name = "append".to_string();
            let instruction = Instruction::CallFun { name, arguments };
            Some((vec![instruction], vec![]))
        },
        None => None
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
