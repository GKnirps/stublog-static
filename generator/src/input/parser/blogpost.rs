use super::super::BlogpostMetadata;
use super::{get_secure_filename, split_file_content, ParseError};
use crate::input::file::FileData;
use crate::input::tag::Tag;
use chrono::DateTime;
use std::collections::HashMap;
use std::path::Path;
use std::time::SystemTime;

fn parse_tags(tagstring: &str) -> Vec<Tag> {
    tagstring
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(Tag::new)
        .filter(|t| !t.normalized_name.is_empty())
        .collect()
}

fn parse_blogpost_metadata(
    props: HashMap<&str, &str>,
    modified_at: SystemTime,
    original_path: &Path,
) -> Result<BlogpostMetadata, ParseError> {
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

    let tags = props
        .get("tags")
        .map(|s| parse_tags(s))
        .unwrap_or_else(Vec::new);
    let category_id = props.get("category").map(|s| s.to_string());
    let allow_html = props
        .get("allow-html")
        .map(|v| *v == "true")
        .unwrap_or(false);

    Ok(BlogpostMetadata {
        title,
        filename,
        date,
        update_date,
        tags,
        category_id,
        allow_html,
        modified_at,
    })
}

pub fn parse_blogpost(file_data: &FileData) -> Result<(BlogpostMetadata, &str), ParseError> {
    let (header_map, content) = split_file_content(&file_data)?;
    let metadata = parse_blogpost_metadata(header_map, file_data.modified_at, &file_data.filename)?;
    Ok((metadata, content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{FixedOffset, TimeZone};
    use std::path::Path;

    #[test]
    fn parse_blogpost_metadata_should_parse_minimal_metadata() {
        // given
        let mut input = HashMap::with_capacity(3);
        input.insert("title", "Lorem ipsum");
        input.insert("filename", "lipsum");
        input.insert("date", "2020-05-11T12:13:14+02:00");

        let modified_at = SystemTime::now();
        let path = Path::new("mad/eye");

        // when
        let result =
            parse_blogpost_metadata(input, modified_at, path).expect("Expected valid result");

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
    }

    #[test]
    fn parse_blogpost_metadata_should_parse_full_metadata() {
        // given
        let mut input = HashMap::with_capacity(3);
        input.insert("title", "Lorem ipsum");
        input.insert("filename", "lipsum");
        input.insert("date", "2020-05-11T12:13:14+02:00");
        input.insert("update-date", "2020-05-25T11:12:13+02:00");
        input.insert("tags", "foo,bar");
        input.insert("category", "bananas");
        input.insert("allow-html", "true");

        let modified_at = SystemTime::now();
        let path = Path::new("mad/eye");

        // when
        let result =
            parse_blogpost_metadata(input, modified_at, path).expect("Expected valid result");

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
    }

    #[test]
    fn parse_blogpost_metadata_should_fail_for_missing_title() {
        // given
        let mut input = HashMap::with_capacity(3);
        input.insert("filename", "lipsum");
        input.insert("date", "2020-05-11T12:13:14+02:00");

        let path = Path::new("mad/eye");

        // when
        let result = parse_blogpost_metadata(input, SystemTime::now(), path);

        // then
        assert_eq!(
            result,
            Err(ParseError::from("Missing title for blogpost: mad/eye"))
        );
    }

    #[test]
    fn parse_blogpost_metadata_should_fail_for_missing_filename() {
        // given
        let mut input = HashMap::with_capacity(3);
        input.insert("title", "Lorem ipsum");
        input.insert("date", "2020-05-11T12:13:14+02:00");

        let path = Path::new("mad/eye");

        // when
        let result = parse_blogpost_metadata(input, SystemTime::now(), path);

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
        let mut input = HashMap::with_capacity(3);
        input.insert("title", "Lorem ipsum");
        input.insert("filename", "lipsum");
        input.insert("date", "2020-05-11+02:00");

        let path = Path::new("mad/eye");

        // when
        let result = parse_blogpost_metadata(input, SystemTime::now(), path);

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
        let mut input = HashMap::with_capacity(3);
        input.insert("title", "Lorem ipsum");
        input.insert("filename", "lipsum");

        let path = Path::new("mad/eye");

        // when
        let result = parse_blogpost_metadata(input, SystemTime::now(), path);

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
