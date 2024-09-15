use chrono::NaiveDate;
use git2::Repository;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct SpecificationFile {
    pub file_path: String,
    pub naive_date: NaiveDate,
}

fn get_json_files_for_directory(
    directory: &str,
) -> Result<HashMap<String, SpecificationFile>, std::io::Error> {
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
                    let key = full_path.file_name().unwrap().to_str().unwrap().to_string();

                    let current_specification_file = specification_files_hashmap.get(&key);

                    match current_specification_file {
                        Some(current_specification_file) => {
                            if date_time > current_specification_file.naive_date {
                                specification_files_hashmap.insert(
                                    key,
                                    SpecificationFile {
                                        file_path: full_path_str.to_string(),
                                        naive_date: date_time,
                                    },
                                );
                            }
                        }
                        None => {
                            specification_files_hashmap.insert(
                                key,
                                SpecificationFile {
                                    file_path: full_path_str.to_string(),
                                    naive_date: date_time,
                                },
                            );
                        }
                    }
                }
            }
        }
    }

    Ok(specification_files_hashmap)
}

fn get_specification_files(specification_path: &str) -> HashMap<String, SpecificationFile> {
    let json_files = get_json_files_for_directory(specification_path);

    json_files.unwrap_or_else(|error| {
        println!(
            "There was an error whilst trying to get JSON files: {0}",
            error
        );
        HashMap::new()
    })
}

fn validate_output_path(output_path: &str) -> bool {
    let output_path_exists = Path::new(output_path).try_exists();

    match output_path_exists {
        Ok(exists) => {
            if !exists {
                println!(
                    "Output path: {0} does not exist. Creating now.",
                    output_path
                );

                if let Err(_) = fs::create_dir(output_path) {
                    return false;
                }
            }

            true
        }
        Err(_) => false,
    }
}

pub fn get_latest_stable_specifications(output_path: &str) -> HashMap<String, SpecificationFile> {
    // Download azure GitHub repo
    // Go through each file and get the latest stable *.json file
    // Put all these files in one flat directory.

    let is_output_path_valid = validate_output_path(output_path);

    if !is_output_path_valid {
        return HashMap::new();
    }

    println!("Getting latest Stable Azure Specifications");
    let url = "https://github.com/Azure/azure-rest-api-specs.git";
    let specification_path = format!("{0}\\specification", output_path);

    let already_downloaded = Path::new(specification_path.as_str()).exists();

    if already_downloaded {
        println!("Azure Repository already downloaded.");
        return get_specification_files(&specification_path);
    }

    match Repository::clone(url, output_path) {
        Ok(_) => get_specification_files(&specification_path),
        Err(error) => {
            eprintln!(
                "Failed to get Azure Specification Repository due to error: {0}",
                error
            );
            HashMap::new()
        }
    }
}
