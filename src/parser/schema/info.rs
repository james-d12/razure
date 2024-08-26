use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Info {
    pub title: String,
    version: String,
    description: Option<String>,
    summary: Option<String>,
    #[serde(rename = "termsOfService")]
    terms_of_service: Option<String>,
}
