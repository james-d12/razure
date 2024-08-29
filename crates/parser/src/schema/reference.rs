use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Reference {
    #[serde(rename = "$ref")]
    pub path: String,
}
