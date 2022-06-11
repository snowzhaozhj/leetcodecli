use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("...")]
pub enum LeetcodeError {
    Any(#[from] anyhow::Error),
    Io(#[from] io::Error),
    Serde(#[from] serde_json::Error),
    Regex(#[from] regex::Error),
    Reqwest(#[from] reqwest::Error),
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
}

pub type Result<T> = anyhow::Result<T, LeetcodeError>;

