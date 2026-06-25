//! Parses sitemap XML into an intermediate shape.
//!
//! A sitemap document is either a `<sitemapindex>` (references to child
//! sitemaps) or a `<urlset>` (page entries). This module turns the raw XML into
//! [`Parsed`]; assembling the composite [`super::sitemap::SitemapDocument`] tree
//! (fetching each child) is the caller's job.

use std::str::FromStr;

use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use quick_xml::Reader;
use quick_xml::escape;
use quick_xml::events::{BytesRef, Event};

use super::sitemap::{ChangeFrequency, SitemapImage, SitemapUrl};

/// The two kinds of sitemap document.
#[derive(Debug, PartialEq)]
pub enum Parsed {
    /// A `<sitemapindex>`: references to child sitemap documents.
    Index(Vec<ChildRef>),
    /// A `<urlset>`: page entries.
    UrlSet(Vec<SitemapUrl>),
}

/// A child sitemap referenced by an index.
#[derive(Debug, Clone, PartialEq)]
pub struct ChildRef {
    pub location: String,
    pub last_modified: Option<DateTime<Utc>>,
}

enum Mode {
    Index,
    UrlSet,
}

#[derive(Default)]
struct ChildBuilder {
    location: Option<String>,
    last_modified: Option<DateTime<Utc>>,
}

#[derive(Default)]
struct UrlBuilder {
    location: Option<String>,
    last_modified: Option<DateTime<Utc>>,
    change_frequency: Option<ChangeFrequency>,
    priority: Option<f32>,
    images: Vec<SitemapImage>,
}

#[derive(Default)]
struct ImageBuilder {
    location: Option<String>,
    title: Option<String>,
    caption: Option<String>,
}

/// Parses sitemap XML, returning the index references or the URL entries.
pub fn parse(xml: &str) -> Result<Parsed, String> {
    let mut reader = Reader::from_str(xml);

    let mut mode: Option<Mode> = None;
    let mut children: Vec<ChildRef> = Vec::new();
    let mut urls: Vec<SitemapUrl> = Vec::new();

    let mut child: Option<ChildBuilder> = None;
    let mut url: Option<UrlBuilder> = None;
    let mut image: Option<ImageBuilder> = None;

    let mut leaf: Option<Vec<u8>> = None;
    let mut text = String::new();

    loop {
        match reader.read_event().map_err(|error| error.to_string())? {
            Event::Start(event) => match event.name().as_ref() {
                b"sitemapindex" => {
                    mode.get_or_insert(Mode::Index);
                }
                b"urlset" => {
                    mode.get_or_insert(Mode::UrlSet);
                }
                b"sitemap" => child = Some(ChildBuilder::default()),
                b"url" => url = Some(UrlBuilder::default()),
                b"image:image" => image = Some(ImageBuilder::default()),
                b"loc" | b"lastmod" | b"changefreq" | b"priority" | b"image:loc"
                | b"image:title" | b"image:caption" => {
                    leaf = Some(event.name().as_ref().to_vec());
                    text.clear();
                }
                _ => {}
            },
            Event::Text(event) if leaf.is_some() => {
                let raw = std::str::from_utf8(event.as_ref()).map_err(|error| error.to_string())?;
                text.push_str(&escape::unescape(raw).map_err(|error| error.to_string())?);
            }
            // quick-xml emits entity references (e.g. `&amp;` in a `<loc>`) as
            // their own event, separate from the surrounding text.
            Event::GeneralRef(event) if leaf.is_some() => {
                if let Some(resolved) = resolve_entity(&event)? {
                    text.push(resolved);
                }
            }
            Event::End(event) => match event.name().as_ref() {
                b"sitemap" => {
                    if let Some(builder) = child.take() {
                        if let Some(location) = builder.location {
                            children.push(ChildRef {
                                location,
                                last_modified: builder.last_modified,
                            });
                        }
                    }
                }
                b"url" => {
                    if let Some(builder) = url.take() {
                        if let Some(entry) = builder.build() {
                            urls.push(entry);
                        }
                    }
                }
                b"image:image" => {
                    if let (Some(builder), Some(parent)) = (image.take(), url.as_mut()) {
                        if let Some(location) = builder.location {
                            if !location.is_empty() {
                                let mut entry = SitemapImage::new(location);
                                entry.title = builder.title;
                                entry.caption = builder.caption;
                                parent.images.push(entry);
                            }
                        }
                    }
                }
                name if leaf.as_deref() == Some(name) => {
                    let value = text.trim().to_string();
                    assign(name, value, child.as_mut(), url.as_mut(), image.as_mut());
                    leaf = None;
                    text.clear();
                }
                _ => {}
            },
            Event::Eof => break,
            _ => {}
        }
    }

    match mode {
        Some(Mode::Index) => Ok(Parsed::Index(children)),
        Some(Mode::UrlSet) => Ok(Parsed::UrlSet(urls)),
        None => {
            Err("unrecognized sitemap document (no <sitemapindex> or <urlset> root)".to_string())
        }
    }
}

impl UrlBuilder {
    fn build(self) -> Option<SitemapUrl> {
        let location = self.location.filter(|location| !location.is_empty())?;

        let source = "xxx".to_string(); // Placeholder; the actual source should be passed in or tracked elsewhere.

        let mut entry = SitemapUrl::new(location, source);
        entry.last_modified = self.last_modified;
        entry.change_frequency = self.change_frequency;
        entry.priority = self.priority;
        entry.images = self.images;
        Some(entry)
    }
}

/// Assigns a leaf element's text to the currently open builder. Image fields win
/// when an `<image:image>` block is open, then URL fields, then index entries.
fn assign(
    name: &[u8],
    value: String,
    child: Option<&mut ChildBuilder>,
    url: Option<&mut UrlBuilder>,
    image: Option<&mut ImageBuilder>,
) {
    if let Some(image) = image {
        match name {
            b"image:loc" => image.location = Some(value),
            b"image:title" => image.title = Some(value),
            b"image:caption" => image.caption = Some(value),
            _ => {}
        }
        return;
    }
    if let Some(url) = url {
        match name {
            b"loc" => url.location = Some(value),
            b"lastmod" => url.last_modified = parse_lastmod(&value),
            b"changefreq" => url.change_frequency = ChangeFrequency::from_str(&value).ok(),
            b"priority" => url.priority = value.parse::<f32>().ok(),
            _ => {}
        }
        return;
    }
    if let Some(child) = child {
        match name {
            b"loc" => child.location = Some(value),
            b"lastmod" => child.last_modified = parse_lastmod(&value),
            _ => {}
        }
    }
}

/// Resolves an XML entity reference to its character: numeric refs (`&#38;`,
/// `&#x26;`) via quick-xml, plus the five predefined named entities. Unknown
/// names yield `None` and are dropped (sitemaps only use these).
fn resolve_entity(reference: &BytesRef) -> Result<Option<char>, String> {
    if let Some(character) = reference
        .resolve_char_ref()
        .map_err(|error| error.to_string())?
    {
        return Ok(Some(character));
    }
    let name = reference.decode().map_err(|error| error.to_string())?;
    Ok(match name.as_ref() {
        "amp" => Some('&'),
        "lt" => Some('<'),
        "gt" => Some('>'),
        "quot" => Some('"'),
        "apos" => Some('\''),
        _ => None,
    })
}

/// Parses a `<lastmod>` value best-effort: RFC 3339 first, then a bare
/// `YYYY-MM-DD` date at midnight UTC; an unparseable value yields `None` rather
/// than failing the whole document.
fn parse_lastmod(value: &str) -> Option<DateTime<Utc>> {
    let value = value.trim();
    if let Ok(parsed) = DateTime::parse_from_rfc3339(value) {
        return Some(parsed.with_timezone(&Utc));
    }
    if let Ok(date) = NaiveDate::parse_from_str(value, "%Y-%m-%d") {
        if let Some(naive) = date.and_hms_opt(0, 0, 0) {
            return Some(Utc.from_utc_datetime(&naive));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{Parsed, parse};
    use crate::sitemap_discovery::sitemap::ChangeFrequency;

    #[test]
    fn parses_index_with_lastmod() {
        let xml = r#"<sitemapindex>
            <sitemap>
                <loc>https://minisforumpc.eu/sitemap_products_1.xml</loc>
                <lastmod>2024-05-01</lastmod>
            </sitemap>
            <sitemap><loc>https://minisforumpc.eu/sitemap_collections_1.xml</loc></sitemap>
        </sitemapindex>"#;

        match parse(xml).expect("parses") {
            Parsed::Index(children) => {
                assert_eq!(children.len(), 2);
                assert_eq!(
                    children[0].location,
                    "https://minisforumpc.eu/sitemap_products_1.xml"
                );
                assert!(children[0].last_modified.is_some());
                assert!(children[1].last_modified.is_none());
            }
            other => panic!("expected index, got {other:?}"),
        }
    }

    #[test]
    fn parses_urlset_with_image_and_fields() {
        let xml = r#"<urlset xmlns:image="http://www.google.com/schemas/sitemap-image/1.1">
            <url>
                <loc>https://minisforumpc.eu/products/um890</loc>
                <lastmod>2024-05-01T10:00:00+00:00</lastmod>
                <changefreq>daily</changefreq>
                <priority>0.8</priority>
                <image:image>
                    <image:loc>https://minisforumpc.eu/img/um890.jpg</image:loc>
                    <image:title>UM890 Pro</image:title>
                </image:image>
            </url>
        </urlset>"#;

        match parse(xml).expect("parses") {
            Parsed::UrlSet(urls) => {
                assert_eq!(urls.len(), 1);
                let entry = &urls[0];
                assert_eq!(entry.location, "https://minisforumpc.eu/products/um890");
                assert!(entry.last_modified.is_some());
                assert_eq!(entry.change_frequency, Some(ChangeFrequency::Daily));
                assert_eq!(entry.priority, Some(0.8));
                assert_eq!(entry.images.len(), 1);
                assert_eq!(
                    entry.images[0].location,
                    "https://minisforumpc.eu/img/um890.jpg"
                );
                assert_eq!(entry.images[0].title.as_deref(), Some("UM890 Pro"));
            }
            other => panic!("expected urlset, got {other:?}"),
        }
    }

    #[test]
    fn rejects_unrecognized_root() {
        assert!(parse("<html><body>nope</body></html>").is_err());
    }

    #[test]
    fn decodes_amp_entity_in_loc() {
        let xml = r#"<sitemapindex><sitemap>
            <loc>https://minisforumpc.eu/sitemap_products_1.xml?from=1&amp;to=2</loc>
        </sitemap></sitemapindex>"#;
        match parse(xml).expect("parses") {
            Parsed::Index(children) => assert_eq!(
                children[0].location,
                "https://minisforumpc.eu/sitemap_products_1.xml?from=1&to=2"
            ),
            other => panic!("expected index, got {other:?}"),
        }
    }
}
