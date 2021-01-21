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
use super::quote::render_quote;
use crate::input::{Blogpost, Category, Quote};
use crate::output::html::HeadData;
use crate::urls::CANONICAL_BASE_URL;

pub fn render_home(blogposts: &[(&Blogpost, Option<&Category>)], qotd: Option<&Quote>) -> Markup {
    let html_content = html! {
        @if let Some(quote) = qotd {
            (render_quote(quote))
        }
        section.blogposts {
            @for (post, cat) in blogposts.iter().rev() {
                (render_blogpost(post, *cat))
            }
        }
    };

    super::base(
        &HeadData::new("Stranger Than Usual")
            .with_canonical_url(CANONICAL_BASE_URL)
            .with_description(Some(
                "Ein Blog über meine Erlebnisse, Gedanken und Rants. Hauptsächlich Rants.",
            ))
            .with_og_type("website"),
        html_content,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_blogpost, create_quote};

    #[test]
    fn render_home_should_render_all_given_blogposts() {
        // given
        let mut post1 = create_blogpost();
        post1.content_markdown = "Post1".to_owned();

        let mut post2 = create_blogpost();
        post2.content_markdown = "Post2".to_owned();

        // when
        let result = render_home(&[(&post1, None), (&post2, None)], None).into_string();

        // then
        println!("Checking rendered html:\n{}", result);
        assert!(result.contains("<section class=\"blogposts\">"));
        let post1_pos = result
            .find("<p>Post1</p>")
            .expect("Expected to find post 1");
        let post2_pos = result
            .find("<p>Post2</p>")
            .expect("Expected to find post 2");
        assert!(
            post1_pos > post2_pos,
            "Expected posts to be in reverse order"
        );
    }

    #[test]
    fn render_home_should_render_quote_if_present() {
        // given
        let post = create_blogpost();
        let quote = create_quote();

        // when
        let result = render_home(&[(&post, None)], Some(&quote)).into_string();

        // then
        println!("Checking rendered html:\n{}", result);
        assert!(result.contains("<article class=\"qotd\"><blockquote"));
    }
}
