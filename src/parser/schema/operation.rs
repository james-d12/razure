use crate::parser::schema::parameter_type::ParameterType;
use crate::parser::schema::reference::Reference;
use crate::parser::schema::response::Response;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Operation {
    #[serde(rename = "operationId")]
    pub id: String,
    #[serde(rename = "x-ms-examples")]
    examples: Option<HashMap<String, Reference>>,
    pub description: Option<String>,
    pub parameters: Vec<ParameterType>,
    responses: HashMap<String, Response>,
}
