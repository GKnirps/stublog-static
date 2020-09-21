use crate::input::BlogpostMetadata;
use crate::paths::{blogpost_path, ATOM_FEED_PATH};

pub static CANONICAL_BASE_URL: &str = "https://blog.strangerthanusual.de";

pub fn blogpost_url(metadata: &BlogpostMetadata) -> String {
    format!("{}{}", CANONICAL_BASE_URL, blogpost_path(metadata))
}

pub fn atom_feed_url() -> String {
    [CANONICAL_BASE_URL, ATOM_FEED_PATH].concat()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_blogpost_metadata;
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
}
