use crate::py2cpp::{Type, Argument, Value, Instruction, Library, get_libraries};
use crate::constants::RE_INPUT;
use crate::instructions::print;

pub fn py2code(var_name: &str, content: &str, newline: bool) -> Option<(Vec<Instruction>, Vec<Library>)> {
    let cap_input = RE_INPUT.captures(content);

    match cap_input {
        Some(data) => {
            // print text
            let content = data.get(1).unwrap().as_str();
            let content = format!("print({})", content);
            let (mut instructions, mut libraries) = print::py2code(content.as_str(), newline).unwrap();
            // save input into variable
            let name = "input".to_string();
            let value = Value::UseVar(var_name.to_string());
            let argument = Argument { type_: Type::Undefined, value };
            instructions.push(
                Instruction::CallFun { name, arguments: vec![argument] }
            );
            let mut string = get_libraries(&["string"]);
            libraries.append(&mut string);

            Some((instructions, libraries))
        },
        None => None
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
