use std::collections::HashMap;
use crate::py2cpp::{Type, Value, Instruction, instruc2value, Library};
use crate::constants::{RE_LOOP, RE_FUN, RE_INT, RE_VAR};
use crate::instructions::custom_fun;

pub fn py2code(body: &mut Vec<Instruction>, fun_types: &HashMap<String, Type>, content: &str) -> Option<(Vec<Instruction>, Vec<Library>)> {
    let cap_return = RE_LOOP.captures(content);

    match cap_return {
        Some(data) => {
            let counter = data.get(1).unwrap().as_str().to_string();
            let mut values = [Value::None, Value::None];
            let params = [
                data.get(2).unwrap().as_str().to_string(),
                data.get(3).unwrap().as_str().to_string()
            ];
            for (index, param) in params.iter().enumerate() {
                values[index] = match param {
                    text if RE_INT.is_match(text) => Value::ConstValue(param.to_string()),
                    text if RE_VAR.is_match(text) => Value::UseVar(param.to_string()),
                    text if RE_FUN.is_match(text) => {
                        let (instructions, _libraries) = custom_fun::py2code(body, fun_types, text).unwrap();
                        instruc2value(&instructions[0])
                    }
                    _ => Value::None
                };
            }
            let [start, end] = values;
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
        values[index] = match param {
            Value::ConstValue(data) | Value::UseVar(data) => data.to_string(),
            Value::CallFun { name, arguments } => {
                custom_fun::code2cpp(name, arguments, false)
            }
            _ => String::new()
        };
    }
    let [start, end] = values;
    format!("for (int {} = {}; {} < {}; {}++) {{}}", counter, start, counter, end, counter)
}
