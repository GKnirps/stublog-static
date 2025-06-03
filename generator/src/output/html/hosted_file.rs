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
use crate::HostedFile;
use crate::input::{Assets, HostedFileMetadata};
use crate::output::RenderError;
use crate::output::cmark::render_cmark;
use crate::output::html::HeadData;
use crate::paths::{files_metadata_index_path, hosted_file_path};
use crate::urls::files_metadata_index_url;
use maud::{Markup, PreEscaped, html};
use std::collections::HashMap;

fn render_file_size(size: u64) -> Markup {
    if size >= 1024 * 1024 {
        let mb_size = (size as f64 * 10.0 / (1024.0 * 1024.0)).round() / 10.0;
        html! {
            "≈ " (mb_size) " MiB"
        }
    } else if size >= 1024 {
        let kb_size = (size as f64 * 10.0 / 1024.0).round() / 10.0;
        html! {
           "≈ " (kb_size) " kiB"
        }
    } else {
        html! {
            (size) " B"
        }
    }
}

fn render_file_data(
    metadata: &HostedFileMetadata,
    hosted_file: &HostedFile,
    all_hosted_files_by_name: &HashMap<&str, &HostedFile>,
) -> Result<Markup, RenderError> {
    Ok(html! {
        div.hosted-file #(metadata.path) {
            h3 {
                "Datei: "
                (metadata.path)
            }
            (PreEscaped(render_cmark(&metadata.description_markdown, false, all_hosted_files_by_name)?))
            footer {
                table {
                    tr {
                        td {"Typ"}
                        td {(metadata.mime_type)}
                    }
                    tr {
                        td {"Größe"}
                        td {(render_file_size(hosted_file.file_size))}
                    }
                    @if let Some(image_metadata) = hosted_file.image_metadata {
                        tr {
                            td {"Bildgröße"}
                            td {(image_metadata.width) "×" (image_metadata.height)}
                        }
                    }
                }
                a.download-link href=(hosted_file_path(metadata)) download=(metadata.path) type=(metadata.mime_type) {
                    "Herunterladen"
                }
            }
        }
    })
}

pub fn render_file_index_page(
    files: &[(&HostedFileMetadata, &HostedFile)],
    current_page: usize,
    num_pages: usize,
    assets: &Assets,
    all_hosted_files_by_name: &HashMap<&str, &HostedFile>,
) -> Result<Markup, RenderError> {
    let html_pager = pager(current_page, num_pages, &files_metadata_index_path);
    let content = html! {
        h2 { "Dateien" }
        (html_pager)
        section {
            @for (metadata, hosted_file) in files {
                (render_file_data(metadata, hosted_file, all_hosted_files_by_name)?)
            }
        }
        (html_pager)
    };

    Ok(super::base(
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
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::ImageMetadata;
    use crate::test_utils::{create_assets, create_hosted_file, create_hosted_file_metadata};
    use camino::Utf8PathBuf;

    #[test]
    fn render_file_size_renders_size_appropriately() {
        // < 1 kiB, render size in bytes
        assert_eq!(&render_file_size(0).into_string(), "0 B");
        assert_eq!(&render_file_size(1023).into_string(), "1023 B");
        // >= 1 kiB but smaller 1 MiB
        assert_eq!(&render_file_size(1024).into_string(), "≈ 1 kiB");
        assert_eq!(&render_file_size(1025).into_string(), "≈ 1 kiB");
        assert_eq!(&render_file_size(1124).into_string(), "≈ 1.1 kiB");
        assert_eq!(&render_file_size(1536).into_string(), "≈ 1.5 kiB");
        assert_eq!(&render_file_size(1024 * 1023).into_string(), "≈ 1023 kiB");
        // >= 1 MiB (we don't render GiB sizes, because we do not intend to host files that large)
        assert_eq!(&render_file_size(1024 * 1024).into_string(), "≈ 1 MiB");
        assert_eq!(&render_file_size(1024 * 1124).into_string(), "≈ 1.1 MiB");
    }

    #[test]
    fn render_file_data_should_render_correctly() {
        // given
        let metadata = create_hosted_file_metadata();
        let mut file = create_hosted_file();
        file.file_size = 1024;
        let all_hosted_files: HashMap<&str, &HostedFile> = HashMap::from([("answer.txt", &file)]);

        // when
        let result = render_file_data(&metadata, &file, &all_hosted_files);

        // then
        let result = result.expect("expected successful rendering").into_string();
        assert_eq!(
            result,
            "<div class=\"hosted-file\" id=\"answer.txt\">\
        <h3>Datei: answer.txt</h3>\
        <p>You\'re <em>really</em> not going to like it.</p>
<footer>\
        <table>\
        <tr><td>Typ</td><td>text/plain</td></tr>\
        <tr><td>Größe</td><td>≈ 1 kiB</td></tr>\
        </table>\
        <a class=\"download-link\" href=\"/file/answer.txt\" download=\"answer.txt\" type=\"text/plain\">\
        Herunterladen\
        </a>\
        </footer>\
        </div>"
        );
    }

    #[test]
    fn render_file_data_should_render_image_size_if_present() {
        // given
        let mut metadata = create_hosted_file_metadata();
        metadata.mime_type = "image/webp".to_owned();
        let mut file = create_hosted_file();
        file.file_size = 1024;
        file.image_metadata = Some(ImageMetadata {
            width: 42,
            height: 9001,
        });
        let all_hosted_files: HashMap<&str, &HostedFile> = HashMap::from([("answer.txt", &file)]);

        // when
        let result = render_file_data(&metadata, &file, &all_hosted_files);

        // then
        let result = result.expect("expected successful rendering").into_string();
        assert_eq!(
            result,
            "<div class=\"hosted-file\" id=\"answer.txt\">\
        <h3>Datei: answer.txt</h3>\
        <p>You\'re <em>really</em> not going to like it.</p>
<footer>\
        <table>\
        <tr><td>Typ</td><td>image/webp</td></tr>\
        <tr><td>Größe</td><td>≈ 1 kiB</td></tr>\
        <tr><td>Bildgröße</td><td>42×9001</td></tr>\
        </table>\
        <a class=\"download-link\" href=\"/file/answer.txt\" download=\"answer.txt\" type=\"image/webp\">\
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
        file1.filename = Utf8PathBuf::from("something_different"); // this path should not be used here
        file1.file_size = 1234;
        let file2 = create_hosted_file();

        let files = &[(&meta1, &file1), (&meta2, &file2)];

        let current_page = 3;
        let num_pages = 5;

        let assets = create_assets();

        let all_hosted_files: HashMap<&str, &HostedFile> =
            HashMap::from([("path", &file1), ("ninjad", &file2)]);

        // when
        let result =
            render_file_index_page(files, current_page, num_pages, &assets, &all_hosted_files);

        // then
        let result = result.expect("expected successful rendering").into_string();
        println!("Checking generated html:\n{result}");
        assert!(result.starts_with("<!DOCTYPE html><html lang=\"de\">"));
        assert!(result.contains("<h3>Datei: first</h3>"));
        assert!(result.contains("<h3>Datei: ninjad</h3>"));
        assert!(result.contains("<title>Stranger Than Usual — Dateien, Seite 4 von 5</title>"));
        assert!(result.contains("<nav class=\"pagination\">"));
    }
}
