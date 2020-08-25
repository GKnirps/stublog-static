use super::{get_secure_filename, split_file_content, ParseError};
use crate::input::file::FileData;
use crate::input::Category;

pub fn parse_categories(inputs: &[FileData]) -> Result<Vec<Category>, ParseError> {
    inputs.iter().map(|input| parse_category(&input)).collect()
}

pub fn parse_category(file_data: &FileData) -> Result<Category, ParseError> {
    let path = &file_data.filename;
    let (header_map, description) = split_file_content(file_data)?;

    let title = header_map
        .get("title")
        .copied()
        .ok_or_else(|| {
            ParseError::new(format!(
                "Missing title in category {}",
                path.to_string_lossy()
            ))
        })?
        .to_owned();

    let id = header_map
        .get("path-name")
        .ok_or_else(|| {
            ParseError::new(format!(
                "Missing target file name for category {}",
                file_data.filename.to_string_lossy()
            ))
        })?
        .to_string();
    let filename = get_secure_filename(&id, path)?;

    Ok(Category {
        title,
        id,
        filename,
        description_markdown: description.to_string(),
        modified_at: file_data.modified_at,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_file_data;
    use std::path::Path;
    use std::time::SystemTime;

    #[test]
    fn parse_category_should_parse_valid_input() {
        // given
        let mut input = create_file_data();
        input.content = "---\npath-name: hot/chocolate\ntitle: Cocoa deliciousness\n---\nChocolate is important".to_owned();
        input.filename = Path::new("df_linux/urist").to_path_buf();
        input.modified_at = SystemTime::now();

        // when
        let category = parse_category(&input).expect("Expected valid result");

        // then
        assert_eq!(category.description_markdown, "Chocolate is important");
        assert_eq!(category.filename, Path::new("chocolate"));
        assert_eq!(category.title, "Cocoa deliciousness");
        assert_eq!(category.id, "hot/chocolate");
        assert_eq!(category.modified_at, input.modified_at);
    }

    #[test]
    fn parse_category_should_fail_for_missing_title() {
        // given
        let mut input = create_file_data();
        input.content = "---\npath-name: foo\n---\n".to_owned();
        input.filename = Path::new("df_linux/urist").to_path_buf();

        // when
        let result = parse_category(&input);

        // then
        assert_eq!(
            result,
            Err(ParseError::from("Missing title in category df_linux/urist"))
        );
    }

    #[test]
    fn parse_category_should_fail_for_missing_filename() {
        // given
        let mut input = create_file_data();
        input.content = "---\ntitle: foo\n---\n".to_owned();
        input.filename = Path::new("df_linux/urist").to_path_buf();

        // when
        let result = parse_category(&input);

        // then
        assert_eq!(
            result,
            Err(ParseError::from(
                "Missing target file name for category df_linux/urist"
            ))
        );
    }
}
