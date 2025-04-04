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

use super::pager::pager;
use crate::input::{Assets, Quote};
use crate::output::RenderError;
use crate::output::cmark::render_cmark;
use crate::output::html::HeadData;
use crate::paths::{QUOTE_FORTUNE_PATH, quote_list_path};
use crate::{HostedFile, urls};
use maud::{Markup, PreEscaped, html};
use std::collections::HashMap;

fn render_quote_source(quote: &Quote) -> Markup {
    html! {
        @if let Some(source_url) = &quote.source_url {
            @if let Some(source_name) = &quote.source_name {
                span.quote-source {
                    "— "
                    a href=(source_url) {
                        (source_name)
                    }
                }
            } @else {
                span.quote-source {
                    "— "
                    a href=(source_url) {
                        "Quelle"
                    }
                }
            }
        } @else if let Some(source_name) = &quote.source_name {
            span.quote-source {
                "— "
                (source_name)
            }
        }
    }
}

pub fn render_quote(
    quote: &Quote,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<Markup, RenderError> {
    Ok(html! {
        div.qotd lang=[quote.language] {
            @if let Some(source_url) = &quote.source_url {
                blockquote cite=(source_url) {
                    (PreEscaped(render_cmark(&quote.content_markdown, false, hosted_files)?))
                }
            } @else {
                blockquote {
                    (PreEscaped(render_cmark(&quote.content_markdown, false, hosted_files)?))
                }
            }
            (render_quote_source(quote))
        }
    })
}

pub fn render_quote_page(
    quote: &Quote,
    assets: &Assets,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<Markup, RenderError> {
    let content = render_quote(quote, hosted_files)?;

    Ok(super::base(
        &HeadData::new("Stranger Than Usual — Zitat", assets)
            .with_canonical_url(&urls::quote_url(quote)),
        content,
    ))
}

pub fn render_quote_list_page(
    quotes: &[Quote],
    current_page: usize,
    num_pages: usize,
    assets: &Assets,
    hosted_files: &HashMap<&str, &HostedFile>,
) -> Result<Markup, RenderError> {
    let html_pager = pager(current_page, num_pages, &quote_list_path);
    let content = html! {
        h2 { "Nicht alles hier ist ein Zitat" }
        a href=(QUOTE_FORTUNE_PATH) download="strangerthanusual-fortune.tar.bz2" {
            "Zitate als fortune-Datenbank herunterladen"
        }
        div.quotes {
            (html_pager)
            @for quote in quotes {
                (render_quote(quote, hosted_files)?)
            }
            (html_pager)
        }
    };

    Ok(super::base(
        &HeadData::new(
            &format!(
                "Stranger Than Usual: Zitate Seite {} von {}",
                current_page + 1,
                num_pages
            ),
            assets,
        )
        .with_canonical_url(&urls::quote_list_url(current_page)),
        content,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::Language;
    use crate::test_utils::{create_assets, create_quote};

    #[test]
    fn render_quote_source_renders_link_with_name_if_url_and_name_are_given() {
        // given
        let quote = create_quote();

        // when
        let result = render_quote_source(&quote).into_string();

        // then
        assert_eq!(
            result,
            "<span class=\"quote-source\">\
        — <a href=\"https://example.com/adent\">Arthur Dent</a>\
        </span>"
        )
    }

    #[test]
    fn render_quote_source_renders_only_text_if_name_but_no_url_is_given() {
        // given
        let mut quote = create_quote();
        quote.source_url = None;

        // when
        let result = render_quote_source(&quote).into_string();

        // then
        assert_eq!(result, "<span class=\"quote-source\">— Arthur Dent</span>")
    }

    #[test]
    fn render_quote_source_renders_link_with_generic_text_if_url_but_no_name_is_given() {
        // given
        let mut quote = create_quote();
        quote.source_name = None;

        // when
        let result = render_quote_source(&quote).into_string();

        // then
        assert_eq!(
            result,
            "<span class=\"quote-source\">\
        — <a href=\"https://example.com/adent\">Quelle</a>\
        </span>"
        )
    }

    #[test]
    fn render_quote_source_renders_nothing_for_absent_source() {
        // given
        let mut quote = create_quote();
        quote.source_name = None;
        quote.source_url = None;

        // when
        let result = render_quote_source(&quote).into_string();

        // then
        assert_eq!(result, "");
    }

    #[test]
    fn render_quote_renders_quote() {
        // given
        let quote = create_quote();
        let hosted_files = HashMap::new();

        // when
        let result = render_quote(&quote, &hosted_files)
            .expect("expected success")
            .into_string();

        // then
        println!("Checking html:\n{result}");
        assert!(result.contains(
            "<blockquote cite=\"https://example.com/adent\"><p>\
        Ford… you're turning into a penguin. Stop it.\
        </p>\n</blockquote>"
        ));
        assert!(result.contains(
            "<span class=\"quote-source\">\
        — <a href=\"https://example.com/adent\">Arthur Dent</a>\
        </span>"
        ));
    }

    #[test]
    fn render_quote_renders_language_if_present() {
        // given
        let mut quote = create_quote();
        quote.language = Some(Language::En);
        let hosted_files = HashMap::new();

        // when
        let result = render_quote(&quote, &hosted_files)
            .expect("expected success")
            .into_string();

        // then
        println!("Checking html:\n{result}");
        assert!(result.contains(r#"<div class="qotd" lang="en">"#));
    }

    #[test]
    fn render_quote_propagates_render_error() {
        // given
        let mut quote = create_quote();
        quote.content_markdown = "![foo](/file/absent.png)".to_string();
        let hosted_files = HashMap::new();

        // when
        let result = render_quote(&quote, &hosted_files);

        // then
        let err = result.expect_err("expected error");
        assert_eq!(
            err,
            RenderError::from("did not find hosted image 'absent.png'")
        );
    }

    #[test]
    fn render_quote_renders_quote_without_cite_url_if_no_source_url_is_given() {
        // given
        let mut quote = create_quote();
        quote.source_url = None;
        let hosted_files = HashMap::new();

        // when
        let result = render_quote(&quote, &hosted_files)
            .expect("expected success")
            .into_string();

        // then
        println!("Checking html:\n{result}");
        assert!(result.contains(
            "<blockquote><p>\
        Ford… you're turning into a penguin. Stop it.\
        </p>\n</blockquote>"
        ));
    }

    #[test]
    fn render_quote_page_renders_full_html_with_quote() {
        // given
        let quote = create_quote();
        let assets = create_assets();
        let hosted_files = HashMap::new();

        // when
        let result = render_quote_page(&quote, &assets, &hosted_files)
            .expect("expected success")
            .into_string();

        // then
        println!("Checking generated html:\n{result}");
        assert!(result.starts_with("<!DOCTYPE html><html lang=\"de\">"));
        assert!(result.contains(
            "<link rel=\"canonical\" href=\"https://blog.strangerthanusual.de/quote/penguin\">"
        ));
        assert!(result.contains("<blockquote"));
    }

    #[test]
    fn render_quote_list_page_renders_all_quotes_with_pager() {
        // given
        let mut quote1 = create_quote();
        quote1.content_markdown = "IM IN UR QUOTE".to_owned();
        let mut quote2 = create_quote();
        quote2.content_markdown = "SAYIN DUMB STUFF".to_owned();

        let current_page = 11;
        let num_pages = 42;

        let assets = create_assets();
        let hosted_files = HashMap::new();

        // when
        let result = render_quote_list_page(
            &[quote1, quote2],
            current_page,
            num_pages,
            &assets,
            &hosted_files,
        )
        .expect("expected success")
        .into_string();

        // then
        println!("Checking generated html:\n{result}");
        assert!(result.starts_with("<!DOCTYPE html><html lang=\"de\">"));
        assert!(result.contains("<p>IM IN UR QUOTE</p>"));
        assert!(result.contains("<p>SAYIN DUMB STUFF</p>"));
        assert!(result.contains("<nav class=\"pagination\">"));
        assert!(result.contains("<li><a href=\"/quotes/12\" title=\"Seite 13\">13</a></li>"));
    }
}
