/*
 * This file is part of stublog-static.
 *
 *  stublog-static is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  stublog-static is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with stublog-static. If not, see <https://www.gnu.org/licenses/>.
 */

use super::file::open_for_write;
use super::html;
use crate::input::{tag::Tag, Assets, Blogpost};
use crate::output::needs_any_update;
use camino::Utf8Path;
use std::collections::HashMap;
use std::fs::create_dir;
use std::io::Write;
use std::iter::IntoIterator;

pub fn blogpost_by_tag<'a, T>(posts: T) -> HashMap<&'a Tag, Vec<&'a Blogpost>>
where
    T: IntoIterator<Item = &'a Blogpost>,
{
    let mut result: HashMap<&'a Tag, Vec<&'a Blogpost>> = HashMap::with_capacity(1024);

    for post in posts {
        for tag in &post.tags {
            let tag_posts = result.entry(tag).or_insert_with(|| Vec::with_capacity(10));
            tag_posts.push(post)
        }
    }

    result
}

fn sort_tags<'a>(posts_by_tag: &HashMap<&'a Tag, Vec<&Blogpost>>) -> Vec<(&'a Tag, usize)> {
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
    dir: &Utf8Path,
    posts_by_tag: &HashMap<&Tag, Vec<&Blogpost>>,
    assets: &Assets,
) -> std::io::Result<()> {
    if !dir.is_dir() {
        create_dir(dir)?;
    }
    let mut filename = dir.to_path_buf();
    filename.push("index.html");

    if !needs_any_update(
        &filename,
        posts_by_tag
            .iter()
            .flat_map(|(_, posts)| posts.iter())
            .map(|p| p.modified_at)
            .chain(assets.modification_dates()),
    ) {
        return Ok(());
    }

    let tags = sort_tags(posts_by_tag);
    let mut writer = open_for_write(&filename)?;
    write!(
        writer,
        "{}",
        html::tag::render_tag_list(&tags, assets).into_string()
    )?;
    writer.into_inner()?.sync_all()
}

pub fn write_tag_pages(
    dir: &Utf8Path,
    posts_by_tag: &HashMap<&Tag, Vec<&Blogpost>>,
    assets: &Assets,
) -> std::io::Result<()> {
    if !dir.is_dir() {
        // TODO: check if the error message here is confusing
        create_dir(dir)?;
    }

    for (tag, posts) in posts_by_tag {
        // TODO: it would be more helpful if we knew which tag failed
        write_tag_page(dir, tag, posts, assets)?;
    }
    Ok(())
}

fn write_tag_page(
    dir: &Utf8Path,
    tag: &Tag,
    posts: &[&Blogpost],
    assets: &Assets,
) -> std::io::Result<()> {
    let mut filename = dir.to_path_buf();
    filename.push(&tag.normalized_name);
    filename.set_extension("html");

    if !needs_any_update(
        &filename,
        posts
            .iter()
            .map(|p| p.modified_at)
            .chain(assets.modification_dates()),
    ) {
        return Ok(());
    }

    let mut writer = open_for_write(&filename)?;
    write!(
        writer,
        "{}",
        html::tag::render_tag_page(tag, posts, assets).into_string()
    )?;
    writer.into_inner()?.sync_all()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_blogpost;

    #[test]
    fn blogpost_metadata_by_tag_should_aggregate_posts() {
        // given
        let mut post1 = create_blogpost();
        post1.tags = vec![Tag::new("foo"), Tag::new("bar")];

        let mut post2 = create_blogpost();
        post2.tags = vec![Tag::new("foo"), Tag::new("blub")];

        let mut post3 = create_blogpost();
        post3.tags = vec![];

        let posts = vec![post1.clone(), post2.clone(), post3];

        // when
        let result = blogpost_by_tag(&posts);

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
        let dummy_post = create_blogpost();

        let mut input: HashMap<&Tag, Vec<&Blogpost>> = HashMap::with_capacity(10);
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
