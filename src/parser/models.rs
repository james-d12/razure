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
    Head,
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
    description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Operation {
    #[serde(rename = "operationId")]
    id: String,
    #[serde(rename = "x-ms-examples")]
    examples: Option<HashMap<String, Reference>>,
    description: Option<String>,
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
    type_id: Option<SchemaType>,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    description: Option<String>,
    schema: Option<Parameter>,
}

#[derive(Debug)]
pub enum SchemaType {
    Object,
    String,
}

#[derive(Debug)]
pub enum SchemaFormat {
    DateTime,
    String,
}

#[derive(Deserialize, Debug)]
pub struct DefinitionProperty {
    description: String,
    description_type: SchemaType,
    format: Option<SchemaFormat>,
}

#[derive(Deserialize, Debug)]
pub struct Definition {
    description: String,
    #[serde(rename = "type")]
    definition_type: SchemaType,
    properties: HashMap<String, DefinitionProperty>,
}

#[derive(Deserialize, Debug)]
pub struct Swagger {
    swagger: String,
    info: Info,
    schemes: Option<Vec<String>>,
    host: Option<String>,
    consumes: Option<Vec<String>>,
    produces: Option<Vec<String>>,
    paths: Option<HashMap<String, HashMap<Method, Option<Operation>>>>,
    //definition: Option<HashMap<String, Definition>>
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let method_str = match self {
            Method::Post => "POST",
            Method::Get => "GET",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Patch => "PATCH",
            Method::Head => "HEAD",
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
            "head" => Ok(Method::Head),
            _ => Err(serde::de::Error::unknown_variant(
                &method_str,
                &["post", "get", "put", "delete", "patch", "head"],
            )),
        }
    }
}

impl<'de> Deserialize<'de> for SchemaType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let definition_type_str = String::deserialize(deserializer)?;

        match definition_type_str.to_lowercase().as_str() {
            "object" => Ok(SchemaType::Object),
            "string" => Ok(SchemaType::String),
            _ => Err(serde::de::Error::unknown_variant(
                &definition_type_str,
                &["object", "string"],
            )),
        }
    }
}

impl<'de> Deserialize<'de> for SchemaFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let schema_format_str = String::deserialize(deserializer)?;

        match schema_format_str.to_lowercase().as_str() {
            "date-time" => Ok(SchemaFormat::DateTime),
            _ => Err(serde::de::Error::unknown_variant(
                &schema_format_str,
                &["date-time"],
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

        for (endpoint, path) in self.paths.as_ref().unwrap() {
            println!("----------------------");
            println!("Endpoint: {0}", endpoint);
            for (method, operation) in path {
                match operation {
                    Some(op) => {
                        println!("Method: {0}", method);
                        println!("Id: {0}", op.id);

                        if let Some(description) = &op.description {
                            println!("Description: {0}", description);
                        }

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
