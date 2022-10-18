use clap::Parser;
use std::path::PathBuf;

/// Command=line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The input file containing the sqs configurations.
    #[clap(short, long, value_parser, display_order = 1)]
    pub config: PathBuf,
}