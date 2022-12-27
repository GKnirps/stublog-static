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

use crate::HostedFile;
use std::cmp::max;
use std::collections::HashMap;
use std::fs::create_dir;
use std::io::Write;
use std::iter::once;
use std::path::Path;

use super::file::open_for_write;
use super::html;
use crate::input::{Assets, Quote};
use crate::input::{Blogpost, Category};
use crate::output::{needs_any_update, needs_update, OutputError, RenderError};

// find categories for all blogposts with a category id. If the ID is present but matches no
// known category, return an error
pub fn find_categories_for_blogposts<'a>(
    blogposts: &'a [Blogpost],
    categories: &'a [Category],
) -> Result<Vec<(&'a Blogpost, Option<&'a Category>)>, RenderError> {
    blogposts
        .iter()
        .map(|post| match &post.category_id {
            Some(id) => categories
                .iter()
                // there should be only a small amount of categories, so we may search through all of them
                .find(|cat| &cat.id == id)
                .map(|cat| (post, Some(cat)))
                .ok_or_else(|| {
                    RenderError::new(format!(
                        "Unable to find category '{}' for blogpost '{}'",
                        id, post.title
                    ))
                }),
            None => Ok((post, None)),
        })
        .collect()
}

pub fn write_blogposts(
    dir: &Path,
    posts: &[(&Blogpost, Option<&Category>)],
    assets: &Assets,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<(), OutputError> {
    if !dir.is_dir() {
        // TODO: check if the error message here is confusing
        create_dir(dir)?;
    }
    for (blogpost, category) in posts {
        // TODO: it would be more helpful if we knew which blogpost failed
        write_blogpost(dir, blogpost, *category, assets, hosted_files)?;
    }
    Ok(())
}

fn write_blogpost(
    dir: &Path,
    blogpost: &Blogpost,
    category: Option<&Category>,
    assets: &Assets,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<(), OutputError> {
    let mut filename = dir.to_path_buf();
    filename.push(&blogpost.filename);
    filename.set_extension("html");
    // FIXME: blogposts may need an update if the relevant hosted files changed
    if !needs_any_update(
        &filename,
        assets
            .modification_dates()
            .iter()
            .copied()
            .chain(once(blogpost.modified_at)),
    ) {
        // target file is newer, no update needed
        return Ok(());
    }
    let mut writer = open_for_write(&filename)?;
    write!(
        writer,
        "{}",
        html::blogpost::render_blogpost_page(blogpost, category, assets, hosted_files)?
            .into_string()
    )?;
    writer
        .into_inner()
        .map_err(OutputError::from)?
        .sync_all()
        .map_err(OutputError::from)
}

fn blogposts_with_categories_need_update(
    target_file: &Path,
    posts: &[(&Blogpost, Option<&Category>)],
    quote: Option<&Quote>,
    assets: &Assets,
) -> bool {
    // get the newest modification date of all blogposts and categories here, only update if the
    // target file is older
    posts
        .iter()
        .map(|(post, cat)| {
            cat.map(|c| max(c.modified_at, post.modified_at))
                .unwrap_or(post.modified_at)
        })
        .chain(quote.map(|q| q.modified_at))
        .chain(assets.modification_dates())
        .max()
        .map(|modified_at| needs_update(target_file, modified_at))
        .unwrap_or(true)
}

pub fn write_home(
    dir: &Path,
    all_posts: &[(&Blogpost, Option<&Category>)],
    qotd: Option<&Quote>,
    assets: &Assets,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<(), OutputError> {
    if !dir.is_dir() {
        create_dir(dir)?;
    }
    let mut filename = dir.to_path_buf();
    filename.push("home.html");

    let posts = if all_posts.len() < 10 {
        all_posts
    } else {
        &all_posts[all_posts.len() - 10..]
    };

    // FIXME: check if hosted files changed
    if !blogposts_with_categories_need_update(&filename, posts, qotd, assets) {
        return Ok(());
    }

    let mut writer = open_for_write(&filename)?;
    write!(
        writer,
        "{}",
        html::home::render_home(posts, qotd, assets, hosted_files)?.into_string()
    )?;
    writer
        .into_inner()
        .map_err(OutputError::from)?
        .sync_all()
        .map_err(OutputError::from)
}

pub fn write_archive(
    dir: &Path,
    all_posts: &[(&Blogpost, Option<&Category>)],
    assets: &Assets,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<(), OutputError> {
    if !dir.is_dir() {
        create_dir(dir)?;
    }

    let chunk_size: usize = 15;
    let num_chunks = all_posts.len() / chunk_size + usize::from(all_posts.len() % chunk_size == 0);

    for (index, chunk) in all_posts.chunks(chunk_size).enumerate() {
        // TODO: it would be more helpful if we knew which chunk failed
        write_archive_page(dir, chunk, index, num_chunks, assets, hosted_files)?;
    }

    Ok(())
}

fn write_archive_page(
    dir: &Path,
    posts: &[(&Blogpost, Option<&Category>)],
    page_index: usize,
    num_pages: usize,
    assets: &Assets,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<(), OutputError> {
    let mut filename = dir.to_path_buf();
    filename.push(format!("{page_index}"));
    filename.set_extension("html");

    // FIXME: blogposts may need an update if the relevant hosted files changed
    if !blogposts_with_categories_need_update(&filename, posts, None, assets) {
        return Ok(());
    }

    let mut writer = open_for_write(&filename)?;

    write!(
        writer,
        "{}",
        html::archive::render_archive(posts, page_index, num_pages, assets, hosted_files)?
            .into_string()
    )?;
    writer
        .into_inner()
        .map_err(OutputError::from)?
        .sync_all()
        .map_err(OutputError::from)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_blogpost, create_category};

    #[test]
    fn find_categories_for_blogposts_should_return_no_category_for_blogposts_without_category() {
        // given
        let mut post = create_blogpost();
        post.category_id = None;
        let posts = &[post];

        let cat = create_category();
        let cats = &[cat];

        // when
        let result = find_categories_for_blogposts(posts, cats).expect("Expected success");

        // then
        assert_eq!(result.len(), 1);
        assert!(result[0].1.is_none());
    }

    #[test]
    fn find_categories_for_blogposts_should_return_correct_categories_for_posts() {
        // given
        let mut post1 = create_blogpost();
        post1.category_id = Some("cat2".to_owned());
        let mut post2 = create_blogpost();
        post2.category_id = Some("cat1".to_owned());
        let posts = &[post1, post2];

        let mut cat1 = create_category();
        cat1.id = "cat1".to_owned();
        let mut cat2 = create_category();
        cat2.id = "cat2".to_owned();

        let cats = &[cat1, cat2];

        // when
        let result = find_categories_for_blogposts(posts, cats).expect("Expected success");

        // then
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].1.map(|cat| &cat.id), Some(&"cat2".to_owned()));
        assert_eq!(result[1].1.map(|cat| &cat.id), Some(&"cat1".to_owned()));
    }

    #[test]
    fn find_categories_for_blogposts_should_fail_if_category_does_not_exist() {
        // given
        let mut post = create_blogpost();
        post.category_id = Some("cat2".to_owned());
        post.title = "post".to_owned();
        let posts = &[post];

        let mut cat = create_category();
        cat.id = "cat1".to_owned();

        let cats = &[cat];

        // when
        let result = find_categories_for_blogposts(posts, cats);

        // then
        assert_eq!(
            result,
            Err(RenderError::from(
                "Unable to find category 'cat2' for blogpost 'post'"
            ))
        );
    }
}
