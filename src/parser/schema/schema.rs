use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SchemaType {
    Null,
    Boolean,
    Object,
    Array,
    Number,
    Integer,
    String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Schema {
    #[serde(rename = "$ref")]
    pub reference: Option<String>,
    pub format: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub default: Option<serde_json::Value>,
    #[serde(rename = "multipleOf")]
    pub multiple_of: Option<f64>,
    pub maximum: Option<f64>,
    #[serde(rename = "exclusiveMaximum")]
    pub exclusive_maximum: Option<bool>,
    pub minimum: Option<f64>,
    #[serde(rename = "exclusiveMinimum")]
    pub exclusive_minimum: Option<bool>,
    #[serde(rename = "maxLength")]
    pub max_length: Option<u64>,
    #[serde(rename = "minLength")]
    pub min_length: Option<u64>,
    pub pattern: Option<String>,
    #[serde(rename = "maxItems")]
    pub max_items: Option<u64>,
    #[serde(rename = "minItems")]
    pub min_items: Option<u64>,
    #[serde(rename = "uniqueItems")]
    pub unique_items: Option<bool>,
    #[serde(rename = "maxProperties")]
    pub max_properties: Option<u64>,
    #[serde(rename = "minProperties")]
    pub min_properties: Option<u64>,
    pub required: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub schema_type: Option<SchemaType>,
}
