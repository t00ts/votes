// error.rs

use std::io;

/// Errors returned by this library
#[derive(Debug)]
pub enum Error {
    /// Disk errors when loading and storing data
    IO(io::Error),
    /// Decoding errors when processing input files
    JSON(serde_json::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::JSON(value)
    }
}