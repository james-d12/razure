use crate::parser::schema::{Definition, Info, Parameter, PathItem};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Swagger {
    pub swagger: String,
    pub info: Option<Info>,
    pub schemes: Option<Vec<String>>,
    pub host: Option<String>,
    pub consumes: Option<Vec<String>>,
    pub produces: Option<Vec<String>>,
    pub paths: Option<HashMap<String, PathItem>>,
    pub parameters: Option<HashMap<String, Parameter>>,
    pub definitions: Option<HashMap<String, Definition>>,
}
