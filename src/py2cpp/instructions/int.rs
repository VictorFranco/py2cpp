use crate::py2cpp::types::{Type, Argument, Value, Instruction, Library, Context};
use crate::py2cpp::constants::{RE_INT_FUN, RE_FUN, RE_STR, RE_VAR};
use crate::py2cpp::instructions::{custom_fun, declare};

pub fn py2code(context: &mut Context, content: &str) -> Result<Option<(Vec<Instruction>, Vec<Library>)>, String> {
    let cap_int = RE_INT_FUN.captures(content);

    match cap_int {
        Some(data) => {
            let mut libraries = Library::get_libraries(&["string"]);
            let mut instructions = Vec::new();
            let type_ = Type::String;

            let content = data.get(1).unwrap().as_str().to_string();
            let result= match content.as_str() {
                text if RE_STR.is_match(text) => Ok(Value::ConstValue(content)),
                text if RE_VAR.is_match(text) => Ok(Value::UseVar(content)),
                text if RE_FUN.is_match(text) => {
                    let cap_fun = RE_FUN.captures(text).unwrap();
                    let fun_name = cap_fun.get(1).unwrap().as_str();
                    let var_name = "aux".to_string();
                    let result = match fun_name {
                        "input" => {
                            let new_var = format!("{} = {}", var_name, text);
                            declare::py2code(context, &new_var)
                        },
                        _ => custom_fun::py2code(context, text)
                    };
                    match result {
                        Ok(option) => {
                            match option {
                                Some((mut fun_instructions, mut fun_libraries)) => {
                                    libraries.append(&mut fun_libraries);
                                    instructions.append(&mut fun_instructions);
                                    Ok(Value::UseVar(var_name))
                                },
                                None => Ok(Value::None)
                            }
                        }
                        Err(error) => Err(error)
                    }
                },
                _ => Ok(Value::None)
            };
            let value;
            match result {
                Ok(val) => value = val,
                Err(error) => return Err(error)
            }
            let argument = Argument { type_, value };
            let name = "int".to_string();
            let arguments = vec![argument];
            instructions.push(Instruction::CallFun { name, arguments });
            Ok(Some((instructions, libraries)))
        },
        None => Ok(None)
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
