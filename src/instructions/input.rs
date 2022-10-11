use regex::Regex;
use crate::py2cpp::{Argument, Instruction, Type, Library, get_libraries};
use crate::instructions::print;

const INPUT: &str = r##"^input\((.*)\)$"##;

pub fn py2code(var_name: &str, content: &str, newline: &str) -> Option<(Vec<Instruction>, Vec<Library>)> {
    let re_input = Regex::new(INPUT).unwrap();
    let cap_print = re_input.captures(content);

    match cap_print {
        Some(data) => {
            // print text
            let content = data.get(1).unwrap().as_str();
            let content = format!("print({})", content);
            let (mut instructions, mut libraries) = print::py2code(content.as_str(), newline).unwrap();
            // save input into variable
            let name = "input".to_string();
            let content = var_name.to_string();
            let argument = Argument { type_: Type::Undefined, content };
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
            format!("cin >> {};", arguments[0].content)
        },
        _ => String::new()
    }
}
