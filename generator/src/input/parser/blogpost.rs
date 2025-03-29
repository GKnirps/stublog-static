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

use super::super::{Blogpost, OgImage};
use super::{ParseError, get_secure_filename, parse_language, split_file_content};
use crate::input::file::FileData;
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

pub fn parse_blogposts(inputs: &[FileData]) -> Result<Vec<Blogpost>, ParseError> {
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
                original_path.as_str()
            ))
        })?
        .to_owned();
    let filename = get_secure_filename(
        props.get("filename").ok_or_else(|| {
            ParseError::new(format!(
                "Missing output filename for blogpost {}",
                original_path.as_str()
            ))
        })?,
        original_path,
    )?;
    let date_string = props.get("date").ok_or_else(|| {
        ParseError::new(format!(
            "Missing date for blogpost {}",
            original_path.as_str()
        ))
    })?;
    let date = DateTime::parse_from_rfc3339(date_string).map_err(|e| {
        ParseError::new(format!(
            "Invalid date '{}': {} (blogpost {})",
            date_string,
            e,
            original_path.as_str()
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
                original_path.as_str()
            ))
        })?),
    };
    let summary = props.get("summary").map(|s| s.to_string());
    let image_path = props.get("image").map(|s| s.to_string());
    let image_alt = props.get("image-alt").map(|s| s.to_string());

    let image = if let Some(path) = image_path {
        let alt = image_alt.ok_or_else(|| {
            ParseError::new(format!(
                "Missing image alt text for blogpost {}",
                original_path.as_str()
            ))
        })?;
        Some(OgImage { path, alt })
    } else {
        None
    };

    let tags = props
        .get("tags")
        .map(|s| parse_tags(s))
        .unwrap_or_else(Vec::new);
    let category_id = props
        .get("category")
        .map(|s| s.to_string())
        .ok_or_else(|| {
            ParseError::new(format!(
                "Missing category for blogpost: {}",
                original_path.as_str()
            ))
        })?;
    let allow_html = props
        .get("allow-html")
        .map(|v| *v == "true")
        .unwrap_or(false);
    let language = props
        .get("language")
        .map(|l| parse_language(l))
        .transpose()?;

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
        language,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::Language;
    use crate::test_utils::create_file_data;
    use camino::Utf8Path;
    use chrono::{FixedOffset, TimeZone};
    use std::time::SystemTime;

    #[test]
    fn parse_blogpost_metadata_should_parse_minimal_metadata() {
        // given
        let content = "---\n\
        title: Lorem ipsum\n\
        filename: lipsum\n\
        date: 2020-05-11T12:13:14+02:00\n\
        category: cat\n\
        ---\n\
        IM IN UR CONTENT\n"
            .to_owned();

        let modified_at = SystemTime::now();
        let filename = Utf8Path::new("mad/eye").to_path_buf();

        let input = FileData {
            content,
            filename,
            modified_at,
        };

        // when
        let result = parse_blogpost(&input).expect("Expected valid result");

        // then
        let expected_date = FixedOffset::east_opt(3600 * 2)
            .unwrap()
            .with_ymd_and_hms(2020, 5, 11, 12, 13, 14)
            .unwrap();
        assert_eq!(result.title, "Lorem ipsum");
        assert_eq!(result.filename, Utf8Path::new("lipsum"));
        assert_eq!(result.date, expected_date);
        assert!(result.update_date.is_none(), "Expected no date");
        // missing tags are allowed, tag vector is empty in that case
        assert!(result.tags.is_empty(), "Expected no tags");
        assert_eq!(result.category_id, "cat");
        assert!(!result.allow_html);
        assert_eq!(result.modified_at, modified_at);
        assert_eq!(result.content_markdown, "IM IN UR CONTENT\n");
        assert_eq!(result.summary, None);
        assert_eq!(result.image, None);
        assert_eq!(result.language, None);
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
        image-alt: A foo in its natural habitat.\n\
        language: de\n\
        ---\n\
        IM IN UR CONTENT\n"
            .to_owned();

        let modified_at = SystemTime::now();
        let filename = Utf8Path::new("mad/eye").to_path_buf();

        let input = FileData {
            content,
            filename,
            modified_at,
        };

        // when
        let result = parse_blogpost(&input).expect("Expected valid result");

        // then
        assert_eq!(result.title, "Lorem ipsum");
        assert_eq!(result.filename, Utf8Path::new("lipsum"));
        assert_eq!(
            result.date,
            FixedOffset::east_opt(3600 * 2)
                .unwrap()
                .with_ymd_and_hms(2020, 5, 11, 12, 13, 14)
                .unwrap()
        );
        assert_eq!(
            result.update_date,
            Some(
                FixedOffset::east_opt(3600 * 2)
                    .unwrap()
                    .with_ymd_and_hms(2020, 5, 25, 11, 12, 13)
                    .unwrap()
            )
        );
        assert_eq!(
            &result.tags,
            &[Tag::new("foo"), Tag::new("bar")],
            "Unexpected tags"
        );
        assert_eq!(result.category_id, "bananas");
        assert!(result.allow_html);
        assert_eq!(result.modified_at, modified_at);
        assert_eq!(result.content_markdown, "IM IN UR CONTENT\n");
        assert_eq!(result.summary, Some("Bogus!".to_owned()));
        assert_eq!(
            result.image,
            Some(OgImage {
                path: "/file/foo.png".to_owned(),
                alt: "A foo in its natural habitat.".to_owned()
            })
        );
        assert_eq!(result.language, Some(Language::De));
    }

    #[test]
    fn parse_blogpost_metadata_should_fail_for_missing_title() {
        // given
        let content = "---\n\
        filename: lipsum\n\
        date: 2020-05-11T12:13:14+02:00\n\
        category: bananas\n\
        ---\n\
        IM IN UR CONTENT\n"
            .to_owned();

        let path = Utf8Path::new("mad/eye").to_path_buf();

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
    fn parse_blogpost_metadata_should_fail_for_missing_category() {
        // given
        let content = "---\n\
        filename: lipsum\n\
        title: Lorem Ipsum\n\
        date: 2020-05-11T12:13:14+02:00\n\
        ---\n\
        IM IN UR CONTENT\n"
            .to_owned();

        let path = Utf8Path::new("mad/eye").to_path_buf();

        let mut input = create_file_data();
        input.filename = path;
        input.content = content;

        // when
        let result = parse_blogpost(&input);

        // then
        assert_eq!(
            result,
            Err(ParseError::from("Missing category for blogpost: mad/eye"))
        );
    }

    #[test]
    fn parse_blogpost_metadata_should_fail_for_missing_filename() {
        // given
        let content = "---\n\
        title: Lorem ipsum\n\
        date: 2020-05-11T12:13:14+02:00\n\
        category: bananas\n\
        ---\n\
        IM IN UR CONTENT\n"
            .to_owned();

        let path = Utf8Path::new("mad/eye").to_path_buf();

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
        category: bananas\n\
        ---\n\
        IM IN UR CONTENT\n"
            .to_owned();

        let path = Utf8Path::new("mad/eye").to_path_buf();

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
        category: bananas\n\
        ---\n\
        IM IN UR CONTENT\n"
            .to_owned();

        let path = Utf8Path::new("mad/eye").to_path_buf();

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
    fn parse_blogpost_metadata_should_fail_for_bad_update_date() {
        // given
        let content = "---\n\
        title: Lorem ipsum\n\
        filename: lipsum\n\
        date: 2020-05-11T12:13:14+02:00\n\
        update-date: 2020-05-11+02:00\n\
        category: cat\n\
        ---\n\
        IM IN UR CONTENT\n"
            .to_owned();

        let modified_at = SystemTime::now();
        let filename = Utf8Path::new("mad/eye").to_path_buf();

        let input = FileData {
            content,
            filename,
            modified_at,
        };

        // when
        let result = parse_blogpost(&input);

        // then
        assert_eq!(
            result,
            Err(ParseError::from(
                "Invalid update date date '2020-05-11+02:00': input contains invalid characters (blogpost mad/eye)"
            ))
        );
    }

    #[test]
    fn parse_blogpost_metadata_should_fail_if_image_is_present_but_image_alt_is_not() {
        // given
        let content = "---\n\
        title: Lorem ipsum\n\
        filename: lipsum\n\
        date: 2020-05-11T12:13:14+02:00\n\
        category: cat\n\
        image: /file/foo.png\n\
        image-alt:\n\
        ---\n\
        IM IN UR CONTENT\n"
            .to_owned();

        let modified_at = SystemTime::now();
        let filename = Utf8Path::new("mad/eye").to_path_buf();

        let input = FileData {
            content,
            filename,
            modified_at,
        };

        // when
        let result = parse_blogpost(&input);

        // then
        assert_eq!(
            result,
            Err(ParseError::from(
                "Missing image alt text for blogpost mad/eye"
            ))
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
