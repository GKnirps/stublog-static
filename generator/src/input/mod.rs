use chrono::{DateTime, FixedOffset};
use std::path::PathBuf;
use std::time::SystemTime;
use tag::Tag;

pub mod file;
pub mod parser;
pub mod tag;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct BlogpostMetadata {
    pub title: String,
    pub filename: PathBuf,
    pub date: DateTime<FixedOffset>,
    pub tags: Vec<Tag>,
    pub category_id: Option<String>,
    pub allow_html: bool,
    pub modified_at: SystemTime,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Category {
    pub title: String,
    pub id: String,
    pub filename: PathBuf,
    pub description_markdown: String,
    pub modified_at: SystemTime,
}
