use std::cmp::max;
use std::fs::create_dir;
use std::io::Write;
use std::path::Path;

use super::file::open_for_write;
use super::html;
use crate::input;
use crate::input::file::FileData;
use crate::input::Category;
use crate::input::{parser, Quote};
use crate::output::needs_update;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Blogpost {
    pub metadata: input::BlogpostMetadata,
    pub content_html: String,
}

// TODO: test this
pub fn parse_blogposts(inputs: &[FileData]) -> Result<Vec<Blogpost>, parser::ParseError> {
    inputs
        .iter()
        .map(|input| parser::blogpost::parse_blogpost(&input))
        .map(|parse_result| {
            parse_result.map(|(metadata, content)| {
                let allow_html = metadata.allow_html;
                Blogpost {
                    metadata,
                    content_html: super::cmark::render_cmark(content, allow_html),
                }
            })
        })
        .collect()
}

// find categories for all blogposts with a category id. If the ID is present but matches no
// known category, return an error
pub fn find_categories_for_blogposts<'a>(
    blogposts: &'a [Blogpost],
    categories: &'a [Category],
) -> Result<Vec<(&'a Blogpost, Option<&'a Category>)>, parser::ParseError> {
    blogposts
        .iter()
        .map(|post| match &post.metadata.category_id {
            Some(id) => categories
                .iter()
                // there should be only a small amount of categories, so we may search through all of them
                .find(|cat| &cat.id == id)
                .map(|cat| (post, Some(cat)))
                .ok_or_else(|| {
                    parser::ParseError::new(format!(
                        "Unable to find category '{}' for blogpost '{}'",
                        id, post.metadata.title
                    ))
                }),
            None => Ok((post, None)),
        })
        .collect()
}

pub fn write_blogposts(
    dir: &Path,
    posts: &[(&Blogpost, Option<&Category>)],
) -> std::io::Result<()> {
    if !dir.is_dir() {
        // TODO: check if the error message here is confusing
        create_dir(dir)?;
    }
    for (blogpost, category) in posts {
        // TODO: it would be more helpful if we knew which blogpost failed
        write_blogpost(dir, blogpost, *category)?;
    }
    Ok(())
}

fn write_blogpost(
    dir: &Path,
    blogpost: &Blogpost,
    category: Option<&Category>,
) -> std::io::Result<()> {
    let mut filename = dir.to_path_buf();
    filename.push(&blogpost.metadata.filename);
    filename.set_extension("html");
    if !needs_update(&filename, blogpost.metadata.modified_at) {
        // target file is newer, no update needed
        return Ok(());
    }
    let mut writer = open_for_write(&filename)?;
    write!(
        writer,
        "{}",
        html::blogpost::render_blogpost_page(blogpost, category).into_string()
    )
}

fn blogposts_with_categories_need_update(
    target_file: &Path,
    posts: &[(&Blogpost, Option<&Category>)],
    quote: Option<&Quote>,
) -> bool {
    // get the newest modification date of all blogposts and categories here, only update if the
    // target file is older
    posts
        .iter()
        .map(|(post, cat)| {
            cat.map(|c| max(c.modified_at, post.metadata.modified_at))
                .unwrap_or(post.metadata.modified_at)
        })
        .chain(quote.map(|q| q.modified_at))
        .max()
        .map(|modified_at| needs_update(target_file, modified_at))
        .unwrap_or(true)
}

pub fn write_home(
    dir: &Path,
    all_posts: &[(&Blogpost, Option<&Category>)],
    qotd: Option<&Quote>,
) -> std::io::Result<()> {
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

    if !blogposts_with_categories_need_update(&filename, posts, qotd) {
        return Ok(());
    }

    let mut writer = open_for_write(&filename)?;
    write!(
        writer,
        "{}",
        html::home::render_home(posts, qotd).into_string()
    )
}

pub fn write_archive(
    dir: &Path,
    all_posts: &[(&Blogpost, Option<&Category>)],
) -> std::io::Result<()> {
    if !dir.is_dir() {
        create_dir(dir)?;
    }

    let chunk_size: usize = 15;
    let num_chunks = all_posts.len() / chunk_size
        + if all_posts.len() % chunk_size == 0 {
            0
        } else {
            1
        };

    for (index, chunk) in all_posts.chunks(chunk_size).enumerate() {
        // TODO: it would be more helpful if we knew which chunk failed
        write_archive_page(dir, chunk, index, num_chunks)?;
    }

    Ok(())
}

fn write_archive_page(
    dir: &Path,
    posts: &[(&Blogpost, Option<&Category>)],
    page_index: usize,
    num_pages: usize,
) -> std::io::Result<()> {
    let mut filename = dir.to_path_buf();
    filename.push(format!("{}", page_index));
    filename.set_extension("html");

    if !blogposts_with_categories_need_update(&filename, posts, None) {
        return Ok(());
    }

    let mut writer = open_for_write(&filename)?;

    write!(
        writer,
        "{}",
        html::archive::render_archive(posts, page_index, num_pages).into_string()
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::parser::ParseError;
    use crate::test_utils::{create_blogpost, create_category};

    #[test]
    fn find_categories_for_blogposts_should_return_no_category_for_blogposts_without_category() {
        // given
        let mut post = create_blogpost();
        post.metadata.category_id = None;
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
        post1.metadata.category_id = Some("cat2".to_owned());
        let mut post2 = create_blogpost();
        post2.metadata.category_id = Some("cat1".to_owned());
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
        post.metadata.category_id = Some("cat2".to_owned());
        post.metadata.title = "post".to_owned();
        let posts = &[post];

        let mut cat = create_category();
        cat.id = "cat1".to_owned();

        let cats = &[cat];

        // when
        let result = find_categories_for_blogposts(posts, cats);

        // then
        assert_eq!(
            result,
            Err(ParseError::from(
                "Unable to find category 'cat2' for blogpost 'post'"
            ))
        );
    }
}
