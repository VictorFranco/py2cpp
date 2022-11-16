use std::collections::HashMap;
use crate::py2cpp::types::{Type, Param, Argument, Value, Instruction, Library};
use crate::py2cpp::constants::{RE_INT_FUN, RE_FUN, RE_STR, RE_VAR};
use crate::py2cpp::instructions::{custom_fun, declare};

pub fn py2code(context: &mut HashMap<String, Param>, content: &str) -> Option<(Vec<Instruction>, Vec<Library>)> {
    let cap_int = RE_INT_FUN.captures(content);

    match cap_int {
        Some(data) => {
            let mut libraries = Library::get_libraries(&["string"]);
            let mut instructions = Vec::new();
            let type_ = Type::String;

            let content = data.get(1).unwrap().as_str().to_string();
            let value= match content.as_str() {
                text if RE_STR.is_match(text) => Value::ConstValue(content),
                text if RE_VAR.is_match(text) => Value::UseVar(content),
                text if RE_FUN.is_match(text) => {
                    let cap_fun = RE_FUN.captures(text).unwrap();
                    let fun_name = cap_fun.get(1).unwrap().as_str();
                    let var_name = "aux".to_string();
                    let (mut fun_instructions, mut fun_libraries) = match fun_name {
                        "input" => {
                            let new_var = format!("{} = {}", var_name, text);
                            declare::py2code(context, &new_var).unwrap()
                        },
                        _ => custom_fun::py2code(context, text).unwrap()
                    };
                    libraries.append(&mut fun_libraries);
                    instructions.append(&mut fun_instructions);
                    Value::UseVar(var_name)
                },
                _ => Value::None
            };
            let argument = Argument { type_, value };
            let name = "int".to_string();
            let arguments = vec![argument];
            instructions.push(Instruction::CallFun { name, arguments });
            Some((instructions, libraries))
        },
        None => None
    }
}

pub fn code2cpp(argument: &Argument) -> String {
    match &argument.value {
        Value::ConstValue(value) | Value::UseVar(value) => {
            format!("stoi({})", value)
        },
        _ => String::new()
    }
}
