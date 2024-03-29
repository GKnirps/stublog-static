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

use crate::input::tag::Tag;
use crate::input::{Blogpost, Category, Quote};
use crate::paths;
use percent_encoding::percent_encode;

pub static CANONICAL_BASE_URL: &str = "https://blog.strangerthanusual.de";

pub fn url_for_absolute_path(path: &str) -> String {
    format!(
        "{}{}",
        CANONICAL_BASE_URL,
        percent_encode(path.as_bytes(), paths::ESCAPE_SET)
    )
}

pub fn blogpost_url(blogpost: &Blogpost) -> String {
    format!("{}{}", CANONICAL_BASE_URL, paths::blogpost_path(blogpost))
}

pub fn archive_url(page_index: usize) -> String {
    format!("{}{}", CANONICAL_BASE_URL, paths::archive_path(page_index))
}

pub fn category_url(category: &Category) -> String {
    format!("{}{}", CANONICAL_BASE_URL, paths::category_path(category))
}

pub fn categories_url() -> String {
    [CANONICAL_BASE_URL, paths::CATEGORIES_PATH].concat()
}

pub fn tag_url(tag: &Tag) -> String {
    format!("{}{}", CANONICAL_BASE_URL, paths::tag_path(tag))
}

pub fn tags_url() -> String {
    [CANONICAL_BASE_URL, paths::TAGLIST_PATH].concat()
}

pub fn atom_feed_url() -> String {
    [CANONICAL_BASE_URL, paths::ATOM_FEED_PATH].concat()
}

pub fn quote_url(quote: &Quote) -> String {
    format!("{}{}", CANONICAL_BASE_URL, paths::quote_path(quote))
}

pub fn quote_list_url(page: usize) -> String {
    format!("{}{}", CANONICAL_BASE_URL, paths::quote_list_path(page))
}

pub fn files_metadata_index_url(page: usize) -> String {
    format!(
        "{}{}",
        CANONICAL_BASE_URL,
        paths::files_metadata_index_path(page)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_blogpost, create_category, create_quote};
    use camino::Utf8Path;

    #[test]
    fn url_for_absolute_path_renders_url_for_any_path() {
        // given
        let path = "/file/handtücher.png";

        // when
        let url = url_for_absolute_path(path);

        // then
        assert_eq!(
            url,
            "https://blog.strangerthanusual.de/file/handt%C3%BCcher.png"
        );
    }

    #[test]
    fn blogpost_url_renders_correct_url() {
        // given
        let mut blogpost = create_blogpost();
        blogpost.filename = Utf8Path::new("dürk").to_path_buf();

        // when
        let url = blogpost_url(&blogpost);

        // then
        assert_eq!(url, "https://blog.strangerthanusual.de/blogposts/d%C3%BCrk");
    }

    #[test]
    fn archive_url_renders_correct_url() {
        // given
        let page_index = 42;

        // when
        let url = archive_url(page_index);

        // then
        assert_eq!(url, "https://blog.strangerthanusual.de/archive/42")
    }

    #[test]
    fn category_url_renders_correct_url() {
        // given
        let mut cat = create_category();
        cat.filename = Utf8Path::new("foobar").to_path_buf();

        // when
        let url = category_url(&cat);

        // then
        assert_eq!(url, "https://blog.strangerthanusual.de/categories/foobar");
    }

    #[test]
    fn quote_url_renders_correct_url() {
        // given
        let mut quote = create_quote();
        quote.filename = Utf8Path::new("marks").to_owned();

        // when
        let url = quote_url(&quote);

        // then
        assert_eq!(url, "https://blog.strangerthanusual.de/quote/marks");
    }

    #[test]
    fn quote_list_url_renders_correct_url() {
        // given
        let index = 42;

        // when
        let url = quote_list_url(index);

        // then
        assert_eq!(url, "https://blog.strangerthanusual.de/quotes/42");
    }

    #[test]
    fn files_metadata_index_url_renders_correct_url() {
        // given
        let index = 99;

        // when
        let url = files_metadata_index_url(index);

        // then
        assert_eq!(url, "https://blog.strangerthanusual.de/files_metadata/99");
    }
}
