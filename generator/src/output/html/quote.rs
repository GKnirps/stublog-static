use crate::input::Quote;
use crate::output::cmark::render_cmark;
use maud::{html, Markup, PreEscaped};

fn render_quote_source(quote: &Quote) -> Markup {
    html! {
        @if let Some(source_url) = &quote.source_url {
            @if let Some(source_name) = &quote.source_name {
                span.quote-source {
                    a href=(source_url) {
                        (source_name)
                    }
                }
            } @else {
                span.quote-source {
                    a href=(source_url) {
                        "Quelle"
                    }
                }
            }
        } @else if let Some(source_name) = &quote.source_name {
            span.quote-source {
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
        <a href=\"https://example.com/adent\">Arthur Dent</a>\
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
        assert_eq!(result, "<span class=\"quote-source\">Arthur Dent</span>")
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
        <a href=\"https://example.com/adent\">Quelle</a>\
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
        <a href=\"https://example.com/adent\">Arthur Dent</a>\
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
}
