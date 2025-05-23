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

use crate::HostedFile;
use crate::input::{Assets, Blogpost, Category};
use crate::output::RenderError;
use crate::output::html::HeadData;
use crate::paths::{blogpost_path, category_path};
use crate::urls::{categories_url, category_url};
use maud::{Markup, PreEscaped, html};
use std::collections::HashMap;

pub fn render_categories_index_page(
    categories: &[(&Category, Vec<&Blogpost>)],
    assets: &Assets,
) -> Markup {
    let content = html! {
        h2.section-heading { "Kategorien" }
        ul {
            @for (cat, blogposts) in categories {
                li {
                    a href=(category_path(cat)) {
                        (cat.title)
                    }
                    " (" (blogposts.len()) " Einträge)"
                }
            }
        }
    };
    super::base(
        &HeadData::new("Stranger Than Usual — Kategorien", assets)
            .with_canonical_url(&categories_url())
            .with_description(Some("Auflistung aller Kategorien dieses Blogs")),
        content,
    )
}

pub fn render_category_page(
    category: &Category,
    blogposts: &[&Blogpost],
    assets: &Assets,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<Markup, RenderError> {
    let content = html! {
        h2.section-heading { "Kategorie: " (category.title) }
        (PreEscaped(crate::output::cmark::render_cmark(&category.description_markdown, false, hosted_files)?))

        h3 { "Diese Kategorie hat " (blogposts.len()) " Einträge" }
        ul {
            @for post in blogposts {
                li {
                    a href=(blogpost_path(post)){
                        (post.title)
                    }
                }
            }
        }
    };
    Ok(super::base(
        &HeadData::new(
            &format!("Stranger Than Usual — {}", &category.title),
            assets,
        )
        .with_canonical_url(&category_url(category))
        .with_description(Some(&category.summary)),
        content,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_assets, create_blogpost, create_category};
    use camino::Utf8Path;

    #[test]
    fn render_categories_index_page_should_render_all_entries() {
        // given
        let dummy_blogpost = create_blogpost();
        let mut cat1 = create_category();
        cat1.title = "Mikhail".to_owned();
        cat1.filename = Utf8Path::new("mika").to_owned();
        let mut cat2 = create_category();
        cat2.title = "Jessica".to_owned();
        cat2.filename = Utf8Path::new("jessi").to_owned();

        let assets = create_assets();

        // when
        let result = render_categories_index_page(
            &[
                (&cat1, vec![&dummy_blogpost, &dummy_blogpost]),
                (&cat2, vec![&dummy_blogpost]),
            ],
            &assets,
        )
        .into_string();

        // then
        println!("Checking rendered html:\n{result}");
        assert!(result.contains(
            r#"<meta name="description" content="Auflistung aller Kategorien dieses Blogs">"#
        ));
        assert!(result.contains("<ul><li><a href=\"/categories/mika\">Mikhail</a> (2 Einträge)</li><li><a href=\"/categories/jessi\">Jessica</a> (1 Einträge)</li></ul>"))
    }

    #[test]
    fn render_category_page_renders_correctly() {
        // given
        let mut post1 = create_blogpost();
        post1.title = "How to be a supervillain".to_owned();
        post1.filename = Utf8Path::new("sprvlln").to_owned();
        let mut post2 = create_blogpost();
        post2.title = "How not to get caught".to_owned();
        post2.filename = Utf8Path::new("caught").to_owned();

        let mut category = create_category();
        category.title = "Supervillainy".to_owned();
        category.summary = "For those who wake up and choose violence".to_owned();
        category.description_markdown = "**Good business!**<div>foo</div>".to_owned();

        let assets = create_assets();

        let hosted_files = HashMap::new();

        // when
        let result = render_category_page(&category, &[&post1, &post2], &assets, &hosted_files)
            .expect("expected success")
            .into_string();

        // then
        println!("Checking rendered html:\n{result}");
        assert!(result.contains(
            r#"<meta name="description" content="For those who wake up and choose violence">"#
        ));
        assert!(result.contains("<h2 class=\"section-heading\">Kategorie: Supervillainy</h2>"));
        assert!(result.contains("<strong>Good business!</strong>&lt;div&gt;foo&lt;/div&gt;"));
        assert!(result.contains("<h3>Diese Kategorie hat 2 Einträge</h3>"));
        assert!(result.contains("<ul><li><a href=\"/blogposts/sprvlln\">How to be a supervillain</a></li><li><a href=\"/blogposts/caught\">How not to get caught</a></li></ul>"))
    }
}
