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

use crate::input::Quote;
use std::io::Write;

// write quote in a nicer way:
// - do not render some markdown stuff such as \ for hard line breaks
// - do not parse markdown, too much of a hassle. Just assume it's looking somewhat okay-ish
fn write_fortune_quote(writer: &mut dyn Write, quote: &Quote) -> std::io::Result<()> {
    for line in quote
        .content_markdown
        .trim()
        .lines()
        .map(|l| l.trim())
        .map(|l| l.trim_end_matches('\\'))
    {
        if line != "%" {
            writeln!(writer, "{line}")?;
        } else {
            writeln!(writer, "% ")?;
        }
    }
    if let Some(source_name) = quote.source_name.as_ref() {
        writeln!(writer, "— {source_name}")
    } else {
        Ok(())
    }
}

pub fn write_fortune_quotes(writer: &mut dyn Write, quotes: &[Quote]) -> std::io::Result<()> {
    if let Some(quote) = quotes.first() {
        write_fortune_quote(writer, quote)?;
    }
    for quote in quotes.iter().skip(1) {
        writeln!(writer, "%")?;
        write_fortune_quote(writer, quote)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_quote;

    #[test]
    fn write_fortune_quote_writes_quote_with_source() {
        // given
        let mut quote = create_quote();
        quote.content_markdown = "\n\n  \nThis is a quote\n  \n".to_owned();
        quote.source_name = Some("Quoty McQuoteface".to_owned());

        let mut buffer: Vec<u8> = Vec::with_capacity(100);

        // when
        write_fortune_quote(&mut buffer, &quote).expect("Expected successful write");
        let result = String::from_utf8(buffer).expect("Expected valid utf-8");

        // then
        assert_eq!(result, "This is a quote\n— Quoty McQuoteface\n");
    }

    #[test]
    fn write_fortune_quote_writes_quote_without_source() {
        // given
        let mut quote = create_quote();
        quote.content_markdown = "\n\n  \n  This is a quote\n  \n".to_owned();
        quote.source_name = None;

        let mut buffer: Vec<u8> = Vec::with_capacity(100);

        // when
        write_fortune_quote(&mut buffer, &quote).expect("Expected successful write");
        let result = String::from_utf8(buffer).expect("Expected valid utf-8");

        // then
        assert_eq!(result, "This is a quote\n");
    }

    #[test]
    fn write_fortune_quote_trims_line_ends() {
        // given
        let mut quote = create_quote();
        quote.content_markdown =
            "\n\n  \nThis is a quote  \nwith multiple\\\nlines\n  \n".to_owned();
        quote.source_name = None;

        let mut buffer: Vec<u8> = Vec::with_capacity(100);

        // when
        write_fortune_quote(&mut buffer, &quote).expect("Expected successful write");
        let result = String::from_utf8(buffer).expect("Expected valid utf-8");

        // then
        assert_eq!(result, "This is a quote\nwith multiple\nlines\n");
    }

    #[test]
    fn write_fortune_quote_kind_of_escapes_percent_only_lines() {
        // given
        let mut quote = create_quote();
        quote.content_markdown = "Q1\n%\nQ1_2".to_owned();
        quote.source_name = None;

        let mut buffer: Vec<u8> = Vec::with_capacity(100);

        // when
        write_fortune_quote(&mut buffer, &quote).expect("Expected successful write");
        let result = String::from_utf8(buffer).expect("Expected valid utf-8");

        // then
        assert_eq!(result, "Q1\n% \nQ1_2\n");
    }

    #[test]
    fn write_fortune_quotes_writes_quotes_separated_by_percent() {
        // given
        let mut q1 = create_quote();
        q1.content_markdown = "Quote #1".to_owned();
        q1.source_name = None;
        let mut q2 = create_quote();
        q2.content_markdown = "Quote #2".to_owned();

        let mut buffer: Vec<u8> = Vec::with_capacity(100);

        // when
        write_fortune_quotes(&mut buffer, &[q1, q2]).expect("Expected successful write");
        let result = String::from_utf8(buffer).expect("Expected valid utf-8");

        // then
        assert_eq!(result, "Quote #1\n%\nQuote #2\n— Arthur Dent\n");
    }

    #[test]
    fn write_fortune_quotes_writes_nothing_for_empty_list() {
        // given
        let mut buffer: Vec<u8> = Vec::with_capacity(100);

        // when
        write_fortune_quotes(&mut buffer, &[]).expect("Expected successful write");

        // then
        assert_eq!(buffer.len(), 0);
    }
}
