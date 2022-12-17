use crate::py2cpp::types::{Value, Instruction, Library, Code, Context};
use crate::py2cpp::constants::{RE_FUN, RE_LOOP, RE_INT, RE_VAR};
use crate::py2cpp::instructions::{custom_fun, len};

pub fn py2code(code: &mut Code, body: &mut Vec<Instruction>, context: &mut Context, content: &str) -> Result<Option<(Vec<Instruction>, Vec<Library>)>, String> {
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
                let result = match param {
                    text if RE_INT.is_match(text) => Ok(Value::ConstValue(param.to_string())),
                    text if RE_VAR.is_match(text) => Ok(Value::UseVar(param.to_string())),
                    text if RE_FUN.is_match(text) => {
                        let cap_fun = RE_FUN.captures(text).unwrap();
                        let fun = cap_fun.get(0).unwrap().as_str();
                        let fun_name = cap_fun.get(1).unwrap().as_str();
                        let result = match fun_name {
                            "len" => len::py2code(fun),
                            _ => custom_fun::py2code(context, text)
                        };
                        match result {
                            Ok(option) => {
                                match option {
                                    Some((instructions, _libraries)) => {
                                        Ok(instructions[0].inst2value())
                                    },
                                    None => Ok(Value::None)
                                }
                            },
                            Err(error) => Err(error)
                        }
                    },
                    _ => Ok(Value::None)
                };
                match result {
                    Ok(value) => values[index] = value,
                    Err(error) => return Err(error)
                }
            }
            let [start, end] = values;
            let value = data.get(4).unwrap().as_str();
            let loop_body = Code::shift_code_left(value);
            let result = code.get_instructions(body, context, loop_body);
            match result {
                Ok(content) => {
                    let instruction = Instruction::Loop { counter, start, end, content };
                    Ok(Some((vec![instruction], vec![])))
                },
                Err(error) => Err(error)
            }
        },
        None => Ok(None)
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
