use clap::Parser;
use std::path::PathBuf;
use critique_api::MediaUniverse;

#[derive(Debug, Parser)]
pub struct CliArgs {
    /// SensCritique username
    pub username: String,
    /// Media type
    #[arg(short, long, default_value = "film", value_parser = parse_media_universe)]
    pub media_type: MediaUniverse,
    /// Add reviews to the output
    #[arg(short, long, default_value_t = true)]
    pub with_reviews: bool,
    /// Output CSV file
    #[arg(short, long, default_value = "output.csv")]
    pub output: PathBuf,
}

fn parse_media_universe(s: &str) -> Result<MediaUniverse, String> {
    MediaUniverse::try_from(s).map_err(|_| "Valid values: movie, book, game, tvShow, comicBook, musicAlbum, musicTrack".to_string())
}
