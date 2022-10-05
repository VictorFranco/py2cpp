use regex::Regex;
use crate::py2cpp::Instruction;

const RETURN: &str = r"return (.*)";

pub fn py2code(content: &str) -> Option<Instruction> {
    let re_return = Regex::new(RETURN).unwrap();
    let cap_return = re_return.captures(content);

    match cap_return {
        Some(data) => {
            let value = data.get(1).unwrap().as_str().to_string();
            Some(Instruction::Return(value))
        },
        None => None
    }
}

pub fn code2cpp(value: &String) -> String {
    format!("return {};", value)
}
