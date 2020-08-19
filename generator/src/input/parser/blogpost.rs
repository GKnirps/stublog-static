use super::super::BlogpostMetadata;
use super::{get_secure_filename, split_file_content, ParseError};
use chrono::DateTime;
use std::collections::HashMap;

fn parse_tags(tagstring: &str) -> Vec<String> {
    tagstring
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase())
        .collect()
}

fn parse_blogpost_metadata(props: HashMap<&str, &str>) -> Result<BlogpostMetadata, ParseError> {
    let title = props
        .get("title")
        .copied()
        .ok_or_else(|| ParseError::from("Missing title"))?
        .to_owned();
    let filename = get_secure_filename(
        props
            .get("filename")
            .ok_or_else(|| ParseError::from("Missing filename"))?,
    )?;
    let date_string = props
        .get("date")
        .ok_or_else(|| ParseError::from("Missing date"))?;
    let date = DateTime::parse_from_rfc3339(date_string)
        .map_err(|e| ParseError::new(format!("Invalid date '{}': {}", date_string, e)))?;
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
        tags,
        category_id,
        allow_html,
    })
}

pub fn parse_blogpost(content: &str) -> Result<(BlogpostMetadata, &str), ParseError> {
    let (header_map, content) = split_file_content(content)?;
    let metadata = parse_blogpost_metadata(header_map)?;
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

        // when
        let result = parse_blogpost_metadata(input).expect("Expected valid result");

        // then
        assert_eq!(result.title, "Lorem ipsum");
        assert_eq!(result.filename, Path::new("lipsum"));
        assert_eq!(
            result.date,
            FixedOffset::east(3600 * 2)
                .ymd(2020, 5, 11)
                .and_hms(12, 13, 14)
        );
        // missing tags are allowed, tag vector is empty in that case
        assert!(result.tags.is_empty(), "Expected no tags");
        assert_eq!(result.category_id, None);
        assert!(!result.allow_html);
    }

    #[test]
    fn parse_blogpost_metadata_should_parse_full_metadata() {
        // given
        let mut input = HashMap::with_capacity(3);
        input.insert("title", "Lorem ipsum");
        input.insert("filename", "lipsum");
        input.insert("date", "2020-05-11T12:13:14+02:00");
        input.insert("tags", "foo,bar");
        input.insert("category", "bananas");
        input.insert("allow-html", "true");

        // when
        let result = parse_blogpost_metadata(input).expect("Expected valid result");

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
            &result.tags,
            &["foo".to_owned(), "bar".to_owned()],
            "Unexpected tags"
        );
        assert_eq!(result.category_id, Some("bananas".to_owned()));
        assert!(result.allow_html);
    }

    #[test]
    fn parse_blogpost_metadata_should_fail_for_missing_title() {
        // given
        let mut input = HashMap::with_capacity(3);
        input.insert("filename", "lipsum");
        input.insert("date", "2020-05-11T12:13:14+02:00");

        // when
        let result = parse_blogpost_metadata(input);

        // then
        assert_eq!(result, Err(ParseError::from("Missing title")));
    }

    #[test]
    fn parse_blogpost_metadata_should_fail_for_missing_filename() {
        // given
        let mut input = HashMap::with_capacity(3);
        input.insert("title", "Lorem ipsum");
        input.insert("date", "2020-05-11T12:13:14+02:00");

        // when
        let result = parse_blogpost_metadata(input);

        // then
        assert_eq!(result, Err(ParseError::from("Missing filename")));
    }

    #[test]
    fn parse_blogpost_metadata_should_fail_for_bad_date() {
        // given
        let mut input = HashMap::with_capacity(3);
        input.insert("title", "Lorem ipsum");
        input.insert("filename", "lipsum");
        input.insert("date", "2020-05-11+02:00");

        // when
        let result = parse_blogpost_metadata(input);

        // then
        assert_eq!(
            result,
            Err(ParseError::from(
                "Invalid date \'2020-05-11+02:00\': input contains invalid characters"
            ))
        );
    }

    #[test]
    fn parse_blogpost_metadata_should_fail_for_missing_date() {
        // given
        let mut input = HashMap::with_capacity(3);
        input.insert("title", "Lorem ipsum");
        input.insert("filename", "lipsum");

        // when
        let result = parse_blogpost_metadata(input);

        // then
        assert_eq!(result, Err(ParseError::from("Missing date")));
    }

    #[test]
    fn parse_tags_should_split_and_trim_and_lowercase_tags() {
        // given
        let tagstring = " foO, Bar, ,fÖbär ";

        // when
        let result = parse_tags(tagstring);

        // then
        assert_eq!(
            &result,
            &["foo".to_owned(), "bar".to_owned(), "föbär".to_owned()]
        );
    }
}
