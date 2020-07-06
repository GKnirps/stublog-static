use crate::input::BlogpostMetadata;
use percent_encoding::{percent_encode, PATH_SEGMENT_ENCODE_SET};

pub fn blogpost_path(metadata: &BlogpostMetadata) -> String {
    format!(
        "/blogpost/{}",
        percent_encode(
            metadata.filename.to_string_lossy().as_bytes(),
            PATH_SEGMENT_ENCODE_SET
        )
    )
}

pub fn archive_path(page: usize) -> String {
    format!("/blogposts/{}", page)
}

pub static TAGLIST_PATH: &str = "/tags";

pub fn tag_path(tag: &str) -> String {
    return format!(
        "{}/{}",
        TAGLIST_PATH,
        percent_encode(tag.as_bytes(), PATH_SEGMENT_ENCODE_SET)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{FixedOffset, TimeZone};
    use std::path::Path;

    #[test]
    fn test_blogpost_path() {
        // given
        let date = FixedOffset::east(3600 * 2)
            .ymd(2020, 5, 11)
            .and_hms(12, 13, 14);
        let metadata = BlogpostMetadata {
            title: "Nevermind".to_owned(),
            filename: Path::new("foö/bar").to_owned(),
            date,
            tags: vec![],
        };

        // when
        let result = blogpost_path(&metadata);

        // then
        assert_eq!(&result, "/blogpost/fo%C3%B6%2Fbar");
    }

    #[test]
    fn test_archive_path() {
        // given
        let page = 42;

        // when
        let result = archive_path(page);

        // then
        assert_eq!(&result, "/blogposts/42");
    }

    #[test]
    fn test_tag_path() {
        // given
        let tag = "högr";

        // when
        let result = tag_path(tag);

        // then
        assert_eq!(&result, "/tags/h%C3%B6gr");
    }
}
