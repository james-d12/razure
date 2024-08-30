use crate::schema::parameter_type::ParameterType;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DefinitionPropertyType {
    Object,
    String,
    Number,
    Integer,
    Boolean,
    Array,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum DefinitionType {
    Object {
        properties: HashMap<String, DefinitionProperty>,
    },
    Array {
        items: Vec<DefinitionType>,
    },
    Other {
        #[serde(flatten)]
        additional: HashMap<String, Value>,
    },
}

#[derive(Debug, Deserialize)]
pub struct DefinitionProperty {
    #[serde(flatten)]
    pub schema: DefinitionType,
    pub description: Option<String>,
    pub pattern: Option<String>,
    #[serde(rename = "type")]
    pub definition_property_type: Option<DefinitionPropertyType>,
    #[serde(rename = "$ref")]
    pub reference: Option<String>,
    #[serde(rename = "readOnly")]
    pub read_only: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Definition {
    #[serde(flatten)]
    pub schema: DefinitionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allOf")]
    pub all_of: Option<Vec<ParameterType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Display for DefinitionPropertyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let property_type_str = match self {
            DefinitionPropertyType::Object => "object",
            DefinitionPropertyType::String => "string",
            DefinitionPropertyType::Number => "number",
            DefinitionPropertyType::Integer => "integer",
            DefinitionPropertyType::Boolean => "boolean",
            DefinitionPropertyType::Array => "array",
        };
        write!(f, "{0}", property_type_str)
    }
}
