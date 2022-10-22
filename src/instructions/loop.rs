use std::collections::HashMap;
use crate::py2cpp::{Type, Value, Instruction, Library, Code, instruc2value};
use crate::constants::{RE_LOOP, RE_FUN, RE_INT, RE_VAR};
use crate::instructions::{print, input, custom_fun, declare, len, r#loop, r#return};

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
                        instruc2value(&instructions[0])
                    }
                    _ => Value::None
                };
            }
            let [start, end] = values;
            let value = data.get(4).unwrap().as_str();
            let body = Code::shift_code_left(value);
            let content = Code::get_instructions(code, body);
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
    let mut result = format!("for (int {} = {}; {} < {}; {}++)", counter, start, counter, end, counter);
    result.push_str( " {\n");
    for instruction in content {
        for _ in 0..tabs {
            result.push_str( "    ");
        }
        let cpp_instruction = match instruction {
            Instruction::CallFun { name, arguments } => {
                let options = [
                    print::code2cpp(name, arguments),
                    input::code2cpp(name, arguments),
                    custom_fun::code2cpp(name, arguments, true)
                ];
                options.join("")
            },
            Instruction::CreateVar { type_, name, value } => {
                declare::code2cpp(type_, name, value)
            },
            Instruction::Loop { counter, start, end, content } => {
                r#loop::code2cpp(counter, start, end, content, tabs + 1)
            },
            Instruction::Return { type_: _, value } => {
                r#return::code2cpp(value)
            }
        };
        result = format!("{}{}\n", result, cpp_instruction);
    }
    result.push_str( "    }");
    result
}
