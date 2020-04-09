use crate::parser::BlogpostMetadata;

pub fn blogpost_path(metadata: &BlogpostMetadata) -> String {
    format!("/blogpost/{}", metadata.filename.to_string_lossy())
}

pub fn archive_path(page: usize) -> String {
    format!("/blogposts/{}", page)
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
            filename: Path::new("foobar").to_owned(),
            date,
        };

        // when
        let result = blogpost_path(&metadata);

        // then
        assert_eq!(&result, "/blogpost/foobar");
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
}
