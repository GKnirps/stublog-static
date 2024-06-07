/*
 * This file is part of stublog-static.
 *
 *  stublog-static is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  stublog-static is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with stublog-static. If not, see <https://www.gnu.org/licenses/>.
 */

use camino::Utf8PathBuf;
use chrono::{DateTime, FixedOffset};
use std::fmt;
use std::fmt::Formatter;
use std::time::SystemTime;
use tag::Tag;

pub mod file;
pub mod hosted_files;
pub mod parser;
pub mod tag;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Language {
    // only a limited set of languages for now
    De,
    En,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::De => "de",
                Language::En => "en",
            }
        )
    }
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Blogpost {
    pub title: String,
    pub filename: Utf8PathBuf,
    pub date: DateTime<FixedOffset>,
    /// this is the official update date
    pub update_date: Option<DateTime<FixedOffset>>,
    pub tags: Vec<Tag>,
    pub category_id: String,
    pub allow_html: bool,
    /// this is the last time the source file was touched
    pub modified_at: SystemTime,
    pub content_markdown: String,
    pub summary: Option<String>,
    pub image: Option<String>,
    pub language: Option<Language>,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Category {
    pub title: String,
    pub id: String,
    pub filename: Utf8PathBuf,
    pub description_markdown: String,
    pub modified_at: SystemTime,
    pub old_id: Option<String>,
}

/// Metadata for a blog content file (e.g. an image) (not the generated html files)
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct HostedFileMetadata {
    pub old_id: Option<String>,
    pub path: String,
    pub mime_type: String,
    pub description: String,
    pub modified_at: SystemTime,
}

// Actual data of a hosted file (not read from the metadata)
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct HostedFile {
    pub filename: Utf8PathBuf,
    pub file_size: u64,
    pub image_metadata: Option<ImageMetadata>,
    pub modified_at: SystemTime,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct ImageMetadata {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Quote {
    pub filename: Utf8PathBuf,
    pub source_name: Option<String>,
    pub source_url: Option<String>,
    pub published: bool,
    pub content_markdown: String,
    pub modified_at: SystemTime,
    pub language: Option<Language>,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Asset {
    pub filename: Utf8PathBuf,
    pub modified_at: SystemTime,
    pub web_path: String,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Assets {
    pub favicon: Asset,
    pub stylesheet: Asset,
}

impl Assets {
    pub fn modification_dates(&self) -> [SystemTime; 2] {
        [self.favicon.modified_at, self.stylesheet.modified_at]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_assets;
    use std::ops::Add;
    use std::time::Duration;

    #[test]
    fn assets_modification_dates_returns_dates() {
        // given
        let favicon_time = SystemTime::now().add(Duration::new(42, 42));
        let stylesheet_time = SystemTime::now().add(Duration::new(11, 11));

        let mut assets = create_assets();
        assets.favicon.modified_at = favicon_time;
        assets.stylesheet.modified_at = stylesheet_time;

        // when
        let result = assets.modification_dates();

        // then
        assert_eq!(result, [favicon_time, stylesheet_time])
    }
}
