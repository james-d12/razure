use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Method {
    Post,
    Get,
    Put,
    Delete,
    Patch,
}

#[derive(Debug)]
pub enum HttpStatus {
    Ok,
    Created,
    Accepted,
    NoContent,
    Default,
}

#[derive(Deserialize, Debug)]
pub struct PathItem {
    #[serde(flatten)]
    operations: HashMap<Method, Option<Operation>>, // Use the `Method` enum here
}

#[derive(Deserialize, Debug)]
pub struct Info {
    title: String,
    version: String,
    description: String,
}

#[derive(Deserialize, Debug)]
pub struct Operation {
    #[serde(rename = "operationId")]
    id: String,
    #[serde(rename = "x-ms-examples")]
    examples: HashMap<String, Reference>,
    description: String,
    parameters: Vec<Parameter>,
    responses: HashMap<String, Response>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Parameter {
    Ref(Reference),          // For `$ref` parameters
    Inline(InlineParameter), // For inline parameters with more details
}

#[derive(Deserialize, Debug)]
pub struct Reference {
    #[serde(rename = "$ref")]
    path: String,
}

// Define the struct for inline parameters
#[derive(Deserialize, Debug)]
pub struct InlineParameter {
    name: Option<String>,
    #[serde(rename = "in")]
    location: Option<String>,
    required: Option<bool>,
    schema: Option<Reference>, // Inline schema reference
}

#[derive(Deserialize, Debug)]
pub struct Response {
    description: String,
    schema: Option<Reference>,
}

#[derive(Deserialize, Debug)]
pub struct Swagger {
    swagger: String,
    info: Info,
    schemes: Vec<String>,
    host: String,
    consumes: Vec<String>,
    produces: Vec<String>,
    paths: HashMap<String, HashMap<Method, Option<Operation>>>,
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let method_str = match self {
            Method::Post => "POST",
            Method::Get => "GET",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Patch => "PATCH",
        };
        write!(f, "{}", method_str)
    }
}

impl<'de> Deserialize<'de> for Method {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let method_str = String::deserialize(deserializer)?;

        match method_str.to_lowercase().as_str() {
            "post" => Ok(Method::Post),
            "get" => Ok(Method::Get),
            "put" => Ok(Method::Put),
            "delete" => Ok(Method::Put),
            "patch" => Ok(Method::Put),
            _ => Err(serde::de::Error::unknown_variant(
                &method_str,
                &["post", "get", "put", "delete", "patch"],
            )),
        }
    }
}

impl<'de> Deserialize<'de> for HttpStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let http_status_str = String::deserialize(deserializer)?;

        match http_status_str.to_lowercase().as_str() {
            "200" => Ok(HttpStatus::Ok),
            "201" => Ok(HttpStatus::Created),
            "202" => Ok(HttpStatus::Accepted),
            "204" => Ok(HttpStatus::NoContent),
            "default" => Ok(HttpStatus::Default),
            _ => Err(serde::de::Error::unknown_variant(
                &http_status_str,
                &["200", "201", "202", "204", "default"],
            )),
        }
    }
}

impl Swagger {
    pub fn walk(&self) {
        println!("{0}", self.info.title);
        for (endpoint, path) in &self.paths {
            println!("----------------------");
            println!("Endpoint: {0}", endpoint);
            for (method, operation) in path {
                match operation {
                    Some(op) => {
                        println!("Method: {0}", method);
                        println!("Id: {0}", op.id);
                        println!("Description: {0}", op.description);

                        for parameter in &op.parameters {
                            match parameter {
                                Parameter::Ref(reference) => {
                                    println!("Ref Parameter");
                                    println!("  Path: {0}", reference.path);
                                }
                                Parameter::Inline(inline) => {
                                    println!("Inline Parameter");
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
