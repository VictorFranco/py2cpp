use regex::Regex;
use crate::py2cpp::{Type, Instruction, Library, INTEGER, STRING, VECTOR, VARIABLE};

const RETURN: &str = r"^return (.*)$";

pub fn py2code(body: &mut Vec<Instruction>, content: &str) -> Option<(Vec<Instruction>, Vec<Library>)> {
    let re_return = Regex::new(RETURN).unwrap();
    let re_int = Regex::new(INTEGER).unwrap();
    let re_str = Regex::new(STRING).unwrap();
    let re_vec = Regex::new(VECTOR).unwrap();
    let re_var = Regex::new(VARIABLE).unwrap();
    let cap_return = re_return.captures(content);

    match cap_return {
        Some(data) => {
            let return_value = data.get(1).unwrap().as_str();
            let mut return_type_ = Type::Undefined;
            if re_int.is_match(return_value) {
                return_type_ = Type::Int;
            }
            if re_str.is_match(return_value) {
                return_type_ = Type::String;
            }
            if re_vec.is_match(return_value) {
                return_type_ = Type::Vector(Box::new(Type::Undefined));
            }
            if re_var.is_match(return_value) {
                for instruction in body.iter() {
                    match instruction {
                        Instruction::CreateVar { type_, name, value: _ } => {
                            if return_value == name {
                                return_type_ = type_.clone();
                            }
                        },
                        _ => {}
                    }
                }
            }
            let type_ = return_type_;
            let value = return_value.to_string();
            let instruction = Instruction::Return { type_, value };
            Some((vec![instruction], vec![]))
        },
        None => None
    }
}

pub fn code2cpp(value: &String) -> String {
    format!("return {};", value)
}
