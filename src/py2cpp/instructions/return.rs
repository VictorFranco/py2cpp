use crate::py2cpp::types::{Type, Instruction, Library};
use crate::py2cpp::constants::{RE_RETURN, RE_INT, RE_STR, RE_VEC, RE_VAR};
use crate::py2cpp::infer::get_var_type;

pub fn py2code(body: &Vec<Instruction>, content: &str) -> Result<Option<(Vec<Instruction>, Vec<Library>)>, String> {
    let cap_return = RE_RETURN.captures(content);

    match cap_return {
        Some(data) => {
            let value = data.get(1).unwrap().as_str().to_string();
            let type_ = match value.as_str() {
                text if RE_INT.is_match(text) => Type::Int,
                text if RE_STR.is_match(text) => Type::String,
                text if RE_VEC.is_match(text) => Type::Vector(Box::new(Type::Undefined)),
                text if RE_VAR.is_match(text) => get_var_type(text, body),
                _ => Type::Undefined
            };
            let instruction = Instruction::Return { type_, value };
            Ok(Some((vec![instruction], vec![])))
        },
        None => Ok(None)
    }
}

pub fn code2cpp(value: &String) -> String {
    format!("return {};", value)
}
