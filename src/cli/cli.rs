use clap::{Arg, Command};
use log::LevelFilter;

pub struct RazureSettings {
    pub output_folder: String,
    pub output_specification_folder: String,
    pub log_level: LevelFilter,
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
        .arg(
            Arg::new("log-level")
                .long("log-level")
                .value_name("LEVEL")
                .help("Sets the log level")
                .required(false)
                .default_value("info"),
        )
}
pub fn get_settings() -> Result<RazureSettings, std::io::Error> {
    let cli = get_cli();
    let matches = cli.get_matches();

    let mut output_folder = String::new();
    let mut output_specification_folder = String::new();
    let mut log_level = LevelFilter::Info;

    if let Some(output_path) = matches.get_one::<String>("output") {
        output_folder = output_path.to_string();
    }

    if let Some(output_specification_path) = matches.get_one::<String>("output-specification") {
        output_specification_folder = output_specification_path.to_string();
    }

    if let Some(log_level_str) = matches.get_one::<String>("log-level") {
        match log_level_str.to_lowercase().as_str() {
            "trace" => log_level = LevelFilter::Trace,
            "info" => log_level = LevelFilter::Info,
            "warn" => log_level = LevelFilter::Warn,
            "error" => log_level = LevelFilter::Error,
            _ => log_level = LevelFilter::Info,
        }
    }

    Ok(RazureSettings {
        output_folder,
        output_specification_folder,
        log_level,
    })
}
