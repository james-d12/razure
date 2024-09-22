use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Reference {
    #[serde(rename = "$ref")]
    pub path: String,
}

#[cfg(test)]
mod tests {
    use crate::parser::schema::Reference;
    use serde_json::from_str;

    #[test]
    fn deserialize_reference() {
        let reference_string = "#/parameters/subscriptionIdParameter";

        let json_string = format!(
            r#"{{
            "$ref": "{reference_string}"
        }}"#
        );

        let reference: Reference = from_str(json_string.as_str()).unwrap();

        assert_eq!(reference.path, reference_string);
    }
}
