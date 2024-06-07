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
use camino::{Utf8Path, Utf8PathBuf};
use std::cmp::max;
use std::collections::HashMap;
use std::fs::create_dir;
use std::io::Write;
use std::iter::once;

use super::file::open_for_write;
use super::html;
use crate::input::{Assets, Quote};
use crate::input::{Blogpost, Category};
use crate::output::{needs_any_update, OutputError, RenderError};

// find categories for all blogposts with a category id. If the ID is present but matches no
// known category, return an error
pub fn find_categories_for_blogposts<'a>(
    blogposts: &'a [Blogpost],
    categories: &'a [Category],
) -> Result<Vec<(&'a Blogpost, &'a Category)>, RenderError> {
    blogposts
        .iter()
        .map(|post| {
            categories
                .iter()
                // there should be only a small amount of categories, so we may search through all of them
                .find(|cat| cat.id == post.category_id)
                .map(|cat| (post, cat))
                .ok_or_else(|| {
                    RenderError::new(format!(
                        "Unable to find category '{}' for blogpost '{}'",
                        post.category_id, post.title
                    ))
                })
        })
        .collect()
}

pub fn write_blogposts(
    dir: &Utf8Path,
    posts: &[(&Blogpost, &Category)],
    assets: &Assets,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<(), OutputError> {
    if !dir.is_dir() {
        // TODO: check if the error message here is confusing
        create_dir(dir)?;
    }
    for (blogpost, category) in posts {
        // TODO: it would be more helpful if we knew which blogpost failed
        write_blogpost(dir, blogpost, category, assets, hosted_files)?;
    }
    Ok(())
}

fn write_blogpost(
    dir: &Utf8Path,
    blogpost: &Blogpost,
    category: &Category,
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
    writer.flush().map_err(OutputError::from)
}

fn blogposts_with_categories_need_update(
    target_file: &Utf8Path,
    posts: &[(&Blogpost, &Category)],
    quote: Option<&Quote>,
    assets: &Assets,
) -> bool {
    // get the newest modification date of all blogposts and categories here, only update if the
    // target file is older
    needs_any_update(
        target_file,
        posts
            .iter()
            .map(|(post, cat)| max(cat.modified_at, post.modified_at))
            .chain(quote.map(|q| q.modified_at))
            .chain(assets.modification_dates()),
    )
}

pub fn write_home(
    dir: &Utf8Path,
    all_posts: &[(&Blogpost, &Category)],
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
    writer.flush().map_err(OutputError::from)
}

pub fn write_archive(
    dir: &Utf8Path,
    all_posts: &[(&Blogpost, &Category)],
    assets: &Assets,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<(), OutputError> {
    if !dir.is_dir() {
        create_dir(dir)?;
    }

    let chunk_size: usize = 15;
    let num_chunks = all_posts.len() / chunk_size + usize::from(all_posts.len() % chunk_size != 0);

    // we need this because when another page is added, all previous pages need to update because the
    // pager needs to include the new page
    let index_updated = if num_chunks > 0 {
        !index_path(dir, num_chunks - 1).exists()
    } else {
        true
    };

    for (index, chunk) in all_posts.chunks(chunk_size).enumerate() {
        // TODO: it would be more helpful if we knew which chunk failed
        write_archive_page(
            dir,
            chunk,
            index,
            num_chunks,
            assets,
            hosted_files,
            index_updated,
        )?;
    }

    Ok(())
}

fn index_path(dir: &Utf8Path, page_index: usize) -> Utf8PathBuf {
    let mut filename = dir.to_path_buf();
    filename.push(format!("{page_index}"));
    filename.set_extension("html");
    filename
}

fn write_archive_page(
    dir: &Utf8Path,
    posts: &[(&Blogpost, &Category)],
    page_index: usize,
    num_pages: usize,
    assets: &Assets,
    hosted_files: &HashMap<&str, &HostedFile>,
    index_updated: bool,
) -> Result<(), OutputError> {
    let filename = index_path(dir, page_index);

    // FIXME: blogposts may need an update if the relevant hosted files changed
    if !blogposts_with_categories_need_update(&filename, posts, None, assets) && !index_updated {
        return Ok(());
    }

    let mut writer = open_for_write(&filename)?;

    write!(
        writer,
        "{}",
        html::archive::render_archive(posts, page_index, num_pages, assets, hosted_files)?
            .into_string()
    )?;
    writer.flush().map_err(OutputError::from)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_blogpost, create_category};

    #[test]
    fn find_categories_for_blogposts_should_return_correct_categories_for_posts() {
        // given
        let mut post1 = create_blogpost();
        post1.category_id = "cat2".to_owned();
        let mut post2 = create_blogpost();
        post2.category_id = "cat1".to_owned();
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
        assert_eq!(result[0].1.id, "cat2");
        assert_eq!(result[1].1.id, "cat1");
    }

    #[test]
    fn find_categories_for_blogposts_should_fail_if_category_does_not_exist() {
        // given
        let mut post = create_blogpost();
        post.category_id = "cat2".to_owned();
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
