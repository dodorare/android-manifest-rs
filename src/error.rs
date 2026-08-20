use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to serialize AndroidManifest.xml. Error: {0}")]
    FailedToSerialize(String),
    #[error("Failed to deserialize AndroidManifest.xml. Error: {0}")]
    FailedToDeserialize(String),
}
