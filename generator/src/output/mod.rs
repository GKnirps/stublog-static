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

use crate::input::{HostedFile, ImageMetadata};
use camino::Utf8Path;
use cmark::hosted_files_from_markdown;
use std::collections::HashMap;
use std::error::Error;
use std::fs::metadata;
use std::time::SystemTime;
use std::{fmt, io};

pub mod blogposts;
pub mod categories;
mod cmark;
pub mod error_pages;
pub mod feed;
mod file;
pub mod hosted_files;
mod html;
pub mod ngingx_cfg;
pub mod quotes;
pub mod tags;

/// Return true if the file referenced by filename has an older modification date than modified_at.
/// If any error occurs (e.g. the file does not exist), return true.
/// otherwise return false
fn needs_update(filename: &Utf8Path, modified_at: SystemTime) -> bool {
    metadata(filename)
        .and_then(|m| m.modified())
        .map(|t| t < modified_at)
        .unwrap_or(true)
}

/// Return true if the file reference by filename has an older modification date than any of
/// the given SystemTimes.
/// If any error occurs (e.g. the file does not exist), return true
fn needs_any_update<T>(filename: &Utf8Path, modification_times: T) -> bool
where
    T: IntoIterator<Item = SystemTime>,
{
    modification_times
        .into_iter()
        .max()
        .map(|t| needs_update(filename, t))
        .unwrap_or(true)
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct RenderError {
    pub msg: String,
}

impl RenderError {
    pub fn new(msg: String) -> RenderError {
        RenderError { msg }
    }

    pub fn from(msg: &str) -> RenderError {
        RenderError {
            msg: msg.to_owned(),
        }
    }
}

impl Error for RenderError {}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(Debug)]
pub enum OutputError {
    IO(io::Error),
    Render(RenderError),
    Xml(quick_xml::Error),
}

impl Error for OutputError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            OutputError::IO(e) => e,
            OutputError::Render(e) => e,
            OutputError::Xml(e) => e,
        })
    }
}

impl fmt::Display for OutputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OutputError::IO(e) => write!(f, "{e}"),
            OutputError::Render(e) => write!(f, "{e}"),
            OutputError::Xml(e) => write!(f, "{e}"),
        }
    }
}

impl From<io::Error> for OutputError {
    fn from(e: io::Error) -> Self {
        OutputError::IO(e)
    }
}

impl From<RenderError> for OutputError {
    fn from(e: RenderError) -> Self {
        OutputError::Render(e)
    }
}

impl<T> From<io::IntoInnerError<T>> for OutputError {
    fn from(e: io::IntoInnerError<T>) -> Self {
        Self::from(io::Error::from(e))
    }
}

impl From<quick_xml::Error> for OutputError {
    fn from(e: quick_xml::Error) -> Self {
        OutputError::Xml(e)
    }
}

pub fn image_metadata_by_path<'a>(
    path: &str,
    hosted_files: &HashMap<&str, &'a HostedFile>,
) -> Result<Option<&'a ImageMetadata>, RenderError> {
    // we only handle image links to /file/, everything else is an error
    let filename = path.strip_prefix("/file/").ok_or_else(|| {
        RenderError::new(format!(
            "hosted image '{path}' does not start with '/file/'"
        ))
    })?;

    let image_metadata = hosted_files
        .get(filename)
        .ok_or_else(|| RenderError::new(format!("did not find hosted image '{filename}'")))?
        .image_metadata
        .as_ref();

    // SVG do not necessarily have width and height, so we render them even if this data is
    // not available
    // TODO: using the file extension to detect an SVG file is a bit dirty. Find a better way
    if !filename.ends_with(".svg") {
        image_metadata.ok_or_else(|| {
            RenderError::new(format!(
                "hosted image '{path}' does not have image metadata",
            ))
        })?;
    }

    Ok(image_metadata)
}

// return the most recent modification date of all local hosted files (paths starting with
// `/file/`) that are referenced in `markdown`. Ignores HTML tags (`<img>` or `<a`> in the
// markdown). Returns errors if path is not in `hosted_files` Returns Ok(None) if no files are
// referenced.
pub fn hosted_files_modified_at_from_markdown(
    markdown: &str,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<Option<SystemTime>, RenderError> {
    hosted_files_from_markdown(markdown)
        .map(|path| {
            let filename = path.strip_prefix("/file/").ok_or_else(|| {
                RenderError::new(format!("unexpected hosted file path: '{path}'"))
            })?;
            Ok(hosted_files
                .get(filename)
                .ok_or_else(|| RenderError::new(format!("hosted file '{path}' has no metadata")))?
                .modified_at)
        })
        .try_fold(None, |max: Option<SystemTime>, t| {
            let t = t?;
            Ok(match max {
                None => Some(t),
                Some(m) => Some(m.max(t)),
            })
        })
}

pub fn hosted_files_modified_at_from_any_markdown<'a>(
    markdown_snippets: impl Iterator<Item = &'a str>,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<Option<SystemTime>, RenderError> {
    markdown_snippets
        .filter_map(|markdown| {
            hosted_files_modified_at_from_markdown(markdown, hosted_files).transpose()
        })
        .try_fold(None, |max: Option<SystemTime>, t| {
            let t = t?;
            match max {
                None => Ok(Some(t)),
                Some(m) => Ok(Some(m.max(t))),
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_hosted_file;
    use std::time::Duration;

    #[test]
    fn image_metadata_by_path_returns_dimensions_of_image_file() {
        // given
        let path = "/file/answer.txt";
        let mut hosted_file = create_hosted_file();
        hosted_file.image_metadata = Some(ImageMetadata {
            width: 42,
            height: 9001,
        });
        let hosted_files: HashMap<&str, &HostedFile> =
            HashMap::from([("answer.txt", &hosted_file)]);

        // when
        let result = image_metadata_by_path(path, &hosted_files);

        // then
        let metadata = result.expect("expected success");
        assert_eq!(
            metadata,
            Some(&ImageMetadata {
                width: 42,
                height: 9001
            })
        );
    }

    #[test]
    fn image_metadata_by_path_fails_for_wrong_prefix() {
        // given
        let path = "/unfile/answer.txt";
        let mut hosted_file = create_hosted_file();
        hosted_file.image_metadata = Some(ImageMetadata {
            width: 42,
            height: 9001,
        });
        let hosted_files: HashMap<&str, &HostedFile> =
            HashMap::from([("answer.txt", &hosted_file)]);

        // when
        let result = image_metadata_by_path(path, &hosted_files);

        // then
        assert_eq!(
            result,
            Err(RenderError::from(
                "hosted image '/unfile/answer.txt' does not start with '/file/'"
            ))
        );
    }

    #[test]
    fn image_metadata_by_path_fails_for_missing_image_file() {
        // given
        let path = "/file/answer.svg";
        let hosted_files: HashMap<&str, &HostedFile> = HashMap::new();

        // when
        let result = image_metadata_by_path(path, &hosted_files);

        // then
        assert_eq!(
            result,
            Err(RenderError::from("did not find hosted image 'answer.svg'"))
        );
    }

    #[test]
    fn image_metadata_by_path_fails_for_missing_dimensions_on_non_svg() {
        // given
        let path = "/file/answer.png";
        let mut hosted_file = create_hosted_file();
        hosted_file.image_metadata = None;
        let hosted_files: HashMap<&str, &HostedFile> =
            HashMap::from([("answer.png", &hosted_file)]);

        // when
        let result = image_metadata_by_path(path, &hosted_files);

        // then
        assert_eq!(
            result,
            Err(RenderError::from(
                "hosted image '/file/answer.png' does not have image metadata"
            ))
        );
    }

    #[test]
    fn image_metadata_by_path_handles_missing_dimensions_on_svg() {
        // given
        let path = "/file/answer.svg";
        let mut hosted_file = create_hosted_file();
        hosted_file.image_metadata = None;
        let hosted_files: HashMap<&str, &HostedFile> =
            HashMap::from([("answer.svg", &hosted_file)]);

        // when
        let result = image_metadata_by_path(path, &hosted_files);

        // then
        let metadata = result.expect("expected success");
        assert_eq!(metadata, None);
    }

    #[test]
    fn hosted_files_modified_at_from_markdown_yields_youngest_modification_date() {
        // given
        let markdown = "[some file](/file/some_file) ![image file](/file/image_file.svg)";

        let file_modified_at = SystemTime::UNIX_EPOCH + Duration::from_secs(42);
        let mut hosted_file = create_hosted_file();
        hosted_file.modified_at = file_modified_at;

        let img_modified_at = SystemTime::UNIX_EPOCH + Duration::from_secs(11);
        let mut hosted_image = create_hosted_file();
        hosted_image.modified_at = img_modified_at;

        let hosted_files: HashMap<&str, &HostedFile> = HashMap::from([
            ("some_file", &hosted_file),
            ("image_file.svg", &hosted_image),
        ]);

        // when
        let time = hosted_files_modified_at_from_markdown(markdown, &hosted_files);

        // then
        assert_eq!(time, Ok(Some(file_modified_at)));
    }

    #[test]
    fn hosted_files_modified_at_from_markdown_yields_nothing_if_no_files_are_referenced() {
        // given
        let markdown = "# Awesome headline!";
        let hosted_files: HashMap<&str, &HostedFile> = HashMap::new();

        // when
        let time = hosted_files_modified_at_from_markdown(markdown, &hosted_files);

        // then
        assert_eq!(time, Ok(None));
    }

    #[test]
    fn hosted_files_modified_at_from_markdown_yields_error_for_missing_file() {
        // given
        let markdown = "[some file](/file/some_file)";
        let hosted_files: HashMap<&str, &HostedFile> = HashMap::new();

        // when
        let time = hosted_files_modified_at_from_markdown(markdown, &hosted_files);

        // then
        assert_eq!(
            time,
            Err(RenderError::from(
                "hosted file '/file/some_file' has no metadata"
            ))
        );
    }

    #[test]
    fn hosted_files_modified_at_from_any_markdown_returns_youngest_modification_date() {
        // given
        let markdown1 = "[some file](/file/some_file)";
        let markdown2 = "![image file](/file/image_file.svg)";

        let file_modified_at = SystemTime::UNIX_EPOCH + Duration::from_secs(42);
        let mut hosted_file = create_hosted_file();
        hosted_file.modified_at = file_modified_at;

        let img_modified_at = SystemTime::UNIX_EPOCH + Duration::from_secs(11);
        let mut hosted_image = create_hosted_file();
        hosted_image.modified_at = img_modified_at;

        let hosted_files: HashMap<&str, &HostedFile> = HashMap::from([
            ("some_file", &hosted_file),
            ("image_file.svg", &hosted_image),
        ]);

        // when
        let time = hosted_files_modified_at_from_any_markdown(
            [markdown2, markdown1].into_iter(),
            &hosted_files,
        );

        // then
        assert_eq!(time, Ok(Some(file_modified_at)));
    }

    #[test]
    fn hosted_files_modified_at_from_any_markdown_returns_none_for_empty_input() {
        // given
        let hosted_files: HashMap<&str, &HostedFile> = HashMap::new();

        // when
        let time = hosted_files_modified_at_from_any_markdown(std::iter::empty(), &hosted_files);

        // then
        assert_eq!(time, Ok(None));
    }

    #[test]
    fn hosted_files_modified_at_from_any_markdown_returns_none_if_input_contains_no_hosted_files() {
        // given
        let markdown1 = "# Boring heading";
        let markdown2 = "*this*";

        let hosted_files: HashMap<&str, &HostedFile> = HashMap::new();

        // when
        let time = hosted_files_modified_at_from_any_markdown(
            [markdown2, markdown1].into_iter(),
            &hosted_files,
        );

        // then
        assert_eq!(time, Ok(None));
    }

    #[test]
    fn hosted_files_modified_at_from_any_markdown_returns_error_for_missing_file() {
        // given
        let markdown1 = "[no file](/file/no_file)";
        let markdown2 = "*this*";

        let hosted_files: HashMap<&str, &HostedFile> = HashMap::new();

        // when
        let time = hosted_files_modified_at_from_any_markdown(
            [markdown2, markdown1].into_iter(),
            &hosted_files,
        );

        // then
        assert_eq!(
            time,
            Err(RenderError::from(
                "hosted file '/file/no_file' has no metadata"
            ))
        );
    }
}
