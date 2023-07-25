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

use super::{get_secure_filename, parse_language, split_file_content, ParseError};
use crate::input::file::FileData;
use crate::input::Quote;

pub fn parse_quotes(inputs: &[FileData]) -> Result<Vec<Quote>, ParseError> {
    inputs
        .iter()
        .map(parse_quote)
        .filter(|r| r.as_ref().map(|q| q.published).unwrap_or(true))
        .collect()
}

fn parse_quote(file_data: &FileData) -> Result<Quote, ParseError> {
    let path = &file_data.filename;
    let (header_map, content_markdown) = split_file_content(file_data)?;

    let id = header_map
        .get("id")
        .ok_or_else(|| ParseError::new(format!("Missing id for quote {}", path.as_str())))?
        .to_string();
    let filename = get_secure_filename(&id, path)?;

    let source_name = header_map.get("source-name").map(|s| s.to_string());
    let source_url = header_map.get("source-url").map(|s| s.to_string());

    let published = header_map
        .get("published")
        .map(|s| *s != "false")
        .unwrap_or(true);

    let modified_at = file_data.modified_at;

    let language = header_map
        .get("language")
        .map(|l| parse_language(l))
        .transpose()?;

    Ok(Quote {
        filename,
        source_name,
        source_url,
        published,
        content_markdown: content_markdown.to_string(),
        modified_at,
        language,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_file_data;
    use camino::Utf8Path;

    #[test]
    fn parse_quote_parses_quote_when_all_fields_are_set() {
        // given
        let mut file_data = create_file_data();
        file_data.content = "---\n\
        id: ../penguin\n\
        source-name: Arthur Dent\n\
        source-url: https://example.com\n\
        published: false\n\
        ---\n\
        Ford… you're turning into a penguin. Stop it."
            .to_owned();

        // when
        let result = parse_quote(&file_data).expect("Expected successful parsing");

        // then
        assert_eq!(result.filename, Utf8Path::new("penguin"));
        assert_eq!(result.source_name, Some("Arthur Dent".to_owned()));
        assert_eq!(result.source_url, Some("https://example.com".to_owned()));
        assert!(!result.published);
        assert_eq!(
            result.content_markdown,
            "Ford… you're turning into a penguin. Stop it."
        );
        assert_eq!(result.modified_at, file_data.modified_at);
    }

    #[test]
    fn parse_quote_parses_quote_when_only_required_fields_are_set() {
        // given
        let mut file_data = create_file_data();
        file_data.content = "---\n\
        id: penguin\n\
        ---\n\
        Ford… you're turning into a penguin. Stop it."
            .to_owned();

        // when
        let result = parse_quote(&file_data).expect("Expected successful parsing");

        // then
        assert_eq!(result.filename, Utf8Path::new("penguin"));
        assert_eq!(result.source_name, None);
        assert_eq!(result.source_url, None);
        assert!(result.published);
        assert_eq!(
            result.content_markdown,
            "Ford… you're turning into a penguin. Stop it."
        );
        assert_eq!(result.modified_at, file_data.modified_at);
    }

    #[test]
    fn parse_quote_fails_if_id_is_missing() {
        // given
        let mut file_data = create_file_data();
        file_data.content = "---\n\
        source-name: Arthur Dent\n\
        source-url: https://example.com\n\
        published: false\n\
        ---\n\
        Ford… you're turning into a penguin. Stop it."
            .to_owned();

        // when
        let result = parse_quote(&file_data);

        // then
        assert_eq!(
            result,
            Err(ParseError::from("Missing id for quote foo/bar.md"))
        )
    }

    #[test]
    fn parse_quotes_omits_unpublished_quotes() {
        // given
        let mut published = create_file_data();
        published.content = "---\nid: in\npublished: true\n---\nIN".to_owned();

        let mut unpublished = create_file_data();
        unpublished.content = "---\nid: out\npublished: false\n---\nOUT".to_owned();

        // when
        let result = parse_quotes(&[published, unpublished]).expect("Expected successful parsing");

        // then
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].content_markdown, "IN");
    }
}
