use std::collections::HashMap;
use crate::py2cpp::types::{Type, Param, Code, Context};

impl Context {

    pub fn get_fun_types(code: &mut Code) -> Context {
        let mut return_types = HashMap::new();
        for fun in code.functions.iter() {
            let param = Param {
                type_: fun.type_.clone(),
                name: fun.name.to_string()
            };
            return_types.insert(
                fun.name.to_string(),
                vec![param]
            );
        }
        Context(return_types)
    }

    pub fn get_type(&mut self, name: &str) -> Type {
        let Param { type_, name: _ } = self.0.get(name).unwrap().last().unwrap().clone();
        type_
    }

}
