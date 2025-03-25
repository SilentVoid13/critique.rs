use thiserror::Error;

pub type Result<T, E = CritiqueError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum CritiqueError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("No data in response")]
    NoDataInResponse,
    #[error("Invalid media universe: {0}")]
    InvalidMediaUniverse(String),
}
