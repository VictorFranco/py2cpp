use regex::Regex;
use crate::py2cpp::{Argument, Instruction, Type, Library, get_libraries};

const PRINT: &str = r##"^print\((.*)\)[^"]*$"##;
const MESSAGES: &str = r##"("[ a-zA-Z0-9]+"|[a-zA-Z][a-zA-Z0-9]+),?"##;

pub fn py2code(content: &str) -> Option<(Vec<Instruction>, Vec<Library>)> {
    let re_print = Regex::new(PRINT).unwrap();
    let re_msgs = Regex::new(MESSAGES).unwrap();
    let cap_print = re_print.captures(content);

    match cap_print {
        Some(data) => {
            let print = data.get(1).unwrap().as_str();
            let caps_msgs = re_msgs.captures_iter(print);
            let name = "print".to_string();
            let mut arguments = Vec::new();

            for cap in caps_msgs {
                let content = cap.get(1).unwrap().as_str().to_string();
                arguments.push(
                    Argument {
                        type_: Type::Undefined,
                        content
                    }
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
    match name.as_str() {
        "print" => {
            let mut result = format!("cout << ");
            for argument in arguments {
                result = format!("{}{} << ", result, argument.content);
            }
            format!("{}endl;", result)
        },
        _ => String::new()
    }
}
