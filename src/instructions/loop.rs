use std::collections::HashMap;
use crate::py2cpp::{Type, Value, Instruction, Library};
use crate::constants::RE_LOOP;

pub fn py2code(_body: &mut Vec<Instruction>, _fun_types: &HashMap<String, Type>, content: &str) -> Option<(Vec<Instruction>, Vec<Library>)> {
    let cap_return = RE_LOOP.captures(content);

    match cap_return {
        Some(data) => {
            let counter = data.get(1).unwrap().as_str().to_string();
            let start = Value::ConstValue(data.get(2).unwrap().as_str().to_string());
            let end = Value::ConstValue(data.get(3).unwrap().as_str().to_string());
            let content = vec![];
            let instruction = Instruction::Loop { counter, start, end, content };
            Some((vec![instruction], vec![]))
        },
        None => None
    }
}

pub fn code2cpp(counter: &String, start: &Value, end: &Value, _content: &Vec<Instruction>) -> String {
    let params = [start, end];
    let mut values = [String::new(), String::new()];
    for (index, param) in params.iter().enumerate() {
        match param {
            Value::ConstValue(data) | Value::UseVar(data) => {
                values[index] = data.to_string();
            },
            _ => {}
        };
    }
    let [start, end] = values;
    format!("for (int {} = {}; {} < {}; {}++) {{}}", counter, start, counter, end, counter)
}
