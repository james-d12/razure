use serde::{Deserialize, Deserializer};
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Method {
    Post,
    Get,
    Put,
    Delete,
    Patch,
    Head,
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let method_str = match self {
            Method::Post => "POST",
            Method::Get => "GET",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Patch => "PATCH",
            Method::Head => "HEAD",
        };
        write!(f, "{}", method_str)
    }
}

impl<'de> Deserialize<'de> for Method {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let method_str = String::deserialize(deserializer)?;

        match method_str.to_lowercase().as_str() {
            "post" => Ok(Method::Post),
            "get" => Ok(Method::Get),
            "put" => Ok(Method::Put),
            "delete" => Ok(Method::Delete),
            "patch" => Ok(Method::Patch),
            "head" => Ok(Method::Head),
            _ => Err(serde::de::Error::unknown_variant(
                &method_str,
                &["post", "get", "put", "delete", "patch", "head"],
            )),
        }
    }
}
