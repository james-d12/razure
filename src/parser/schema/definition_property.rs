use crate::parser::schema::schema_format::SchemaFormat;
use crate::parser::schema::schema_type::SchemaType;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DefinitionProperty {
    description: String,
    description_type: SchemaType,
    format: Option<SchemaFormat>,
    #[serde(rename = "readOnly")]
    read_only: Option<bool>,
}
