use std::collections::HashMap;
use crate::py2cpp::types::{Type, Value, Instruction, Library, Code};
use crate::py2cpp::constants::{RE_FUN, RE_LOOP, RE_INT, RE_VAR};
use crate::py2cpp::instructions::{custom_fun, len};

pub fn py2code(code: &mut Code, body: &mut Vec<Instruction>, fun_types: &HashMap<String, Type>, content: &str) -> Option<(Vec<Instruction>, Vec<Library>)> {
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
                        let cap_fun = RE_FUN.captures(text).unwrap();
                        let fun = cap_fun.get(0).unwrap().as_str();
                        let fun_name = cap_fun.get(1).unwrap().as_str();
                        let (instructions, _libraries) = match fun_name {
                            "len" => len::py2code(fun).unwrap(),
                            _ => custom_fun::py2code(body, fun_types, text).unwrap()
                        };
                        instructions[0].inst2value()
                    }
                    _ => Value::None
                };
            }
            let [start, end] = values;
            let value = data.get(4).unwrap().as_str();
            let loop_body = Code::shift_code_left(value);
            let content = code.get_instructions(body, loop_body);
            let instruction = Instruction::Loop { counter, start, end, content };
            Some((vec![instruction], vec![]))
        },
        None => None
    }
}

pub fn code2cpp(counter: &String, start: &Value, end: &Value, content: &Vec<Instruction>, tabs: u32) -> String {
    let params = [start, end];
    let mut values = [String::new(), String::new()];
    for (index, param) in params.iter().enumerate() {
        values[index] = match param {
            Value::ConstValue(data) | Value::UseVar(data) => data.to_string(),
            Value::CallFun { name, arguments } => {
                match name.as_str() {
                    "len" => len::code2cpp(&arguments[0]),
                    _ => custom_fun::code2cpp(name, arguments, false)
                }
            }
            _ => String::new()
        };
    }
    let [start, end] = values;
    let header = format!("for (int {} = {}; {} < {}; {}++)", counter, start, counter, end, counter);
    let mut body = Instruction::insts2cpp(content, tabs);
    for _ in 1..tabs {
        body.push_str( "    ");
    }

    format!("{} {{\n{}}}", header, body)
}
