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

use maud::{html, Markup};

use super::blogpost::render_blogpost;
use super::pager::pager;
use crate::input::{Blogpost, Category};
use crate::output::html::HeadData;
use crate::paths::archive_path;
use crate::urls::archive_url;

pub fn render_archive(
    blogposts: &[(&Blogpost, Option<&Category>)],
    current_page: usize,
    num_pages: usize,
) -> Markup {
    let html_pager = pager(current_page, num_pages, &archive_path);
    let html_content = html! {
        div.blogposts {
            (html_pager)
                @for (post, cat) in blogposts {
                    (render_blogpost(post, *cat))
                }
            (html_pager)
        }
    };

    super::base(
        &HeadData::new("Stranger Than Usual — Archiv")
            .with_canonical_url(&archive_url(current_page)),
        html_content,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_blogpost;

    #[test]
    fn render_archive_renders_blogpost_and_pager() {
        // given
        let mut blogpost1 = create_blogpost();
        blogpost1.content_markdown = "hello".to_owned();
        let mut blogpost2 = create_blogpost();
        blogpost2.content_markdown = "world".to_owned();

        let current_page = 3;
        let num_pages = 10;

        // when
        let result = render_archive(
            &[(&blogpost1, None), (&blogpost2, None)],
            current_page,
            num_pages,
        )
        .into_string();

        // then
        println!("Checking html: {}", result);
        assert!(
            result.contains("<div class=\"entry\"><p>hello</p>\n</div>"),
            "Expected first article on page."
        );
        assert!(
            result.contains("<div class=\"entry\"><p>world</p>\n</div>"),
            "Expected second article on page."
        );
        assert!(
            result.contains("<nav class=\"pagination\">"),
            "Expected to find a pager"
        );
    }
}
