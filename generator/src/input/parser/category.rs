use super::{get_secure_filename, split_file_content, ParseError};
use crate::input::Category;

pub fn parse_categories(inputs: &[String]) -> Result<Vec<Category>, ParseError> {
    // TODO: it would be helpful if we knew which category failed to parse
    inputs.iter().map(|input| parse_category(&input)).collect()
}

pub fn parse_category(content: &str) -> Result<Category, ParseError> {
    let (header_map, description) = split_file_content(content)?;

    let title = header_map
        .get("title")
        .copied()
        .ok_or_else(|| ParseError::from("Missing title"))?
        .to_owned();

    let id = header_map
        .get("path-name")
        .ok_or_else(|| ParseError::from("Missing file name"))?
        .to_string();
    let filename = get_secure_filename(&id)?;

    Ok(Category {
        title,
        id,
        filename,
        description_markdown: description.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn parse_category_should_parse_valid_input() {
        // given
        let input =
            "---\npath-name: hot/chocolate\ntitle: Cocoa deliciousness\n---\nChocolate is important";

        // when
        let category = parse_category(input).expect("Expected valid result");

        // then
        assert_eq!(category.description_markdown, "Chocolate is important");
        assert_eq!(category.filename, Path::new("chocolate"));
        assert_eq!(category.title, "Cocoa deliciousness");
        assert_eq!(category.id, "hot/chocolate");
    }

    #[test]
    fn parse_category_should_fail_for_missing_title() {
        // given
        let input = "---\npath-name: foo\n---\n";

        // when
        let result = parse_category(input);

        // then
        assert_eq!(result, Err(ParseError::from("Missing title")));
    }

    #[test]
    fn parse_category_should_fail_for_missing_filename() {
        // given
        let input = "---\ntitle: foo\n---\n";

        // when
        let result = parse_category(input);

        // then
        assert_eq!(result, Err(ParseError::from("Missing file name")));
    }
}
