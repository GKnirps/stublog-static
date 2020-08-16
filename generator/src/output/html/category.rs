use super::super::categories::Category;
use crate::paths::{blogpost_path, category_path};
use maud::{html, Markup, PreEscaped};

pub fn render_categories_index_page(categories: &[Category]) -> Markup {
    let content = html! {
        h2.section-heading { "Kategorien" }
        ul {
            @for cat in categories {
                li {
                    a href=(category_path(&cat.metadata)) {
                        (cat.metadata.title)
                    }
                    " (" (cat.blogposts.len()) " Einträge)"
                }
            }
        }
    };
    super::base("Stranger Than Usual — Kategorien", content)
}

pub fn render_category_page(category: &Category) -> Markup {
    let content = html! {
        h2.section-heading { "Kategorie: " (category.metadata.title) }
        (PreEscaped(&category.description_html))

        h3 { "Diese Kategorie hat " (category.blogposts.len()) " Einträge" }
        ul {
            @for post in &category.blogposts {
                li {
                    a href=(blogpost_path(&post.metadata)){
                        (post.metadata.title)
                    }
                }
            }
        }
    };
    super::base(
        &format!("Stranger Than Usual — {}", &category.metadata.title),
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
        let mut cat1 = create_category(vec![&dummy_blogpost, &dummy_blogpost]);
        let mut cat2 = create_category(vec![&dummy_blogpost]);
        cat1.metadata.title = "Mikhail".to_owned();
        cat1.metadata.filename = Path::new("mika").to_owned();
        cat2.metadata.title = "Jessica".to_owned();
        cat2.metadata.filename = Path::new("jessi").to_owned();

        // when
        let result = render_categories_index_page(&[cat1, cat2]).into_string();

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

        let mut category = create_category(vec![&post1, &post2]);
        category.metadata.title = "Supervillainy".to_owned();
        category.description_html = "<p>Good business!</p>".to_owned();

        // when
        let result = render_category_page(&category).into_string();

        // then
        println!("Checking rendered html:\n{}", result);
        assert!(result.contains("<h2 class=\"section-heading\">Kategorie: Supervillainy</h2>"));
        assert!(result.contains("<p>Good business!</p>"));
        assert!(result.contains("<h3>Diese Kategorie hat 2 Einträge</h3>"));
        assert!(result.contains("<ul><li><a href=\"/blogposts/sprvlln\">How to be a supervillain</a></li><li><a href=\"/blogposts/caught\">How not to get caught</a></li></ul>"))
    }
}
