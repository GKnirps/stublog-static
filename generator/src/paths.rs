use crate::input::{tag::Tag, BlogpostMetadata, Category, HostedFile, Quote};
use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

const ESCAPE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'`')
    .add(b'?')
    .add(b'{')
    .add(b'}')
    .add(b'#');

pub fn blogpost_path(metadata: &BlogpostMetadata) -> String {
    format!(
        "/blogposts/{}",
        percent_encode(metadata.filename.to_string_lossy().as_bytes(), ESCAPE_SET)
    )
}

pub fn archive_path(page: usize) -> String {
    format!("/archive/{}", page)
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
        percent_encode(category.filename.to_string_lossy().as_bytes(), ESCAPE_SET)
    )
}

pub static ATOM_FEED_PATH: &str = "/feed.atom";

pub fn hosted_file_path(hosted_file: &HostedFile) -> String {
    format!(
        "/file/{}",
        percent_encode(hosted_file.path.as_bytes(), ESCAPE_SET)
    )
}

pub fn quote_path(quote: &Quote) -> String {
    format!(
        "/quote/{}",
        percent_encode(quote.filename.to_string_lossy().as_bytes(), ESCAPE_SET)
    )
}

pub fn quote_list_path(page: usize) -> String {
    format!("/quotes/{}", page)
}

pub static QUOTE_FORTUNE_PATH: &str = "/quotes/strangerthanusual.tar.bz2";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        create_blogpost_metadata, create_category, create_hosted_file, create_quote,
    };
    use std::path::Path;

    #[test]
    fn test_blogpost_path() {
        // given
        let mut metadata = create_blogpost_metadata();
        metadata.filename = Path::new("foö-bar").to_owned();

        // when
        let result = blogpost_path(&metadata);

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
        category.filename = Path::new("sömewher e").to_owned();
        category.id = "notthis".to_owned();

        // when
        let result = category_path(&category);

        // then
        assert_eq!(result, "/categories/s%C3%B6mewher%20e");
    }

    #[test]
    fn test_hosted_file_path() {
        // given
        let mut hosted_file = create_hosted_file();
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
        quote.filename = Path::new("wtf?").to_owned();

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
}
