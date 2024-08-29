use filesystem::filesystem::get_latest_stable_specifications;
use generator::parameters::{create_parameters_file, generate_parameters};
use parser::parser::parse_specification_file;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output_path = "C:\\Users\\User\\Downloads\\Output";
    let specifications = get_latest_stable_specifications(output_path);

    let mut failed_parses = 0;
    let specification_len = specifications.len();

    let mut all_parameters: HashMap<String, String> = HashMap::new();

    for (key, specification_file) in specifications.iter() {
        let result = parse_specification_file(specification_file);

        match result {
            Some(swagger) => {
                let parameters = generate_parameters(specification_file, &swagger);
                if let Some(parameters) = parameters {
                    all_parameters.extend(parameters)
                }
            }
            None => failed_parses += 1,
        }
    }

    let result = create_parameters_file(&all_parameters);

    match result {
        Ok(_) => println!("Created Parameters.rs successfully!"),
        Err(error) => eprintln!("Could not create Parameters.rs file: {0}", error),
    }

    println!(
        "Total number of failed parses: {0}/{1}",
        failed_parses, specification_len
    );

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
