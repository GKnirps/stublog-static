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
#![forbid(unsafe_code)]

mod input;
mod output;
mod paths;
mod urls;

#[cfg(test)]
mod test_utils;

use crate::input::parser::asset::read_assets;
use crate::input::{tag::Tag, Blogpost, Category, HostedFile, HostedFileMetadata, Quote};
use crate::output::error_pages::write_404;
use camino::{Utf8Path, Utf8PathBuf};
use input::file;
use input::parser::{
    blogpost, category::parse_categories, files_index::parse_all_file_metadata, quote::parse_quotes,
};
use output::{blogposts, categories, feed, hosted_files, ngingx_cfg, quotes, tags};
use std::collections::{HashMap, HashSet};
use std::env;

struct CliParams {
    show_help: bool,
    generate_cfg: bool,
    indir: String,
    odir: String,
}

fn parse_cli_params() -> Result<CliParams, String> {
    let raw: Vec<String> = env::args().skip(1).collect();
    let show_help = raw.iter().any(|p| p == "--help");
    let generate_cfg = raw.iter().any(|p| p == "--generate-cfg");
    let mut args = raw.into_iter().filter(|p| !p.starts_with("--"));
    let indir = args
        .next()
        .ok_or_else(|| "Missing input dir param".to_owned())?;
    let odir = args
        .next()
        .ok_or_else(|| "Missing output dir param".to_owned())?;
    Ok(CliParams {
        show_help,
        generate_cfg,
        indir,
        odir,
    })
}

fn main() -> Result<(), String> {
    let params = parse_cli_params()?;

    if params.show_help {
        println!("stu generator v{}", env!("CARGO_PKG_VERSION"));
        println!("generates html and stuff for the Stranger Than Usual blog");
        println!("usage: generator [--generate-cfg] [--help] INPUT_DIR OUTPUT_DIR");
        return Ok(());
    }

    if params.generate_cfg {
        generate_config(&params.indir, &params.odir)
    } else {
        generate_blog(&params.indir, &params.odir)
    }
}

fn generate_config(indir: &str, odir: &str) -> Result<(), String> {
    let categories_indir: Utf8PathBuf = [indir, "categories"].iter().collect();
    let raw_categories = file::read_files_sorted(&categories_indir)
        .map_err(|e| format!("Failed to parse all categories: {e}"))?;
    let categories = parse_categories(&raw_categories)
        .map_err(|e| format!("Failed to parse all categories: {e}"))?;

    let hosted_files_indir: Utf8PathBuf = [indir, "files_index"].iter().collect();
    let raw_hosted_files = file::read_files_sorted(&hosted_files_indir)
        .map_err(|e| format!("Failed to read all hosted files: {e}"))?;
    let hosted_files = parse_all_file_metadata(&raw_hosted_files)
        .map_err(|e| format!("Unable to parse all file metadata: {e}"))?;

    ngingx_cfg::write_config_file(Utf8Path::new(odir), &categories, &hosted_files)
        .map_err(|e| format!("Unable to write nginx config: {e}"))
}

fn generate_blog(indir: &str, odir: &str) -> Result<(), String> {
    let asset_path: Utf8PathBuf = [odir, "assets"].iter().collect();
    let assets = read_assets(&asset_path).map_err(|e| format!("Failed to read assets: {e}"))?;

    write_404(Utf8Path::new(odir), &assets)
        .map_err(|e| format!("Failed to write 404 error: {e}"))?;

    let hosted_files_meta_indir: Utf8PathBuf = [indir, "files_index"].iter().collect();
    let raw_hosted_files_meta = file::read_files_sorted(&hosted_files_meta_indir)
        .map_err(|e| format!("Failed to read all hosted files: {e}"))?;
    let hosted_files_meta = parse_all_file_metadata(&raw_hosted_files_meta)
        .map_err(|e| format!("Unable to parse all file metadata: {e}"))?;

    let hosted_files_indir: Utf8PathBuf = [indir, "file"].iter().collect();
    let hosted_files = input::hosted_files::list_all_files(&hosted_files_indir)
        .map_err(|e| format!("Unable to list all hosted files:{e}"))?;

    let (hosted_file_pairs, hosted_files_by_name) =
        match_hosted_files(&hosted_files_meta, &hosted_files)?;

    let categories_indir: Utf8PathBuf = [indir, "categories"].iter().collect();
    let raw_categories = file::read_files_sorted(&categories_indir)
        .map_err(|e| format!("Failed to parse all categories: {e}"))?;
    let categories = parse_categories(&raw_categories)
        .map_err(|e| format!("Failed to parse all categories: {e}"))?;
    check_duplicate_categories(&categories)?;

    let blogpost_indir: Utf8PathBuf = [indir, "blogposts"].iter().collect();
    let raw_blogposts = file::read_files_sorted(&blogpost_indir)
        .map_err(|e| format!("Failed to read all blogposts: {e}"))?;
    let blogposts = blogpost::parse_blogposts(&raw_blogposts)
        .map_err(|e| format!("Failed to parse all blogposts: {e}"))?;
    check_duplicate_blogpost_names(&blogposts)?;

    let quotes_indir: Utf8PathBuf = [indir, "quotes"].iter().collect();
    let raw_quotes = file::read_files_sorted(&quotes_indir)
        .map_err(|e| format!("failed to read all quotes: {e}"))?;
    let published_quotes =
        parse_quotes(&raw_quotes).map_err(|e| format!("failed parse all quotes: {e}"))?;
    check_duplicate_quote_names(&published_quotes)?;

    let categorized_blogposts =
        blogposts::find_categories_for_blogposts(&blogposts, &categories)
            .map_err(|e| format!("Error while matching blogpost with categories: {e}"))?;
    let blogpost_dir: Utf8PathBuf = [odir, "blogposts"].iter().collect();
    blogposts::write_blogposts(
        &blogpost_dir,
        &categorized_blogposts,
        &assets,
        &hosted_files_by_name,
    )
    .map_err(|e| format!("Failed to write all blogposts: {e}"))?;

    let archive_dir: Utf8PathBuf = [odir, "archive"].iter().collect();
    blogposts::write_archive(
        &archive_dir,
        &categorized_blogposts,
        &assets,
        &hosted_files_by_name,
    )
    .map_err(|e| format!("Failed to write archive: {e}"))?;
    blogposts::write_home(
        Utf8Path::new(odir),
        &categorized_blogposts,
        published_quotes.last(),
        &assets,
        &hosted_files_by_name,
    )
    .map_err(|e| format!("Failed to write home page: {e}"))?;

    feed::write_atom_feed(Utf8Path::new(odir), &blogposts, &hosted_files_by_name)?;

    let quote_dir: Utf8PathBuf = [odir, "quote"].iter().collect();
    quotes::write_quote_pages(
        &quote_dir,
        &published_quotes,
        &assets,
        &hosted_files_by_name,
    )
    .map_err(|e| format!("Unable to write all quote pages: {e}"))?;
    let quote_list_dir: Utf8PathBuf = [odir, "quotes"].iter().collect();
    quotes::write_quote_list_pages(
        &quote_list_dir,
        &published_quotes,
        &assets,
        &hosted_files_by_name,
    )
    .map_err(|e| format!("Unable to write all quote lists: {e}"))?;
    quotes::write_quote_fortune_file(&quote_list_dir, &published_quotes)
        .map_err(|e| format!("Unable to write quote fortune file: {e}"))?;

    let post_by_tags = tags::blogpost_by_tag(&blogposts);
    check_index_tag(&post_by_tags)?;
    let tags_dir: Utf8PathBuf = [odir, "tags"].iter().collect();
    tags::write_tag_index(&tags_dir, &post_by_tags, &assets)
        .map_err(|e| format!("Failed to write tag index page: {e}"))?;
    tags::write_tag_pages(&tags_dir, &post_by_tags, &assets)
        .map_err(|e| format!("Failed to write tag pages: {e}"))?;

    let category_dir: Utf8PathBuf = [odir, "categories"].iter().collect();
    let categories_with_posts = categories::categories_with_blogposts(&categories, &blogposts);
    categories::write_category_index(&category_dir, &categories_with_posts, &assets)
        .map_err(|e| format!("Failed to write category index page: {e}"))?;
    categories::write_category_pages(
        &category_dir,
        &categories_with_posts,
        &assets,
        &hosted_files_by_name,
    )
    .map_err(|e| format!("Failed to write all category pages: {e}"))?;

    let hosted_files_index_dir: Utf8PathBuf = [odir, "files_metadata"].iter().collect();
    hosted_files::write_hosted_file_index_pages(
        &hosted_files_index_dir,
        &hosted_file_pairs,
        &assets,
    )
    .map_err(|e| format!("Unable to write all file metadata pages: {e}"))
}

fn check_duplicate_blogpost_names(posts: &[Blogpost]) -> Result<(), String> {
    let mut seen: HashSet<&Utf8Path> = HashSet::with_capacity(posts.len());
    for post in posts {
        if seen.contains(&post.filename.as_path()) {
            return Err(format!("Blogpost name {} is a duplicate!", post.filename));
        }
        seen.insert(&post.filename);
    }
    Ok(())
}

// right now, a tag named "index" would not be supported, as we use index.html for the list of all tags
// we may need a different way to handle this some time, but for now this is just an illegal case
// that must be caught
fn check_index_tag(post_by_tags: &HashMap<&Tag, Vec<&Blogpost>>) -> Result<(), String> {
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
    let mut seen: HashSet<&Utf8Path> = HashSet::with_capacity(categories.len());
    for cat in categories {
        if seen.contains(&cat.filename.as_path()) {
            return Err(format!(
                "Category filename {} is a duplicate!",
                cat.filename
            ));
        }
        seen.insert(cat.filename.as_path());
    }

    if seen.contains(&Utf8Path::new("index")) {
        return Err("'index' must not be a category path name.".to_owned());
    }

    Ok(())
}

fn check_duplicate_quote_names(quotes: &[Quote]) -> Result<(), String> {
    let mut seen: HashSet<&Utf8Path> = HashSet::with_capacity(quotes.len());
    for quote in quotes {
        if seen.contains(&quote.filename.as_path()) {
            return Err(format!("Quote name {} is a duplicate!", quote.filename));
        }
        seen.insert(&quote.filename);
    }
    Ok(())
}

type MatchedFiles<'meta, 'file> = (
    Vec<(&'meta HostedFileMetadata, &'file HostedFile)>,
    HashMap<&'file str, &'file HostedFile>,
);
fn match_hosted_files<'meta, 'file>(
    hosted_files_meta: &'meta [HostedFileMetadata],
    hosted_files: &'file [HostedFile],
) -> Result<MatchedFiles<'meta, 'file>, String> {
    let mut seen: HashSet<&str> = HashSet::with_capacity(hosted_files_meta.len());
    for hosted_file in hosted_files_meta {
        if seen.contains(&hosted_file.path as &str) {
            return Err(format!(
                "There is more than one metadata file for hosted file {}",
                hosted_file.path
            ));
        }
        seen.insert(&hosted_file.path);
    }
    let files_by_name: HashMap<&str, &HostedFile> = hosted_files
        .iter()
        .map(|f| (f.filename.as_str(), f))
        .collect();

    let mut no_metadata_files = files_by_name
        .keys()
        .filter(|f| !seen.contains(*f))
        .peekable();
    if no_metadata_files.peek().is_some() {
        return Err(format!(
            "The following file(s) are present, but not referenced by metadata: {}",
            no_metadata_files.copied().collect::<Vec<&str>>().join(", ")
        ));
    }

    let result: Vec<(&HostedFileMetadata, &HostedFile)> = hosted_files_meta
        .iter()
        .map(|fm| {
            files_by_name
                .get(&fm.path as &str)
                .ok_or_else(|| {
                    format!(
                        "Found metadata for file '{}' but not the actual file",
                        fm.path
                    )
                })
                .map(|f| (fm, *f))
        })
        .collect::<Result<Vec<(&HostedFileMetadata, &HostedFile)>, String>>()?;

    if hosted_files.len() != hosted_files_meta.len() {
        return Err(format!(
            "Number of hosted files does not match number of metadata files ({} vs {})",
            hosted_files.len(),
            hosted_files_meta.len()
        ));
    }

    Ok((result, files_by_name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::tag::Tag;
    use crate::test_utils::{
        create_blogpost, create_category, create_hosted_file, create_hosted_file_metadata,
        create_quote,
    };

    #[test]
    fn check_duplicate_blogposts_names_returns_ok_for_no_duplicates() {
        // given
        let mut post1 = create_blogpost();
        post1.filename = Utf8PathBuf::from("foobar");
        let mut post2 = create_blogpost();
        post2.filename = Utf8PathBuf::from("foo");
        let mut post3 = create_blogpost();
        post3.filename = Utf8PathBuf::from("bar");

        // when
        let result = check_duplicate_blogpost_names(&[post1, post2, post3]);

        // then
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn check_duplicate_blogposts_names_returns_error_for_duplicates() {
        // given
        let mut post1 = create_blogpost();
        post1.filename = Utf8PathBuf::from("foobar");
        let mut post2 = create_blogpost();
        post2.filename = Utf8PathBuf::from("foo");
        let mut post3 = create_blogpost();
        post3.filename = Utf8PathBuf::from("foobar");

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
        let dummy_post = create_blogpost();
        let footag = Tag::new("foobar");
        let bartag = Tag::new("barfoo");
        let mut tags: HashMap<&Tag, Vec<&Blogpost>> = HashMap::with_capacity(10);
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
        let dummy_post = create_blogpost();
        let mut tags: HashMap<&Tag, Vec<&Blogpost>> = HashMap::with_capacity(10);
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
        cat.filename = Utf8Path::new("index").to_path_buf();

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
        cat1.filename = Utf8Path::new("cat1").to_path_buf();
        let mut cat2 = create_category();
        cat2.filename = Utf8Path::new("cat1").to_path_buf();

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
        cat1.filename = Utf8Path::new("cat1").to_path_buf();
        let mut cat2 = create_category();
        cat2.filename = Utf8Path::new("cat2").to_path_buf();

        // when
        let result = check_duplicate_categories(&[cat1, cat2]);

        // then
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn check_duplicate_quote_names_returns_ok_for_no_duplicates() {
        // given
        let mut quote1 = create_quote();
        quote1.filename = Utf8PathBuf::from("foobar");
        let mut quote2 = create_quote();
        quote2.filename = Utf8PathBuf::from("foo");
        let mut quote3 = create_quote();
        quote3.filename = Utf8PathBuf::from("bar");

        // when
        let result = check_duplicate_quote_names(&[quote1, quote2, quote3]);

        // then
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn check_duplicate_quote_names_returns_error_for_duplicates() {
        // given
        let mut quote1 = create_quote();
        quote1.filename = Utf8PathBuf::from("foobar");
        let mut quote2 = create_quote();
        quote2.filename = Utf8PathBuf::from("foo");
        let mut quote3 = create_quote();
        quote3.filename = Utf8PathBuf::from("foobar");

        // when
        let result = check_duplicate_quote_names(&[quote1, quote2, quote3]);

        // then
        assert_eq!(result, Err("Quote name foobar is a duplicate!".to_owned()));
    }

    #[test]
    fn match_hosted_files_matches_files_with_metadata() {
        // given
        let mut metadata1 = create_hosted_file_metadata();
        metadata1.path = "file1".to_string();
        let mut file1 = create_hosted_file();
        file1.filename = Utf8PathBuf::from("file1");

        let mut metadata2 = create_hosted_file_metadata();
        metadata2.path = "file2".to_string();
        let mut file2 = create_hosted_file();
        file2.filename = Utf8PathBuf::from("file2");

        let metadata = &[metadata1, metadata2];
        let files = &[file2, file1];

        // when
        let result = match_hosted_files(metadata, files).expect("Expected successful match");

        // then
        assert_eq!(
            &result.0,
            &[(&metadata[0], &files[1]), (&metadata[1], &files[0])]
        );
        assert_eq!(
            result.1,
            [("file1", &files[1]), ("file2", &files[0])]
                .into_iter()
                .collect()
        )
    }

    #[test]
    fn match_hosted_files_fails_for_unmatched_metadata() {
        // given
        let metadata = &mut [create_hosted_file_metadata(), create_hosted_file_metadata()];
        metadata[1].path = "file".to_string();
        metadata[0].path = "unmatched".to_string();
        let file = &mut [create_hosted_file()];
        file[0].filename = Utf8PathBuf::from("file");

        // when
        let result = match_hosted_files(metadata, file);

        // then
        assert_eq!(
            result,
            Err("Found metadata for file 'unmatched' but not the actual file".to_string())
        );
    }

    #[test]
    fn match_hosted_files_fails_for_unmatched_file() {
        // given
        let metadata = &mut [create_hosted_file_metadata()];
        metadata[0].path = "file".to_string();
        let files = &mut [create_hosted_file(), create_hosted_file()];
        files[1].filename = Utf8PathBuf::from("unmatched");
        files[0].filename = Utf8PathBuf::from("file");

        // when
        let result = match_hosted_files(metadata, files);

        // then
        assert_eq!(
            result,
            Err(
                "The following file(s) are present, but not referenced by metadata: unmatched"
                    .to_string()
            )
        );
    }
}
