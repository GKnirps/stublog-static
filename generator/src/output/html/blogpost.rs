use super::super::cmark::render_blogpost_content;
use crate::input::{Blogpost, Category};
use crate::output::html::HeadData;
use crate::paths::{blogpost_path, category_path, tag_path};
use crate::urls::blogpost_url;
use maud::{html, Markup, PreEscaped};

pub fn render_blogpost(blogpost: &Blogpost, category: Option<&Category>) -> Markup {
    let permalink = blogpost_path(blogpost);
    html! {
        article.blogpost {
            h2.posttitle {
                a href=(permalink) rel="bookmark" {
                    (blogpost.title)
                }
            }
            div.entry { (PreEscaped(render_blogpost_content(&blogpost))) }
            footer {
                span.post-time {
                    (super::time(&blogpost.date))
                }
                @if let Some(cat) = &category {
                    span.category {
                        "Kategorie: "
                        a href=(category_path(cat)) { (cat.title) }
                    }
                }
                @if !blogpost.tags.is_empty() {
                    span.tags {
                        "Tags: "
                        @for (i, tag) in blogpost.tags.iter().enumerate() {
                            a href=(tag_path(tag)) {
                                (tag.name)
                            }
                            @if i + 1 < blogpost.tags.len() { ", " }
                        }
                    }
                }
            }
        }
    }
}

pub fn render_blogpost_page(blogpost: &Blogpost, category: Option<&Category>) -> Markup {
    super::base(
        &HeadData::new(&blogpost.title).with_canonical_url(&blogpost_url(&blogpost)),
        render_blogpost(blogpost, category),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_blogpost, create_category};

    #[test]
    fn render_blogpost_should_render_blogpost_with_footer() {
        // given
        let blogpost = create_blogpost();
        let category = create_category();

        // when
        let result = render_blogpost(&blogpost, Some(&category)).into_string();

        // then
        println!("Checking rendered html:\n{}", result);
        assert!(result
            .contains("<h2 class=\"posttitle\"><a href=\"/blogposts/foobar\" rel=\"bookmark\">Nevermind</a></h2>"));
        assert!(result.contains("<div class=\"entry\"><p><em>foo</em>bar</p>\n</div>"));
        assert!(result.contains("11.05.2020 12:13"));
        assert!(result.contains("<a href=\"/categories/chocolate\">Cocoa</a>"));
    }

    #[test]
    fn render_blogpost_should_not_render_absent_category() {
        // given
        let blogpost = create_blogpost();

        // when
        let result = render_blogpost(&blogpost, None).into_string();

        // then
        println!("Checking rendered html:\n{}", result);
        assert!(!result.contains("<span class=\"category\">"));
    }

    #[test]
    fn render_blogpost_page_should_render_blogpost() {
        // given
        let blogpost = create_blogpost();
        let category = create_category();

        // when
        let result = render_blogpost_page(&blogpost, Some(&category)).into_string();

        // then
        println!("Checking rendered html:\n{}", result);
        assert!(result.contains("<div class=\"entry\"><p><em>foo</em>bar</p>\n</div>"));
        assert!(result.contains("<title>Nevermind</title>"));
        assert!(result.contains("11.05.2020 12:13"));
        assert!(result.contains("<a href=\"/categories/chocolate\">Cocoa</a>"));
    }
}
