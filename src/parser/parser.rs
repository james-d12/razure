use crate::filesystem::SpecificationFile;
use crate::parser::schema::Swagger;
use serde_json::{error::Result, from_reader};
use std::fs::File;
use std::io::BufReader;

pub fn parse_specification_file(specification_file: &SpecificationFile) -> Option<Swagger> {
    let file = File::open(&specification_file.file_path).expect("file not found");
    let reader = BufReader::new(file);

    let swagger: Result<Swagger> = from_reader(reader);

    match swagger {
        Ok(swagger) => Some(swagger),
        Err(error) => {
            eprintln!(
                "Could not parse: {0} due to error: {1}",
                specification_file.file_path, error
            );
            None
        }
    }
}
