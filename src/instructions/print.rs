use crate::py2cpp::{Type, Argument, Value, Instruction, Library, get_libraries};
use crate::constants::{RE_PRINT, RE_MSGS, RE_INT, RE_STR, RE_VAR};

pub fn py2code(content: &str, newline: bool) -> Option<(Vec<Instruction>, Vec<Library>)> {
    let cap_print = RE_PRINT.captures(content);

    match cap_print {
        Some(data) => {
            let print = data.get(1).unwrap().as_str();
            let caps_msgs = RE_MSGS.captures_iter(print);
            let name = "print".to_string();
            let mut arguments = Vec::new();

            for cap in caps_msgs {
                let content = cap.get(1).unwrap().as_str().to_string();
                let (type_, value) = match content.as_str() {
                    text if RE_VAR.is_match(text) => (Type::Undefined, Value::UseVar(content)),
                    text if RE_INT.is_match(text) => (Type::Int, Value::ConstValue(content)),
                    text if RE_STR.is_match(text) => (Type::String, Value::ConstValue(content)),
                    _ => (Type::Undefined, Value::None)
                };
                arguments.push(
                    Argument { type_, value }
                );
            }

            let value = Value::ConstValue(newline.to_string());
            arguments.push(
                Argument {
                    type_: Type::Undefined,
                    value
                }
            );

            let instruction = Instruction::CallFun { name, arguments };
            let libraries = get_libraries(&["iostream"]);
            Some((vec![instruction], libraries))
        },
        None => None
    }
}

pub fn code2cpp(name: &String, arguments: &Vec<Argument>) -> String {
    match name.as_str() {
        "print" => {
            let mut result = format!("cout");
            for index in 0..arguments.len() - 1  {
                if index > 0 {
                    result.push_str(" << \" \"");
                }
                let argument = &arguments.get(index).unwrap().value;
                match argument {
                    Value::ConstValue(value) | Value::UseVar(value) => {
                        result = format!("{} << {}", result, value);
                    },
                    _ => {}
                }
            }
            let newline = &arguments.last().unwrap().value;
            match newline {
                Value::ConstValue(value) => {
                    if value == "true" {
                        result.push_str(" << endl");
                    }
                }
                _ => {}
            }
            format!("{};", result)
        },
        _ => String::new()
    }
}
