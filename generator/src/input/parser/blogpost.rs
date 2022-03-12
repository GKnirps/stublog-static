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

use super::super::Blogpost;
use super::{get_secure_filename, split_file_content, ParseError};
use crate::input::file::FileData;
use crate::input::parser;
use crate::input::tag::Tag;
use chrono::DateTime;

fn parse_tags(tagstring: &str) -> Vec<Tag> {
    tagstring
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(Tag::new)
        .filter(|t| !t.normalized_name.is_empty())
        .collect()
}

pub fn parse_blogposts(inputs: &[FileData]) -> Result<Vec<Blogpost>, parser::ParseError> {
    inputs.iter().map(parse_blogpost).collect()
}

fn parse_blogpost(file_data: &FileData) -> Result<Blogpost, ParseError> {
    let (props, content) = split_file_content(file_data)?;
    let original_path = &file_data.filename;

    let title = props
        .get("title")
        .copied()
        .ok_or_else(|| {
            ParseError::new(format!(
                "Missing title for blogpost: {}",
                original_path.to_string_lossy()
            ))
        })?
        .to_owned();
    let filename = get_secure_filename(
        props.get("filename").ok_or_else(|| {
            ParseError::new(format!(
                "Missing output filename for blogpost {}",
                original_path.to_string_lossy()
            ))
        })?,
        original_path,
    )?;
    let date_string = props.get("date").ok_or_else(|| {
        ParseError::new(format!(
            "Missing date for blogpost {}",
            original_path.to_string_lossy()
        ))
    })?;
    let date = DateTime::parse_from_rfc3339(date_string).map_err(|e| {
        ParseError::new(format!(
            "Invalid date '{}': {} (blogpost {})",
            date_string,
            e,
            original_path.to_string_lossy()
        ))
    })?;
    let update_date_string = props.get("update-date");
    let update_date = match update_date_string {
        None => None,
        Some(date_str) => Some(DateTime::parse_from_rfc3339(date_str).map_err(|e| {
            ParseError::new(format!(
                "Invalid update date date '{}': {} (blogpost {})",
                date_str,
                e,
                original_path.to_string_lossy()
            ))
        })?),
    };
    let summary = props.get("summary").map(|s| s.to_string());
    let image = props.get("image").map(|s| s.to_string());

    let tags = props
        .get("tags")
        .map(|s| parse_tags(s))
        .unwrap_or_else(Vec::new);
    let category_id = props.get("category").map(|s| s.to_string());
    let allow_html = props
        .get("allow-html")
        .map(|v| *v == "true")
        .unwrap_or(false);

    Ok(Blogpost {
        title,
        filename,
        date,
        update_date,
        tags,
        category_id,
        allow_html,
        modified_at: file_data.modified_at,
        content_markdown: content.to_owned(),
        summary,
        image,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_file_data;
    use chrono::{FixedOffset, TimeZone};
    use std::path::Path;
    use std::time::SystemTime;

    #[test]
    fn parse_blogpost_metadata_should_parse_minimal_metadata() {
        // given
        let content = "---\n\
        title: Lorem ipsum\n\
        filename: lipsum\n\
        date: 2020-05-11T12:13:14+02:00\n\
        ---\n\
        IM IN UR CONTENT\n"
            .to_owned();

        let modified_at = SystemTime::now();
        let filename = Path::new("mad/eye").to_path_buf();

        let input = FileData {
            content,
            filename,
            modified_at,
        };

        // when
        let result = parse_blogpost(&input).expect("Expected valid result");

        // then
        let expected_date = FixedOffset::east(3600 * 2)
            .ymd(2020, 5, 11)
            .and_hms(12, 13, 14);
        assert_eq!(result.title, "Lorem ipsum");
        assert_eq!(result.filename, Path::new("lipsum"));
        assert_eq!(result.date, expected_date);
        assert!(result.update_date.is_none(), "Expected no date");
        // missing tags are allowed, tag vector is empty in that case
        assert!(result.tags.is_empty(), "Expected no tags");
        assert_eq!(result.category_id, None);
        assert!(!result.allow_html);
        assert_eq!(result.modified_at, modified_at);
        assert_eq!(result.content_markdown, "IM IN UR CONTENT\n");
        assert_eq!(result.summary, None);
        assert_eq!(result.image, None);
    }

    #[test]
    fn parse_blogpost_metadata_should_parse_full_metadata() {
        // given
        let content = "---\n\
        title: Lorem ipsum\n\
        filename: lipsum\n\
        date: 2020-05-11T12:13:14+02:00\n\
        update-date: 2020-05-25T11:12:13+02:00\n\
        tags: foo,bar\n\
        category: bananas\n\
        allow-html: true\n\
        summary: Bogus!\n\
        image: /file/foo.png\n\
        ---\n\
        IM IN UR CONTENT\n"
            .to_owned();

        let modified_at = SystemTime::now();
        let filename = Path::new("mad/eye").to_path_buf();

        let input = FileData {
            content,
            filename,
            modified_at,
        };

        // when
        let result = parse_blogpost(&input).expect("Expected valid result");

        // then
        assert_eq!(result.title, "Lorem ipsum");
        assert_eq!(result.filename, Path::new("lipsum"));
        assert_eq!(
            result.date,
            FixedOffset::east(3600 * 2)
                .ymd(2020, 5, 11)
                .and_hms(12, 13, 14)
        );
        assert_eq!(
            result.update_date,
            Some(
                FixedOffset::east(3600 * 2)
                    .ymd(2020, 5, 25)
                    .and_hms(11, 12, 13)
            )
        );
        assert_eq!(
            &result.tags,
            &[Tag::new("foo"), Tag::new("bar")],
            "Unexpected tags"
        );
        assert_eq!(result.category_id, Some("bananas".to_owned()));
        assert!(result.allow_html);
        assert_eq!(result.modified_at, modified_at);
        assert_eq!(result.content_markdown, "IM IN UR CONTENT\n");
        assert_eq!(result.summary, Some("Bogus!".to_owned()));
        assert_eq!(result.image, Some("/file/foo.png".to_owned()));
    }

    #[test]
    fn parse_blogpost_metadata_should_fail_for_missing_title() {
        // given
        let content = "---\n\
        filename: lipsum\n\
        date: 2020-05-11T12:13:14+02:00\n\
        ---\n\
        IM IN UR CONTENT\n"
            .to_owned();

        let path = Path::new("mad/eye").to_path_buf();

        let mut input = create_file_data();
        input.filename = path;
        input.content = content;

        // when
        let result = parse_blogpost(&input);

        // then
        assert_eq!(
            result,
            Err(ParseError::from("Missing title for blogpost: mad/eye"))
        );
    }

    #[test]
    fn parse_blogpost_metadata_should_fail_for_missing_filename() {
        // given
        let content = "---\n\
        title: Lorem ipsum\n\
        date: 2020-05-11T12:13:14+02:00\n\
        ---\n\
        IM IN UR CONTENT\n"
            .to_owned();

        let path = Path::new("mad/eye").to_path_buf();

        let mut input = create_file_data();
        input.filename = path;
        input.content = content;

        // when
        let result = parse_blogpost(&input);

        // then
        assert_eq!(
            result,
            Err(ParseError::from(
                "Missing output filename for blogpost mad/eye"
            ))
        );
    }

    #[test]
    fn parse_blogpost_metadata_should_fail_for_bad_date() {
        // given
        let content = "---\n\
        title: Lorem ipsum\n\
        filename: lipsum\n\
        date: 2020-05-11+02:00\n\
        ---\n\
        IM IN UR CONTENT\n"
            .to_owned();

        let path = Path::new("mad/eye").to_path_buf();

        let mut input = create_file_data();
        input.filename = path;
        input.content = content;

        // when
        let result = parse_blogpost(&input);

        // then
        assert_eq!(
            result,
            Err(ParseError::from(
                "Invalid date \'2020-05-11+02:00\': input contains invalid characters (blogpost mad/eye)"
            ))
        );
    }

    #[test]
    fn parse_blogpost_metadata_should_fail_for_missing_date() {
        // given
        let content = "---\n\
        title: Lorem ipsum\n\
        filename: lipsum\n\
        ---\n\
        IM IN UR CONTENT\n"
            .to_owned();

        let path = Path::new("mad/eye").to_path_buf();

        let mut input = create_file_data();
        input.filename = path;
        input.content = content;

        // when
        let result = parse_blogpost(&input);

        // then
        assert_eq!(
            result,
            Err(ParseError::from("Missing date for blogpost mad/eye"))
        );
    }

    #[test]
    fn parse_tags_should_split_and_trim_and_lowercase_tags() {
        // given
        let tagstring = " foO, Bar, ,fÖbär , '\"";

        // when
        let result = parse_tags(tagstring);

        // then
        assert_eq!(
            &result,
            &[Tag::new("foo"), Tag::new("bar"), Tag::new("föbär")]
        );
    }
}
