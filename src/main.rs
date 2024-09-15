mod cli;
mod filesystem;
mod generator;
mod parser;

use filesystem::get_latest_stable_specifications;
use generator::generate;
use std::time::Instant;

fn main() {
    let settings = cli::get_settings();

    let now = Instant::now();
    let specifications = get_latest_stable_specifications(settings.output_folder.as_str());

    generate(&specifications);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.?}", elapsed);
}
