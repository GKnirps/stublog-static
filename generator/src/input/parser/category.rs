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

use super::{get_secure_filename, split_file_content, ParseError};
use crate::input::file::FileData;
use crate::input::Category;

pub fn parse_categories(inputs: &[FileData]) -> Result<Vec<Category>, ParseError> {
    inputs.iter().map(parse_category).collect()
}

fn parse_category(file_data: &FileData) -> Result<Category, ParseError> {
    let path = &file_data.filename;
    let (header_map, description) = split_file_content(file_data)?;

    let title = header_map
        .get("title")
        .copied()
        .ok_or_else(|| ParseError::new(format!("Missing title in category {}", path.as_str())))?
        .to_owned();

    let id = header_map
        .get("path-name")
        .ok_or_else(|| {
            ParseError::new(format!(
                "Missing target file name for category {}",
                path.as_str()
            ))
        })?
        .to_string();
    let filename = get_secure_filename(&id, path)?;

    let old_id = header_map.get("old-id").map(|id| id.to_string());

    Ok(Category {
        title,
        id,
        filename,
        description_markdown: description.to_string(),
        modified_at: file_data.modified_at,
        old_id,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_file_data;
    use camino::Utf8Path;
    use std::time::SystemTime;

    #[test]
    fn parse_category_should_parse_valid_input() {
        // given
        let mut input = create_file_data();
        input.content = "---\npath-name: hot/chocolate\ntitle: Cocoa deliciousness\nold-id: 42\n---\nChocolate is important".to_owned();
        input.filename = Utf8Path::new("df_linux/urist").to_path_buf();
        input.modified_at = SystemTime::now();

        // when
        let category = parse_category(&input).expect("Expected valid result");

        // then
        assert_eq!(category.description_markdown, "Chocolate is important");
        assert_eq!(category.filename, Utf8Path::new("chocolate"));
        assert_eq!(category.title, "Cocoa deliciousness");
        assert_eq!(category.id, "hot/chocolate");
        assert_eq!(category.modified_at, input.modified_at);
        assert_eq!(category.old_id, Some("42".to_owned()));
    }

    #[test]
    fn parse_category_should_handle_missing_nonessential_fields() {
        // given
        let mut input = create_file_data();
        input.content = "---\npath-name: hot/chocolate\ntitle: Cocoa deliciousness\n---\nChocolate is important".to_owned();
        input.filename = Utf8Path::new("df_linux/urist").to_path_buf();
        input.modified_at = SystemTime::now();

        // when
        let category = parse_category(&input).expect("Expected valid result");

        // then
        assert_eq!(category.old_id, None);
    }

    #[test]
    fn parse_category_should_fail_for_missing_title() {
        // given
        let mut input = create_file_data();
        input.content = "---\npath-name: foo\n---\n".to_owned();
        input.filename = Utf8Path::new("df_linux/urist").to_path_buf();

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
        input.filename = Utf8Path::new("df_linux/urist").to_path_buf();

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
