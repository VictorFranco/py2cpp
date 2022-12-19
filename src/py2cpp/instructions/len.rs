use crate::py2cpp::types::{Type, Argument, Value, Instruction, Library, Context};
use crate::py2cpp::constants::RE_LEN;

pub fn py2code(context: &mut Context, content: &str) -> Result<Option<(Vec<Instruction>, Vec<Library>)>, String> {
    let cap_int = RE_LEN.captures(content);

    match cap_int {
        Some(data) => {
            let type_ = Type::Vector(Box::new(Type::Undefined));
            let content = data.get(1).unwrap().as_str().to_string();
            let result = context.get_type(&content);
            let value;
            match result {
                Ok(_) => value = Value::ConstValue(content),
                Err(error) => return Err(error)
            }
            let argument = Argument { type_, value };
            let name = "len".to_string();
            let arguments = vec![argument];
            let instruction = Instruction::CallFun { name, arguments };
            Ok(Some((vec![instruction], vec![])))
        },
        None => Ok(None)
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
