use crate::filesystem::SpecificationFile;
use crate::parser::schema::Swagger;
use log::error;
use serde_json::{error::Result, from_reader};
use std::fs::File;
use std::io::BufReader;

pub fn parse_specification_file(specification_file: &SpecificationFile) -> Result<Swagger> {
    let file = File::open(&specification_file.file_path).expect("file not found");
    let reader = BufReader::new(file);

    let swagger: Result<Swagger> = from_reader(reader);

    match swagger {
        Ok(swagger) => Ok(swagger),
        Err(error) => {
            error!(
                "Could not parse: {0} due to error: {1}",
                specification_file.file_path, error
            );
            Err(error)
        }
    }
}
