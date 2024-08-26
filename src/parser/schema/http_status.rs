use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub enum HttpStatus {
    Ok,
    Created,
    Accepted,
    NoContent,
    Default,
}

impl<'de> Deserialize<'de> for HttpStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let http_status_str = String::deserialize(deserializer)?;

        match http_status_str.to_lowercase().as_str() {
            "200" => Ok(HttpStatus::Ok),
            "201" => Ok(HttpStatus::Created),
            "202" => Ok(HttpStatus::Accepted),
            "204" => Ok(HttpStatus::NoContent),
            "default" => Ok(HttpStatus::Default),
            _ => Err(serde::de::Error::unknown_variant(
                &http_status_str,
                &["200", "201", "202", "204", "default"],
            )),
        }
    }
}
