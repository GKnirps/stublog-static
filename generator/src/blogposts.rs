use pulldown_cmark::{html::push_html, Parser};
use std::fs::{create_dir, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

use super::html;
use super::parser;

fn render_cmark(input: &str) -> String {
    let parser = Parser::new(input);
    let mut buf = String::with_capacity(input.len() * 2);
    push_html(&mut buf, parser);
    buf
}

pub struct Blogpost {
    pub metadata: parser::BlogpostMetadata,
    pub content_html: String,
}

pub fn parse_blogposts<'a>(inputs: &'a [String]) -> Result<Vec<Blogpost>, parser::ParseError> {
    // TODO: it would be more helpful if we knew which blogpost failed to parse
    inputs
        .iter()
        .map(|input| parser::parse_blogpost(&input))
        .map(|parse_result| {
            parse_result.map(|(metadata, content)| Blogpost {
                metadata,
                content_html: render_cmark(content),
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
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&filename)?;
    let mut writer = BufWriter::new(file);
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

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&filename)?;
    let mut writer = BufWriter::new(file);
    write!(writer, "{}", html::home::render_home(posts).into_string())
}
