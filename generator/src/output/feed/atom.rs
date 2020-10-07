use super::super::cmark::render_blogpost_content;
use crate::input::Blogpost;
use crate::urls::{atom_feed_url, blogpost_url, CANONICAL_BASE_URL};
use chrono::{FixedOffset, TimeZone};
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use std::io::Write;

fn write_leaf<T: Write>(
    writer: &mut Writer<T>,
    name: &str,
    attributes: &[(&str, &str)],
    content: &str,
) -> Result<(), quick_xml::Error> {
    let attr_iter = attributes.iter().copied();
    writer.write_event(Event::Start(
        BytesStart::borrowed_name(name.as_bytes()).with_attributes(attr_iter),
    ))?;
    writer.write_event(Event::Text(BytesText::from_plain_str(content)))?;
    writer.write_event(Event::End(BytesEnd::borrowed(name.as_bytes())))?;
    Ok(())
}

fn write_link<T: Write>(
    writer: &mut Writer<T>,
    href: &str,
    rel: &str,
    mime_type: &str,
) -> Result<(), quick_xml::Error> {
    let attributes = &[("href", href), ("rel", rel), ("type", mime_type)];
    let attr_iter = attributes.iter().copied();
    writer.write_event(Event::Empty(
        BytesStart::borrowed_name(b"link").with_attributes(attr_iter),
    ))?;
    Ok(())
}

fn write_entry<T: Write>(
    writer: &mut Writer<T>,
    blogpost: &Blogpost,
) -> Result<(), quick_xml::Error> {
    writer.write_event(Event::Start(BytesStart::borrowed_name(b"entry")))?;

    write_leaf(
        writer,
        "id",
        &[],
        &format!(
            "tag:strangerthanusual.de,2005:Blogpost/{}",
            blogpost.filename.to_string_lossy()
        ),
    )?;
    write_leaf(writer, "title", &[], &blogpost.title)?;
    write_leaf(writer, "published", &[], &blogpost.date.to_rfc3339())?;
    let updated = blogpost
        .update_date
        .as_ref()
        .unwrap_or(&blogpost.date)
        .to_rfc3339();
    write_leaf(writer, "updated", &[], &updated)?;

    writer.write_event(Event::Start(BytesStart::borrowed_name(b"author")))?;
    write_leaf(writer, "name", &[], "Knirps")?;
    writer.write_event(Event::End(BytesEnd::borrowed(b"author")))?;

    if let Some(ref summary) = blogpost.summary {
        write_leaf(writer, "summary", &[], summary)?;
    }

    write_leaf(
        writer,
        "content",
        &[("type", "html")],
        &render_blogpost_content(&blogpost),
    )?;

    write_link(writer, &blogpost_url(&blogpost), "alternate", "text/html")?;

    writer.write_event(Event::End(BytesEnd::borrowed(b"entry")))?;
    Ok(())
}

pub fn write_feed<T: Write>(
    writer: &mut Writer<T>,
    blogposts: &[Blogpost],
) -> Result<(), quick_xml::Error> {
    let updated = blogposts
        .iter()
        .map(|post| post.update_date.as_ref().unwrap_or(&post.date))
        .max()
        .cloned()
        // if no blogposts exist, just pick some random fixed date in the past (before any
        // possible blogpost)
        .unwrap_or_else(|| {
            FixedOffset::east(3600 * 2)
                .ymd(1970, 1, 1)
                .and_hms(00, 00, 00)
        });

    writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), None)))?;
    let feed_attr = &[("xml:lang", "de"), ("xmlns", "http://www.w3.org/2005/Atom")];
    writer.write_event(Event::Start(
        BytesStart::borrowed_name(b"feed").with_attributes(feed_attr.iter().copied()),
    ))?;

    write_leaf(writer, "id", &[], "tag:strangerthanusual.de,2005:/feed")?;
    write_leaf(writer, "title", &[], "Stranger Than Usual")?;
    write_leaf(writer, "updated", &[], &updated.to_rfc3339())?;

    write_link(writer, CANONICAL_BASE_URL, "alternate", "text/html")?;
    write_link(writer, &atom_feed_url(), "self", "application/atom+xml")?;

    for post in blogposts.iter().rev() {
        write_entry(writer, post)?;
    }

    writer.write_event(Event::End(BytesEnd::borrowed(b"feed")))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_blogpost;
    use chrono::Duration;
    use std::io::Cursor;

    #[test]
    fn write_leaf_writes_leaf_with_attributes_and_content() {
        // given
        let name = "foobar";
        let attributes = &[("escape", "ar\"st"), ("key", "value")];
        let content = "Hou<|i&ni";
        let mut writer = Writer::new(Cursor::new(Vec::with_capacity(1000)));

        // when
        write_leaf(&mut writer, name, attributes, content).expect("Expected successful write");

        // then
        let result = writer.into_inner().into_inner();
        let result_str: String = String::from_utf8(result).expect("valid utf-8");

        assert_eq!(
            result_str,
            "<foobar escape=\"ar&quot;st\" key=\"value\">Hou&lt;|i&amp;ni</foobar>"
        );
    }

    #[test]
    fn write_link_writes_valid_link_tag() {
        // given
        let href = "https://blog.strangerthanusual.de/";
        let rel = "relrelrel";
        let mime_type = "text/plain";
        let mut writer = Writer::new(Cursor::new(Vec::with_capacity(100)));

        // when
        write_link(&mut writer, href, rel, mime_type).expect("Expected successful write");

        // then
        let result = writer.into_inner().into_inner();
        let result_str: String = String::from_utf8(result).expect("valid utf-8");

        assert_eq!(result_str, "<link href=\"https://blog.strangerthanusual.de/\" rel=\"relrelrel\" type=\"text/plain\"/>");
    }

    #[test]
    fn write_entry_writes_full_entry() {
        // given
        let post = create_blogpost();
        let mut writer = Writer::new(Cursor::new(Vec::with_capacity(1000)));

        // when
        write_entry(&mut writer, &post).expect("Expected successful write");

        // then
        let result = writer.into_inner().into_inner();
        let result_str: String = String::from_utf8(result).expect("valid utf-8");

        assert_eq!(result_str, "<entry>\
        <id>tag:strangerthanusual.de,2005:Blogpost/foobar</id>\
        <title>Nevermind</title>\
        <published>2020-05-11T12:13:14+02:00</published>\
        <updated>2020-05-25T12:13:14+02:00</updated>\
        <author><name>Knirps</name></author>\
        <summary>foo!</summary>\
        <content type=\"html\">&lt;p&gt;&lt;em&gt;foo&lt;/em&gt;bar&lt;/p&gt;\n</content>\
        <link href=\"https://blog.strangerthanusual.de/blogposts/foobar\" rel=\"alternate\" type=\"text/html\"/>\
        </entry>");
    }

    #[test]
    fn write_entry_uses_creation_date_if_update_date_is_missing() {
        // given
        let mut post = create_blogpost();
        post.update_date = None;
        let mut writer = Writer::new(Cursor::new(Vec::with_capacity(1000)));

        // when
        write_entry(&mut writer, &post).expect("Expected successful write");

        // then
        let result = writer.into_inner().into_inner();
        let result_str: String = String::from_utf8(result).expect("valid utf-8");

        assert!(result_str.contains("<published>2020-05-11T12:13:14+02:00</published>"));
        assert!(result_str.contains("<updated>2020-05-11T12:13:14+02:00</updated>"));
    }

    #[test]
    fn write_feed_writes_valid_feed() {
        // given
        let mut post1 = create_blogpost();
        post1.title = "p1".to_owned();
        // make sure the update date is used (and not the date) if possible,
        // by making the update date earlier than the creation date (which is not a realistic scenario)
        let latest_update = post1.date - Duration::seconds(42);
        post1.update_date = Some(latest_update.clone());
        // the other post ist earlier than the first one
        let mut post2 = create_blogpost();
        post2.title = "p2".to_owned();
        post2.update_date = None;
        post2.date = post1.date - Duration::seconds(100);

        let posts = &[post1, post2];

        let mut writer = Writer::new(Cursor::new(Vec::with_capacity(1000)));

        // when
        write_feed(&mut writer, posts).expect("Expected successful write");

        // then
        let result = writer.into_inner().into_inner();
        let result_str: String = String::from_utf8(result).expect("valid utf-8");

        assert_eq!(result_str, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
        <feed xml:lang=\"de\" xmlns=\"http://www.w3.org/2005/Atom\">\
        <id>tag:strangerthanusual.de,2005:/feed</id>\
        <title>Stranger Than Usual</title>\
        <updated>2020-05-11T12:12:32+02:00</updated>\
        <link href=\"https://blog.strangerthanusual.de\" rel=\"alternate\" type=\"text/html\"/>\
        <link href=\"https://blog.strangerthanusual.de/feed.atom\" rel=\"self\" type=\"application/atom+xml\"/>\
        <entry>\
        <id>tag:strangerthanusual.de,2005:Blogpost/foobar</id>\
        <title>p2</title>\
        <published>2020-05-11T12:11:34+02:00</published>\
        <updated>2020-05-11T12:11:34+02:00</updated>\
        <author><name>Knirps</name></author>\
        <summary>foo!</summary>\
        <content type=\"html\">&lt;p&gt;&lt;em&gt;foo&lt;/em&gt;bar&lt;/p&gt;\n</content>\
        <link href=\"https://blog.strangerthanusual.de/blogposts/foobar\" rel=\"alternate\" type=\"text/html\"/>\
        </entry><entry>\
        <id>tag:strangerthanusual.de,2005:Blogpost/foobar</id>\
        <title>p1</title>\
        <published>2020-05-11T12:13:14+02:00</published>\
        <updated>2020-05-11T12:12:32+02:00</updated>\
        <author><name>Knirps</name></author>\
        <summary>foo!</summary>\
        <content type=\"html\">&lt;p&gt;&lt;em&gt;foo&lt;/em&gt;bar&lt;/p&gt;\n</content>\
        <link href=\"https://blog.strangerthanusual.de/blogposts/foobar\" rel=\"alternate\" type=\"text/html\"/>\
        </entry>\
        </feed>");
    }

    #[test]
    fn write_feed_uses_fallback_update_date_if_no_blogposts_are_present() {
        // given
        let posts = &[];

        let mut writer = Writer::new(Cursor::new(Vec::with_capacity(1000)));

        // when
        write_feed(&mut writer, posts).expect("Expected successful write");

        // then
        let result = writer.into_inner().into_inner();
        let result_str: String = String::from_utf8(result).expect("valid utf-8");

        assert_eq!(result_str, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
        <feed xml:lang=\"de\" xmlns=\"http://www.w3.org/2005/Atom\">\
        <id>tag:strangerthanusual.de,2005:/feed</id>\
        <title>Stranger Than Usual</title>\
        <updated>1970-01-01T00:00:00+02:00</updated>\
        <link href=\"https://blog.strangerthanusual.de\" rel=\"alternate\" type=\"text/html\"/>\
        <link href=\"https://blog.strangerthanusual.de/feed.atom\" rel=\"self\" type=\"application/atom+xml\"/>\
        </feed>");
    }
}
