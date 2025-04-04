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

use crate::input::{Blogpost, Category, HostedFileMetadata, Quote, tag::Tag};
use percent_encoding::{AsciiSet, CONTROLS, percent_encode};

pub const ESCAPE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'`')
    .add(b'?')
    .add(b'{')
    .add(b'}')
    .add(b'#');

pub fn blogpost_path(blogpost: &Blogpost) -> String {
    format!(
        "/blogposts/{}",
        percent_encode(blogpost.filename.as_str().as_bytes(), ESCAPE_SET)
    )
}

pub fn archive_path(page: usize) -> String {
    format!("/archive/{page}")
}

pub static TAGLIST_PATH: &str = "/tags";

pub fn tag_path(tag: &Tag) -> String {
    format!(
        "{}/{}",
        TAGLIST_PATH,
        percent_encode(tag.normalized_name.as_bytes(), ESCAPE_SET)
    )
}

pub static CATEGORIES_PATH: &str = "/categories";

pub fn category_path(category: &Category) -> String {
    format!(
        "{}/{}",
        CATEGORIES_PATH,
        percent_encode(category.filename.as_str().as_bytes(), ESCAPE_SET)
    )
}

pub static ATOM_FEED_PATH: &str = "/feed.atom";

pub fn hosted_file_path(hosted_file: &HostedFileMetadata) -> String {
    format!(
        "/file/{}",
        percent_encode(hosted_file.path.as_bytes(), ESCAPE_SET)
    )
}

pub fn quote_path(quote: &Quote) -> String {
    format!(
        "/quote/{}",
        percent_encode(quote.filename.as_str().as_bytes(), ESCAPE_SET)
    )
}

pub fn quote_list_path(page: usize) -> String {
    format!("/quotes/{page}")
}

pub static QUOTE_FORTUNE_PATH: &str = "/quotes/strangerthanusual.tar.bz2";

pub static FILES_METADATA_BASE_PATH: &str = "/files_metadata";

pub fn files_metadata_index_path(page: usize) -> String {
    format!("{FILES_METADATA_BASE_PATH}/{page}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        create_blogpost, create_category, create_hosted_file_metadata, create_quote,
    };
    use camino::Utf8Path;

    #[test]
    fn test_blogpost_path() {
        // given
        let mut blogpost = create_blogpost();
        blogpost.filename = Utf8Path::new("foö-bar").to_owned();

        // when
        let result = blogpost_path(&blogpost);

        // then
        assert_eq!(&result, "/blogposts/fo%C3%B6-bar");
    }

    #[test]
    fn test_archive_path() {
        // given
        let page = 42;

        // when
        let result = archive_path(page);

        // then
        assert_eq!(&result, "/archive/42");
    }

    #[test]
    fn test_tag_path() {
        // given
        let tag = Tag::new("hög-r");

        // when
        let result = tag_path(&tag);

        // then
        assert_eq!(&result, "/tags/h%C3%B6g-r");
    }

    #[test]
    fn test_category_path() {
        // given
        let mut category = create_category();
        category.filename = Utf8Path::new("sömewher e").to_owned();
        category.id = "notthis".to_owned();

        // when
        let result = category_path(&category);

        // then
        assert_eq!(result, "/categories/s%C3%B6mewher%20e");
    }

    #[test]
    fn test_hosted_file_path() {
        // given
        let mut hosted_file = create_hosted_file_metadata();
        hosted_file.old_id = Some("notthis".to_owned());
        hosted_file.path = "äh?".to_owned();

        // when
        let result = hosted_file_path(&hosted_file);

        // then
        assert_eq!(result, "/file/%C3%A4h%3F");
    }

    #[test]
    fn test_quote_path() {
        // given
        let mut quote = create_quote();
        quote.filename = Utf8Path::new("wtf?").to_owned();

        // when
        let result = quote_path(&quote);

        // then
        assert_eq!(result, "/quote/wtf%3F");
    }

    #[test]
    fn test_quote_list_path() {
        // given
        let index = 11;

        // when
        let result = quote_list_path(index);

        // then
        assert_eq!(result, "/quotes/11");
    }

    #[test]
    fn test_files_metadata_index_path() {
        // given
        let index = 32;

        // when
        let result = files_metadata_index_path(index);

        // then
        assert_eq!(result, "/files_metadata/32")
    }
}
