use super::file::open_for_write;
use super::html;
use crate::input::{BlogpostMetadata, Tag};
use std::collections::HashMap;
use std::fs::create_dir;
use std::io::Write;
use std::iter::IntoIterator;
use std::path::Path;

pub fn blogpost_metadata_by_tag<'a, T>(posts: T) -> HashMap<&'a Tag, Vec<&'a BlogpostMetadata>>
where
    T: IntoIterator<Item = &'a BlogpostMetadata>,
{
    let mut result: HashMap<&'a Tag, Vec<&'a BlogpostMetadata>> = HashMap::with_capacity(1024);

    for post in posts {
        for tag in &post.tags {
            let tag_posts = result.entry(tag).or_insert_with(|| Vec::with_capacity(10));
            tag_posts.push(post)
        }
    }

    result
}

fn sort_tags<'a>(posts_by_tag: &HashMap<&'a Tag, Vec<&BlogpostMetadata>>) -> Vec<(&'a Tag, usize)> {
    let mut tags: Vec<(&Tag, usize)> = posts_by_tag
        .iter()
        .map(|(tag, posts)| (*tag, posts.len()))
        .collect();
    tags.sort_by(|(tag1, num1), (tag2, num2)| {
        if num2 == num1 {
            tag1.name.cmp(&tag2.name)
        } else {
            num2.cmp(num1)
        }
    });

    tags
}

pub fn write_tag_index(
    dir: &Path,
    posts_by_tag: &HashMap<&Tag, Vec<&BlogpostMetadata>>,
) -> std::io::Result<()> {
    if !dir.is_dir() {
        create_dir(dir)?;
    }
    let mut filename = dir.to_path_buf();
    filename.push("index.html");

    let tags = sort_tags(posts_by_tag);
    let mut writer = open_for_write(&filename)?;
    write!(
        writer,
        "{}",
        html::tag::render_tag_list(&tags).into_string()
    )
}

pub fn write_tag_pages(
    dir: &Path,
    posts_by_tag: &HashMap<&Tag, Vec<&BlogpostMetadata>>,
) -> std::io::Result<()> {
    if !dir.is_dir() {
        // TODO: check if the error message here is confusing
        create_dir(dir)?;
    }

    for (tag, posts) in posts_by_tag {
        // TODO: it would be more helpful if we knew which tag failed
        write_tag_page(dir, *tag, posts)?;
    }
    Ok(())
}

fn write_tag_page(dir: &Path, tag: &Tag, posts: &[&BlogpostMetadata]) -> std::io::Result<()> {
    let mut filename = dir.to_path_buf();
    filename.push(&tag.normalized_name);
    filename.set_extension("html");

    let mut writer = open_for_write(&filename)?;
    write!(
        writer,
        "{}",
        html::tag::render_tag_page(tag, posts).into_string()
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
        post1.tags = vec![Tag::new("foo"), Tag::new("bar")];

        let mut post2 = create_blogpost_metadata();
        post2.tags = vec![Tag::new("foo"), Tag::new("blub")];

        let mut post3 = create_blogpost_metadata();
        post3.tags = vec![];

        let posts = vec![post1.clone(), post2.clone(), post3];

        // when
        let result = blogpost_metadata_by_tag(&posts);

        // then
        assert_eq!(result.len(), 3, "Expected 3 distinct tags");
        assert_eq!(
            result.get(&Tag::new("foo")),
            Some(&vec![&post1, &post2]),
            "Unexpected posts for tag foo"
        );
        assert_eq!(
            result.get(&Tag::new("bar")),
            Some(&vec![&post1]),
            "Unexpected post for tag bar"
        );
        assert_eq!(
            result.get(&Tag::new("blub")),
            Some(&vec![&post2]),
            "Unexpected post for tag bar"
        );
    }

    #[test]
    fn sort_tags_should_sort_tags_by_occurences_and_name() {
        // given
        let dummy_post = create_blogpost_metadata();

        let mut input: HashMap<&Tag, Vec<&BlogpostMetadata>> = HashMap::with_capacity(10);
        let tag_foo = Tag::new("foo");
        let tag_a = Tag::new("a");
        let tag_bar = Tag::new("bar");
        input.insert(&tag_foo, vec![&dummy_post]);
        input.insert(&tag_a, vec![&dummy_post, &dummy_post]);
        input.insert(&tag_bar, vec![&dummy_post, &dummy_post]);

        // when
        let result = sort_tags(&input);

        // then
        assert_eq!(
            &result,
            &[
                (&Tag::new("a"), 2),
                (&Tag::new("bar"), 2),
                (&Tag::new("foo"), 1)
            ]
        );
    }
}
