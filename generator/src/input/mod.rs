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

use chrono::{DateTime, FixedOffset};
use std::path::PathBuf;
use std::time::SystemTime;
use tag::Tag;

pub mod file;
pub mod parser;
pub mod tag;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Blogpost {
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
    pub content_markdown: String,
    pub summary: Option<String>,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Category {
    pub title: String,
    pub id: String,
    pub filename: PathBuf,
    pub description_markdown: String,
    pub modified_at: SystemTime,
    pub old_id: Option<String>,
}

/// Metadata for a blog content file (e.g. an image) (not the generated html files)
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct HostedFile {
    pub old_id: Option<String>,
    pub path: String,
    pub mime_type: String,
    pub description: String,
    pub modified_at: SystemTime,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Quote {
    pub filename: PathBuf,
    pub source_name: Option<String>,
    pub source_url: Option<String>,
    pub published: bool,
    pub content_markdown: String,
    pub modified_at: SystemTime,
}
