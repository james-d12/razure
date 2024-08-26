use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub enum SchemaFormat {
    DateTime,
    String,
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
