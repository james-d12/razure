use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SchemaType {
    Object {
        #[serde(flatten)]
        properties: HashMap<String, SchemaProperty>,
        #[serde(flatten)]
        additional: HashMap<String, Value>,
    },
    Array {
        items: Box<SchemaType>,
        #[serde(flatten)]
        additional: HashMap<String, Value>,
    },
    String {
        #[serde(flatten)]
        additional: HashMap<String, Value>,
    },
    Number {
        #[serde(flatten)]
        additional: HashMap<String, Value>,
    },
    Integer {
        #[serde(flatten)]
        additional: HashMap<String, Value>,
    },
    Boolean {
        #[serde(flatten)]
        additional: HashMap<String, Value>,
    },
}

#[derive(Debug, Deserialize)]
pub struct SchemaProperty {
    #[serde(flatten)]
    pub schema: SchemaType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub schema_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "$ref")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "readOnly")]
    pub read_only: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SchemaDefinition {
    #[serde(flatten)]
    pub schema: SchemaType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allOf: Option<Vec<Reference>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Reference {
    #[serde(rename = "$ref")]
    pub ref_path: String,
}
