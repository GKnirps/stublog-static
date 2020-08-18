use crate::input::Category;
use crate::output::blogposts::Blogpost;
use crate::paths::{blogpost_path, category_path};
use maud::{html, Markup, PreEscaped};

pub fn render_categories_index_page(categories: &[(&Category, Vec<&Blogpost>)]) -> Markup {
    let content = html! {
        h2.section-heading { "Kategorien" }
        ul {
            @for (cat, blogposts) in categories {
                li {
                    a href=(category_path(&cat)) {
                        (cat.title)
                    }
                    " (" (blogposts.len()) " Einträge)"
                }
            }
        }
    };
    super::base("Stranger Than Usual — Kategorien", content)
}

pub fn render_category_page(category: &Category, blogposts: &[&Blogpost]) -> Markup {
    let content = html! {
        h2.section-heading { "Kategorie: " (category.title) }
        (PreEscaped(crate::output::render_cmark(&category.description_markdown)))

        h3 { "Diese Kategorie hat " (blogposts.len()) " Einträge" }
        ul {
            @for post in blogposts {
                li {
                    a href=(blogpost_path(&post.metadata)){
                        (post.metadata.title)
                    }
                }
            }
        }
    };
    super::base(
        &format!("Stranger Than Usual — {}", &category.title),
        content,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_blogpost, create_category};
    use std::path::Path;

    #[test]
    fn render_categories_index_page_should_render_all_entries() {
        // given
        let dummy_blogpost = create_blogpost();
        let mut cat1 = create_category();
        cat1.title = "Mikhail".to_owned();
        cat1.filename = Path::new("mika").to_owned();
        let mut cat2 = create_category();
        cat2.title = "Jessica".to_owned();
        cat2.filename = Path::new("jessi").to_owned();

        // when
        let result = render_categories_index_page(&[
            (&cat1, vec![&dummy_blogpost, &dummy_blogpost]),
            (&cat2, vec![&dummy_blogpost]),
        ])
        .into_string();

        // then
        println!("Checking rendered html:\n{}", result);
        assert!(result.contains("<ul><li><a href=\"/categories/mika\">Mikhail</a> (2 Einträge)</li><li><a href=\"/categories/jessi\">Jessica</a> (1 Einträge)</li></ul>"))
    }

    #[test]
    fn render_category_page_renders_correctly() {
        // given
        let mut post1 = create_blogpost();
        post1.metadata.title = "How to be a supervillain".to_owned();
        post1.metadata.filename = Path::new("sprvlln").to_owned();
        let mut post2 = create_blogpost();
        post2.metadata.title = "How not to get caught".to_owned();
        post2.metadata.filename = Path::new("caught").to_owned();

        let mut category = create_category();
        category.title = "Supervillainy".to_owned();
        category.description_markdown = "**Good business!**".to_owned();

        // when
        let result = render_category_page(&category, &[&post1, &post2]).into_string();

        // then
        println!("Checking rendered html:\n{}", result);
        assert!(result.contains("<h2 class=\"section-heading\">Kategorie: Supervillainy</h2>"));
        assert!(result.contains("<strong>Good business!</strong>"));
        assert!(result.contains("<h3>Diese Kategorie hat 2 Einträge</h3>"));
        assert!(result.contains("<ul><li><a href=\"/blogposts/sprvlln\">How to be a supervillain</a></li><li><a href=\"/blogposts/caught\">How not to get caught</a></li></ul>"))
    }
}
