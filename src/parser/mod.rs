mod schema;
use crate::filesystem::SpecificationFile;
use crate::parser::schema::swagger::Swagger;
use serde_json::from_reader;
use std::fs::File;
use std::io::BufReader;

pub fn parse_specification_file(specification_file: &SpecificationFile) -> bool {
    //println!("Parsing Specification File: {0}", specification_file.file_path);
    let file = File::open(&specification_file.file_path).expect("file not found");
    let reader = BufReader::new(file);

    // Deserialize the JSON content to `Swagger`.
    let swagger: serde_json::error::Result<Swagger> = from_reader(reader);

    match swagger {
        Ok(swagger) => {
            println!("--------{0}------", specification_file.file_path);
            swagger.walk();
            true
        }
        Err(error) => {
            eprintln!(
                "Could not parse: {0} due to error: {1}",
                specification_file.file_path, error
            );
            false
        }
    }
}
