mod filesystem;
mod parser;

use crate::filesystem::get_latest_stable_specifications;
use crate::parser::parse_specification_file;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output_path = "C:\\Users\\User\\Downloads\\Output";
    let specifications = get_latest_stable_specifications(output_path);

    let mut failed_parses = 0;
    let specification_len = specifications.len();

    for (key, specification_file) in specifications.iter() {
        let result = parse_specification_file(specification_file, false);

        if !result {
            failed_parses += 1;
        }
    }

    println!(
        "Total number of failed parses: {0}/{1}",
        failed_parses, specification_len
    );

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
