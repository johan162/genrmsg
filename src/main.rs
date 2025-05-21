pub mod args;
pub mod code_builder;
pub mod generate;
pub mod markdown;
pub mod spec;
pub mod validate;

use crate::args::Args;
use crate::generate::{assign_message_numbers, generate_rust_code};
use crate::markdown::generate_message_markdown_table;
use crate::spec::MessageDefinition;
use clap::Parser;
use colored::*;
use log::info;
use std::fs;
use std::path::Path;
use validate::{validate_message_numbering, validate_schema};

/// Initialize the logger based on verbosity level
fn init_logging(verbose: u8) {
    let log_level = match verbose {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    env_logger::Builder::new().filter_level(log_level).init();
}

/// Validate the schema of a YAML file
fn validate_schema_file(yaml_content: &str) -> Result<(), Box<dyn std::error::Error>> {
    match validate_schema(yaml_content) {
        Ok(_) => {
            println!("{}", "Schema validation successful!".green());
            Ok(())
        }
        Err(e) => {
            println!("{}", format!("Schema validation error: {e}").red());
            Err(e.into())
        }
    }
}

/// Read and parse the message definition file
fn process_input_file(
    args: &Args,
    input_file: &str,
) -> Result<MessageDefinition, Box<dyn std::error::Error>> {
    info!("Reading message specification from: \"{}\"", input_file);

    // Read the YAML file
    let yaml_content = fs::read_to_string(input_file)?;

    // Check validation if requested
    if args.validate_only {
        validate_schema_file(&yaml_content)?;
        std::process::exit(0);
    }

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

    Ok(spec)
}

/// Generate Rust code and write it to the output file
fn generate_code(
    args: &Args,
    spec: &MessageDefinition,
) -> Result<String, Box<dyn std::error::Error>> {
    // Determine output file path
    let output_file = args
        .output
        .clone()
        .unwrap_or_else(|| spec.settings.output_file.clone());

    let rust_code = generate_rust_code(spec);

    // Create parent directories if they don't exist
    if let Some(parent) = Path::new(&output_file).parent() {
        fs::create_dir_all(parent)?;
    }

    // Write the output file
    fs::write(&output_file, &rust_code)?;

    info!("Successfully generated message code to: \"{output_file}\"");

    Ok(output_file)
}

/// Generate Markdown table documentation if requested
fn generate_markdown_table(
    args: &Args,
    spec: &MessageDefinition,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !args.mdtable {
        info!("Skipping Markdown table generation");
        return Ok(());
    }

    info!("Generating Markdown table of messages");
    let markdown_table = generate_message_markdown_table(spec);

    // Convert String to PathBuf before using with_extension
    let markdown_file = std::path::PathBuf::from(output_file).with_extension("md");
    fs::write(&markdown_file, markdown_table)?;

    info!(
        "Successfully generated message specification table to: \"{}\"",
        markdown_file.display()
    );

    Ok(())
}

pub fn update_yaml_with_message_numbers(
    input_file: &str,
    spec: &MessageDefinition,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a backup of the original file
    let yaml_content = fs::read_to_string(input_file)?;
    let backup_file = format!("{}.bak", input_file);
    fs::write(&backup_file, &yaml_content)?;

    // Write updated spec with explicit message numbers
    let updated_yaml = serde_yaml::to_string(spec)?;
    fs::write(input_file, updated_yaml)?;

    info!(
        "Updated message specification with explicit numbering: {}",
        input_file
    );
    info!("Original file backed up to: {}", backup_file);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize logging
    init_logging(args.verbose);

    // Process input file
    let spec = process_input_file(&args, &args.input)?;

    if let Err(e) = validate_message_numbering(&spec) {
        eprintln!("Error: {}", e);
        return Err(e.into());
    }

    // Generate code
    let output_file = generate_code(&args, &spec)?;

    // Generate Markdown table if requested
    generate_markdown_table(&args, &spec, &output_file)?;

    // After generating the code
    if args.update_yaml {
        update_yaml_with_message_numbers(&args.input, &spec)?;
        info!("Message numbering has been locked in the YAML file");
    }

    Ok(())
}
