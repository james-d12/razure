use serde::{Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SchemaFormat {
    #[serde(rename = "date-time")]
    DateTime,
    Email,
    Uuid,
    Uri,
    Hostname,
    Ipv4,
    Ipv6,
    Float,
    Double,
    Int32,
    Int64,
    Binary,
    Byte,
    Password,
    None,
}
