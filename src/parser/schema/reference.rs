use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Reference {
    #[serde(rename = "$ref")]
    pub path: String,
}
