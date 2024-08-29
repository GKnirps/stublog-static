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
use crate::output::{image_metadata_by_path, RenderError};
use crate::HostedFile;
use pulldown_cmark::{html::push_html, Event, Parser};
use pulldown_cmark::{CowStr, Tag, TagEnd};
use pulldown_cmark_escape::{escape_href, escape_html};
use std::collections::HashMap;

struct CustomImageTagIterator<'ev, 'meta, T: Iterator<Item = Event<'ev>>> {
    source: T,
    hosted_files: &'meta HashMap<&'meta str, &'meta HostedFile>,
}

impl<'ev, 'meta, T: Iterator<Item = Event<'ev>>> Iterator
    for CustomImageTagIterator<'ev, 'meta, T>
{
    type Item = Result<Event<'ev>, RenderError>;

    fn next(&mut self) -> Option<Self::Item> {
        let event = self.source.next()?;
        Some(match &event {
            Event::Start(Tag::Image {
                link_type: _,
                dest_url: url,
                title,
                id: _,
            }) => {
                let image_metadata = match image_metadata_by_path(url, self.hosted_files) {
                    Err(e) => {
                        return Some(Err(e));
                    }
                    Ok(m) => m,
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
                escape_href(&mut img_tag, url)
                    .expect("Expected write to string buffer to not fail");
                img_tag.push_str("\" ");
                if !title.is_empty() {
                    img_tag.push_str("title=\"");
                    escape_html(&mut img_tag, title)
                        .expect("Expected write to string buffer to not fail");
                    img_tag.push_str("\" ");
                }
                img_tag.push_str("alt=\"");
                let mut next = self.source.next();
                while next != Some(Event::End(TagEnd::Image)) {
                    match next {
                        Some(
                            Event::InlineHtml(alt_text)
                            | Event::Code(alt_text)
                            | Event::Text(alt_text),
                        ) => {
                            escape_html(&mut img_tag, &alt_text)
                                .expect("Expected write to string buffer to not fail");
                        }
                        None => {
                            return Some(Err(RenderError::from(
                                    "expected image tag end event, but found end of stream (this may be a bug on my side)",
                                )));
                        }
                        Some(e) => {
                            return Some(Err(RenderError::new(format!("expected text event for image alt attribute, got '{e:?}' (this is due to a workaround in this blog, some other elements may also be possible but unlikely)"))));
                        }
                    }
                    next = self.source.next();
                }
                img_tag.push_str("\">");
                Ok(Event::Html(CowStr::from(img_tag)))
            }
            Event::End(TagEnd::Image) => Err(RenderError::from("stray image end event")),
            _ => Ok(event),
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.source.size_hint()
    }
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
        let parsed_with_custom_img_tags = CustomImageTagIterator {
            source: parser
                .into_iter()
                // We have to do the HTML escaping _before_ the image stuff because the image stuff uses HTML events to do its thing
                .map(|event| match event {
                    Event::Html(html) => Event::Text(html),
                    Event::InlineHtml(html) => Event::Text(html),
                    // the other events are listed explicitly so I can avoid nasty surprises if they
                    // add a new event that contains HTML
                    Event::Start(_)
                    | Event::End(_)
                    | Event::Text(_)
                    | Event::Code(_)
                    | Event::InlineMath(_)
                    | Event::DisplayMath(_)
                    | Event::FootnoteReference(_)
                    | Event::SoftBreak
                    | Event::HardBreak
                    | Event::Rule
                    | Event::TaskListMarker(_) => event,
                }),
            hosted_files,
        }
        .collect::<Result<Vec<Event>, RenderError>>()?;
        push_html(&mut buf, parsed_with_custom_img_tags.into_iter())
    } else {
        // Assigning image sizes may fail. If we don't collect all events in a Vec here, error handling
        // is far more complicated
        let parsed_with_custom_img_tags = CustomImageTagIterator {
            source: parser,
            hosted_files,
        }
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
    fn render_cmark_should_escape_inline_html_if_required() {
        // given
        let markdown = "<a href=\"https://f.oo\">bar</a>";
        let hosted_files = HashMap::new();

        // when
        let html = render_cmark(markdown, false, &hosted_files).expect("expected no error");

        // then
        assert_eq!(
            html,
            "<p>&lt;a href=\"https://f.oo\"&gt;bar&lt;/a&gt;</p>\n"
        );
    }

    // TODO: due to the trick I use to escape HTML, escaped flow elements (such as diff) do not have a <p> around them
    // whereas escaped phrasing element (such as span, see next test) do. This is not very
    // consistent and I should change it.
    #[test]
    fn render_cmark_should_escape_flow_html_if_required() {
        // given
        let markdown = "<div>bar</div>";
        let hosted_files = HashMap::new();

        // when
        let html = render_cmark(markdown, false, &hosted_files).expect("expected no error");

        // then
        assert_eq!(html, "&lt;div&gt;bar&lt;/div&gt;");
    }

    #[test]
    fn render_cmark_should_escape_phrasing_html_if_required() {
        // given
        let markdown = "<span>bar</span>";
        let hosted_files = HashMap::new();

        // when
        let html = render_cmark(markdown, false, &hosted_files).expect("expected no error");

        // then
        assert_eq!(html, "<p>&lt;span&gt;bar&lt;/span&gt;</p>\n");
    }

    #[test]
    fn render_cmark_should_escape_html_around_markdown() {
        // given
        let markdown = "<div>First\n\n**second**\n\n ### third\n</div>";
        let hosted_files = HashMap::new();

        // when
        let html = render_cmark(markdown, false, &hosted_files).expect("expected no error");

        // then
        assert_eq!(
            html,
            "&lt;div&gt;First\n<p><strong>second</strong></p>\n<h3>third</h3>\n&lt;/div&gt;"
        );
    }

    #[test]
    fn render_cmark_should_escape_unknown_html_tags() {
        // given
        let markdown = "<rhubarb>foo</rhubarb>";
        let hosted_files = HashMap::new();

        // when
        let html = render_cmark(markdown, false, &hosted_files).expect("expected no error");

        // then
        assert_eq!(html, "<p>&lt;rhubarb&gt;foo&lt;/rhubarb&gt;</p>\n");
    }

    #[test]
    fn render_cmark_should_escape_html_comments() {
        // given
        let markdown = "<!-- foo -->";
        let hosted_files = HashMap::new();

        // when
        let html = render_cmark(markdown, false, &hosted_files).expect("expected no error");

        // then
        assert_eq!(html, "&lt;!-- foo --&gt;");
    }

    #[test]
    fn render_cmark_should_not_escape_html_escape_sequences() {
        // given
        let markdown = "&amp;&";
        let hosted_files = HashMap::new();

        // when
        let html = render_cmark(markdown, false, &hosted_files).expect("expected no error");

        // then
        assert_eq!(html, "<p>&amp;&amp;</p>\n");
    }

    #[test]
    fn render_cmark_should_add_image_dimension() {
        // given
        let markdown =
            "![a cat is >stealing< a fish](/file/lolca\"t.png \"icanhasfish? kthxbye&\")";
        let mut hosted_file = create_hosted_file();
        hosted_file.image_metadata = Some(ImageMetadata {
            width: 42,
            height: 9001,
        });
        let mut hosted_files = HashMap::with_capacity(1);
        hosted_files.insert("lolca\"t.png", &hosted_file);

        // when
        let html = render_cmark(markdown, false, &hosted_files).expect("expected no error");

        // then
        assert_eq!(html, "<p><img width=\"42\" height=\"9001\" src=\"/file/lolca%22t.png\" title=\"icanhasfish? kthxbye&amp;\
        \" alt=\"a cat is &gt;stealing&lt; a fish\"></p>\n");
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
    fn render_cmark_should_escape_image_attributes_correctly() {
        // given
        let markdown =
            "![a cat is \"borrowing\" a fish](/file/lolcat.png \"icanhasfish? \\\"kthxbye\")";
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
        assert_eq!(html, "<p><img width=\"42\" height=\"9001\" src=\"/file/lolcat.png\" title=\"icanhasfish? &quot;kthxbye\" alt=\"a cat is &quot;borrowing&quot; a fish\"></p>\n");
    }

    #[test]
    fn render_cmark_should_escape_image_attributes_correctly_if_html_escaping_is_disabled() {
        // given
        let markdown =
            "![a cat is \"borrowing\" a fish](/file/lolcat.png \"icanhasfish? \\\"kthxbye\")";
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
        assert_eq!(html, "<p><img width=\"42\" height=\"9001\" src=\"/file/lolcat.png\" title=\"icanhasfish? &quot;kthxbye\" alt=\"a cat is &quot;borrowing&quot; a fish\"></p>\n");
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

    #[test]
    fn render_cmark_should_not_fail_if_svg_file_has_no_image_metadata() {
        // given
        let markdown = "![a cat is stealing a fish](/file/lolcat.svg \"icanhasfish? kthxbye\")";
        let mut hosted_file = create_hosted_file();
        hosted_file.image_metadata = None;
        let mut hosted_files = HashMap::with_capacity(1);
        hosted_files.insert("lolcat.svg", &hosted_file);

        // when
        let html = render_cmark(markdown, false, &hosted_files).expect("expected success");

        // then
        assert_eq!(&html, "<p><img src=\"/file/lolcat.svg\" title=\"icanhasfish? kthxbye\" alt=\"a cat is stealing a fish\"></p>\n");
    }

    #[test]
    fn render_cmark_should_fail_if_svg_file_cannot_be_found() {
        // given
        let markdown = "![a cat is stealing a fish](/file/lolcat.svg \"icanhasfish? kthxbye\")";
        let hosted_files = HashMap::new();

        // when
        let result = render_cmark(markdown, false, &hosted_files);

        // then
        assert_eq!(
            result,
            Err(RenderError::from("did not find hosted image 'lolcat.svg'"))
        );
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
