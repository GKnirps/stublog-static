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

use super::Language;
use super::file::FileData;
use camino::{Utf8Path, Utf8PathBuf};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

pub mod asset;
pub mod blogpost;
pub mod category;
pub mod files_index;
pub mod quote;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct ParseError {
    pub msg: String,
}

impl ParseError {
    pub fn new(msg: String) -> ParseError {
        ParseError { msg }
    }

    // only used in tests right now
    #[cfg(test)]
    pub fn from(msg: &str) -> ParseError {
        ParseError {
            msg: msg.to_owned(),
        }
    }
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

fn parse_metadata_line<'a>(
    line: &'a str,
    original_filename: &Utf8Path,
) -> Result<(&'a str, &'a str), ParseError> {
    let mut split = line.splitn(2, ':');
    let name = split
        .next()
        .ok_or_else(|| {
            ParseError::new(format!(
                "Header line '{}' has no key ({})",
                line,
                original_filename.as_str()
            ))
        })?
        .trim();
    let value = split
        .next()
        .ok_or_else(|| {
            ParseError::new(format!(
                "Header line '{}' has no value ({})",
                line,
                original_filename.as_str()
            ))
        })?
        .trim();
    Ok((name, value))
}

fn parse_metadata<'a>(
    head: &'a str,
    original_filename: &Utf8Path,
) -> Result<HashMap<&'a str, &'a str>, ParseError> {
    head.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|l| parse_metadata_line(l, original_filename))
        .filter(|p| {
            p.as_ref()
                .map(|(_, value)| !value.is_empty())
                .unwrap_or(true)
        })
        .collect()
}

pub fn parse_language(input: &str) -> Result<Language, ParseError> {
    match input.trim() {
        "de" => Ok(Language::De),
        "en" => Ok(Language::En),
        _ => Err(ParseError::new(format!("unknown language: '{input}'"))),
    }
}

/// get a secure filename from a string (i.e. just a filename, no path, especially no "up" path)
fn get_secure_filename(path: &str, source_file_path: &Utf8Path) -> Result<Utf8PathBuf, ParseError> {
    let unchecked = Utf8Path::new(path);

    let filename = unchecked.file_name().ok_or_else(|| {
        ParseError::new(format!(
            "Empty filename for source file {}",
            source_file_path.as_str()
        ))
    })?;

    Ok(Utf8Path::new(filename).to_path_buf())
}

fn split_file_content(file_data: &FileData) -> Result<(HashMap<&str, &str>, &str), ParseError> {
    let mut sections = file_data.content.splitn(3, "---");
    let original_path = &file_data.filename;

    if sections.next().map(|s| s.trim().is_empty()) != Some(true) {
        return Err(ParseError::new(format!(
            "Content before header in file {}",
            original_path.as_str()
        )));
    }

    let header_raw = sections.next().ok_or_else(|| {
        ParseError::new(format!(
            "No header found in file {}",
            original_path.as_str()
        ))
    })?;
    let header_map = parse_metadata(header_raw, original_path)?;

    let content = sections.next().map(str::trim_start).ok_or_else(|| {
        ParseError::new(format!(
            "No content after header in file {}",
            original_path.as_str()
        ))
    })?;

    Ok((header_map, content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_file_data;

    #[test]
    fn parse_metadata_line_should_return_split_line() {
        // given
        // implicitly tested here: trimming and colons in the value
        let line = " title : Slaves to Armok: God of blood: Chapter II: Dwarf Fortress ";
        let path = Utf8Path::new("df_linux/urist");

        // when
        let (key, value) = parse_metadata_line(line, path).expect("Expected successful parsing");

        //then
        assert_eq!(key, "title");
        assert_eq!(
            value,
            "Slaves to Armok: God of blood: Chapter II: Dwarf Fortress"
        );
    }

    #[test]
    fn parse_metadata_line_should_fail_for_empty_line() {
        // given
        let path = Utf8Path::new("df_linux/urist");

        // when
        let result = parse_metadata_line("", path);

        // then
        assert_eq!(
            result,
            Err(ParseError::from(
                "Header line '' has no value (df_linux/urist)"
            ))
        );
    }

    #[test]
    fn parse_metadata_line_should_fail_for_missing_colon() {
        // given
        let line = "title";
        let path = Utf8Path::new("df_linux/urist");

        // when
        let result = parse_metadata_line(line, path);

        // then
        assert_eq!(
            result,
            Err(ParseError::from(
                "Header line 'title' has no value (df_linux/urist)"
            ))
        );
    }

    #[test]
    fn parse_metadata_should_parse_metadata_and_ignore_empty_lines() {
        // given
        let metadata = "\n \ntitle: Colon Cancer\n\t\ntags:foo,bar\nempty: \t \n\n";
        let path = Utf8Path::new("df_linux/urist");

        // when
        let result = parse_metadata(metadata, path).expect("Expected valid result");

        // then
        assert_eq!(result.len(), 2);
        assert_eq!(result.get("title"), Some(&"Colon Cancer"));
        assert_eq!(result.get("tags"), Some(&"foo,bar"));
    }

    #[test]
    fn parse_metadata_should_fail_on_bad_line() {
        // given
        let metadata = "title: Colon Cancer\nfoobar\ntags: foo,bar";
        let path = Utf8Path::new("df_linux/urist");

        // when
        let result = parse_metadata(metadata, path);

        // then
        assert_eq!(
            result,
            Err(ParseError::from(
                "Header line 'foobar' has no value (df_linux/urist)"
            ))
        );
    }

    #[test]
    fn get_secure_filename_should_return_filename() {
        // given
        let input = "foo.bar";
        let path = Utf8Path::new("df_linux/urist");

        // when
        let result = get_secure_filename(input, path).expect("Expected valid result");

        // then
        assert_eq!(result, Utf8Path::new("foo.bar"));
    }

    #[test]
    fn get_secure_filename_should_return_only_filename() {
        // given
        let input = "/etc/../var/www/foo.bar";
        let path = Utf8Path::new("df_linux/urist");

        // when
        let result = get_secure_filename(input, path).expect("Expected valid result");

        // then
        assert_eq!(result, Utf8Path::new("foo.bar"));
    }

    #[test]
    fn get_secure_filename_should_fail_for_empty_path() {
        // given
        let input = "";
        let path = Utf8Path::new("df_linux/urist");

        // when
        let result = get_secure_filename(input, path);

        // then
        assert_eq!(
            result,
            Err(ParseError::from(
                "Empty filename for source file df_linux/urist"
            ))
        );
    }

    #[test]
    fn split_file_content_should_return_valid_results() {
        // given
        let mut input = create_file_data();
        input.content = " \n  \n---\ntitle: foo\n---\n\ncontent\n".to_owned();
        input.filename = Utf8Path::new("df_linux/urist").to_path_buf();

        // when
        let (header, content) = split_file_content(&input).expect("Expected valid result");

        // then
        assert_eq!(header.len(), 1);
        assert_eq!(header.get("title"), Some(&"foo"));
        assert_eq!(content, "content\n");
    }

    #[test]
    fn split_content_should_reject_empty_file() {
        // given
        let mut input = create_file_data();
        input.content = "\n".to_owned();
        input.filename = Utf8Path::new("df_linux/urist").to_path_buf();

        // when
        let result = split_file_content(&input);

        // then
        assert_eq!(
            result,
            Err(ParseError::from("No header found in file df_linux/urist"))
        );
    }

    #[test]
    fn split_content_should_reject_content_before_header() {
        // given
        let mut input = create_file_data();
        input.content = "something\n---\nitle: foo\n---\ncontent\n".to_owned();
        input.filename = Utf8Path::new("df_linux/urist").to_path_buf();

        // when
        let result = split_file_content(&input);

        // then
        assert_eq!(
            result,
            Err(ParseError::from(
                "Content before header in file df_linux/urist"
            ))
        );
    }

    #[test]
    fn split_content_should_reject_missing_content() {
        // given
        let mut input = create_file_data();
        // header never closes => no content (a closing header would be valid: empty content)
        input.content = "---\nitle: foo\n".to_owned();
        input.filename = Utf8Path::new("df_linux/urist").to_path_buf();

        // when
        let result = split_file_content(&input);

        // then
        assert_eq!(
            result,
            Err(ParseError::from(
                "No content after header in file df_linux/urist"
            ))
        );
    }

    #[test]
    fn parse_language_works_for_all_inputs() {
        assert_eq!(parse_language("de"), Ok(Language::De));
        assert_eq!(parse_language("en"), Ok(Language::En));
        assert_eq!(
            parse_language("sindarin"),
            Err(ParseError::from("unknown language: 'sindarin'"))
        );
    }
}
