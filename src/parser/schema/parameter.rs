use crate::parser::schema::reference::Reference;
use crate::parser::schema::schema_type::SchemaType;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Parameter {
    name: Option<String>,
    #[serde(rename = "in")]
    location: Option<String>,
    required: Option<bool>,
    schema: Option<Reference>, // Inline schema reference
    type_id: Option<SchemaType>,
}
