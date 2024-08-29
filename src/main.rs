mod filesystem;
mod parser;

mod generator;

use crate::filesystem::get_latest_stable_specifications;
use crate::parser::parse_specification_file;
use std::time::Instant;
use crate::generator::parameters::generate_parameters;

fn main() {
    let now = Instant::now();
    let output_path = "C:\\Users\\User\\Downloads\\Output";
    let specifications = get_latest_stable_specifications(output_path);

    let mut failed_parses = 0;
    let specification_len = specifications.len();

    for (key, specification_file) in specifications.iter().take(100) {
        let result = parse_specification_file(specification_file, false);
        
        match result {
            Some(swagger) => {
                generate_parameters(specification_file, &swagger);
            }
            None => failed_parses += 1
        }
    }

    println!(
        "Total number of failed parses: {0}/{1}",
        failed_parses, specification_len
    );

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
