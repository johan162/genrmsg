pub use clap::Parser;

#[derive(Parser)]
#[clap(
    name = "genrmsg",
    about = "Generate Rust message code from YAML specification",
    author = "Johan Persson <johan162@gmail.com>",
    version = "1.1.0"
)]
pub struct Args {
    /// Sets the input YAML file
    #[clap(value_name = "INPUT_SPECIFICATION")]
    pub input: String,

    /// Sets the output Rust file (overrides the one in YAML if specified)
    #[clap(short = 'o', long = "output", value_name = "FILE")]
    pub output: Option<String>,

    /// Sets the message prefix (defaults to "M")
    #[clap(
        short = 'p',
        long = "prefix",
        value_name = "PREFIX",
        default_value = "M"
    )]
    pub prefix: String,

    /// Starting number for message numbering (defaults to 1)
    #[clap(
        short = 's',
        long = "start",
        value_name = "NUMBER",
        default_value = "1"
    )]
    pub start_number: u32,

    /// Number of digits to use for the message number (defaults to 3)
    #[clap(
        short = 'd',
        long = "digits",
        value_name = "DIGITS",
        default_value = "3"
    )]
    pub num_digits: u8,

    /// Enable prefixing of message names (disabled by default)
    #[clap(short = 'm', long = "prefix-messages", action = clap::ArgAction::SetTrue, default_value = "false")]
    pub prefix_messages: bool,

    /// Generate a Markdown table documenting all messages. Same name as the input file with .md extension.
    #[clap(short = 't', long = "mdtable", action = clap::ArgAction::SetTrue, default_value = "false")]
    pub mdtable: bool,

    /// Sets the verbosity level, 0=Error, 1=Warn, 2=Info, 3=Debug, 4=Trace (default is 0)
    #[clap(
        short = 'v',
        long = "verbose",
        value_name = "LEVEL",
        default_value = "0"
    )]
    pub verbose: u8,

    /// Validate that the YAML file follows the schema but don't generate code
    #[clap(short = 'y', long = "validate-yaml", action = clap::ArgAction::SetTrue, default_value = "false")]
    pub validate_only: bool,

    /// Lock the last generated prefix numbers in YAML so all messages get the same number next time
    #[clap(short = 'l', long = "lock-yaml", action = clap::ArgAction::SetTrue, default_value = "false")]
    pub lock_yaml: bool,

    /// Reset all message numbering (for major version changes)
    #[clap(short = 'R', long = "reset-numbering", action = clap::ArgAction::SetTrue, default_value = "false")]
    pub reset_numbering: bool,

}
