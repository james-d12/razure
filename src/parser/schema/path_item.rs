use crate::parser::schema::method::Method;
use crate::parser::schema::operation::Operation;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct PathItem {
    #[serde(flatten)]
    operations: HashMap<Method, Option<Operation>>, // Use the `Method` enum here
}
