use pulldown_cmark::{html::push_html, Event, Parser};

pub fn render_cmark(input: &str, allow_html: bool) -> String {
    let parser = Parser::new(input);
    let mut buf = String::with_capacity(input.len() * 2);
    if !allow_html {
        push_html(
            &mut buf,
            parser.map(|event| match event {
                Event::Html(html) => Event::Text(html),
                _ => event,
            }),
        )
    } else {
        push_html(&mut buf, parser);
    }
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_cmark_should_escape_html_if_required() {
        // given
        let markdown = "<a href=\"https://f.oo\">bar</a>";

        // when
        let html = render_cmark(markdown, false);

        // then
        assert_eq!(
            html,
            "<p>&lt;a href=&quot;https://f.oo&quot;&gt;bar&lt;/a&gt;</p>\n"
        );
    }

    #[test]
    fn render_cmark_should_not_escape_html_if_required() {
        // given
        let markdown = "<a href=\"https://f.oo\">bar</a>";

        // when
        let html = render_cmark(markdown, true);

        // then
        assert_eq!(html, "<p><a href=\"https://f.oo\">bar</a></p>\n");
    }
}
