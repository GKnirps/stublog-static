use chrono::{DateTime, FixedOffset};
use std::path::PathBuf;

pub mod file;
pub mod parser;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct BlogpostMetadata {
    pub title: String,
    pub filename: PathBuf,
    pub date: DateTime<FixedOffset>,
    pub tags: Vec<String>,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct CategoryMetadata {
    pub title: String,
    pub filename: PathBuf,
}
