use crate::input::tag::Tag;
use crate::input::{Blogpost, Category, Quote};
use crate::paths;

pub static CANONICAL_BASE_URL: &str = "https://blog.strangerthanusual.de";

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_blogpost, create_category, create_quote};
    use std::path::Path;

    #[test]
    fn blogpost_url_renders_correct_url() {
        // given
        let mut blogpost = create_blogpost();
        blogpost.filename = Path::new("dürk").to_path_buf();

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
        cat.filename = Path::new("foobar").to_path_buf();

        // when
        let url = category_url(&cat);

        // then
        assert_eq!(url, "https://blog.strangerthanusual.de/categories/foobar");
    }

    #[test]
    fn quote_url_renders_correct_url() {
        // given
        let mut quote = create_quote();
        quote.filename = Path::new("marks").to_owned();

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
}
