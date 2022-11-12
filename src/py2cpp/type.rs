use crate::py2cpp::types::Type;

impl Type {

    pub fn type2cpp(&self) -> String {
        match self {
            Type::Vector(type_) => {
                format!("vector<{}>", type_.type2cpp())
            },
            _ => match self {
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
