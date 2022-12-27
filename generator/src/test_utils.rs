/*
 * This file is part of stublog-static.
 *
 *  stublog-static is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  stublog-static is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with stublog-static. If not, see <https://www.gnu.org/licenses/>.
 */

use crate::input::file::FileData;
use crate::input::{tag::Tag, Asset, Assets, Blogpost, Category, HostedFileMetadata, Quote};
use crate::HostedFile;
use chrono::{FixedOffset, TimeZone};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub fn create_blogpost() -> Blogpost {
    let date = FixedOffset::east_opt(3600 * 2)
        .unwrap()
        .with_ymd_and_hms(2020, 5, 11, 12, 13, 14)
        .unwrap();
    let update_date = FixedOffset::east_opt(3600 * 2)
        .unwrap()
        .with_ymd_and_hms(2020, 5, 25, 12, 13, 14)
        .unwrap();
    Blogpost {
        title: "Nevermind".to_owned(),
        filename: Path::new("foobar").to_owned(),
        date,
        update_date: Some(update_date),
        tags: vec![Tag::new("foo"), Tag::new("bar")],
        category_id: Some("bananas".to_owned()),
        allow_html: false,
        modified_at: SystemTime::now(),
        content_markdown: "*foo*bar".to_owned(),
        summary: Some("foo!".to_owned()),
        image: Some("/recipes.png".to_owned()),
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

pub fn create_hosted_file_metadata() -> HostedFileMetadata {
    HostedFileMetadata {
        old_id: None,
        path: "answer.txt".to_owned(),
        mime_type: "text/plain".to_owned(),
        description: "You're really not going to like it.".to_owned(),
        modified_at: SystemTime::now(),
    }
}

pub fn create_hosted_file() -> HostedFile {
    HostedFile {
        filename: PathBuf::from("answer.txt"),
        file_size: 42,
        image_metadata: None,
        modified_at: SystemTime::now(),
    }
}

pub fn create_quote() -> Quote {
    Quote {
        filename: Path::new("penguin").to_owned(),
        source_name: Some("Arthur Dent".to_owned()),
        source_url: Some("https://example.com/adent".to_owned()),
        published: true,
        content_markdown: "Ford… you're turning into a penguin. Stop it.".to_owned(),
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

pub fn create_assets() -> Assets {
    Assets {
        favicon: Asset {
            filename: PathBuf::from("../dist/assets/favicon.png"),
            modified_at: SystemTime::now(),
            web_path: "/assets/favicon.png".to_owned(),
        },
        stylesheet: Asset {
            filename: PathBuf::from("../dist/assets/style.css"),
            modified_at: SystemTime::now(),
            web_path: "/assets/style.css".to_owned(),
        },
    }
}
