use maud::{html, Markup};

use super::blogpost::render_blogpost;
use crate::blogposts::Blogpost;

// TODO: create links to every blogpost
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
    use crate::parser::BlogpostMetadata;
    use chrono::{FixedOffset, TimeZone};
    use std::path::Path;

    #[test]
    fn render_home_should_render_all_given_blogposts() {
        // given
        let date = FixedOffset::east(3600 * 2)
            .ymd(2020, 5, 11)
            .and_hms(12, 13, 14);
        let post1 = Blogpost {
            content_html: "Post1".to_owned(),
            metadata: BlogpostMetadata {
                title: "t1".to_owned(),
                filename: Path::new("foo1").to_owned(),
                date,
            },
        };
        let post2 = Blogpost {
            content_html: "Post2".to_owned(),
            metadata: BlogpostMetadata {
                title: "t2".to_owned(),
                filename: Path::new("foo2").to_owned(),
                date: date.clone(),
            },
        };

        // when
        let result = render_home(&[post1, post2]).into_string();

        // then
        println!("Checking rendered html:\n{}", result);
        assert!(result.contains("<div class=\"blogposts\">"));
        assert!(result.contains("Post1"));
        assert!(result.contains("Post2"));
    }
}
