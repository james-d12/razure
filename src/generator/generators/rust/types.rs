use crate::generator::ConversionType;
use crate::parser::schema::{DefinitionPropertyType, PropertyType};

impl ConversionType for DefinitionPropertyType {
    fn get_type_as_string(&self) -> Option<&str> {
        match self {
            DefinitionPropertyType::String => Some("String"),
            DefinitionPropertyType::Number => Some("f32"),
            DefinitionPropertyType::Integer => Some("i32"),
            DefinitionPropertyType::Boolean => Some("bool"),
            DefinitionPropertyType::Object => Some("u8"),
            DefinitionPropertyType::Array => Some("u8"),
        }
    }
}

impl ConversionType for PropertyType {
    fn get_type_as_string(&self) -> Option<&str> {
        match self {
            PropertyType::String => Some("String"),
            PropertyType::Number => Some("f32"),
            PropertyType::Integer => Some("i32"),
            PropertyType::Boolean => Some("bool"),
            _ => None,
        }
    }
}
