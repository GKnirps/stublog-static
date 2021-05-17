/*
 * This file is part of stublog-static.
 *
 *  stublog-static is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU   Affero General Public License as published by
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

use crate::input::{tag::Tag, Assets, Blogpost};
use crate::output::html::HeadData;
use crate::paths::{blogpost_path, tag_path};
use crate::urls;
use maud::{html, Markup};

pub fn render_tag_list(tags: &[(&Tag, usize)], assets: &Assets) -> Markup {
    let content = html! {
        h2.section-heading { "Alle Tags" }
        ul {
            @for (tag, num) in tags {
                li {
                    a href=(tag_path(tag)) {
                        (tag.name) " (" (num) ")"
                    }
                }
            }
        }
    };

    super::base(
        &HeadData::new("Stranger Than Usual — Tags", assets)
            .with_canonical_url(&urls::tags_url())
            // make bots not keep tags out of the index
            // friggin' search sites mostly list my tag pages which are not useful in that context
            .with_noindex(),
        content,
    )
}

pub fn render_tag_page(tag: &Tag, posts: &[&Blogpost], assets: &Assets) -> Markup {
    let title = format!("Stranger Than Usual — Tag: {}", tag.name);
    let content = html! {
        h2.section-heading {
            "Es gibt " (posts.len()) " Einträge mit dem Tag „" (tag.name) "“"
        }
        ul {
            @for post in posts {
                li {
                    a href=(blogpost_path(post)) {
                        (post.title)
                    }
                }
            }
        }
    };
    super::base(
        &HeadData::new(&title, assets)
            .with_canonical_url(&urls::tag_url(tag))
            // make bots not keep tags out of the index
            // friggin' search sites mostly list my tag pages which are not useful in that context
            .with_noindex(),
        content,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_assets, create_blogpost};
    use std::path::PathBuf;

    #[test]
    fn render_tag_list_renders_list_of_tags() {
        // given
        let tags = &[(&Tag::new("foo"), 2), (&Tag::new("bar"), 42)];
        let assets = create_assets();

        // when
        let result = render_tag_list(tags, &assets).into_string();

        // then
        println!("Checking html:\n{}", result);
        assert!(result.contains(
            "<ul><li><a href=\"/tags/foo\">foo (2)</a></li>\
        <li><a href=\"/tags/bar\">bar (42)</a></li></ul>",
        ))
    }

    #[test]
    fn render_tag_page_renders_all_entries() {
        // given
        let mut post1 = create_blogpost();
        post1.title = "Posty McPostface".to_owned();
        post1.filename = PathBuf::from("postface");

        let mut post2 = create_blogpost();
        post2.title = "Shaped like itself".to_owned();
        post2.filename = PathBuf::from("shaped");

        let tag = Tag::new("stuff");

        let assets = create_assets();

        // when
        let result = render_tag_page(&tag, &[&post1, &post2], &assets).into_string();

        // then
        println!("Checking html:\n{}", result);
        assert!(
            result.contains("<title>Stranger Than Usual — Tag: stuff</title>"),
            "Expected a title"
        );
        assert!(
            result.contains(
                "<h2 class=\"section-heading\">Es gibt 2 Einträge mit dem Tag „stuff“</h2>"
            ),
            "Expected a section heading"
        );
        assert!(
            result.contains(
                "<ul><li><a href=\"/blogposts/postface\">Posty McPostface</a></li><li>\
        <a href=\"/blogposts/shaped\">Shaped like itself</a></li></ul>"
            ),
            "Expected a list of tags"
        )
    }
}
