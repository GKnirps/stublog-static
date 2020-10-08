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
    let description: Option<&str> = blogpost.summary.as_ref().map(|s| {
        let r: &str = s;
        r
    });
    super::base(
        &HeadData::new(&blogpost.title)
            .with_canonical_url(&blogpost_url(&blogpost))
            .with_description(description),
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
        assert!(result.contains("<meta rel=\"description\" content=\"foo!\">"));
        assert!(result.contains("<div class=\"entry\"><p><em>foo</em>bar</p>\n</div>"));
        assert!(result.contains("<title>Nevermind</title>"));
        assert!(result.contains("11.05.2020 12:13"));
        assert!(result.contains("<a href=\"/categories/chocolate\">Cocoa</a>"));
    }
}
