use std::fs::create_dir;
use std::io::Write;
use std::path::Path;

use super::file::open_for_write;
use super::html;
use crate::input;
use crate::input::parser;

pub struct Blogpost {
    pub metadata: input::BlogpostMetadata,
    pub content_html: String,
}

pub fn parse_blogposts(inputs: &[String]) -> Result<Vec<Blogpost>, parser::ParseError> {
    // TODO: it would be more helpful if we knew which blogpost failed to parse
    inputs
        .iter()
        .map(|input| parser::blogpost::parse_blogpost(&input))
        .map(|parse_result| {
            parse_result.map(|(metadata, content)| Blogpost {
                metadata,
                content_html: super::render_cmark(content),
            })
        })
        .collect()
}

pub fn write_blogposts(dir: &Path, posts: &[Blogpost]) -> std::io::Result<()> {
    if !dir.is_dir() {
        // TODO: check if the error message here is confusing
        create_dir(dir)?;
    }
    for blogpost in posts {
        // TODO: it would be more helpful if we knew which blogpost failed
        write_blogpost(dir, &blogpost)?;
    }
    Ok(())
}

fn write_blogpost(dir: &Path, blogpost: &Blogpost) -> std::io::Result<()> {
    let mut filename = dir.to_path_buf();
    filename.push(&blogpost.metadata.filename);
    filename.set_extension("html");
    let mut writer = open_for_write(&filename)?;
    write!(
        writer,
        "{}",
        html::blogpost::render_blogpost_page(blogpost).into_string()
    )
}

pub fn write_home(dir: &Path, all_posts: &[Blogpost]) -> std::io::Result<()> {
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

    let mut writer = open_for_write(&filename)?;
    write!(writer, "{}", html::home::render_home(posts).into_string())
}

pub fn write_archive(dir: &Path, all_posts: &[Blogpost]) -> std::io::Result<()> {
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
    posts: &[Blogpost],
    page_index: usize,
    num_pages: usize,
) -> std::io::Result<()> {
    let mut filename = dir.to_path_buf();
    filename.push(format!("{}", page_index));
    filename.set_extension("html");

    let mut writer = open_for_write(&filename)?;

    write!(
        writer,
        "{}",
        html::archive::render_archive(posts, page_index, num_pages).into_string()
    )
}
