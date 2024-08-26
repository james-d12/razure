use crate::parser::schema::info::Info;
use crate::parser::schema::method::Method;
use crate::parser::schema::operation::Operation;
use crate::parser::schema::parameter_type::ParameterType;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Swagger {
    swagger: String,
    info: Option<Info>,
    schemes: Option<Vec<String>>,
    host: Option<String>,
    consumes: Option<Vec<String>>,
    produces: Option<Vec<String>>,
    paths: Option<HashMap<String, HashMap<Method, Option<Operation>>>>,
    //definition: Option<HashMap<String, Definition>>
}

impl Swagger {
    pub fn walk(&self) {
        match &self.info {
            Some(info) => println!("--------{0}-------- ", info.title),
            None => println!("----------------------"),
        }

        for (endpoint, path) in self.paths.as_ref().unwrap() {
            println!(" Endpoint: {0}", endpoint);
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
                                    println!("  Parameter: ");
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
