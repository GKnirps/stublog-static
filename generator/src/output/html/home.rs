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
        div.blogposts {
            @for (post, cat) in blogposts.iter().rev() {
                (render_blogpost(post, *cat))
            }
        }
    };

    super::base(
        &HeadData::new("Stranger Than Usual").with_canonical_url(CANONICAL_BASE_URL),
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
        assert!(result.contains("<div class=\"blogposts\">"));
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
        assert!(result.contains("<div class=\"qotd\"><blockquote"));
    }
}
