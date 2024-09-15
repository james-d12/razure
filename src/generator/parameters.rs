use crate::generator::string_formatter::{format_name_as_valid_struct_identifier, RustType};
use crate::parser::schema::{PropertyType, Swagger};
use std::collections::HashMap;

impl RustType for PropertyType {
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

fn create_struct_simple_type(name: &String, struct_type: String) -> String {
    let formatted_name = format_name_as_valid_struct_identifier(name);
    format!(r#"pub struct {0}({1});"#, formatted_name, struct_type)
}

pub fn generate_parameters(swagger: &Swagger) -> Option<HashMap<String, String>> {
    let mut structs: HashMap<String, String> = HashMap::new();

    match &swagger.parameters {
        Some(parameters) => {
            for (name, parameter) in parameters {
                if let Some(property_type) = &parameter.property_type {
                    match property_type {
                        PropertyType::String | PropertyType::Integer | PropertyType::Number => {
                            let property_type_string = property_type.get_type_as_string();

                            if let Some(property_type_string) = property_type_string {
                                let struct_string = create_struct_simple_type(
                                    name,
                                    property_type_string.to_string(),
                                );
                                //println!("{0}", &struct_string);
                                structs.insert(name.to_string(), struct_string);
                            }
                        }
                        PropertyType::Object => {
                            println!("Object type for {0}", name)
                        }
                        _ => {}
                    }
                }
            }

            Some(structs)
        }
        None => None,
    }
}
