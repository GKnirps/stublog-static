use super::super::blogposts::Blogpost;
use crate::input::Category;
use crate::paths::{blogpost_path, category_path, tag_path};
use maud::{html, Markup, PreEscaped};

pub fn render_blogpost(blogpost: &Blogpost, category: Option<&Category>) -> Markup {
    let metadata = &blogpost.metadata;
    let permalink = blogpost_path(metadata);
    html! {
        article.blogpost {
            div.entry { (PreEscaped(&blogpost.content_html)) }
            footer {
                span.post-time {
                    (super::time(&metadata.date))
                }
                @if let Some(cat) = &category {
                    span.category {
                        "Kategorie: "
                        a href=(category_path(cat)) { (cat.title) }
                    }
                }
                @if !metadata.tags.is_empty() {
                    span.tags {
                        "Tags: "
                        @for (i, tag) in metadata.tags.iter().enumerate() {
                            a href=(tag_path(tag)) {
                                (tag.name)
                            }
                            @if i + 1 < metadata.tags.len() { ", " }
                        }
                    }
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

pub fn render_blogpost_page(blogpost: &Blogpost, category: Option<&Category>) -> Markup {
    super::base(
        &blogpost.metadata.title,
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
        assert!(result.contains("<div class=\"entry\"><p><em>foo</em>bar</p></div>"));
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
        assert!(result.contains("<div class=\"entry\"><p><em>foo</em>bar</p></div>"));
        assert!(result.contains("<title>Nevermind</title>"));
        assert!(result.contains("11.05.2020 12:13"));
        assert!(result.contains("<a href=\"/categories/chocolate\">Cocoa</a>"));
    }
}
