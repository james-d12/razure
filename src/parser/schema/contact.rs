use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Contact {
    pub name: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>,
}

#[cfg(test)]
mod test {
    use serde_json::from_str;
    use crate::parser::schema::contact::Contact;

    #[test]
    fn deserialize_contact() {
        let json_string = r#"{
            "name": "Test Contact Name",
            "url": "http://www.test.com",
            "email": "support@test.io"
        }"#;

        let contact: Contact = from_str(json_string).unwrap();

        assert_eq!(contact.name.unwrap(), "Test Contact Name");
        assert_eq!(contact.url.unwrap(), "http://www.test.com");
        assert_eq!(contact.email.unwrap(), "support@test.io");
    }
}
