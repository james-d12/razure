use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SchemaType {
    Object,
    String,
    Number,
    Integer,
    Boolean,
    Array,
}