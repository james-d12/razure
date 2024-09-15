use clap::{Arg, ArgMatches, Command};

pub struct RazureSettings {
    pub output_folder: String,
}

fn get_cli() -> Command {
    Command::new("Razure")
        .version("0.1")
        .author("James Durban")
        .about("Generates Rust bindings from Azure OpenAPI (V2) specification.")
        .arg_required_else_help(true)
        .arg(
            Arg::new("output")
                .short('o') // Alias `-o` for short flag
                .long("output") // `--output` for long flag
                .value_name("FILE") // Label for the value in help messages
                .help("Sets the output file path")
                .required(true), // This argument is required
        )
}
pub fn get_settings() -> RazureSettings {
    let cli = get_cli();
    let matches = cli.get_matches();

    let mut output_folder = String::new();

    if let Some(output_path) = matches.get_one::<String>("output") {
        output_folder = output_path.to_string();
    }

    RazureSettings {
        output_folder: output_folder,
    }
}
