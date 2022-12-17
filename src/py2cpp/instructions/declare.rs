use rand::Rng;
use crate::py2cpp::types::{Type, Param, Value, Instruction, Library, Context};
use crate::py2cpp::constants::{RE_FUN, RE_DEC, RE_EXP, RE_AT, RE_INT, RE_STR, RE_VEC, RE_VAR};
use crate::py2cpp::instructions::{input, custom_fun, int, len, at};

pub fn py2code(context: &mut Context, content: &str) -> Result<Option<(Vec<Instruction>, Vec<Library>)>, String> {
    let cap_dec = RE_DEC.captures(content);

    match cap_dec {
        Some(data) => {
            let mut libraries = Vec::new();
            let mut instructions = Vec::new();
            let name = data.get(1).unwrap().as_str().to_string();
            let content = data.get(2).unwrap().as_str().to_string();
            let mut first_declare = false;
            let result: Result<(Type, Value), String> = match content.as_str() {
                text if RE_EXP.is_match(text) => {
                    let result = Value::exp2value(context, text);
                    match result {
                        Ok(value) => Ok((Type::Int, value)),
                        Err(error) => Err(error)
                    }
                },
                text if RE_INT.is_match(text) => Ok((Type::Int, Value::ConstValue(content))),
                text if RE_STR.is_match(text) => Ok((Type::String, Value::ConstValue(content))),
                text if RE_VAR.is_match(text) => {
                    let result = context.get_type(text);
                    match result {
                        Ok(type_) => Ok((type_, Value::UseVar(content))),
                        Err(error) => Err(error)
                    }
                },
                text if RE_VEC.is_match(text) => {
                    libraries = Library::get_libraries(&["vector"]);
                    Ok((Type::Vector(Box::new(Type::Undefined)), Value::None))
                },
                text if RE_AT.is_match(text) => {
                    let result = at::py2code(context, text);
                    match result {
                        Ok(some) => {
                            match some {
                                Some((at_instructions, _)) => Ok((Type::Int, at_instructions[0].inst2value())),
                                None => Err(String::new())
                            }
                        },
                        Err(error) => Err(error)
                    }
                },
                text if RE_FUN.is_match(text) => {
                    let cap_fun = RE_FUN.captures(text).unwrap();
                    let fun_name = cap_fun.get(1).unwrap().as_str();
                    let result = match fun_name {
                        "input" => {
                            first_declare = true;
                            let result = input::py2code(context, &name, text, false);
                            match result {
                                Ok(some) => {
                                    match some {
                                        Some((mut input_instructions, input_libraries)) => {
                                            instructions.append(&mut input_instructions);
                                            Ok((Type::String, Value::None, input_libraries))
                                        },
                                        None => Err(String::new())
                                    }
                                },
                                Err(error) => Err(error)
                            }
                        },
                        "int" => {
                            let result = int::py2code(context, text);
                            match result {
                                Ok(some) => {
                                    match some {
                                        Some((mut int_instructions, int_libraries)) => {
                                            instructions.append(&mut int_instructions);
                                            let call_instr = instructions.pop().unwrap();
                                            let value = call_instr.inst2value();
                                            Ok((Type::Int, value, int_libraries))
                                        },
                                        None => Err(String::new())
                                    }
                                },
                                Err(error) => Err(error)
                            }
                        },
                        "len" => {
                            let result = len::py2code(context, text);
                            match result {
                                Ok(some) => {
                                    match some {
                                        Some((len_instructions, len_libraries)) => {
                                            Ok((Type::Int, len_instructions[0].inst2value(), len_libraries))
                                        },
                                        None => Err(String::new())
                                    }
                                },
                                Err(error) => Err(error)
                            }
                        },
                        _ => {
                            let result = custom_fun::py2code(context, text);
                            match result {
                                Ok(some) => {
                                    match some {
                                        Some((custom_instructions, custom_libraries)) => {
                                            match context.get_type(fun_name) {
                                                Ok(type_)  => {
                                                    Ok((type_, custom_instructions[0].inst2value(), custom_libraries))
                                                },
                                                Err(error) => Err(error)
                                            }
                                        },
                                        None => Err(String::new())
                                    }
                                },
                                Err(error) => Err(error)
                            }
                        }
                    };
                    match result {
                        Ok((fun_type, fun_value, mut fun_libraries)) => {
                            libraries.append(&mut fun_libraries);
                            Ok((fun_type, fun_value))
                        },
                        Err(error) => Err(error)
                    }
                },
                _ => {
                    match content.as_str() != "" {
                        true => Ok((Type::Undefined, Value::ConstValue(content))),
                        false => Err("Falta asignar el valor".to_string())
                    }
                }
            };

            match result {
                Ok(_) => {},
                Err(error) => return Err(error)
            }

            let (type_, value) = result.ok().unwrap();

            let mut declare = Instruction::CreateVar {
                type_: type_.clone(),
                name: name.to_string(),
                value: value.clone()
            };

            match context.0.get_mut(&name) {
                Some(vec) => {
                    let mut new_vec = vec.clone();
                    for param in vec.iter() {
                        if name == param.name {
                            let type_ = type_.clone();
                            let name = name.to_string();
                            let value = value.clone();
                            declare = match type_ == param.type_ {
                                true => Instruction::ReassignVar { type_, name, value },
                                false => {
                                    let name = format!("{}{}", name, rand::thread_rng().gen_range(0..100));
                                    new_vec.push(
                                        Param {
                                            type_: type_.clone(),
                                            name: name.to_string(),
                                        }
                                    );
                                    Instruction::CreateVar { type_, name, value }
                                }
                            };
                        }
                    }
                    vec.append(&mut new_vec);
                },
                None => {
                    let param = Param {
                        type_: type_.clone(),
                        name: name.to_string(),
                    };
                    context.0.insert(
                        name.to_string(),
                        vec![param]
                    );
                }
            };

            match first_declare {
                true  => instructions.insert(0, declare),
                false => instructions.push(declare)
            }
            Ok(Some((instructions, libraries)))
        },
        None => Ok(None)
    }
}

pub fn code2cpp(type_: &Type, name: &String, value: &Value, reuse_var: bool) -> String {
    let result = match reuse_var {
        true  => format!("{} {}", type_.type2cpp(), name),
        false => format!("{}", name)
    };
    match value {
        Value::ConstValue(value) | Value::UseVar(value) => {
            format!("{} = {};", result, value)
        },
        Value::CallFun { name, arguments } => {
            let value = match name.as_str() {
                "int" => int::code2cpp(&arguments[0]),
                "len" => len::code2cpp(&arguments[0]),
                "at"  => at::code2cpp(name, arguments),
                _ => custom_fun::code2cpp(name, arguments, false)
            };
            format!("{} = {};", result, value)
        },
        Value::Expression { operators, values } => {
            format!("{} = {};", result, Value::exp2cpp(operators, values))
        },
        Value::None => {
            format!("{};", result)
        }
    }
}
