use clap::{Arg, Command};

pub struct RazureSettings {
    pub output_folder: String,
    pub output_specification_folder: String,
}

fn get_cli() -> Command {
    Command::new("Razure")
        .version("0.1")
        .author("James Durban")
        .about("Generates Rust bindings from Azure OpenAPI (V2) specification.")
        .arg_required_else_help(true)
        .arg(
            Arg::new("output")
                .long("output") // `--output` for long flag
                .value_name("FILE") // Label for the value in help messages
                .help("Sets the output file path")
                .required(true), // This argument is required
        )
        .arg(
            Arg::new("output-specification")
                .long("output-specification") // `--output` for long flag
                .value_name("FILE") // Label for the value in help messages
                .help("Sets the output file path for downloading the azure specifications")
                .required(true), // This argument is required
        )
}
pub fn get_settings() -> Result<RazureSettings, std::io::Error> {
    let cli = get_cli();
    let matches = cli.get_matches();

    let mut output_folder = String::new();
    let mut output_specification_folder = String::new();

    if let Some(output_path) = matches.get_one::<String>("output") {
        output_folder = output_path.to_string();
    }

    if let Some(output_specification_path) = matches.get_one::<String>("output-specification") {
        output_specification_folder = output_specification_path.to_string();
    }

    Ok(RazureSettings {
        output_folder: output_folder,
        output_specification_folder: output_specification_folder,
    })
}
