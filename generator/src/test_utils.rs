use crate::blogposts::Blogpost;
use crate::parser::BlogpostMetadata;
use chrono::{FixedOffset, TimeZone};
use std::path::Path;

pub fn create_blogpost() -> Blogpost {
    let date = FixedOffset::east(3600 * 2)
        .ymd(2020, 5, 11)
        .and_hms(12, 13, 14);
    let metadata = BlogpostMetadata {
        title: "Nevermind".to_owned(),
        filename: Path::new("foobar").to_owned(),
        date,
    };
    let content_html = "<p><em>foo</em>bar</p>".to_owned();

    Blogpost {
        metadata,
        content_html,
    }
}
