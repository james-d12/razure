use crate::string_formatter::{format_name_as_valid_struct_identifier, RustType};
use parser::schema::parameter::PropertyType;
use parser::schema::swagger::Swagger;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Write};

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

pub fn create_parameters_file(
    name: &str,
    parameter_structs: &HashMap<String, String>,
) -> Result<(), Error> {
    let mut parameters_file =
        File::create(format!("C:/Users/User/Downloads/razure-output/{name}.rs"))?;

    for (name, parameter_struct) in parameter_structs {
        let mut str = parameter_struct.clone();
        str.push('\n');
        parameters_file.write_all(str.as_ref())?;
    }

    Ok(())
}
