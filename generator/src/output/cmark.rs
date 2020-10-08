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

use crate::input::Blogpost;
use pulldown_cmark::{html::push_html, Event, Parser};

pub fn render_cmark(input: &str, allow_html: bool) -> String {
    let parser = Parser::new(input);
    let mut buf = String::with_capacity(input.len() * 2);
    if !allow_html {
        push_html(
            &mut buf,
            parser.map(|event| match event {
                Event::Html(html) => Event::Text(html),
                _ => event,
            }),
        )
    } else {
        push_html(&mut buf, parser);
    }
    buf
}

pub fn render_blogpost_content(blogpost: &Blogpost) -> String {
    render_cmark(&blogpost.content_markdown, blogpost.allow_html)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_blogpost;

    #[test]
    fn render_cmark_should_escape_html_if_required() {
        // given
        let markdown = "<a href=\"https://f.oo\">bar</a>";

        // when
        let html = render_cmark(markdown, false);

        // then
        assert_eq!(
            html,
            "<p>&lt;a href=&quot;https://f.oo&quot;&gt;bar&lt;/a&gt;</p>\n"
        );
    }

    #[test]
    fn render_cmark_should_not_escape_html_if_required() {
        // given
        let markdown = "<a href=\"https://f.oo\">bar</a>";

        // when
        let html = render_cmark(markdown, true);

        // then
        assert_eq!(html, "<p><a href=\"https://f.oo\">bar</a></p>\n");
    }

    #[test]
    fn render_blogpost_content_should_escape_html_by_default() {
        // given
        let mut blogpost = create_blogpost();
        blogpost.allow_html = false;
        blogpost.content_markdown = "<em>dangit</em>".to_owned();

        // when
        let html = render_blogpost_content(&blogpost);

        // then
        assert_eq!(html, "<p>&lt;em&gt;dangit&lt;/em&gt;</p>\n")
    }

    #[test]
    fn render_blogpost_content_should_not_escape_html_if_told_so() {
        // given
        let mut blogpost = create_blogpost();
        blogpost.allow_html = true;
        blogpost.content_markdown = "<em>dangit</em>".to_owned();

        // when
        let html = render_blogpost_content(&blogpost);

        // then
        assert_eq!(html, "<p><em>dangit</em></p>\n")
    }
}
