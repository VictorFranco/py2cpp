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

    pub fn get_type(&mut self, name: &str) -> Result<Type, String> {
        match self.0.get(name) {
            Some(data) => Ok(data.last().unwrap().type_.clone()),
            None => Err(format!("La variable o función \"{name}\" no encontrada"))
        }
    }

}
