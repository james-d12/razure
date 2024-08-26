use crate::parser::schema::info::Info;
use crate::parser::schema::method::Method;
use crate::parser::schema::operation::Operation;
use crate::parser::schema::parameter_type::ParameterType;
use serde::Deserialize;
use std::collections::HashMap;
use crate::parser::schema::definition::Definition;

#[derive(Deserialize, Debug)]
pub struct Swagger {
    swagger: String,
    info: Option<Info>,
    schemes: Option<Vec<String>>,
    host: Option<String>,
    consumes: Option<Vec<String>>,
    produces: Option<Vec<String>>,
    paths: Option<HashMap<String, HashMap<Method, Option<Operation>>>>,
    //definitions: Option<HashMap<String, Definition>>
}

impl Swagger {
    pub fn walk(&self) {
        for (endpoint, path) in self.paths.as_ref().unwrap() {
            println!(" Path: {0}", endpoint);
            for (method, operation) in path {
                match operation {
                    Some(op) => {
                        println!("  Method: {0}", method);
                        println!("  Id: {0}", op.id);

                        if let Some(description) = &op.description {
                            println!("  Description: {0}", description);
                        }

                        for parameter in &op.parameters {
                            match parameter {
                                ParameterType::Reference(reference) => {
                                    println!("  Ref Parameter");
                                    println!("    Path: {0}", reference.path);
                                }
                                ParameterType::Parameter(inline) => {
                                    println!("  Inline Parameter: ");
                                }
                            }
                        }
                    }
                    None => continue,
                }
            }
        }
    }
}
