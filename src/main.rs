mod cli;
mod filesystem;
mod generator;
mod parser;

use crate::generator::Generator;
use filesystem::get_latest_stable_specifications;
use generator::rust::generator::RustGenerator;
use std::time::Instant;

fn main() {
    let settings = cli::get_settings();

    match settings {
        Ok(settings) => {
            let now = Instant::now();
            let specifications =
                get_latest_stable_specifications(settings.output_specification_folder.as_str());

            match specifications {
                Ok(specifications) => {
                    let mut rust_generator = RustGenerator::default();
                    rust_generator.generate(settings.output_folder.as_str(), &specifications);

                    let elapsed = now.elapsed();
                    println!("Elapsed: {:.?}", elapsed);
                }
                Err(error) => {
                    eprintln!("{error}")
                }
            }
        }
        Err(error) => eprintln!("{error}"),
    }
}
