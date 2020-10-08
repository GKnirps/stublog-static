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

use crate::input::{Blogpost, Category};
use crate::output::html::HeadData;
use crate::paths::{blogpost_path, category_path};
use crate::urls::{categories_url, category_url};
use maud::{html, Markup, PreEscaped};

pub fn render_categories_index_page(categories: &[(&Category, Vec<&Blogpost>)]) -> Markup {
    let content = html! {
        h2.section-heading { "Kategorien" }
        ul {
            @for (cat, blogposts) in categories {
                li {
                    a href=(category_path(&cat)) {
                        (cat.title)
                    }
                    " (" (blogposts.len()) " Einträge)"
                }
            }
        }
    };
    super::base(
        &HeadData::new("Stranger Than Usual — Kategorien").with_canonical_url(&categories_url()),
        content,
    )
}

pub fn render_category_page(category: &Category, blogposts: &[&Blogpost]) -> Markup {
    let content = html! {
        h2.section-heading { "Kategorie: " (category.title) }
        (PreEscaped(crate::output::cmark::render_cmark(&category.description_markdown, false)))

        h3 { "Diese Kategorie hat " (blogposts.len()) " Einträge" }
        ul {
            @for post in blogposts {
                li {
                    a href=(blogpost_path(&post)){
                        (post.title)
                    }
                }
            }
        }
    };
    super::base(
        &HeadData::new(&format!("Stranger Than Usual — {}", &category.title))
            .with_canonical_url(&category_url(&category)),
        content,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_blogpost, create_category};
    use std::path::Path;

    #[test]
    fn render_categories_index_page_should_render_all_entries() {
        // given
        let dummy_blogpost = create_blogpost();
        let mut cat1 = create_category();
        cat1.title = "Mikhail".to_owned();
        cat1.filename = Path::new("mika").to_owned();
        let mut cat2 = create_category();
        cat2.title = "Jessica".to_owned();
        cat2.filename = Path::new("jessi").to_owned();

        // when
        let result = render_categories_index_page(&[
            (&cat1, vec![&dummy_blogpost, &dummy_blogpost]),
            (&cat2, vec![&dummy_blogpost]),
        ])
        .into_string();

        // then
        println!("Checking rendered html:\n{}", result);
        assert!(result.contains("<ul><li><a href=\"/categories/mika\">Mikhail</a> (2 Einträge)</li><li><a href=\"/categories/jessi\">Jessica</a> (1 Einträge)</li></ul>"))
    }

    #[test]
    fn render_category_page_renders_correctly() {
        // given
        let mut post1 = create_blogpost();
        post1.title = "How to be a supervillain".to_owned();
        post1.filename = Path::new("sprvlln").to_owned();
        let mut post2 = create_blogpost();
        post2.title = "How not to get caught".to_owned();
        post2.filename = Path::new("caught").to_owned();

        let mut category = create_category();
        category.title = "Supervillainy".to_owned();
        category.description_markdown = "**Good business!**<div>foo</div>".to_owned();

        // when
        let result = render_category_page(&category, &[&post1, &post2]).into_string();

        // then
        println!("Checking rendered html:\n{}", result);
        assert!(result.contains("<h2 class=\"section-heading\">Kategorie: Supervillainy</h2>"));
        assert!(result.contains("<strong>Good business!</strong>&lt;div&gt;foo&lt;/div&gt;"));
        assert!(result.contains("<h3>Diese Kategorie hat 2 Einträge</h3>"));
        assert!(result.contains("<ul><li><a href=\"/blogposts/sprvlln\">How to be a supervillain</a></li><li><a href=\"/blogposts/caught\">How not to get caught</a></li></ul>"))
    }
}
