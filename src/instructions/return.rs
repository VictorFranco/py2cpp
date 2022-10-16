use crate::py2cpp::{Type, Instruction, Library};
use crate::constants::{RE_RETURN, RE_INT, RE_STR, RE_VEC, RE_VAR};

pub fn py2code(body: &mut Vec<Instruction>, content: &str) -> Option<(Vec<Instruction>, Vec<Library>)> {
    let cap_return = RE_RETURN.captures(content);

    match cap_return {
        Some(data) => {
            let value = data.get(1).unwrap().as_str().to_string();
            let type_ = match value.as_str() {
                text if RE_INT.is_match(text) => Type::Int,
                text if RE_STR.is_match(text) => Type::String,
                text if RE_VEC.is_match(text) => Type::Vector(Box::new(Type::Undefined)),
                text if RE_VAR.is_match(text) => {
                    let mut return_type_ = Type::Undefined;
                    for instruction in body.iter() {
                        match instruction {
                            Instruction::CreateVar { type_, name, value: _ } => {
                                if text == name {
                                    return_type_ = type_.clone();
                                }
                            },
                            _ => {}
                        }
                    }
                    return_type_
                },
                _ => Type::Undefined
            };
            let instruction = Instruction::Return { type_, value };
            Some((vec![instruction], vec![]))
        },
        None => None
    }
}

pub fn code2cpp(value: &String) -> String {
    format!("return {};", value)
}
