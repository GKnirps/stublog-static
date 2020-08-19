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
    pub category_id: Option<String>,
    pub allow_html: bool,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Category {
    pub title: String,
    pub id: String,
    pub filename: PathBuf,
    pub description_markdown: String,
}
