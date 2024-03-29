use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct CliArgs {
    /// SensCritique username
    pub username: String,
    /// Add reviews to the output
    #[arg(short, long, default_value_t = true)]
    pub with_reviews: bool,
    /// Output CSV file
    #[arg(short, long, default_value = "output.csv")]
    pub output: PathBuf,
}
