use crate::py2cpp::types::{Type, Argument, Value, Instruction, Library, Context};
use crate::py2cpp::constants::RE_INPUT;
use crate::py2cpp::instructions::print;

pub fn py2code(context: &mut Context,var_name: &str, content: &str, newline: bool) -> Result<Option<(Vec<Instruction>, Vec<Library>)>, String> {
    let cap_input = RE_INPUT.captures(content);

    match cap_input {
        Some(data) => {
            // print text
            let content = data.get(1).unwrap().as_str();
            let content = format!("print({})", content);
            let mut instructions = vec![];
            let mut libraries = vec![];
            let result = print::py2code(context, content.as_str(), newline);
            match result {
                Ok(option) => {
                    match option {
                        Some((instrs, libs)) => {
                            instructions = instrs;
                            libraries = libs;
                        },
                        None => {}
                    }
                },
                Err(error) => return Err(error)
            }
            // save input into variable
            let name = "input".to_string();
            let value = Value::UseVar(var_name.to_string());
            let argument = Argument { type_: Type::Undefined, value };
            instructions.push(
                Instruction::CallFun { name, arguments: vec![argument] }
            );
            let mut string = Library::get_libraries(&["string"]);
            libraries.append(&mut string);

            Ok(Some((instructions, libraries)))
        },
        None => Ok(None)
    }
}

pub fn code2cpp(name: &String, arguments: &Vec<Argument>) -> String {
    match name.as_str() {
        "input" => {
            match &arguments[0].value {
                Value::UseVar(value) => format!("cin >> {};", value),
                _ => String::new()
            }
        },
        _ => String::new()
    }
}
