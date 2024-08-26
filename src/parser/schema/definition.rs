use crate::parser::schema::definition_property::DefinitionProperty;
use crate::parser::schema::schema_type::SchemaType;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Definition {
    description: String,
    #[serde(rename = "type")]
    definition_type: SchemaType,
    properties: HashMap<String, DefinitionProperty>,
}
