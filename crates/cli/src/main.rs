use filesystem::filesystem::get_latest_stable_specifications;
use generator::generate::generate;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output_path = "C:\\Users\\User\\Downloads\\Output";
    let specifications = get_latest_stable_specifications(output_path);

    generate(&specifications);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.?}", elapsed);
}
