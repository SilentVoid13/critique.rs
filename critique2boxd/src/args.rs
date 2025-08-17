use clap::Parser;
use critique_api::{MediaUniverse, valid_media_universes};
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct CliArgs {
    /// SensCritique username
    pub username: String,
    /// Media type
    #[arg(short, long, default_value = "movie", value_parser = parse_media_universe)]
    pub media_type: MediaUniverse,
    /// Add reviews to the output
    #[arg(short, long, default_value_t = true)]
    pub with_reviews: bool,
    /// Output CSV file
    #[arg(short, long, default_value = "output.csv")]
    pub output: PathBuf,
}

fn parse_media_universe(s: &str) -> Result<MediaUniverse, String> {
    MediaUniverse::try_from(s)
        .map_err(|_| format!("Valid values: {}", valid_media_universes().join(", ")))
}
