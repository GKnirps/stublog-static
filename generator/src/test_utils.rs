use crate::blogposts::Blogpost;
use crate::input::BlogpostMetadata;
use crate::input::Category;
use chrono::{FixedOffset, TimeZone};
use std::path::Path;

pub fn create_blogpost_metadata() -> BlogpostMetadata {
    let date = FixedOffset::east(3600 * 2)
        .ymd(2020, 5, 11)
        .and_hms(12, 13, 14);
    BlogpostMetadata {
        title: "Nevermind".to_owned(),
        filename: Path::new("foobar").to_owned(),
        date,
        tags: vec!["foo".to_owned(), "bar".to_owned()],
        category_id: Some("bananas".to_owned()),
        allow_html: false,
    }
}

pub fn create_blogpost() -> Blogpost {
    let metadata = create_blogpost_metadata();
    let content_html = "<p><em>foo</em>bar</p>".to_owned();

    Blogpost {
        metadata,
        content_html,
    }
}

pub fn create_category() -> Category {
    Category {
        title: "Cocoa".to_owned(),
        filename: Path::new("chocolate").to_owned(),
        id: "fesazu".to_owned(),
        description_markdown: "## Chocolate!!!111".to_owned(),
    }
}
