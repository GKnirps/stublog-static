use super::file::open_for_write;
use super::html;
use crate::input::parser::BlogpostMetadata;
use std::collections::HashMap;
use std::fs::create_dir;
use std::io::Write;
use std::iter::IntoIterator;
use std::path::Path;

pub fn blogpost_metadata_by_tag<'a, T>(posts: T) -> HashMap<&'a str, Vec<&'a BlogpostMetadata>>
where
    T: IntoIterator<Item = &'a BlogpostMetadata>,
{
    let mut result: HashMap<&'a str, Vec<&'a BlogpostMetadata>> = HashMap::with_capacity(1024);

    for post in posts {
        for tag in &post.tags {
            let tag_posts = result.entry(tag).or_insert_with(|| Vec::with_capacity(10));
            tag_posts.push(post)
        }
    }

    result
}

pub fn write_tag_pages(
    dir: &Path,
    posts_by_tag: &HashMap<&str, Vec<&BlogpostMetadata>>,
) -> std::io::Result<()> {
    if !dir.is_dir() {
        // TODO: check if the error message here is confusing
        create_dir(dir)?;
    }

    for (tag_name, posts) in posts_by_tag {
        // TODO: it would be more helpful if we knew which tag failed
        write_tag_page(dir, *tag_name, posts)?;
    }
    Ok(())
}

fn write_tag_page(dir: &Path, tag_name: &str, posts: &[&BlogpostMetadata]) -> std::io::Result<()> {
    let mut filename = dir.to_path_buf();
    filename.push(tag_name);
    filename.set_extension("html");

    let mut writer = open_for_write(&filename)?;
    write!(
        writer,
        "{}",
        html::tag::render_tag_page(tag_name, posts).into_string()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_blogpost_metadata;

    #[test]
    fn blogpost_metadata_by_tag_should_aggregate_posts() {
        // given
        let mut post1 = create_blogpost_metadata();
        post1.tags = vec!["foo".to_owned(), "bar".to_owned()];

        let mut post2 = create_blogpost_metadata();
        post2.tags = vec!["foo".to_owned(), "blub".to_owned()];

        let mut post3 = create_blogpost_metadata();
        post3.tags = vec![];

        let posts = vec![post1.clone(), post2.clone(), post3];

        // when
        let result = blogpost_metadata_by_tag(&posts);

        // then
        assert_eq!(result.len(), 3, "Expected 3 distinct tags");
        assert_eq!(
            result.get("foo"),
            Some(&vec![&post1, &post2]),
            "Unexpected posts for tag foo"
        );
        assert_eq!(
            result.get("bar"),
            Some(&vec![&post1]),
            "Unexpected post for tag bar"
        );
        assert_eq!(
            result.get("blub"),
            Some(&vec![&post2]),
            "Unexpected post for tag bar"
        );
    }
}
