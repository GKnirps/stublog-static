use maud::{html, Markup};

use super::super::blogposts::Blogpost;
use super::blogpost::render_blogpost;
use crate::input::Category;

pub fn render_home(blogposts: &[(&Blogpost, Option<&Category>)]) -> Markup {
    let html_content = html! {
        div.blogposts {
            @for (post, cat) in blogposts.iter().rev() {
                (render_blogpost(post, *cat))
            }
        }
    };

    super::base("Stranger Than Usual", html_content)
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
        let result = render_home(&[(&post1, None), (&post2, None)]).into_string();

        // then
        println!("Checking rendered html:\n{}", result);
        assert!(result.contains("<div class=\"blogposts\">"));
        let post1_pos = result.find("Post1").expect("Expected to find post 1");
        let post2_pos = result.find("Post2").expect("Expected to find post 2");
        assert!(
            post1_pos > post2_pos,
            "Expected posts to be in reverse order"
        );
    }
}
