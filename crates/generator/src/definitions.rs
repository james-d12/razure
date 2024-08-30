use crate::string_formatter::{format_name_as_valid_struct_identifier, RustType};
use parser::schema::definition::{DefinitionProperty, DefinitionPropertyType, DefinitionType};
use parser::schema::swagger::Swagger;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{Error, Write};

impl RustType for DefinitionPropertyType {
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

fn create_struct(name: &String, properties: &HashMap<String, DefinitionProperty>) -> String {
    let formatted_name = format_name_as_valid_struct_identifier(name);

    let mut properties_string = String::new();

    for (property_name, property) in properties {
        if let Some(property_type) = &property.definition_property_type {
            let property_type_str = property_type.get_type_as_string();

            if let Some(property_type_str) = property_type_str {
                let formatted_property = format!("pub {0}: {1},", property_name, property_type_str);
                properties_string.push_str(formatted_property.as_str());
            }
        }
    }

    format!(
        "pub struct {0} {{ {1} }}",
        formatted_name, properties_string
    )
}

pub fn generate_definitions(swagger: &Swagger) -> Option<HashMap<String, String>> {
    let mut structs: HashMap<String, String> = HashMap::new();

    if let Some(definitions) = &swagger.definitions {
        for (name, definition) in definitions {
            match &definition.schema {
                DefinitionType::Object { properties } => {
                    let struct_str = create_struct(name, properties);
                    structs.insert(name.to_string(), struct_str);
                }
                _ => {}
            }
        }

        Some(structs)
    } else {
        None
    }
}

pub fn create_definitions_file(definition_structs: &HashMap<String, String>) -> Result<(), Error> {
    let mut definitions_file =
        File::create("C:/Users/User/Downloads/razure-output/definitions.rs")?;

    for (name, parameter_struct) in definition_structs {
        let mut str = parameter_struct.clone();
        str.push('\n');
        definitions_file.write_all(str.as_ref())?;
    }

    Ok(())
}
