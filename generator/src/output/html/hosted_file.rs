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

use super::pager::pager;
use crate::input::{Assets, HostedFileMetadata};
use crate::output::html::HeadData;
use crate::paths::{files_metadata_index_path, hosted_file_path};
use crate::urls::files_metadata_index_url;
use crate::HostedFile;
use maud::{html, Markup};

fn render_file_data(metadata: &HostedFileMetadata, hosted_file: &HostedFile) -> Markup {
    html! {
        div.hosted-file #(metadata.path) {
            h3 {
                "Datei: "
                (metadata.path)
            }
            p.file-description {
                (metadata.description)
            }
            footer {
                table {
                    tr {
                        td {"Typ"}
                        td {(metadata.mime_type)}
                    }
                    tr {
                        td {"Größe"}
                        td {(hosted_file.file_size)}
                    }
                }
                a.download-link href=(hosted_file_path(metadata)) download=(metadata.path) type=(metadata.mime_type) {
                    "Herunterladen"
                }
            }
        }
    }
}

pub fn render_file_index_page(
    files: &[(&HostedFileMetadata, &HostedFile)],
    current_page: usize,
    num_pages: usize,
    assets: &Assets,
) -> Markup {
    let html_pager = pager(current_page, num_pages, &files_metadata_index_path);
    let content = html! {
        h2 { "Dateien" }
        (html_pager)
        section {
            @for (metadata, hosted_file) in files {
                (render_file_data(metadata, hosted_file))
            }
        }
        (html_pager)
    };

    super::base(
        &HeadData::new(
            &format!(
                "Stranger Than Usual — Dateien, Seite {} von {}",
                current_page + 1,
                num_pages,
            ),
            assets,
        )
        .with_canonical_url(&files_metadata_index_url(current_page))
        .with_description(Some(
            "Eine Liste von Dateien, die hier gehostet und in Blogposts referenziert werden",
        )),
        content,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_assets, create_hosted_file, create_hosted_file_metadata};
    use std::path::PathBuf;

    #[test]
    fn render_file_data_should_render_correctly() {
        // given
        let metadata = create_hosted_file_metadata();
        let file = create_hosted_file();

        // when
        let result = render_file_data(&metadata, &file).into_string();

        // then
        assert_eq!(
            result,
            "<div class=\"hosted-file\" id=\"answer.txt\">\
        <h3>Datei: answer.txt</h3>\
        <p class=\"file-description\">You\'re really not going to like it.</p>\
        <footer>\
        <table>\
        <tr><td>Typ</td><td>text/plain</td></tr>\
        <tr><td>Größe</td><td>42</td></tr>\
        </table>\
        <a class=\"download-link\" href=\"/file/answer.txt\" download=\"answer.txt\" type=\"text/plain\">\
        Herunterladen\
        </a>\
        </footer>\
        </div>"
        );
    }

    #[test]
    fn render_file_index_page_should_render_important_data() {
        // given
        let mut meta1 = create_hosted_file_metadata();
        meta1.path = "first".to_owned();
        let mut meta2 = create_hosted_file_metadata();
        meta2.path = "ninjad".to_owned();
        let mut file1 = create_hosted_file();
        file1.filename = PathBuf::from("something_different"); // this path should not be used here
        file1.file_size = 1234;
        let file2 = create_hosted_file();

        let files = &[(&meta1, &file1), (&meta2, &file2)];

        let current_page = 3;
        let num_pages = 5;

        let assets = create_assets();

        // when
        let result = render_file_index_page(files, current_page, num_pages, &assets).into_string();

        // then
        println!("Checking generated html:\n{}", result);
        assert!(result.starts_with("<!DOCTYPE html><html lang=\"de\">"));
        assert!(result.contains("<h3>Datei: first</h3>"));
        assert!(result.contains("<h3>Datei: ninjad</h3>"));
        assert!(result.contains("<title>Stranger Than Usual — Dateien, Seite 4 von 5</title>"));
        assert!(result.contains("<nav class=\"pagination\">"));
    }
}
