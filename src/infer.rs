use std::collections::HashMap;
use crate::py2cpp::{Code, Argument, Instruction, Type, Value};
use crate::constants::NATIVE_FUNS;

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

pub fn param_types(code: &mut Code) {
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
                }
                _ => {}
            };
        }
    }

    // update param types
    for fun in code.functions.iter_mut() {
        let fun_name = fun.name.to_string();
        if !called_funs.contains(&fun_name) {
            continue;                                       // exclude uncalled functions
        }
        for (call_index, call_name) in called_funs.iter().enumerate() {
            if &fun_name == call_name {
                for (arg_index, mut param) in fun.params.iter_mut().enumerate() {
                    match param.type_ {
                        Type::Undefined => {
                            param.type_ = fun_types[call_index][arg_index].clone();
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}

pub fn return_types(code: &mut Code) {
    for fun in code.functions.iter_mut() {
        let mut there_is_return = false;
        for instruction in fun.body.iter() {
            match instruction {
                Instruction::Return { type_, value: _ } => {
                    there_is_return = true;
                    fun.type_ = type_.clone();
                },
                _ => {}
            }
        }
        if !there_is_return {
            fun.type_ = Type::Void;
        }
    }
}

pub fn var_types(code: &mut Code) {
    let mut return_types = HashMap::new();
    for fun in code.functions.iter() {
        return_types.insert(fun.name.clone(), fun.type_.clone());
    }
    for fun in code.functions.iter_mut() {
        for instruction in fun.body.iter_mut() {
            match instruction {
                Instruction::CreateVar { type_, name: _, value } => {
                    match value {
                        Value::CallFun { name, arguments: _ } => {
                            match return_types.get(name) {
                                Some(data) => *type_ = data.clone(),
                                None => {}
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }
}
