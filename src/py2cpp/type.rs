use crate::py2cpp::types::Type;

impl Type {

    pub fn type2cpp(type_: &Type) -> String {
        match type_ {
            Type::Vector(type_) => {
                format!("vector<{}>", Type::type2cpp(type_))
            },
            _ => match type_ {
                Type::Int => "int",
                Type::String => "string",
                Type::Void => "void",
                Type::Undefined => "undefined",
                Type::Generic => "T",
                _ => ""
            }.to_string()
        }
    }

}
