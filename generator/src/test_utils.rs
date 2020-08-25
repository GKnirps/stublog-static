use crate::blogposts::Blogpost;
use crate::input::file::FileData;
use crate::input::{BlogpostMetadata, Category, Tag};
use chrono::{FixedOffset, TimeZone};
use std::path::Path;
use std::time::SystemTime;

pub fn create_blogpost_metadata() -> BlogpostMetadata {
    let date = FixedOffset::east(3600 * 2)
        .ymd(2020, 5, 11)
        .and_hms(12, 13, 14);
    BlogpostMetadata {
        title: "Nevermind".to_owned(),
        filename: Path::new("foobar").to_owned(),
        date,
        tags: vec![Tag::new("foo"), Tag::new("bar")],
        category_id: Some("bananas".to_owned()),
        allow_html: false,
        modified_at: SystemTime::now(),
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
        modified_at: SystemTime::now(),
    }
}

pub fn create_file_data() -> FileData {
    FileData {
        content: "---\nfoo: bar\n---\n\nSomething".to_owned(),
        filename: Path::new("foo/bar.md").to_path_buf(),
        modified_at: SystemTime::now(),
    }
}
