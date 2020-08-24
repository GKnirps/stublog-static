#![feature(proc_macro_hygiene)]

use std::env::args;
use std::path::{Path, PathBuf};

mod input;
mod output;
mod paths;

#[cfg(test)]
mod test_utils;

use crate::input::{BlogpostMetadata, Category, Tag};
use input::file;
use input::parser::category::parse_categories;
use output::{blogposts, categories, tags};
use std::collections::{HashMap, HashSet};

fn main() -> Result<(), String> {
    let mut arg = args();
    let prog_name = arg.next().expect("Expected at least one argument");
    let indir = arg
        .next()
        .ok_or_else(|| format!("Usage: {} <input dir> <output dir>", prog_name))?;
    let odir = arg
        .next()
        .ok_or_else(|| format!("Usage: {} <input dir> <output dir>", prog_name))?;

    let categories_indir: PathBuf = [&indir, "categories"].iter().collect();
    let raw_categories = file::read_files_sorted(&Path::new(&categories_indir))
        .map_err(|e| format!("Failed to parse all categories: {}", e))?;
    let categories = parse_categories(&raw_categories)
        .map_err(|e| format!("Failed to parse all categories: {}", e))?;
    check_duplicate_categories(&categories)?;

    let blogpost_indir: PathBuf = [&indir, "blogposts"].iter().collect();
    let raw_blogposts = file::read_files_sorted(&blogpost_indir)
        .map_err(|e| format!("Failed to read all blogposts: {}", e))?;
    let blogposts = blogposts::parse_blogposts(&raw_blogposts)
        .map_err(|e| format!("Failed to parse all blogposts: {}", e))?;
    check_duplicate_blogpost_names(&blogposts)?;

    let categorized_blogposts =
        blogposts::find_categories_for_blogposts(&blogposts, &categories)
            .map_err(|e| format!("Error while matching blogpost with categories: {}", e))?;
    let blogpost_dir: PathBuf = [&odir, "blogposts"].iter().collect();
    blogposts::write_blogposts(&blogpost_dir, &categorized_blogposts)
        .map_err(|e| format!("Failed to write all blogposts: {}", e))?;

    let archive_dir: PathBuf = [&odir, "archive"].iter().collect();
    blogposts::write_archive(&archive_dir, &categorized_blogposts)
        .map_err(|e| format!("Failed to write archive: {}", e))?;
    blogposts::write_home(Path::new(&odir), &categorized_blogposts)
        .map_err(|e| format!("Failed to write home page: {}", e))?;

    let post_by_tags =
        tags::blogpost_metadata_by_tag(blogposts.iter().map(|blogpost| &blogpost.metadata));
    check_index_tag(&post_by_tags)?;
    let tags_dir: PathBuf = [&odir, "tags"].iter().collect();
    tags::write_tag_index(&tags_dir, &post_by_tags)
        .map_err(|e| format!("Failed to write tag index page: {}", e))?;
    tags::write_tag_pages(&tags_dir, &post_by_tags)
        .map_err(|e| format!("Failed to write tag pages: {}", e))?;

    let category_dir: PathBuf = [&odir, "categories"].iter().collect();
    let categories_with_posts = categories::categories_with_blogposts(&categories, &blogposts);
    categories::write_category_index(&category_dir, &categories_with_posts)
        .map_err(|e| format!("Failed to write category index page: {}", e))?;
    categories::write_category_pages(&category_dir, &categories_with_posts)
        .map_err(|e| format!("Failed to write all category pages: {}", e))
}

fn check_duplicate_blogpost_names(posts: &[blogposts::Blogpost]) -> Result<(), String> {
    let mut seen: HashSet<&Path> = HashSet::with_capacity(posts.len());
    for post in posts {
        if seen.contains(&post.metadata.filename.as_path()) {
            return Err(format!(
                "Blogpost name {} is a duplicate!",
                post.metadata.filename.to_string_lossy()
            ));
        }
        seen.insert(&post.metadata.filename);
    }
    Ok(())
}

// right now, a tag named "index" would not be supported, as we use index.html for the list of all tags
// we may need a different way to handle this some time, but for now this is just an illegal case
// that must be caught
fn check_index_tag(post_by_tags: &HashMap<&Tag, Vec<&BlogpostMetadata>>) -> Result<(), String> {
    if post_by_tags.contains_key(&Tag::new("index")) {
        Err(
            "'index' must not be a tag name. If you need index as a tag name, find another \
        file name for the tag index"
                .to_owned(),
        )
    } else {
        Ok(())
    }
}

fn check_duplicate_categories(categories: &[Category]) -> Result<(), String> {
    let mut seen: HashSet<&Path> = HashSet::with_capacity(categories.len());
    for cat in categories {
        if seen.contains(&cat.filename.as_path()) {
            return Err(format!(
                "Category filename {} is a duplicate!",
                cat.filename.to_string_lossy()
            ));
        }
        seen.insert(&cat.filename.as_path());
    }

    if seen.contains(&Path::new("index")) {
        return Err("'index' must not be a category path name.".to_owned());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_blogpost, create_blogpost_metadata, create_category};

    #[test]
    fn check_duplicate_blogposts_names_returns_ok_for_no_duplicates() {
        // given
        let mut post1 = create_blogpost();
        post1.metadata.filename = PathBuf::from("foobar");
        let mut post2 = create_blogpost();
        post2.metadata.filename = PathBuf::from("foo");
        let mut post3 = create_blogpost();
        post3.metadata.filename = PathBuf::from("bar");

        // when
        let result = check_duplicate_blogpost_names(&[post1, post2, post3]);

        // then
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn check_duplicate_blogposts_names_returns_error_for_duplicates() {
        // given
        let mut post1 = create_blogpost();
        post1.metadata.filename = PathBuf::from("foobar");
        let mut post2 = create_blogpost();
        post2.metadata.filename = PathBuf::from("foo");
        let mut post3 = create_blogpost();
        post3.metadata.filename = PathBuf::from("foobar");

        // when
        let result = check_duplicate_blogpost_names(&[post1, post2, post3]);

        // then
        assert_eq!(
            result,
            Err("Blogpost name foobar is a duplicate!".to_owned())
        );
    }

    #[test]
    fn check_index_tag_returns_ok_if_index_is_no_tag() {
        // given
        let dummy_post = create_blogpost_metadata();
        let footag = Tag::new("foobar");
        let bartag = Tag::new("barfoo");
        let mut tags: HashMap<&Tag, Vec<&BlogpostMetadata>> = HashMap::with_capacity(10);
        tags.insert(&footag, vec![&dummy_post]);
        tags.insert(&bartag, vec![&dummy_post]);

        // when
        let result = check_index_tag(&tags);

        // then
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn check_index_tag_returns_err_if_index_is_tag() {
        // given
        let dummy_post = create_blogpost_metadata();
        let mut tags: HashMap<&Tag, Vec<&BlogpostMetadata>> = HashMap::with_capacity(10);
        let footag = Tag::new("foobar");
        let indextag = Tag::new("index");
        tags.insert(&footag, vec![&dummy_post]);
        tags.insert(&indextag, vec![&dummy_post]);

        // when
        let result = check_index_tag(&tags);

        // then
        assert!(result.is_err());
    }

    #[test]
    fn check_duplicate_categories_returns_err_if_index_is_filename() {
        // given
        let mut cat = create_category();
        cat.filename = Path::new("index").to_path_buf();

        // when
        let result = check_duplicate_categories(&[cat]);

        // then
        assert_eq!(
            result,
            Err("'index' must not be a category path name.".to_owned())
        );
    }

    #[test]
    fn check_duplicate_categories_returns_err_if_filename_is_duplicate() {
        // given
        let mut cat1 = create_category();
        cat1.filename = Path::new("cat1").to_path_buf();
        let mut cat2 = create_category();
        cat2.filename = Path::new("cat1").to_path_buf();

        // when
        let result = check_duplicate_categories(&[cat1, cat2]);

        // then
        assert_eq!(
            result,
            Err("Category filename cat1 is a duplicate!".to_owned())
        );
    }

    #[test]
    fn check_duplicate_categories_returns_ok_if_no_filename_collisions_occur() {
        // given
        let mut cat1 = create_category();
        cat1.filename = Path::new("cat1").to_path_buf();
        let mut cat2 = create_category();
        cat2.filename = Path::new("cat2").to_path_buf();

        // when
        let result = check_duplicate_categories(&[cat1, cat2]);

        // then
        assert_eq!(result, Ok(()));
    }
}
