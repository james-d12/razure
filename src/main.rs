mod models;

use crate::models::Swagger;
use chrono::NaiveDate;
use git2::Repository;
use serde_json::from_reader;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;
use std::{fs, result};

struct SpecificationFile {
    pub file_path: String,
    pub naive_date: NaiveDate,
}

fn get_json_files_for_directory(directory: &str) -> Result<HashMap<String, SpecificationFile>, std::io::Error> {
    let mut specification_files_hashmap: HashMap<String, SpecificationFile> = HashMap::new();

    let files = fs::read_dir(directory)?;

    for file in files.flatten() {
        if file.path().is_dir() {
            let json_files_for_directory =
                get_json_files_for_directory(file.path().to_str().unwrap())?;
            specification_files_hashmap.extend(json_files_for_directory);
        }

        if file.path().is_file() {
            if let Some(extension) = file.path().extension() {
                let full_path = file.path().clone();
                let full_path_str = full_path.to_str().unwrap();

                let contains_example = full_path_str.contains("example");
                let contains_preview = full_path_str.contains("preview");
                let extension_is_json = extension == "json";

                if contains_example || contains_preview || !extension_is_json {
                    continue;
                }

                let parent = full_path.parent().unwrap();

                let date_time = NaiveDate::parse_from_str(
                    parent.file_name().unwrap().to_str().unwrap(),
                    "%Y-%m-%d",
                );

                if let Ok(date_time) = date_time {
                    let key = full_path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();

                    let current_specification_file = specification_files_hashmap.get(&key);

                    match current_specification_file {
                        Some(current_specification_file) => {
                            if &date_time > &current_specification_file.naive_date {
                                specification_files_hashmap.insert(key, SpecificationFile {
                                    file_path: full_path_str.to_string(),
                                    naive_date: date_time,
                                });
                            }
                        }
                        None => {
                            specification_files_hashmap.insert(key, SpecificationFile {
                                file_path: full_path_str.to_string(),
                                naive_date: date_time,
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(specification_files_hashmap)
}

fn iterate_over_specifications(specification_path: &str) -> HashMap<String, SpecificationFile> {
    let json_files = get_json_files_for_directory(specification_path);

    match json_files {
        Ok(json_files) => {
            json_files
        }
        Err(error) => {
            println!("There was an error whilst trying to get JSON files: {0}", error);
            HashMap::new()
        }
    }
}

fn get_latest_stable_specifications() -> HashMap<String, SpecificationFile> {
    // Download azure GitHub repo
    // Go through each file and get the latest stable *.json file
    // Put all these files in one flat directory.

    println!("Getting latest Stable Azure Specifications");
    let url = "https://github.com/Azure/azure-rest-api-specs.git";
    let output_path = "C:\\Users\\User\\Downloads\\Output";
    let specification_path = format!("{0}\\specification", output_path);

    let already_downloaded = Path::new(output_path).exists();

    if already_downloaded {
        println!("Azure Repository already downloaded.");
        return iterate_over_specifications(&specification_path);
    }

    match Repository::clone(url, output_path) {
        Ok(_) => iterate_over_specifications(&specification_path),
        Err(error) => {
            eprintln!(
                "Failed to get Azure Specification Repository due to error: {0}",
                error
            );
            HashMap::new()
        }
    }
}

fn parse_specification_file(specification_file: &SpecificationFile) -> bool  {
    //println!("Parsing Specification File: {0}", specification_file.file_path);
    let file = File::open(&specification_file.file_path).expect("file not found");
    let reader = BufReader::new(file);

    // Deserialize the JSON content to `Swagger`.
    let swagger: serde_json::error::Result<Swagger> = from_reader(reader);

    match swagger {
        Ok(Swagger) => true,
        Err(error) => {
            eprintln!("Could not parse: {0} due to error: {1}", specification_file.file_path, error);
            false
        }
    }
}

fn main() {
    let now = Instant::now();
    let specifications = get_latest_stable_specifications();
    
    let mut failed_parses = 0;
    let specification_len = specifications.len();
    
    for (key, specification_file) in specifications {
        let result = parse_specification_file(&specification_file);
        
        if result == true { failed_parses += 1 } 
    }
    
    
    println!("Total number of failed parses: {0}/{1}", failed_parses, specification_len);
    
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
