use crate::input::tag::Tag;
use crate::input::{BlogpostMetadata, Category};
use crate::paths;

pub static CANONICAL_BASE_URL: &str = "https://blog.strangerthanusual.de";

pub fn blogpost_url(metadata: &BlogpostMetadata) -> String {
    format!("{}{}", CANONICAL_BASE_URL, paths::blogpost_path(metadata))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_blogpost_metadata, create_category};
    use std::path::Path;

    #[test]
    fn blogpost_url_renders_correct_url() {
        // given
        let mut metadata = create_blogpost_metadata();
        metadata.filename = Path::new("dürk").to_path_buf();

        // when
        let url = blogpost_url(&metadata);

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
}
