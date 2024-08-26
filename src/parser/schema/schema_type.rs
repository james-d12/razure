use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub enum SchemaType {
    Object,
    String,
    Number,
    Integer,
    Boolean,
    Array,
}

impl<'de> Deserialize<'de> for SchemaType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let definition_type_str = String::deserialize(deserializer)?;

        match definition_type_str.to_lowercase().as_str() {
            "string" => Ok(SchemaType::String),
            "number" => Ok(SchemaType::Number),
            "integer" => Ok(SchemaType::Integer),
            "boolean" => Ok(SchemaType::Boolean),
            "array" => Ok(SchemaType::Array),
            "object" => Ok(SchemaType::Object),
            _ => Err(serde::de::Error::unknown_variant(
                &definition_type_str,
                &["object", "string"],
            )),
        }
    }
}
