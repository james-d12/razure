use chrono::NaiveDate;
use git2::Repository;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

type SpecificationFiles = HashMap<String, SpecificationFile>;

#[derive(Debug, Error)]
pub enum SpecificationFileError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),
}

pub struct SpecificationFile {
    pub file_name: String,
    pub file_path: String,
    pub domain_name: String,
    pub naive_date: NaiveDate,
}

fn get_domain_name_for_file(path_item: &Path) -> Option<String> {
    let great_grand_parent = path_item.parent()?.parent()?.parent()?;
    let domain_name = great_grand_parent.file_name()?.to_str()?.to_string();
    Some(domain_name)
}

fn get_json_files_for_directory(
    directory: &PathBuf,
) -> Result<SpecificationFiles, SpecificationFileError> {
    let mut specification_files_hashmap: HashMap<String, SpecificationFile> = HashMap::new();

    let files = fs::read_dir(directory)?;

    for file in files.flatten() {
        if file.path().is_dir() {
            let json_files_for_directory = get_json_files_for_directory(&file.path())?;
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
                    let file_name = full_path.file_name().unwrap().to_str().unwrap().to_string();
                    let domain_name = get_domain_name_for_file(&full_path).unwrap_or_default();

                    let key = format!(
                        "{domain_name}-{key_file_name}",
                        domain_name = domain_name.to_lowercase(),
                        key_file_name = file_name.to_lowercase()
                    );

                    let current_specification_file = specification_files_hashmap.get(&key);

                    match current_specification_file {
                        Some(current_specification_file) => {
                            if date_time > current_specification_file.naive_date {
                                specification_files_hashmap.insert(
                                    key,
                                    SpecificationFile {
                                        file_name: file_name,
                                        file_path: full_path_str.to_string(),
                                        domain_name: domain_name.to_string(),
                                        naive_date: date_time,
                                    },
                                );
                            }
                        }
                        None => {
                            specification_files_hashmap.insert(
                                key,
                                SpecificationFile {
                                    file_name: file_name,
                                    file_path: full_path_str.to_string(),
                                    domain_name: domain_name.to_string(),
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

fn validate_output_path(output_path: &str) -> Result<(), SpecificationFileError> {
    let output_path_exists = Path::new(output_path).try_exists()?;

    if !output_path_exists {
        println!("Creating path: {0}.", output_path);
        fs::create_dir(output_path)?;
    }

    Ok(())
}

pub fn get_latest_stable_specifications(
    output_path: &str,
) -> Result<SpecificationFiles, SpecificationFileError> {
    validate_output_path(output_path)?;

    println!("Getting latest Stable Azure Specifications");
    const AZURE_REST_API_GITHUB_URL: &str = "https://github.com/Azure/azure-rest-api-specs.git";
    let specification_path = PathBuf::from(output_path).join("specification");

    if specification_path.exists() {
        println!("Azure Repository already downloaded.");
        return get_json_files_for_directory(&specification_path);
    }

    Repository::clone(AZURE_REST_API_GITHUB_URL, output_path)?;
    get_json_files_for_directory(&specification_path)
}
