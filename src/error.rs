use displaydoc::Display;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Display, Debug, Error)]
pub enum Error {
    /// Failed to serialize AndroidManifest.xml. Error: {0}
    FailedToSerialize(String),
    /// Failed to deserialize AndroidManifest.xml. Error: {0}
    FailedToDeserialize(String),
}
