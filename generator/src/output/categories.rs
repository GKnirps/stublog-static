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

use std::cmp::max;
use std::fs::create_dir;
use std::io::Write;
use std::iter::once;
use std::path::Path;

use super::file::open_for_write;
use super::html;
use crate::input::{Assets, Blogpost, Category};
use crate::output::needs_any_update;
use std::time::SystemTime;

pub fn categories_with_blogposts<'a>(
    categories: &'a [Category],
    blogposts: &'a [Blogpost],
) -> Vec<(&'a Category, Vec<&'a Blogpost>)> {
    categories
        .iter()
        .map(|cat| {
            (
                cat,
                blogposts
                    .iter()
                    .filter(|post| post.category_id.as_ref() == Some(&cat.id))
                    .collect(),
            )
        })
        .collect()
}

fn newest_modification(category: &Category, posts: &[&Blogpost]) -> SystemTime {
    let newest_post = posts.iter().map(|post| post.modified_at).max();
    newest_post
        .map(|t| max(t, category.modified_at))
        .unwrap_or(category.modified_at)
}

pub fn write_category_index(
    dir: &Path,
    categories: &[(&Category, Vec<&Blogpost>)],
    assets: &Assets,
) -> std::io::Result<()> {
    if !dir.is_dir() {
        // TODO: check if the error message here is confusing
        create_dir(dir)?;
    }
    let mut filename = dir.to_path_buf();
    filename.push("index.html");

    if !needs_any_update(
        &filename,
        categories
            .iter()
            .map(|(cat, posts)| newest_modification(cat, posts))
            .chain(assets.modification_dates()),
    ) {
        return Ok(());
    }

    let mut writer = open_for_write(&filename)?;
    write!(
        writer,
        "{}",
        html::category::render_categories_index_page(categories, assets).into_string()
    )
}

pub fn write_category_pages(
    dir: &Path,
    categories: &[(&Category, Vec<&Blogpost>)],
    assets: &Assets,
) -> std::io::Result<()> {
    if !dir.is_dir() {
        // TODO: check if the error message here is confusing
        create_dir(dir)?;
    }
    for (cat, blogposts) in categories {
        write_category_page(dir, cat, blogposts, assets)?;
    }
    Ok(())
}

fn write_category_page(
    dir: &Path,
    category: &Category,
    blogposts: &[&Blogpost],
    assets: &Assets,
) -> std::io::Result<()> {
    let mut filename = dir.to_path_buf();
    filename.push(&category.filename);
    filename.set_extension("html");

    if !needs_any_update(
        &filename,
        assets
            .modification_dates()
            .iter()
            .copied()
            .chain(once(newest_modification(category, blogposts))),
    ) {
        return Ok(());
    }

    let mut writer = open_for_write(&filename)?;
    write!(
        writer,
        "{}",
        html::category::render_category_page(category, blogposts, assets).into_string()
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_blogpost, create_category};

    #[test]
    fn categories_with_blogposts_assigns_blogposts_for_categories() {
        // given
        let mut cat1 = create_category();
        cat1.id = "one".to_owned();
        let mut cat2 = create_category();
        cat2.id = "two".to_owned();

        let cats = &[cat1, cat2];

        let mut post1 = create_blogpost();
        post1.category_id = Some("one".to_owned());
        post1.title = "p1".to_owned();
        let mut post2 = create_blogpost();
        post2.category_id = Some("one".to_owned());
        post2.title = "p2".to_owned();
        let mut post3 = create_blogpost();
        post3.category_id = Some("noone".to_owned());
        post3.title = "p3".to_owned();
        let mut post4 = create_blogpost();
        post4.category_id = None;
        post4.title = "p4".to_owned();

        let posts = &[post1, post2, post3, post4];

        // when
        let result = categories_with_blogposts(cats, posts);

        // then
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0.id, "one");
        assert_eq!(result[0].1, vec!(&posts[0], &posts[1]));
        assert_eq!(result[1].0.id, "two");
        assert_eq!(result[1].1, Vec::<&Blogpost>::new());
    }
}
