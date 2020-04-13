use chrono::{DateTime, FixedOffset};
use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct ParseError {
    pub msg: String,
}

impl ParseError {
    fn new(msg: String) -> ParseError {
        ParseError { msg }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct BlogpostMetadata {
    pub title: String,
    pub filename: PathBuf,
    pub date: DateTime<FixedOffset>,
    pub tags: Vec<String>,
}

fn parse_metadata_line(line: &str) -> Result<(&str, &str), ParseError> {
    let mut split = line.splitn(2, ':');
    let name = split
        .next()
        .ok_or_else(|| ParseError::new(format!("Header line '{}' has no key", line)))?
        .trim();
    let value = split
        .next()
        .ok_or_else(|| ParseError::new(format!("Header line '{}' has no value", line)))?
        .trim();
    Ok((name, value))
}

fn parse_metadata(head: &str) -> Result<HashMap<&str, &str>, ParseError> {
    head.lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_metadata_line)
        .collect()
}

/// get a secure filename from a string (i.e. just a filename, no path, especially no "up" path)
fn get_secure_filename(path: &str) -> Result<PathBuf, ParseError> {
    let unchecked = Path::new(path);

    let filename = unchecked
        .file_name()
        .ok_or_else(|| ParseError::new("Empty filename".to_owned()))?;

    Ok(Path::new(filename).to_path_buf())
}

fn parse_tags(tagstring: &str) -> Vec<String> {
    tagstring
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect()
}

fn parse_blogpost_metadata(props: HashMap<&str, &str>) -> Result<BlogpostMetadata, ParseError> {
    let title = props
        .get("title")
        .copied()
        .ok_or_else(|| ParseError::new("Missing title".to_owned()))?
        .to_owned();
    let filename = get_secure_filename(
        props
            .get("filename")
            .ok_or_else(|| ParseError::new("Missing filename".to_owned()))?,
    )?;
    let date_string = props
        .get("date")
        .ok_or_else(|| ParseError::new("Missing date".to_owned()))?;
    let date = DateTime::parse_from_rfc3339(date_string)
        .map_err(|e| ParseError::new(format!("Invalid date '{}': {}", date_string, e)))?;
    let tags = props
        .get("tags")
        .map(|s| parse_tags(s))
        .unwrap_or_else(|| vec![]);

    Ok(BlogpostMetadata {
        title,
        filename,
        date,
        tags,
    })
}

fn split_file_content(content: &str) -> Result<(HashMap<&str, &str>, &str), ParseError> {
    let mut sections = content.splitn(3, "---");

    if sections.next().map(|s| s.trim().is_empty()) != Some(true) {
        return Err(ParseError::new("Content before header".to_owned()));
    }

    let header_raw = sections
        .next()
        .ok_or_else(|| ParseError::new("No header found".to_owned()))?;
    let header_map = parse_metadata(header_raw)?;

    let content = sections
        .next()
        .ok_or_else(|| ParseError::new("No content after header".to_owned()))?;

    Ok((header_map, content))
}

pub fn parse_blogpost(content: &str) -> Result<(BlogpostMetadata, &str), ParseError> {
    let (header_map, content) = split_file_content(content)?;
    let metadata = parse_blogpost_metadata(header_map)?;
    Ok((metadata, content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn parse_metadata_line_should_return_split_line() {
        // given
        // implicitely tested here: trimming and colons in the value
        let line = " title : Slaves to Armok: God of blood: Chapter II: Dwarf Fortress ";

        // when
        let (key, value) = parse_metadata_line(line).expect("Expected successful parsing");

        //then
        assert_eq!(key, "title");
        assert_eq!(
            value,
            "Slaves to Armok: God of blood: Chapter II: Dwarf Fortress"
        );
    }

    #[test]
    fn parse_metadata_line_should_fail_for_empty_line() {
        // when
        let result = parse_metadata_line("");

        // then
        assert_eq!(
            result,
            Err(ParseError::new("Header line '' has no value".to_owned()))
        );
    }

    #[test]
    fn parse_metadata_line_should_fail_for_missing_colon() {
        // given
        let line = "title";

        // when
        let result = parse_metadata_line(line);

        // then
        assert_eq!(
            result,
            Err(ParseError::new(
                "Header line 'title' has no value".to_owned()
            ))
        );
    }

    #[test]
    fn parse_metadata_should_parse_metadata_and_ignore_empty_lines() {
        let metadata = "\n \ntitle: Colon Cancer\n\t\ntags:foo,bar\n\n";

        // when
        let result = parse_metadata(metadata).expect("Expected valid result");

        // then
        assert_eq!(result.len(), 2);
        assert_eq!(result.get("title"), Some(&"Colon Cancer"));
        assert_eq!(result.get("tags"), Some(&"foo,bar"));
    }

    #[test]
    fn parse_metadata_should_fail_on_bad_line() {
        // given
        let metadata = "title: Colon Cancer\nfoobar\ntags: foo,bar";

        // when
        let result = parse_metadata(metadata);

        // then
        assert_eq!(
            result,
            Err(ParseError::new(
                "Header line 'foobar' has no value".to_owned()
            ))
        );
    }

    #[test]
    fn get_secure_filename_should_return_filename() {
        // given
        let input = "foo.bar";

        // when
        let result = get_secure_filename(input).expect("Expected valid result");

        // then
        assert_eq!(result, Path::new("foo.bar"));
    }

    #[test]
    fn get_secure_filename_should_return_only_filename() {
        // given
        let input = "/etc/../var/www/foo.bar";

        // when
        let result = get_secure_filename(input).expect("Expected valid result");

        // then
        assert_eq!(result, Path::new("foo.bar"));
    }

    #[test]
    fn get_secure_filename_should_fail_for_empty_path() {
        // given
        let input = "";

        // when
        let result = get_secure_filename(input);

        // then
        assert_eq!(result, Err(ParseError::new("Empty filename".to_owned())));
    }

    #[test]
    fn parse_tags_should_split_and_trim_tags() {
        // given
        let tagstring = " foo, bar, ,foobar ";

        // when
        let result = parse_tags(tagstring);

        // then
        assert_eq!(
            &result,
            &["foo".to_owned(), "bar".to_owned(), "foobar".to_owned()]
        );
    }

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
    }

    #[test]
    fn parse_blogpost_metadata_should_parse_full_metadata() {
        // given
        let mut input = HashMap::with_capacity(3);
        input.insert("title", "Lorem ipsum");
        input.insert("filename", "lipsum");
        input.insert("date", "2020-05-11T12:13:14+02:00");
        input.insert("tags", "foo,bar");

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
        assert_eq!(result, Err(ParseError::new("Missing title".to_owned())));
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
        assert_eq!(result, Err(ParseError::new("Missing filename".to_owned())));
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
            Err(ParseError::new(
                "Invalid date \'2020-05-11+02:00\': input contains invalid characters".to_owned()
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
        assert_eq!(result, Err(ParseError::new("Missing date".to_owned())));
    }

    #[test]
    fn split_file_content_should_return_valid_results() {
        // given
        let input = " \n  \n---\ntitle: foo\n---\n\ncontent\n";

        // when
        let (header, content) = split_file_content(input).expect("Expected valid result");

        // then
        assert_eq!(header.len(), 1);
        assert_eq!(header.get("title"), Some(&"foo"));
        assert_eq!(content, "\n\ncontent\n");
    }

    #[test]
    fn split_content_should_reject_empty_file() {
        // given
        let input = "\n";

        // when
        let result = split_file_content(input);

        // then
        assert_eq!(result, Err(ParseError::new("No header found".to_owned())));
    }

    #[test]
    fn split_content_should_reject_content_before_header() {
        // given
        let input = "something\n---\nitle: foo\n---\ncontent\n";

        // when
        let result = split_file_content(input);

        // then
        assert_eq!(
            result,
            Err(ParseError::new("Content before header".to_owned()))
        );
    }

    #[test]
    fn split_content_should_reject_missing_content() {
        // given
        // header never closes => no content (a closing header would be valid: empty content)
        let input = "---\nitle: foo\n";

        // when
        let result = split_file_content(input);

        // then
        assert_eq!(
            result,
            Err(ParseError::new("No content after header".to_owned()))
        );
    }
}