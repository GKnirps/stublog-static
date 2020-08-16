use std::fs::create_dir;
use std::io::Write;
use std::path::Path;

use super::blogposts::Blogpost;
use super::file::open_for_write;
use super::html;
use crate::input;
use crate::input::parser;

pub struct Category<'blogpost> {
    pub metadata: input::CategoryMetadata,
    pub description_html: String,
    pub blogposts: Vec<&'blogpost Blogpost>,
}

pub fn parse_categories(inputs: &[String]) -> Result<Vec<Category>, parser::ParseError> {
    // TODO: it would be helpful if we knew which category failed to parse
    inputs
        .iter()
        .map(|input| parser::category::parse_category(&input))
        .map(|parse_result| {
            parse_result.map(|(metadata, description)| Category {
                metadata,
                description_html: super::render_cmark(description),
                // TODO: assign correct blogposts here
                blogposts: vec![],
            })
        })
        .collect()
}

pub fn write_category_index(dir: &Path, categories: &[Category]) -> std::io::Result<()> {
    if !dir.is_dir() {
        // TODO: check if the error message here is confusing
        create_dir(dir)?;
    }
    let mut filename = dir.to_path_buf();
    filename.push("index.html");

    let mut writer = open_for_write(&filename)?;
    write!(
        writer,
        "{}",
        html::category::render_categories_index_page(&categories).into_string()
    )
}

pub fn write_category_pages(dir: &Path, categories: &[Category]) -> std::io::Result<()> {
    if !dir.is_dir() {
        // TODO: check if the error message here is confusing
        create_dir(dir)?;
    }
    for cat in categories {
        write_category_page(dir, &cat)?;
    }
    Ok(())
}

fn write_category_page(dir: &Path, category: &Category) -> std::io::Result<()> {
    let mut filename = dir.to_path_buf();
    filename.push(&category.metadata.filename);
    filename.set_extension("html");

    let mut writer = open_for_write(&filename)?;
    write!(
        writer,
        "{}",
        html::category::render_category_page(&category).into_string()
    )
}
