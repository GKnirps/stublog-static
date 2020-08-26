use crate::input::{tag::Tag, BlogpostMetadata, Category};
use percent_encoding::{percent_encode, PATH_SEGMENT_ENCODE_SET};

pub fn blogpost_path(metadata: &BlogpostMetadata) -> String {
    format!(
        "/blogposts/{}",
        percent_encode(
            metadata.filename.to_string_lossy().as_bytes(),
            PATH_SEGMENT_ENCODE_SET
        )
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
        percent_encode(tag.normalized_name.as_bytes(), PATH_SEGMENT_ENCODE_SET)
    )
}

pub static CATEGORIES_PATH: &str = "/categories";

pub fn category_path(category: &Category) -> String {
    format!(
        "{}/{}",
        CATEGORIES_PATH,
        percent_encode(
            category.filename.to_string_lossy().as_bytes(),
            PATH_SEGMENT_ENCODE_SET
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_blogpost_metadata, create_category};
    use std::path::Path;

    #[test]
    fn test_blogpost_path() {
        // given
        let mut metadata = create_blogpost_metadata();
        metadata.filename = Path::new("foö/bar").to_owned();

        // when
        let result = blogpost_path(&metadata);

        // then
        assert_eq!(&result, "/blogposts/fo%C3%B6%2Fbar");
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
        let tag = Tag::new("högr");

        // when
        let result = tag_path(&tag);

        // then
        assert_eq!(&result, "/tags/h%C3%B6gr");
    }

    #[test]
    fn test_category_path() {
        // given
        let mut category = create_category();
        category.filename = Path::new("sömewhere").to_owned();
        category.id = "notthis".to_owned();

        // when
        let result = category_path(&category);

        // then
        assert_eq!(result, "/categories/s%C3%B6mewhere");
    }
}
