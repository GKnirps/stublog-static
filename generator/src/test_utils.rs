use crate::blogposts::Blogpost;
use crate::input::file::FileData;
use crate::input::{tag::Tag, BlogpostMetadata, Category, HostedFile};
use chrono::{FixedOffset, TimeZone};
use std::path::Path;
use std::time::SystemTime;

pub fn create_blogpost_metadata() -> BlogpostMetadata {
    let date = FixedOffset::east(3600 * 2)
        .ymd(2020, 5, 11)
        .and_hms(12, 13, 14);
    let update_date = FixedOffset::east(3600 * 2)
        .ymd(2020, 5, 25)
        .and_hms(12, 13, 14);
    BlogpostMetadata {
        title: "Nevermind".to_owned(),
        filename: Path::new("foobar").to_owned(),
        date,
        update_date: Some(update_date),
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
        old_id: None,
    }
}

pub fn create_hosted_file() -> HostedFile {
    HostedFile {
        old_id: None,
        path: "answer.txt".to_owned(),
        mime_type: "text/plain".to_owned(),
        description: "You're really not going to like it.".to_owned(),
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
