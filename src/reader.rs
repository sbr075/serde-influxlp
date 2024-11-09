use std::char;

use crate::{datatypes::Element, de::Position};

use super::error::{Error, Result};

const BACKSLASH: char = '\\';
const DOUBLEQUOTE: char = '"';
const COMMA: char = ',';
const EQUALSIGN: char = '=';
const WHITESPACE: char = ' ';
const NEWLINE: char = '\n';

pub(crate) struct Reader<'a> {
    /// Line currently being parsed
    input: &'a str,

    /// Remaining lines to parse
    lines: Vec<&'a str>,

    /// Previously parrsed element
    prev: Element,

    /// Next expected element to parse
    next: Element,

    /// Fields to parse
    ///
    /// If a field is not included, e.g., tags, in the list but is present in
    /// the input it can be skipped instead of parsing through slowly
    elements: &'a [&'a str],

    position: Position,
}

impl<'a> Reader<'a> {
    pub fn new(s: &'a str) -> Self {
        // Remove comment- and empty lines
        let mut lines: Vec<&'a str> = s
            .trim()
            .split("\n")
            .filter_map(|l| match !(l.starts_with("#") || l.is_empty()) {
                true => Some(l.trim()),
                false => None,
            })
            .collect();

        Self {
            input: lines.remove(0),
            lines,
            prev: Element::Measurement,
            next: Element::Measurement,
            elements: &[],
            position: Position::default(),
        }
    }

    pub fn next_line(&mut self) {
        if self.input.is_empty() {
            self.input = self.lines.remove(0);
            self.position.line += 1;
            self.position.column = 0;

            self.prev = Element::Measurement;
            self.next = Element::Measurement;
            self.elements = &[];
        }
    }

    pub fn get_position(&self) -> Position {
        self.position.clone()
    }

    pub fn set_elements(&mut self, elements: &'a [&'a str]) -> Result<()> {
        // Hack: only set elements if they are empty, prevents it from being overwritten
        // when deserializing tags/fields structs
        if self.elements.is_empty() {
            self.elements = elements;
        }

        Ok(())
    }

    pub fn peek_char(&self) -> Result<char> {
        self.input.chars().next().ok_or(Error::unexpected_eof())
    }

    fn discard_next_char(&mut self) -> Result<()> {
        let ch = self.peek_char()?;
        self.input = &self.input[ch.len_utf8()..];
        self.position.column += 1;
        Ok(())
    }

    fn unescape_tag_key(&self, key: String) -> String {
        key.replace(r"\=", "=")
            .replace(r"\,", ",")
            .replace(r"\ ", " ")
    }

    fn unescape_tag_value(&self, value: &'a str) -> String {
        value
            .replace(r"\=", "=")
            .replace(r"\,", ",")
            .replace(r"\ ", " ")
    }

    fn unescape_field_key(&self, key: String) -> String {
        key.replace(r"\=", "=")
            .replace(r"\,", ",")
            .replace(r"\ ", " ")
    }

    fn unescape_field_value(&self, value: &'a str) -> String {
        let unescaped = match value.starts_with("\"") && value.ends_with("\"") {
            true => &value[1..value.len() - 1],
            false => &value,
        };

        unescaped.replace("\\\"", "\"").replace("\\\\", "\\")
    }

    pub fn has_next_line(&mut self) -> bool {
        !self.lines.is_empty()
    }

    /// Check if there are anymore keys to parse in current element
    pub fn has_next_key(&mut self) -> Result<bool> {
        let has_next = match self.next {
            // There should always be a measurement key
            Element::Measurement => true,

            // Tag set is done whenever a whitespace is reached
            Element::Tags => match self.peek_char()? {
                WHITESPACE => {
                    self.discard_next_char()?;
                    false
                }
                _ => true,
            },

            // Field set is done whenever a whitespace, newline is reached, or if there are no more
            // characters remaining
            Element::Fields => match self.peek_char() {
                Ok(c) => match c {
                    WHITESPACE | NEWLINE => {
                        self.discard_next_char()?;
                        false
                    }
                    _ => true,
                },
                Err(_) => false,
            },

            // Timestamp is done whenever a whitespace, newline is reached, or if there are no more
            // characters remaining
            Element::Timestamp => match self.peek_char() {
                Ok(c) => match c {
                    WHITESPACE | NEWLINE => {
                        self.discard_next_char()?;
                        false
                    }
                    _ => true,
                },
                Err(_) => false,
            },
        };

        Ok(has_next)
    }

    /// Parse the next element key
    pub fn next_element_key(&mut self) -> Result<String> {
        let key = match self.next {
            // The measurement key is not parsed and will always be "measurement"
            Element::Measurement => "measurement".to_string(),

            // If the previous key was measurement, the current key is just tags (the struct name)
            // else we parse the key from the tag set and unescape it
            Element::Tags => {
                let key = if self.prev.is_measurement() {
                    "tags".to_string()
                } else {
                    self.next_element_value()?
                };

                self.prev = Element::Tags;
                self.unescape_tag_key(key)
            }

            // If the previous key was either tags or measurement, the current key is just tags (the
            // struct name) else we parse the key from the field set and unescape it
            Element::Fields => {
                let key = if self.prev.is_tags() || self.prev.is_measurement() {
                    "fields".to_string()
                } else {
                    self.next_element_value()?
                };

                self.prev = Element::Fields;
                self.unescape_field_key(key)
            }

            // As with measurement there is no keys to parse and the key will always just be
            // timestamp
            Element::Timestamp => "timestamp".to_string(),
        };

        Ok(key)
    }

    /// Discard characters until the next element is reached
    fn discard_element(&mut self) {
        let mut in_quote = false;
        let mut is_escaped = false;

        let end = self
            .input
            .char_indices()
            .find(|(_, c)| {
                if *c == BACKSLASH {
                    is_escaped = true;
                    false
                } else if *c == DOUBLEQUOTE && !is_escaped {
                    in_quote = !in_quote;
                    false
                } else if (!is_escaped && !in_quote) && *c == WHITESPACE || *c == NEWLINE {
                    true
                } else {
                    is_escaped = false;
                    false
                }
            })
            .map(|(i, _)| i + 1)
            .unwrap_or(self.input.len());

        self.position.column += end;
        self.input = &self.input[end..];
    }

    /// Determine whether we are done parsing the current element
    fn determine_next_element(&mut self) -> Result<Element> {
        let element = match self.next {
            // Parsing the `measurement` element determines the next parsing step, with two valid
            // paths: `tags` or `fields`.
            //
            // The next step depends on the upcoming character:
            // 1. A comma (`,`)
            //    - A comma following the measurement indicates a tag set is included. If tags are
            //      not specified in the target struct, this element will be discarded.
            // 2. A whitespace (` `)
            //    - If whitespace follows the measurement, there is no tag set, and we proceed to
            //      parse the field set.
            Element::Measurement => match self.peek_char()? {
                COMMA => {
                    self.discard_next_char()?;
                    if !self.elements.contains(&"tags") {
                        self.discard_element();

                        Element::Fields
                    } else {
                        Element::Tags
                    }
                }
                WHITESPACE => {
                    self.discard_next_char()?;
                    Element::Fields
                }
                c => return Err(Error::unexpected_char(c, self.get_position())),
            },

            // After parsing tags, the parser either continues with additional tags or moves on to
            // the field set.
            //
            // The next step depends on the upcoming character:
            // 1. A comma (`,`) or equal sign (`=`)
            //    - If the next character is a comma or equals sign, we are still within the tag set
            //      and should continue parsing.
            // 2. A whitespace (` `)
            //    - Whitespace indicates the end of the tag set, and the parser moves on to the
            //      field set.
            Element::Tags => match self.peek_char()? {
                COMMA | EQUALSIGN => {
                    self.discard_next_char()?;
                    Element::Tags
                }
                WHITESPACE => Element::Fields,
                c => return Err(Error::unexpected_char(c, self.get_position())),
            },

            // After parsing fields, the parser either continues with additional fields or moves on
            // to the timestamp element.
            //
            // The next step depends on the upcoming character:
            // 1. A comma (`,`) or equal sign (`=`)
            //    - If a comma or equals sign follows, we remain within the field set and continue
            //      parsing fields.
            // 2. A whitespace (` `) or newline (`\n`)
            //    - Whitespace or a newline signifies the end of the field set. The timestamp is
            //      parsed next unless the line ends, in which case parsing stops and relevant
            //      fields reset before the next line begins.
            //
            // If no characters remain, the parser can safely proceed, as the next fields will reset
            // before new parsing starts.
            Element::Fields => match self.peek_char() {
                Ok(c) => match c {
                    COMMA | EQUALSIGN => {
                        self.discard_next_char()?;
                        Element::Fields
                    }
                    WHITESPACE | NEWLINE => Element::Timestamp,
                    c => return Err(Error::unexpected_char(c, self.get_position())),
                },
                Err(_) => Element::Fields,
            },
            _ => Element::Timestamp,
        };

        Ok(element)
    }

    /// Parses and returns the next key or value from the current element
    fn parse_value(&mut self) -> Result<&'a str> {
        let mut in_quote = false;
        let mut is_escaped = false;

        // Find next element
        let end = self
            .input
            .find(|c: char| {
                if c == BACKSLASH {
                    is_escaped = true;
                    false
                } else if c == DOUBLEQUOTE && !is_escaped {
                    in_quote = !in_quote;
                    false
                } else if (!is_escaped && !in_quote)
                    && (c == EQUALSIGN || c == COMMA || c == WHITESPACE)
                    || c == NEWLINE
                {
                    true
                } else {
                    is_escaped = false;
                    false
                }
            })
            .unwrap_or(self.input.len());

        self.position.column += end;
        let value = &self.input[..end];
        self.input = &self.input[end..];

        Ok(value)
    }

    /// Parse the next element value and unescapes the string
    pub fn next_element_value(&mut self) -> Result<String> {
        let value = self.parse_value()?;

        let value = if self.prev.is_tags() {
            self.unescape_tag_value(value)
        } else if self.prev.is_fields() {
            self.unescape_field_value(value)
        } else {
            value.to_string()
        };

        self.next = self.determine_next_element()?;
        Ok(value)
    }
}
