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

use crate::input::Blogpost;
use crate::output::RenderError;
use crate::HostedFile;
use pulldown_cmark::escape::{escape_href, escape_html};
use pulldown_cmark::{html::push_html, Event, Parser};
use pulldown_cmark::{CowStr, Tag};
use std::collections::HashMap;

fn handle_images<'ev>(
    event: Event<'ev>,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<Event<'ev>, RenderError> {
    Ok(match &event {
        Event::Start(Tag::Image(_, url, title)) => {
            // we only handle image links to /file/, everything else is an error
            let filename = url.strip_prefix("/file/").ok_or_else(|| {
                RenderError::new(format!(
                    "hosted image '{}' does not start with '/file/'",
                    url
                ))
            })?;
            // TODO: simplify once we have SVG image sizes (this is a workaround because we do not, right now)
            let image_metadata = if !filename.ends_with(".svg") {
                Some(
                    hosted_files
                        .get(filename)
                        .ok_or_else(|| {
                            RenderError::new(format!("did not find hosted image '{}'", filename))
                        })?
                        .image_metadata
                        .ok_or_else(|| {
                            RenderError::new(format!(
                                "hosted image '{}' does not have image metadata",
                                url
                            ))
                        })?,
                )
            } else {
                None
            };
            let mut img_tag = if let Some(image_metadata) = image_metadata {
                format!(
                    "<img width=\"{}\" height=\"{}\" src=\"",
                    image_metadata.width, image_metadata.height
                )
            } else {
                "<img src=\"".to_string()
            };
            img_tag.reserve(256);
            escape_href(&mut img_tag, url).expect("Expected write to string buffer to not fail");
            img_tag.push_str("\" ");
            if !title.is_empty() {
                img_tag.push_str("title=\"");
                escape_html(&mut img_tag, title)
                    .expect("Expected write to string buffer to not fail");
                img_tag.push_str("\" ");
            }
            img_tag.push_str("alt=\"");
            Event::Html(CowStr::from(img_tag))
        }
        Event::End(Tag::Image(_, _, _)) => Event::Html(CowStr::Borrowed("\">")),
        _ => event,
    })
}
pub fn render_cmark(
    input: &str,
    allow_html: bool,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<String, RenderError> {
    let parser = Parser::new(input);
    let mut buf = String::with_capacity(input.len() * 2);
    if !allow_html {
        // Assigning image sizes may fail. If we don't collect all events in a Vec here, error handling
        // is far more complicated
        let parsed_with_custom_img_tags = parser
            .into_iter()
            // We have to do the HTML escaping _before_ the image stuff because the image stuff uses HTML events to do its thing
            .map(|event| match event {
                Event::Html(html) => Event::Text(html),
                _ => event,
            })
            .map(|e| handle_images(e, hosted_files))
            .collect::<Result<Vec<Event>, RenderError>>()?;
        push_html(&mut buf, parsed_with_custom_img_tags.into_iter())
    } else {
        // Assigning image sizes may fail. If we don't collect all events in a Vec here, error handling
        // is far more complicated
        let parsed_with_custom_img_tags = parser
            .map(|e| handle_images(e, hosted_files))
            .collect::<Result<Vec<Event>, RenderError>>()?;
        push_html(&mut buf, parsed_with_custom_img_tags.into_iter());
    }
    Ok(buf)
}

pub fn render_blogpost_content(
    blogpost: &Blogpost,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<String, RenderError> {
    render_cmark(
        &blogpost.content_markdown,
        blogpost.allow_html,
        hosted_files,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::ImageMetadata;
    use crate::test_utils::{create_blogpost, create_hosted_file};

    #[test]
    fn render_cmark_should_escape_html_if_required() {
        // given
        let markdown = "<a href=\"https://f.oo\">bar</a>";
        let hosted_files = HashMap::new();

        // when
        let html = render_cmark(markdown, false, &hosted_files).expect("expected no error");

        // then
        assert_eq!(
            html,
            "<p>&lt;a href=&quot;https://f.oo&quot;&gt;bar&lt;/a&gt;</p>\n"
        );
    }

    #[test]
    fn render_cmark_should_add_image_dimension() {
        // given
        let markdown = "![a cat is stealing a fish](/file/lolcat.png \"icanhasfish? kthxbye\")";
        let mut hosted_file = create_hosted_file();
        hosted_file.image_metadata = Some(ImageMetadata {
            width: 42,
            height: 9001,
        });
        let mut hosted_files = HashMap::with_capacity(1);
        hosted_files.insert("lolcat.png", &hosted_file);

        // when
        let html = render_cmark(markdown, false, &hosted_files).expect("expected no error");

        // then
        assert_eq!(html, "<p><img width=\"42\" height=\"9001\" src=\"/file/lolcat.png\" title=\"icanhasfish? kthxbye\" alt=\"a cat is stealing a fish\"></p>\n");
    }

    #[test]
    fn render_cmark_should_add_image_dimension_if_html_escaping_is_disabled() {
        // given
        let markdown = "![a cat is stealing a fish](/file/lolcat.png \"icanhasfish? kthxbye\")";
        let mut hosted_file = create_hosted_file();
        hosted_file.image_metadata = Some(ImageMetadata {
            width: 42,
            height: 9001,
        });
        let mut hosted_files = HashMap::with_capacity(1);
        hosted_files.insert("lolcat.png", &hosted_file);

        // when
        let html = render_cmark(markdown, true, &hosted_files).expect("expected no error");

        // then
        assert_eq!(html, "<p><img width=\"42\" height=\"9001\" src=\"/file/lolcat.png\" title=\"icanhasfish? kthxbye\" alt=\"a cat is stealing a fish\"></p>\n");
    }

    #[test]
    fn render_cmark_should_handle_missing_title() {
        // given
        let markdown = "![a cat is stealing a fish](/file/lolcat.png)";
        let mut hosted_file = create_hosted_file();
        hosted_file.image_metadata = Some(ImageMetadata {
            width: 42,
            height: 9001,
        });
        let mut hosted_files = HashMap::with_capacity(1);
        hosted_files.insert("lolcat.png", &hosted_file);

        // when
        let html = render_cmark(markdown, false, &hosted_files).expect("expected no error");

        // then
        assert_eq!(html, "<p><img width=\"42\" height=\"9001\" src=\"/file/lolcat.png\" alt=\"a cat is stealing a fish\"></p>\n");
    }

    #[test]
    fn render_cmark_should_fail_if_image_file_path_is_wrong() {
        // given
        let markdown =
            "![a cat is stealing a fish](/different/lolcat.png \"icanhasfish? kthxbye\")";
        let mut hosted_file = create_hosted_file();
        hosted_file.image_metadata = Some(ImageMetadata {
            width: 42,
            height: 9001,
        });
        let mut hosted_files = HashMap::with_capacity(1);
        hosted_files.insert("lolcat.png", &hosted_file);

        // when
        let result = render_cmark(markdown, false, &hosted_files);

        // then
        assert_eq!(
            result,
            Err(RenderError::from(
                "hosted image '/different/lolcat.png' does not start with '/file/'"
            ))
        );
    }

    #[test]
    fn render_cmark_should_fail_if_image_file_cannot_be_found() {
        // given
        let markdown = "![a cat is stealing a fish](/file/lolcat.png \"icanhasfish? kthxbye\")";
        let hosted_files = HashMap::new();

        // when
        let result = render_cmark(markdown, false, &hosted_files);

        // then
        assert_eq!(
            result,
            Err(RenderError::from("did not find hosted image 'lolcat.png'"))
        );
    }

    #[test]
    fn render_cmark_should_fail_if_file_has_no_image_metadata() {
        // given
        let markdown = "![a cat is stealing a fish](/file/lolcat.png \"icanhasfish? kthxbye\")";
        let mut hosted_file = create_hosted_file();
        hosted_file.image_metadata = None;
        let mut hosted_files = HashMap::with_capacity(1);
        hosted_files.insert("lolcat.png", &hosted_file);

        // when
        let result = render_cmark(markdown, false, &hosted_files);

        // then
        assert_eq!(
            result,
            Err(RenderError::from(
                "hosted image '/file/lolcat.png' does not have image metadata"
            ))
        );
    }

    // only temporary, until we have SVG image sizes
    #[test]
    fn render_cmark_should_not_fail_if_svg_file_has_no_image_metadata() {
        // given
        let markdown = "![a cat is stealing a fish](/file/lolcat.svg \"icanhasfish? kthxbye\")";
        let mut hosted_file = create_hosted_file();
        hosted_file.image_metadata = None;
        let mut hosted_files = HashMap::with_capacity(1);
        hosted_files.insert("lolcat.png", &hosted_file);

        // when
        let html = render_cmark(markdown, false, &hosted_files).expect("expected success");

        // then
        assert_eq!(&html, "<p><img src=\"/file/lolcat.svg\" title=\"icanhasfish? kthxbye\" alt=\"a cat is stealing a fish\"></p>\n");
    }

    #[test]
    fn render_cmark_should_not_escape_html_if_required() {
        // given
        let markdown = "<a href=\"https://f.oo\">bar</a>";
        let hosted_files = HashMap::new();

        // when
        let html = render_cmark(markdown, true, &hosted_files).expect("expected no error");

        // then
        assert_eq!(html, "<p><a href=\"https://f.oo\">bar</a></p>\n");
    }

    #[test]
    fn render_blogpost_content_should_escape_html_by_default() {
        // given
        let mut blogpost = create_blogpost();
        blogpost.allow_html = false;
        blogpost.content_markdown = "<em>dangit</em>".to_owned();
        let hosted_files = HashMap::new();

        // when
        let html = render_blogpost_content(&blogpost, &hosted_files).expect("expected success");

        // then
        assert_eq!(html, "<p>&lt;em&gt;dangit&lt;/em&gt;</p>\n")
    }

    #[test]
    fn render_blogpost_content_should_not_escape_html_if_told_so() {
        // given
        let mut blogpost = create_blogpost();
        blogpost.allow_html = true;
        blogpost.content_markdown = "<em>dangit</em>".to_owned();
        let hosted_files = HashMap::new();

        // when
        let html = render_blogpost_content(&blogpost, &hosted_files).expect("expected success");

        // then
        assert_eq!(html, "<p><em>dangit</em></p>\n")
    }
}
