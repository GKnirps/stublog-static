use chrono::{DateTime, FixedOffset};
use std::path::PathBuf;
use std::time::SystemTime;

pub mod file;
pub mod parser;

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
pub struct Tag {
    pub name: String,
    pub normalized_name: String,
}

impl Tag {
    pub fn new(name: &str) -> Tag {
        let lowercase = name.to_lowercase();
        // We _could_ use a cow-str here, but this whole stuff is fast enough as it is.
        let normalized_name: String = lowercase
            .to_lowercase()
            .trim()
            .chars()
            .filter(|c| *c != '\'' && *c != '"')
            .map(|c| if c == ' ' { '-' } else { c })
            .collect();
        Tag {
            name: lowercase,
            normalized_name,
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Category {
    pub title: String,
    pub id: String,
    pub filename: PathBuf,
    pub description_markdown: String,
    pub modified_at: SystemTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_new_normalizes_tags_correctly() {
        // given
        let raw_name = "Foo's \"bar\"";

        // when
        let tag = Tag::new(raw_name);

        // then
        assert_eq!(tag.name, "foo's \"bar\"");
        assert_eq!(tag.normalized_name, "foos-bar");
    }
}
