use crate::blogposts::Blogpost;
use maud::{html, Markup, PreEscaped};

pub fn render_blogpost(blogpost: &Blogpost) -> Markup {
    let metadata = &blogpost.metadata;
    html! {
        article.blogpost {
            div.entry { (PreEscaped(&blogpost.content_html)) }
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
    }
}

pub fn render_blogpost_page(blogpost: &Blogpost) -> Markup {
    super::base(&blogpost.metadata.title, render_blogpost(blogpost))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blogposts::Blogpost;
    use crate::parser::BlogpostMetadata;
    use chrono::{FixedOffset, TimeZone};
    use std::path::Path;

    fn create_blogpost() -> Blogpost {
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

    #[test]
    fn render_blogpost_should_render_blogpost_with_footer() {
        // given
        let blogpost = create_blogpost();

        // when
        let result = render_blogpost(&blogpost).into_string();

        // then
        println!("Checking rendered html:\n{}", result);
        assert!(result.contains("<div class=\"entry\"><p><em>foo</em>bar</p></div>"));
        assert!(result.contains("11.05.2020 12:13"));
    }

    #[test]
    fn render_blogpost_page_should_render_blogpost() {
        // given
        let blogpost = create_blogpost();

        // when
        let result = render_blogpost_page(&blogpost).into_string();

        // then
        println!("Checking rendered html:\n{}", result);
        assert!(result.contains("<div class=\"entry\"><p><em>foo</em>bar</p></div>"));
        assert!(result.contains("<title>Nevermind</title>"));
        assert!(result.contains("11.05.2020 12:13"));
    }
}
