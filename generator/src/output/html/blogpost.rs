use super::super::blogposts::Blogpost;
use crate::paths::blogpost_path;
use maud::{html, Markup, PreEscaped};

pub fn render_blogpost(blogpost: &Blogpost) -> Markup {
    let metadata = &blogpost.metadata;
    let permalink = blogpost_path(metadata);
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
                span.permalink {
                    // TODO: is there a way to make permalinks nicer (e.g. use the headline of the
                    // article as link)?
                    a href=(permalink) {
                        "Permalink"
                    }
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
    use crate::test_utils::create_blogpost;

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