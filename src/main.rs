pub mod args;
pub mod generate;
pub mod markdown;
pub mod spec;

use crate::args::Args;
use crate::generate::{assign_message_numbers, generate_rust_code};
use crate::markdown::generate_message_markdown_table;
use crate::spec::MessageDefinition;
use clap::Parser;
use log::info;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize logging based on verbosity
    let log_level = match args.verbose {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    env_logger::Builder::new().filter_level(log_level).init();

    info!("Reading message specification from: \"{}\"", args.input);

    let input_file = &args.input;

    // Read the YAML file
    let yaml_content = fs::read_to_string(input_file)?;

    // Parse the YAML specification
    let mut spec: MessageDefinition = serde_yaml::from_str(&yaml_content)?;

    // Assign message numbers, respecting the prefix_messages flag
    assign_message_numbers(
        &mut spec,
        &args.prefix,
        args.start_number,
        args.num_digits,
        args.prefix_messages,
    );

    // Generate Rust code
    let output_file = args
        .output
        .unwrap_or_else(|| spec.settings.output_file.clone());
    let rust_code = generate_rust_code(&spec);

    // Create parent directories if they don't exist
    if let Some(parent) = Path::new(&output_file).parent() {
        fs::create_dir_all(parent)?;
    }

    // Write the output file
    fs::write(&output_file, rust_code)?;

    info!(
        "Successfully generated message code to: \"{output_file}\""
    );

    // Generate Markdown table of messages
    if args.mdtable {
        info!("Generating Markdown table of messages");
        let markdown_table = generate_message_markdown_table(&spec);
        // Convert String to PathBuf before using with_extension
        let markdown_file = std::path::PathBuf::from(&output_file).with_extension("md");
        fs::write(&markdown_file, markdown_table)?;
        info!(
            "Successfully generated message specification table to: \"{}\"",
            markdown_file.display()
        );
    } else {
        info!("Skipping Markdown table generation");
    }

    Ok(())
}
