use super::{get_secure_filename, split_file_content, ParseError};
use crate::input::CategoryMetadata;
use std::collections::HashMap;

fn parse_category_metadata(props: HashMap<&str, &str>) -> Result<CategoryMetadata, ParseError> {
    let title = props
        .get("title")
        .copied()
        .ok_or_else(|| ParseError::from("Missing title"))?
        .to_owned();
    let filename = get_secure_filename(
        props
            .get("path-name")
            .ok_or_else(|| ParseError::from("Missing file name"))?,
    )?;

    Ok(CategoryMetadata { title, filename })
}

pub fn parse_category(content: &str) -> Result<(CategoryMetadata, &str), ParseError> {
    let (header_map, description) = split_file_content(content)?;
    let metadata = parse_category_metadata(header_map)?;
    Ok((metadata, description))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    #[test]
    fn parse_category_metadata_should_parse_valid_metadata() {
        // given
        let mut input: HashMap<&str, &str> = HashMap::with_capacity(3);
        input.insert("title", "Summelsarium");
        // only the filename part of a path should be used (here: only "sums")
        input.insert("path-name", "foo/sums");
        input.insert("something-else", "blablabla");

        // when
        let result = parse_category_metadata(input).expect("Expected successful parsing");

        // then
        assert_eq!(
            result,
            CategoryMetadata {
                title: "Summelsarium".to_owned(),
                filename: PathBuf::from("sums")
            }
        );
    }

    #[test]
    fn parse_category_metadata_should_fail_for_missing_title() {
        // given
        let mut input: HashMap<&str, &str> = HashMap::with_capacity(3);
        input.insert("path-name", "sums");

        // when
        let result = parse_category_metadata(input);

        // then
        assert_eq!(result, Err(ParseError::from("Missing title")));
    }

    #[test]
    fn parse_category_metadata_should_fail_for_missing_filename() {
        // given
        let mut input: HashMap<&str, &str> = HashMap::with_capacity(3);
        input.insert("title", "Summelsarium");

        // when
        let result = parse_category_metadata(input);

        // then
        assert_eq!(result, Err(ParseError::from("Missing file name")));
    }

    #[test]
    fn parse_category_should_parse_valid_input() {
        // given
        let input =
            "---\npath-name: chocolate\ntitle: Cocoa deliciousness\n---\nChocolate is important";

        // when
        let (meta, content) = parse_category(input).expect("Expected valid result");

        // then
        assert_eq!(content, "Chocolate is important");
        assert_eq!(&meta.filename, Path::new("chocolate"));
        assert_eq!(meta.title, "Cocoa deliciousness");
    }
}
