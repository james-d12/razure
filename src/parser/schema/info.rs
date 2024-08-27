use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Info {
    pub title: String,
    pub version: String,
    pub description: Option<String>,
    pub summary: Option<String>,
    #[serde(rename = "termsOfService")]
    pub terms_of_service: Option<String>,
}
