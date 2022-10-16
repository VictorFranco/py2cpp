use crate::py2cpp::{Type, Argument, Value, Instruction, Library, get_libraries};
use crate::constants::RE_INT_FUN;

pub fn py2code(content: &str) -> Option<(Vec<Instruction>, Vec<Library>)> {
    let cap_int = RE_INT_FUN.captures(content);

    match cap_int {
        Some(data) => {
            let type_ = Type::String;
            let content = data.get(1).unwrap().as_str();
            let value = Value::ConstValue(content.to_string());
            let argument = Argument { type_, value };
            let name = "int".to_string();
            let arguments = vec![argument];
            let instruction = Instruction::CallFun { name, arguments };
            let libraries = get_libraries(&["string"]);
            Some((vec![instruction], libraries))
        },
        None => None
    }
}

pub fn code2cpp(argument: &Argument) -> String {
    match &argument.value {
        Value::ConstValue(value) | Value::UseVar(value) => {
            format!("stoi({})", value)
        },
        _ => String::new()
    }
}
