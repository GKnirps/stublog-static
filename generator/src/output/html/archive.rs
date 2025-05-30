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

use maud::{Markup, html};

use super::blogpost::render_blogpost;
use super::pager::pager;
use crate::HostedFile;
use crate::input::{Assets, Blogpost, Category};
use crate::output::RenderError;
use crate::output::html::HeadData;
use crate::paths::archive_path;
use crate::urls::archive_url;
use std::collections::HashMap;

static DATE_FORMAT_GERMAN: &str = "%d.%m.%Y";

pub fn render_archive(
    blogposts: &[(&Blogpost, &Category)],
    current_page: usize,
    num_pages: usize,
    assets: &Assets,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<Markup, RenderError> {
    let description = match blogposts.len() {
        0 => "Keine Blogposts".to_string(),
        1 => format!(
            "Blogposts vom {}",
            blogposts[0].0.date.format(DATE_FORMAT_GERMAN)
        ),
        _ => format!(
            "Blogposts vom {} bis zum {}",
            blogposts[0].0.date.format(DATE_FORMAT_GERMAN),
            blogposts.last().unwrap().0.date.format(DATE_FORMAT_GERMAN)
        ),
    };
    let html_pager = pager(current_page, num_pages, &archive_path);
    let html_content = html! {
        div.blogposts {
            (html_pager)
                @for (post, cat) in blogposts {
                    (render_blogpost(post, cat, hosted_files)?)
                }
            (html_pager)
        }
    };

    Ok(super::base(
        &HeadData::new(
            &format!("Stranger Than Usual — Archiv, Seite {}", current_page + 1),
            assets,
        )
        .with_canonical_url(&archive_url(current_page))
        .with_description(Some(&description)),
        html_content,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_assets, create_blogpost, create_category};
    use chrono::{FixedOffset, TimeZone};

    #[test]
    fn render_archive_renders_blogpost_and_pager() {
        // given
        let category = create_category();
        let mut blogpost1 = create_blogpost();
        blogpost1.content_markdown = "hello".to_owned();
        blogpost1.date = FixedOffset::east_opt(3600 * 2)
            .unwrap()
            .with_ymd_and_hms(2025, 5, 11, 1, 2, 3)
            .unwrap();
        let mut blogpost2 = create_blogpost();
        blogpost2.content_markdown = "world".to_owned();
        blogpost2.date = FixedOffset::east_opt(3600 * 2)
            .unwrap()
            .with_ymd_and_hms(2025, 5, 25, 1, 2, 3)
            .unwrap();

        let current_page = 3;
        let num_pages = 10;

        let assets = create_assets();

        let hosted_files = HashMap::new();

        // when
        let result = render_archive(
            &[(&blogpost1, &category), (&blogpost2, &category)],
            current_page,
            num_pages,
            &assets,
            &hosted_files,
        )
        .expect("expected success")
        .into_string();

        // then
        println!("Checking html: {result}");
        assert!(
            result.contains("<title>Stranger Than Usual — Archiv, Seite 4</title>"),
            "Expected title with page number"
        );
        assert!(
            result.contains(
                r#"<meta name="description" content="Blogposts vom 11.05.2025 bis zum 25.05.2025">"#
            ),
            "Expected meta description with correct dates"
        );
        assert!(
            result.contains("<div class=\"entry\"><p>hello</p>\n</div>"),
            "Expected first article on page."
        );
        assert!(
            result.contains("<div class=\"entry\"><p>world</p>\n</div>"),
            "Expected second article on page."
        );
        assert!(
            result.contains("<nav class=\"pagination\">"),
            "Expected to find a pager"
        );
    }

    #[test]
    fn render_archive_renders_correctly_for_only_one_post() {
        // given
        let category = create_category();
        let mut blogpost1 = create_blogpost();
        blogpost1.content_markdown = "hello".to_owned();
        blogpost1.date = FixedOffset::east_opt(3600 * 2)
            .unwrap()
            .with_ymd_and_hms(2025, 5, 11, 1, 2, 3)
            .unwrap();

        let current_page = 3;
        let num_pages = 10;

        let assets = create_assets();

        let hosted_files = HashMap::new();

        // when
        let result = render_archive(
            &[(&blogpost1, &category)],
            current_page,
            num_pages,
            &assets,
            &hosted_files,
        )
        .expect("expected success")
        .into_string();

        // then
        println!("Checking html: {result}");
        assert!(
            result.contains("<title>Stranger Than Usual — Archiv, Seite 4</title>"),
            "Expected title with page number"
        );
        assert!(
            result.contains(r#"<meta name="description" content="Blogposts vom 11.05.2025">"#),
            "Expected meta description with correct dates"
        );
        assert!(
            result.contains("<div class=\"entry\"><p>hello</p>\n</div>"),
            "Expected first article on page."
        );
        assert!(
            result.contains("<nav class=\"pagination\">"),
            "Expected to find a pager"
        );
    }

    #[test]
    fn render_archive_renders_correctly_for_empty_posts_list() {
        // given
        let current_page = 3;
        let num_pages = 10;

        let assets = create_assets();

        let hosted_files = HashMap::new();

        // when
        let result = render_archive(&[], current_page, num_pages, &assets, &hosted_files)
            .expect("expected success")
            .into_string();

        // then
        println!("Checking html: {result}");
        assert!(
            result.contains("<title>Stranger Than Usual — Archiv, Seite 4</title>"),
            "Expected title with page number"
        );
        assert!(
            result.contains(r#"<meta name="description" content="Keine Blogposts">"#),
            "Expected meta description with correct dates"
        );
        assert!(
            result.contains("<nav class=\"pagination\">"),
            "Expected to find a pager"
        );
    }
}
