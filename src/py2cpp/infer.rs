use crate::py2cpp::types::{Type, Argument, Instruction, Value, Code, Context};
use crate::py2cpp::constants::NATIVE_FUNS;

fn store_arg_types(name: &String, called_funs: &mut Vec<String>, fun_types: &mut Vec<Vec<Type>>, arguments: &Vec<Argument>) {
    let mut arg_types = Vec::new();
    if !NATIVE_FUNS.contains(&name.as_str()) {
        for argument in arguments.iter() {
            arg_types.push(argument.type_.clone());
            match &argument.value {
                Value::CallFun { name, arguments } => {
                    store_arg_types(&name, called_funs, fun_types, &arguments);
                },
                _ => {}
            }
        }
        called_funs.push(name.to_string());     // store function name
        fun_types.push(arg_types);              // store argument types
    }
}

pub fn param_types(code: &mut Code) -> Result<(), String> {
    let mut called_funs = Vec::new();
    let mut fun_types = Vec::new();

    // find argument types
    for fun in code.functions.iter() {
        for instruction in fun.body.iter() {
            match instruction {
                Instruction::CallFun { name, arguments } => {
                    store_arg_types(name, &mut called_funs, &mut fun_types, arguments);
                },
                Instruction::CreateVar { type_: _, name: _, value } => {
                    match value {
                        Value::CallFun { name, arguments } => {
                            store_arg_types(name, &mut called_funs, &mut fun_types, arguments);
                        },
                        _ => {}
                    }
                },
                Instruction::ReassignVar { type_: _, name: _, value } => {
                    match value {
                        Value::CallFun { name, arguments } => {
                            store_arg_types(name, &mut called_funs, &mut fun_types, arguments);
                        },
                        _ => {}
                    }
                },
                _ => {}
            };
        }
    }

    // update param types
    let mut context = Context::get_fun_types(code);
    for fun in code.functions.iter_mut() {
        let fun_name = fun.name.to_string();
        if !called_funs.contains(&fun_name) {
            continue;                                       // exclude uncalled functions
        }
        for (call_index, call_name) in called_funs.iter().enumerate() {
            if &fun_name == call_name {
                for (arg_index, mut param) in fun.params.iter_mut().enumerate() {
                    let arg_type = fun_types[call_index][arg_index].clone();  // argument type
                    param.type_ = match &param.type_ {
                        Type::Undefined => arg_type,
                        param_type if param_type != &arg_type => Type::Generic,
                        _ => param.type_.clone()
                    }
                }
            }
        }
        for param in fun.params.iter() {
            context.0.insert(param.name.to_string(), vec![param.clone()]);
        }
        for instruction in fun.body.iter() {
            match instruction {
                Instruction::Loop { counter: _, start, end, content } => {
                    match start {
                        Value::UseVar(variable) => {
                            match context.get_type(variable) {
                                Ok(type_) => {
                                    if type_ != Type::Int {
                                        return Err("Se debe usar números para definir los limites del for".to_string());
                                    }
                                },
                                Err(_) => {}
                            }
                        },
                        _ => {}
                    }
                    match end {
                        Value::UseVar(variable) => {
                            match context.get_type(variable) {
                                Ok(type_) => {
                                    if type_ != Type::Int {
                                        return Err("Se debe usar números para definir los limites del for".to_string());
                                    }
                                },
                                Err(_) => {}
                            }
                        },
                        _ => {}
                    }
                    for instruction in content.iter() {
                        match instruction {
                            Instruction::ReassignVar { type_: _, name: _, value } => {
                                match value {
                                    Value::Expression { operators: _, values } => {
                                        for value in values {
                                            match value {
                                                Value::CallFun { name, arguments } => {
                                                    match name.as_str() {
                                                        "at" => {
                                                            match &arguments.get(0).unwrap().value {
                                                                Value::UseVar(data) => {
                                                                    match context.get_type(&data) {
                                                                        Ok(data) => {
                                                                            if data != Type::Vector(Box::new(Type::Int)) {
                                                                                return Err("Error no se puede asignar un String a un int".to_string())
                                                                            }
                                                                        },
                                                                        Err(_) => {}
                                                                    }
                                                                }, _ => {}
                                                            }
                                                        }, _ => {}
                                                    }
                                                }, _ => {}
                                            }
                                        }
                                    }, _ => {}
                                }
                            }, _ => {}
                        }
                    }
                }, _ => {}
            }
        }
    }
    Ok(())
}

pub fn get_return_type(body: &mut Vec<Instruction>) -> Type {
    for instruction in body.iter() {
        match instruction {
            Instruction::Return { type_, value: _ } => {
                return type_.clone();
            },
            _ => {}
        }
    }
    Type::Void
}

pub fn get_var_type(var_name: &str, body: &Vec<Instruction>) -> Type {
    let mut result = Type::Undefined;
    for instruction in body.iter() {
        match instruction {
            Instruction::CreateVar { type_, name, value: _ } => {
                if var_name == name {
                    result = type_.clone();
                }
            },
            _ => {}
        }
    }
    result
}
