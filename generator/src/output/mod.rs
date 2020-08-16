pub mod blogposts;
pub mod categories;
mod file;
mod html;
pub mod tags;

use pulldown_cmark::{html::push_html, Parser};

fn render_cmark(input: &str) -> String {
    let parser = Parser::new(input);
    let mut buf = String::with_capacity(input.len() * 2);
    push_html(&mut buf, parser);
    buf
}
