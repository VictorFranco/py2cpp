use crate::types::{Type, Argument, Value, Instruction, Library};
use crate::constants::RE_LEN;

pub fn py2code(content: &str) -> Option<(Vec<Instruction>, Vec<Library>)> {
    let cap_int = RE_LEN.captures(content);

    match cap_int {
        Some(data) => {
            let type_ = Type::Vector(Box::new(Type::Undefined));
            let content = data.get(1).unwrap().as_str().to_string();
            let value = Value::ConstValue(content);
            let argument = Argument { type_, value };
            let name = "len".to_string();
            let arguments = vec![argument];
            let instruction = Instruction::CallFun { name, arguments };
            Some((vec![instruction], vec![]))
        },
        None => None
    }
}

pub fn code2cpp(argument: &Argument) -> String {
    match &argument.value {
        Value::ConstValue(value) | Value::UseVar(value) => {
            format!("{}.size()", value)
        },
        _ => String::new()
    }
}
