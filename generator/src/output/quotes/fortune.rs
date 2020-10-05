use crate::input::Quote;
use std::io::Write;

// TODO: check if markdown looks nice in quotes (especially linebreaks via \)
// TODO: check if percent characters in quotes make problems
// TODO: check linebreaks at the end of each quote
pub fn write_fortune_quotes(writer: &mut dyn Write, quotes: &[Quote]) -> std::io::Result<()> {
    if let Some(quote) = quotes.first() {
        writeln!(writer, "{}", quote.content_markdown)?;
    }
    for quote in quotes.iter().skip(1) {
        writeln!(writer, "%\n{}", quote.content_markdown)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_quote;

    #[test]
    fn write_fortune_quotes_writes_quotes_separated_by_percent() {
        // given
        let mut q1 = create_quote();
        q1.content_markdown = "Quote #1".to_owned();
        let mut q2 = create_quote();
        q2.content_markdown = "Quote #2".to_owned();

        let mut buffer: Vec<u8> = Vec::with_capacity(100);

        // when
        write_fortune_quotes(&mut buffer, &[q1, q2]).expect("Expected successful write");
        let result = String::from_utf8(buffer).expect("Expected valid utf-8");

        // then
        assert_eq!(result, "Quote #1\n%\nQuote #2\n");
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
