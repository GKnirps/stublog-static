use crate::input::Quote;
use crate::output::cmark::render_cmark;
use crate::urls;
use maud::{html, Markup, PreEscaped};

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

pub fn render_quote(quote: &Quote) -> Markup {
    html! {
        div.qotd {
            @if let Some(source_url) = &quote.source_url {
                blockquote cite=(source_url) {
                    (PreEscaped(render_cmark(&quote.content_markdown, false)))
                }
            } @else {
                blockquote {
                    (PreEscaped(render_cmark(&quote.content_markdown, false)))
                }
            }
            (render_quote_source(quote))
        }
    }
}

pub fn render_quote_page(quote: &Quote) -> Markup {
    let content = render_quote(quote);

    super::base(
        "Stranger Than Usual — Zitat",
        content,
        Some(&urls::quote_url(&quote)),
    )
}

pub fn render_quote_list_page(quotes: &[Quote], current_page: usize, num_pages: usize) -> Markup {
    let content = html! {
        div.quotes {
            @for quote in quotes {
                (render_quote(quote))
            }
        }
    };

    super::base(
        &format!(
            "Stranger Than Usual: Zitate Seite {} von {}",
            current_page + 1,
            num_pages
        ),
        content,
        Some(&urls::quote_list_url(current_page)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_quote;

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

        // when
        let result = render_quote(&quote).into_string();

        // then
        println!("Checking html:\n{}", result);
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
    fn render_quote_renders_quote_without_cite_url_if_no_source_url_is_given() {
        // given
        let mut quote = create_quote();
        quote.source_url = None;

        // when
        let result = render_quote(&quote).into_string();

        // then
        println!("Checking html:\n{}", result);
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

        // when
        let result = render_quote_page(&quote).into_string();

        // then
        println!("Checking generated html:\n{}", result);
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

        // when
        let result =
            render_quote_list_page(&[quote1, quote2], current_page, num_pages).into_string();

        // then
        println!("Checking generated html:\n{}", result);
        assert!(result.starts_with("<!DOCTYPE html><html lang=\"de\">"));
        assert!(result.contains("<p>IM IN UR QUOTE</p>"));
        assert!(result.contains("<p>SAYIN DUMB STUFF</p>"));
        todo!("Put a pager on the page and test it")
    }
}
