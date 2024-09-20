use std::collections::HashMap;
use crate::filesystem::SpecificationFile;

pub trait Generator {
    fn generate(&mut self, output_path: &str, specifications: &HashMap<String, SpecificationFile>);    
    fn is_empty(&self) -> bool;
}

pub trait ConversionType {
    fn get_type_as_string(&self) -> Option<&str>;
}