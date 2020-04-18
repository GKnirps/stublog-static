use crate::input::parser::BlogpostMetadata;
use crate::paths::blogpost_path;
use maud::{html, Markup};

pub fn render_tag_page(tag_name: &str, posts: &[&BlogpostMetadata]) -> Markup {
    let title = format!("Stranger Than Usual — Tag: {}", tag_name);
    let content = html! {
        h2.section-heading {
            "Es gibt " (posts.len()) " Einträge mit dem Tag „" (tag_name) "“"
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
    super::base(&title, content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_blogpost_metadata;
    use std::path::PathBuf;

    #[test]
    fn render_tag_page_renders_all_entries() {
        // given
        let mut post1 = create_blogpost_metadata();
        post1.title = "Posty McPostface".to_owned();
        post1.filename = PathBuf::from("postface");

        let mut post2 = create_blogpost_metadata();
        post2.title = "Shaped like itself".to_owned();
        post2.filename = PathBuf::from("shaped");

        let tag_name = "stuff";

        // when
        let result = render_tag_page(tag_name, &[&post1, &post2]).into_string();

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
                "<ul><li><a href=\"/blogpost/postface\">Posty McPostface</a></li><li>\
        <a href=\"/blogpost/shaped\">Shaped like itself</a></li></ul>"
            ),
            "Expected a list of tags"
        )
    }
}
