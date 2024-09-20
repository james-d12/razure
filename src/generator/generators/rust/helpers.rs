use std::collections::HashMap;
use crate::generator::ConversionType;
use crate::generator::rust::{format_field_as_valid_field_identifier, format_name_as_valid_struct_identifier};
use crate::parser::schema::DefinitionProperty;

pub fn create_struct_simple_type(name: &String, struct_type: String) -> String {
    let formatted_name = format_name_as_valid_struct_identifier(name);
    format!(r#"pub struct {0}({1});"#, formatted_name, struct_type)
}

pub fn create_struct(name: &String, properties: &HashMap<String, DefinitionProperty>) -> String {
    let formatted_name = format_name_as_valid_struct_identifier(name);

    let mut properties_string = String::new();

    for (property_name, property) in properties {
        if let Some(property_type) = &property.definition_property_type {
            let property_type_str = property_type.get_type_as_string();
            let formatted_property_name = format_field_as_valid_field_identifier(property_name);

            if let Some(property_type_str) = property_type_str {
                let formatted_property =
                    format!("pub {0}: {1},", formatted_property_name, property_type_str);
                properties_string.push_str(formatted_property.as_str());
            }
        }
    }

    format!(
        "pub struct {0} {{ {1} }}",
        formatted_name, properties_string
    )
}