use maud::{html, Markup};

use super::super::blogposts::Blogpost;
use super::blogpost::render_blogpost;
use super::pager::pager;
use crate::input::Category;

pub fn render_archive(
    blogposts: &[(&Blogpost, Option<&Category>)],
    current_page: usize,
    num_pages: usize,
) -> Markup {
    let html_pager = pager(current_page, num_pages);
    let html_content = html! {
        div.blogposts {
            (html_pager)
                @for (post, cat) in blogposts {
                    (render_blogpost(post, *cat))
                }
            (html_pager)
        }
    };

    super::base("Stranger Than Usual — Archiv", html_content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_blogpost;

    #[test]
    fn render_archive_renders_blogpost_and_pager() {
        // given
        let mut blogpost1 = create_blogpost();
        blogpost1.content_html = "<p>hello</p>".to_owned();
        let mut blogpost2 = create_blogpost();
        blogpost2.content_html = "<p>world</p>".to_owned();

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
            result.contains("<div class=\"entry\"><p>hello</p></div>"),
            "Expected first article on page."
        );
        assert!(
            result.contains("<div class=\"entry\"><p>world</p></div>"),
            "Expected second article on page."
        );
        assert!(
            result.contains("<nav class=\"pagination\">"),
            "Expected to find a pager"
        );
    }
}
