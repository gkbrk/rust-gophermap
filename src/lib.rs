//! gophermap is a Rust crate that can parse and generate Gopher responses.
//! It can be used to implement Gopher clients and servers. It doesn't handle
//! any I/O on purpose. This library is meant to be used by other servers and
//! clients in order to avoid re-implementing the gophermap logic.
//!
#![forbid(unsafe_code)]

use std::io::Write;

/// A single entry in a Gopher map. This struct can be filled in order to
/// generate Gopher responses. It can also be the result of parsing one.
pub struct GopherEntry<'a> {
    /// The type of the link
    pub item_type: ItemType,
    /// The human-readable description of the link. Displayed on the UI.
    pub display_string: &'a str,
    /// The target page (selector) of the link
    pub selector: &'a str,
    /// The host for the target of the link
    pub host: &'a str,
    /// The port for the target of the link
    pub port: u16,
}

impl<'a> GopherEntry<'a> {
    /// Parse a line into a Gopher directory entry.
    /// ```rust
    /// use gophermap::GopherEntry;
    /// let entry = GopherEntry::from("1Floodgap Home	/home	gopher.floodgap.com	70\r\n")
    ///     .unwrap();
    /// assert_eq!(entry.selector, "/home");
    /// ```
    pub fn from(line: &'a str) -> Option<Self> {
        let line = {
            let mut chars = line.chars();
            if !(chars.next_back()? == '\n' && chars.next_back()? == '\r') {
                return None;
            }
            chars.as_str()
        };

        let mut parts = line.split('\t');

        Some(GopherEntry {
            item_type: ItemType::from(line.chars().next()?),
            display_string: {
                let part = parts.next()?;
                let (index, _) = part.char_indices().skip(1).next()?;
                &part[index..]
            },
            selector: parts.next()?,
            host: parts.next()?,
            port: parts.next()?.parse().ok()?,
        })
    }

    /// Serializes a Gopher entry into bytes. This function can be used to
    /// generate Gopher responses.
    pub fn write<W>(&self, mut buf: W) -> std::io::Result<()>
    where
        W: Write,
    {
        write!(
            buf,
            "{}{}\t{}\t{}\t{}\r\n",
            self.item_type.to_char(),
            self.display_string,
            self.selector,
            self.host,
            self.port
        )?;
        Ok(())
    }
}

pub struct GopherMenu<W>
where
    W: Write,
{
    target: W,
}

impl<'a, W> GopherMenu<&'a W>
where
    &'a W: Write,
{
    pub fn with_write(target: &'a W) -> Self {
        GopherMenu { target: &target }
    }

    pub fn info(&self, text: &str) -> std::io::Result<()> {
        self.write_entry(ItemType::Info, text, "FAKE", "fake.host", 1)
    }

    pub fn error(&self, text: &str) -> std::io::Result<()> {
        self.write_entry(ItemType::Error, text, "FAKE", "fake.host", 1)
    }

    pub fn write_entry(
        &self,
        item_type: ItemType,
        text: &str,
        selector: &str,
        host: &str,
        port: u16,
    ) -> std::io::Result<()> {
        GopherEntry {
            item_type,
            display_string: text,
            selector,
            host,
            port,
        }
        .write(self.target)
    }

    pub fn end(&mut self) -> std::io::Result<()> {
        write!(self.target, ".\r\n")
    }
}

/// Item type for a Gopher directory entry
#[derive(Debug, PartialEq)]
pub enum ItemType {
    /// Item is a file
    File,
    /// Item is a directory
    Directory,
    /// Item is a CSO phone-book server
    CsoServer,
    /// Error
    Error,
    /// Item is a BinHexed Macintosh file.
    BinHex,
    /// Item is a DOS binary archive of some sort.
    /// Client must read until the TCP connection closes. Beware.
    DosBinary,
    /// Item is a UNIX uuencoded file.
    Uuencoded,
    /// Item is an Index-Search server.
    Search,
    /// Item points to a text-based telnet session.
    Telnet,
    /// Item is a binary file!
    /// Client must read until the TCP connection closes. Beware.
    Binary,
    /// Item is a redundant server
    RedundantServer,
    /// Item points to a text-based tn3270 session.
    Tn3270,
    /// Item is a GIF format graphics file.
    Gif,
    /// Item is some sort of image file. Client decides how to display.
    Image,
    /// Informational message
    Info,
    /// Other types
    Other(char),
}

impl ItemType {
    /// Parses a char into an Item Type
    pub fn from(c: char) -> Self {
        match c {
            '0' => ItemType::File,
            '1' => ItemType::Directory,
            '2' => ItemType::CsoServer,
            '3' => ItemType::Error,
            '4' => ItemType::BinHex,
            '5' => ItemType::DosBinary,
            '6' => ItemType::Uuencoded,
            '7' => ItemType::Search,
            '8' => ItemType::Telnet,
            '9' => ItemType::Binary,
            '+' => ItemType::RedundantServer,
            'T' => ItemType::Tn3270,
            'g' => ItemType::Gif,
            'I' => ItemType::Image,
            'i' => ItemType::Info,
            c => ItemType::Other(c),
        }
    }

    /// Turns an Item Type into a char
    pub fn to_char(&self) -> char {
        match self {
            ItemType::File => '0',
            ItemType::Directory => '1',
            ItemType::CsoServer => '2',
            ItemType::Error => '3',
            ItemType::BinHex => '4',
            ItemType::DosBinary => '5',
            ItemType::Uuencoded => '6',
            ItemType::Search => '7',
            ItemType::Telnet => '8',
            ItemType::Binary => '9',
            ItemType::RedundantServer => '+',
            ItemType::Tn3270 => 'T',
            ItemType::Gif => 'g',
            ItemType::Image => 'I',
            ItemType::Info => 'i',
            ItemType::Other(c) => *c,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_pairs() -> Vec<(String, GopherEntry<'static>)> {
        let mut pairs = Vec::new();

        pairs.push((
            "1Floodgap Home	/home	gopher.floodgap.com	70\r\n".to_owned(),
            GopherEntry {
                item_type: ItemType::Directory,
                display_string: "Floodgap Home",
                selector: "/home",
                host: "gopher.floodgap.com",
                port: 70,
            },
        ));

        pairs.push((
            "iWelcome to my page	FAKE	(NULL)	0\r\n".to_owned(),
            GopherEntry {
                item_type: ItemType::Info,
                display_string: "Welcome to my page",
                selector: "FAKE",
                host: "(NULL)",
                port: 0,
            },
        ));

        return pairs;
    }

    #[test]
    fn test_parse() {
        for (raw, parsed) in get_test_pairs() {
            let entry = GopherEntry::from(&raw).unwrap();
            assert_eq!(entry.item_type, parsed.item_type);
            assert_eq!(entry.display_string, parsed.display_string);
            assert_eq!(entry.selector, parsed.selector);
            assert_eq!(entry.host, parsed.host);
            assert_eq!(entry.port, parsed.port);
        }
    }

    #[test]
    fn test_write() {
        for (raw, parsed) in get_test_pairs() {
            let mut output = Vec::new();
            parsed.write(&mut output).unwrap();
            let line = String::from_utf8(output).unwrap();
            assert_eq!(raw, line);
        }
    }
}
