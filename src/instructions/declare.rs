use regex::Regex;
use crate::py2cpp::{Type, type2cpp, Value, Instruction, Library, get_libraries, INTEGER, STRING, VECTOR, CUSTOM_FUN};
use crate::instructions::{custom_fun, input, int};

const DECLARE: &str = r##"(?m)^([a-zA-Z][a-zA-Z0-9]*)\s*=\s*(\d+|"[a-zA-Z0-9: ]*"|\[\]|([a-zA-Z][a-zA-Z0-9]*)\(.*\)?)$"##;

fn instruc2value(instruction: &Instruction) -> Value {
    match instruction {
        Instruction::CallFun { name, arguments } => {
            let name = name.to_string();
            let arguments = arguments.to_vec();
            Value::CallFun { name, arguments }
        },
       _ => Value::None
    }
}

pub fn py2code(body: &mut Vec<Instruction>, content: &str) -> Option<(Vec<Instruction>, Vec<Library>)> {
    let re_dec = Regex::new(DECLARE).unwrap();
    let re_int = Regex::new(INTEGER).unwrap();
    let re_str = Regex::new(STRING).unwrap();
    let re_vec = Regex::new(VECTOR).unwrap();
    let re_fun = Regex::new(CUSTOM_FUN).unwrap();
    let cap_dec = re_dec.captures(content);

    match cap_dec {
        Some(data) => {
            let mut libraries = Vec::new();
            let mut instructions = Vec::new();
            let name = data.get(1).unwrap().as_str().to_string();
            let content = data.get(2).unwrap().as_str().to_string();
            let (type_, value) = match content.as_str() {
                text if re_fun.is_match(text) => {
                    let fun_name = data.get(3).unwrap().as_str();
                    let (fun_type, fun_value, mut fun_libraries) = match fun_name {
                        "input" => {
                            let (mut input_instructions, input_libraries) = input::py2code(&name, text, false).unwrap();
                            instructions.append(&mut input_instructions);
                            (Type::String, Value::None, input_libraries)
                        },
                        "int" => {
                            let (int_instructions, int_libraries) = int::py2code(text).unwrap();
                            (Type::Int, instruc2value(&int_instructions[0]), int_libraries)
                        },
                        _ => {
                            let (custom_instructions, custom_libraries) = custom_fun::py2code(body, text).unwrap();
                            (Type::Undefined, instruc2value(&custom_instructions[0]), custom_libraries)
                        }
                    };
                    libraries.append(&mut fun_libraries);
                    (fun_type, fun_value)
                },
                text if re_int.is_match(text) => (Type::Int, Value::ConstValue(content)),
                text if re_str.is_match(text) => (Type::String, Value::ConstValue(content)),
                text if re_vec.is_match(text) => {
                    libraries = get_libraries(&["vector"]);
                    (Type::Vector(Box::new(Type::Undefined)), Value::None)
                },
                _ => (Type::Undefined, Value::None)
            };
            let instruction = Instruction::CreateVar { type_, name, value };
            instructions.insert(0, instruction);
            Some((instructions, libraries))
        },
        None => None
    }
}

pub fn code2cpp(type_: &Type, name: &String, value: &Value) -> String {
    let var_name = name;
    match value {
        Value::ConstValue(value) | Value::UseVar(value) => {
            format!("{} {} = {};", type2cpp(type_), name, value)
        },
        Value::CallFun { name, arguments } => {
            let value = match name.as_str() {
                "int" => int::code2cpp(&arguments[0]),
                _ => custom_fun::code2cpp(name, arguments, false)
            };
            format!("{} {} = {};", type2cpp(type_), var_name, value)
        },
        Value::None => {
            format!("{} {};", type2cpp(type_), name)
        }
    }
}
