use crate::parser::schema::operation::Operation;
use crate::parser::schema::parameter::Parameter;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct PathItem {
    #[serde(rename = "$ref")]
    reference: Option<String>,
    get: Option<Operation>,
    put: Option<Operation>,
    post: Option<Operation>,
    delete: Option<Operation>,
    options: Option<Operation>,
    head: Option<Operation>,
    patch: Option<Operation>,
    parameters: Option<HashMap<String, Parameter>>,
}

impl PathItem {
    pub fn get_operations(&self) -> HashMap<String, &Operation> {
        let mut operations: HashMap<String, &Operation> = HashMap::new();

        if let Some(get) = &self.get {
            operations.insert("GET".to_string(), &get);
        }

        if let Some(put) = &self.put {
            operations.insert("PUT".to_string(), &put);
        }

        if let Some(post) = &self.post {
            operations.insert("POST".to_string(), &post);
        }

        if let Some(delete) = &self.delete {
            operations.insert("DELETE".to_string(), &delete);
        }

        if let Some(options) = &self.options {
            operations.insert("OPTIONS".to_string(), &options);
        }

        if let Some(head) = &self.head {
            operations.insert("HEAD".to_string(), &head);
        }

        if let Some(patch) = &self.patch {
            operations.insert("PATCH".to_string(), &patch);
        }

        operations
    }
}
