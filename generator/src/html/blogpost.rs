use crate::parser::BlogpostMetadata;
use maud::{html, Markup, PreEscaped};
use pulldown_cmark::{html::push_html, Parser};

fn render_cmark(input: &str) -> String {
    let parser = Parser::new(input);
    let mut buf = String::with_capacity(input.len() * 2);
    push_html(&mut buf, parser);
    buf
}

pub fn render_blogpost(metadata: &BlogpostMetadata, content: &str) -> Markup {
    let html_content = html! {
        article.blogpost {
            div.entry { (PreEscaped(render_cmark(content))) }
            footer {
                span.post-time {
                    (super::time(&metadata.date))
                }
                span.category {
                    "Kategorie: TODO"
                }
                span.tags {
                    "Tags: TODO"
                }
            }
        }
    };
    super::base(&metadata.title, html_content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{FixedOffset, TimeZone};
    use std::path::Path;

    #[test]
    fn render_blogpost_should_render_blogpost_with_footer() {
        // given
        let date = FixedOffset::east(3600 * 2)
            .ymd(2020, 5, 11)
            .and_hms(12, 13, 14);
        let metadata = BlogpostMetadata {
            title: "Nevermind".to_owned(),
            filename: Path::new("foobar").to_owned(),
            date,
        };
        let content = "*foo*bar";

        // when
        let result = render_blogpost(&metadata, content).into_string();

        // then
        println!("Checking rendered html:\n{}", result);
        assert!(result.contains("<div class=\"entry\"><p><em>foo</em>bar</p>\n</div>"));
        assert!(result.contains("<title>Nevermind</title>"));
        assert!(result.contains("11.05.2020 12:13"));
    }
}
