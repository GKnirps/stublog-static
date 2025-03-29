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

use super::{ParseError, split_file_content};
use crate::input::HostedFileMetadata;
use crate::input::file::FileData;

pub fn parse_all_file_metadata(inputs: &[FileData]) -> Result<Vec<HostedFileMetadata>, ParseError> {
    inputs.iter().map(parse_file_metadata).collect()
}

fn parse_file_metadata(file_data: &FileData) -> Result<HostedFileMetadata, ParseError> {
    let source_path = &file_data.filename;
    let (header_map, description) = split_file_content(file_data)?;

    let old_id = header_map.get("old-id").map(|s| s.to_string());

    let path = header_map
        .get("path")
        .ok_or_else(|| {
            ParseError::new(format!(
                "Missing path in hosted file {}",
                source_path.as_str()
            ))
        })?
        .to_string();

    let mime_type = header_map
        .get("mime-type")
        .ok_or_else(|| {
            ParseError::new(format!(
                "Missing mime type in hosted file {}",
                source_path.as_str()
            ))
        })?
        .to_string();

    Ok(HostedFileMetadata {
        old_id,
        path,
        mime_type,
        description: description.to_string(),
        modified_at: file_data.modified_at,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_file_data;
    use camino::Utf8Path;

    #[test]
    fn parse_file_metadata_parses_valid_content() {
        // given
        let mut input = create_file_data();
        input.content = "---\nold-id: 42\npath: foobar.png\nmime-type: image/png\n---\
        \n\ndes-des-des-description!"
            .to_string();

        // when
        let result = parse_file_metadata(&input).expect("Expected successful parsing");

        // then
        assert_eq!(result.old_id, Some("42".to_string()));
        assert_eq!(result.path, "foobar.png");
        assert_eq!(result.mime_type, "image/png");
        assert_eq!(result.description, "des-des-des-description!");

        assert_eq!(result.modified_at, input.modified_at);
    }

    #[test]
    fn parse_file_metadata_parses_if_non_essential_fields_are_absent() {
        // given
        let mut input = create_file_data();
        input.content =
            "---\npath: foobar.png\nmime-type: image/png\n---\n\ndes-des-des-description!"
                .to_string();

        // when
        let result = parse_file_metadata(&input).expect("Expected successful parsing");

        // then
        assert_eq!(result.old_id, None);
    }

    #[test]
    fn parse_file_metadata_fails_for_missing_path() {
        // given
        let mut input = create_file_data();
        input.content =
            "---\nold-id: 42\nmime-type: image/png\n---\n\ndes-des-des-description!".to_string();
        input.filename = Utf8Path::new("df_linux/urist").to_path_buf();

        // when
        let result = parse_file_metadata(&input);

        // then
        assert_eq!(
            result,
            Err(ParseError::from(
                "Missing path in hosted file df_linux/urist"
            ))
        );
    }

    #[test]
    fn parse_file_metadata_fails_for_missing_type() {
        // given
        let mut input = create_file_data();
        input.content =
            "---\nold-id: 42\npath: foobar.png\n---\n\ndes-des-des-description!".to_string();
        input.filename = Utf8Path::new("df_linux/urist").to_path_buf();

        // when
        let result = parse_file_metadata(&input);

        // then
        assert_eq!(
            result,
            Err(ParseError::from(
                "Missing mime type in hosted file df_linux/urist"
            ))
        );
    }
}
