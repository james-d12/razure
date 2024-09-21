use crate::generator::terminal::generate_cargo_project;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::Path;

pub fn create_file(
    file_path: &String,
    parameter_structs: &HashMap<String, String>,
) -> Result<(), Error> {
    let mut parameters_file = File::create(file_path)?;

    for parameter_struct in parameter_structs.values() {
        let mut str = parameter_struct.clone();
        str.push('\n');
        parameters_file.write_all(str.as_ref())?;
    }

    Ok(())
}

pub fn create_lib_file(
    output_path: &str,
    file_names: &BTreeMap<String, String>,
) -> Result<(), Error> {
    let lib_file_path = format!("{output_path}/src/lib.rs");
    let mut lib_file = File::create_new(lib_file_path)?;

    for (_, file_mod_statement) in file_names {
        lib_file.write_all(file_mod_statement.as_ref())?
    }

    Ok(())
}

pub fn create_project(output_path: &str) -> Result<bool, Error> {
    let path = Path::new(output_path);

    if !path.exists() {
        println!("Path: {output_path} does not exist, creating now.");
        fs::create_dir(output_path)?
    }

    Ok(generate_cargo_project(output_path))
}
