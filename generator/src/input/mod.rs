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
    /// this is the official update date
    pub update_date: Option<DateTime<FixedOffset>>,
    pub tags: Vec<Tag>,
    pub category_id: Option<String>,
    pub allow_html: bool,
    /// this is the last time the source file was touched
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

/// Metadata for a blog content file (e.g. an image) (not the generated html files
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct HostedFile {
    pub old_id: Option<String>,
    pub path: String,
    pub mime_type: String,
    pub description: String,
    pub modified_at: SystemTime,
}
