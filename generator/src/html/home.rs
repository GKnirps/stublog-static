use maud::{html, Markup};

use super::blogpost::render_blogpost;
use crate::blogposts::Blogpost;

pub fn render_home(blogposts: &[Blogpost]) -> Markup {
    let html_content = html! {
        div.blogposts {
            @for post in blogposts {
                (render_blogpost(post))
            }
        }
    };

    super::base("Strangert Than Usual", html_content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_blogpost;

    #[test]
    fn render_home_should_render_all_given_blogposts() {
        // given
        let mut post1 = create_blogpost();
        post1.content_html = "Post1".to_owned();

        let mut post2 = create_blogpost();
        post2.content_html = "Post2".to_owned();

        // when
        let result = render_home(&[post1, post2]).into_string();

        // then
        println!("Checking rendered html:\n{}", result);
        assert!(result.contains("<div class=\"blogposts\">"));
        assert!(result.contains("Post1"));
        assert!(result.contains("Post2"));
    }
}
