use crate::{datatypes::Element, error::Result, Error};

use super::datatypes::{Position, BACKSLASH, COMMA, DOUBLEQUOTE, EQUALSIGN, NEWLINE, WHITESPACE};

pub(crate) trait Reader<'de> {
    /// Skip the current line
    #[doc(hidden)]
    fn skip_line(&mut self) {
        while let Ok(c) = self.peek_char() {
            self.skip_char();

            if c == NEWLINE {
                break;
            }
        }

        self.set_next_line();
    }

    /// Skip until the next non ascii whitespace
    #[doc(hidden)]
    fn skip_whitespace(&mut self) {
        while let Ok(c) = self.peek_char() {
            if !c.is_ascii_whitespace() {
                break;
            }

            self.skip_char();
        }
    }

    /// Skip until the next valid line
    #[doc(hidden)]
    fn skip_until_valid_line(&mut self) -> Result<()> {
        loop {
            // Skip until first non ascii whitespace character
            self.skip_whitespace();

            // If the first character is a # this is a comment line
            let c = self.peek_char()?;
            if c == b'#' {
                self.skip_line();
                continue;
            }

            break;
        }

        Ok(())
    }

    /// Parse measurement from input
    #[doc(hidden)]
    fn parse_measurement(&mut self) -> String {
        let mut result = Vec::new();

        let mut is_escaped = false;
        while let Ok(c) = self.peek_char() {
            if !is_escaped && (c == COMMA || c == WHITESPACE) {
                break;
            }

            // Skip backslash if its used as an escape character
            self.skip_char();
            if c == BACKSLASH && !is_escaped {
                is_escaped = true;
                continue;
            }

            is_escaped = false;
            result.push(c);
        }

        // Bytes should never be invalid
        String::from_utf8(result).unwrap()
    }

    /// Parse tag key from input
    #[doc(hidden)]
    fn parse_tag_key(&mut self) -> String {
        let mut result = Vec::new();

        let mut is_escaped = false;
        while let Ok(c) = self.peek_char() {
            if !is_escaped && (c == COMMA || c == EQUALSIGN || c == WHITESPACE) {
                break;
            }

            // Skip backslash if its used as an escape character
            self.skip_char();
            if c == BACKSLASH && !is_escaped {
                is_escaped = true;
                continue;
            }

            is_escaped = false;
            result.push(c);
        }

        // Bytes should never be invalid
        String::from_utf8(result).unwrap()
    }

    /// Parse tag value from input
    ///
    /// Calls [Self::parse_tag_key] as these two have the same escape characters
    #[doc(hidden)]
    fn parse_tag_value(&mut self) -> String {
        self.parse_tag_key()
    }

    /// Parse field key from input
    ///
    /// Calls [Self::parse_tag_key] as these two have the same escape characters
    #[doc(hidden)]
    fn parse_field_key(&mut self) -> String {
        self.parse_tag_key()
    }

    /// Parse field value from input
    #[doc(hidden)]
    fn parse_field_value(&mut self) -> String {
        let mut result = Vec::new();

        let mut is_escaped = false;
        let mut in_quote = false;
        while let Ok(c) = self.peek_char() {
            if (!is_escaped && !in_quote)
                && (c == COMMA || c == EQUALSIGN || c.is_ascii_whitespace())
            {
                break;
            }

            // Skip backslash if its used as an escape character
            self.skip_char();
            if c == BACKSLASH && !is_escaped {
                is_escaped = true;
                continue;
            }

            if !is_escaped && c == DOUBLEQUOTE {
                in_quote = !in_quote;
            };

            is_escaped = false;
            result.push(c);
        }

        if result.starts_with(b"\"") && result.ends_with(b"\"") {
            result = result[1..result.len() - 1].to_vec();
        }

        // Bytes should never be invalid
        String::from_utf8(result).unwrap()
    }

    /// Parse timestamp from input
    #[doc(hidden)]
    fn parse_timestamp(&mut self) -> String {
        let mut result = Vec::new();

        while let Ok(c) = self.peek_char() {
            self.skip_char();
            if c.is_ascii_whitespace() {
                break;
            }

            result.push(c);
        }

        // Bytes should never be invalid
        String::from_utf8(result).unwrap()
    }

    /// Get the current position of the reader
    #[doc(hidden)]
    fn get_position(&self) -> Position;

    /// Tell the reader not to skip reading tags
    #[doc(hidden)]
    fn include_tags(&mut self);

    #[doc(hidden)]
    fn tags_included(&self) -> bool;

    /// Look at the next character in the current input without consuming
    #[doc(hidden)]
    fn peek_char(&mut self) -> Result<u8>;

    /// Skip the next character in the current input
    #[doc(hidden)]
    fn skip_char(&mut self);

    /// Discard the next element
    ///
    /// Used if tags is not specified in the result type `T`
    #[doc(hidden)]
    fn discard_next_element(&mut self) {
        let mut in_quote = false;
        let mut is_escaped = false;

        while let Ok(c) = self.peek_char() {
            if (!is_escaped && !in_quote) && c.is_ascii_whitespace() {
                break;
            }

            self.skip_char();
            if c == BACKSLASH && !is_escaped {
                is_escaped = true;
            }

            if !is_escaped && c == DOUBLEQUOTE {
                in_quote = !in_quote;
            }

            is_escaped = false;
            self.skip_char();
        }
    }

    /// Getter function for fetching the previous element
    #[doc(hidden)]
    fn get_prev_element(&self) -> &Element;

    /// Setter function for setting the previous element
    #[doc(hidden)]
    fn set_prev_element(&mut self, prev: Element);

    /// Getter function for fetching the next element
    #[doc(hidden)]
    fn get_next_element(&self) -> &Element;

    /// Setter function for setting the next element
    #[doc(hidden)]
    fn set_next_element(&mut self, next: Element);

    /// Determine what the next element in the line will be
    #[doc(hidden)]
    fn determine_next_element(&mut self) -> Result<()> {
        let next = match self.get_next_element() {
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
                    self.skip_char();

                    if !self.tags_included() {
                        self.discard_next_element();

                        Element::Fields
                    } else {
                        Element::Tags
                    }
                }
                WHITESPACE => {
                    self.skip_char();
                    Element::Fields
                }
                c => return Err(Error::unexpected_char(c as char, self.get_position())),
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
                    self.skip_char();
                    Element::Tags
                }
                WHITESPACE => Element::Fields,
                c => return Err(Error::unexpected_char(c as char, self.get_position())),
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
                Ok(c) => {
                    if c.is_ascii_whitespace() {
                        Element::Timestamp
                    } else {
                        match c {
                            COMMA | EQUALSIGN => {
                                self.skip_char();
                                Element::Fields
                            }
                            WHITESPACE => Element::Timestamp,
                            c => {
                                return Err(Error::unexpected_char(c as char, self.get_position()))
                            }
                        }
                    }
                }
                Err(_) => Element::Fields,
            },
            _ => Element::Timestamp,
        };

        self.set_next_element(next);
        Ok(())
    }

    /// Check if there are any more lines to deserialize
    #[doc(hidden)]
    fn has_next_line(&mut self) -> bool;

    /// Sets the next line to be deserialized
    ///
    /// Returns an error if there are no more lines to deserialize. Should
    /// therefor be used in conjuction with `has_next_line` to ensure success
    #[doc(hidden)]
    fn set_next_line(&mut self);

    /// Check if there are any more keys in the current line to deserialize
    #[doc(hidden)]
    fn has_next_key(&mut self) -> Result<bool> {
        let has_next = match self.get_next_element() {
            // There should always be a measurement key
            Element::Measurement => true,

            // Tag set is done whenever a whitespace is reached
            Element::Tags => match self.peek_char()? {
                WHITESPACE => {
                    self.skip_char();
                    false
                }
                _ => true,
            },

            // Field set is done whenever a whitespace, newline is reached, or if there are no more
            // characters remaining
            Element::Fields => match self.peek_char() {
                Ok(c) => {
                    if c.is_ascii_whitespace() {
                        self.skip_char();
                        false
                    } else {
                        true
                    }
                }
                Err(_) => false,
            },

            // Timestamp is done whenever a whitespace, newline is reached, or if there are no more
            // characters remaining. This might be an unneccessary check
            Element::Timestamp => match self.peek_char() {
                Ok(c) => {
                    if c.is_ascii_whitespace() {
                        self.skip_char();
                        false
                    } else {
                        true
                    }
                }
                Err(_) => false,
            },
        };

        Ok(has_next)
    }

    /// Fetch the next key in the current element to deserialize
    #[doc(hidden)]
    fn get_next_key(&mut self) -> Result<String> {
        let key = match self.get_next_element() {
            // The measurement key is not parsed and will always be "measurement"
            Element::Measurement => "measurement".to_string(),

            // If the previous key was measurement, the current key is just tags (the struct name)
            // else we parse the key from the tag set and unescape it
            Element::Tags => {
                let key = if self.get_prev_element().is_measurement() {
                    "tags".to_string()
                } else {
                    let key = self.parse_tag_key();
                    self.determine_next_element()?;
                    key
                };

                self.set_prev_element(Element::Tags);
                key
            }

            // If the previous key was either tags or measurement, the current key is just tags (the
            // struct name) else we parse the key from the field set and unescape it
            Element::Fields => {
                let prev = self.get_prev_element();
                let key = if prev.is_tags() || prev.is_measurement() {
                    "fields".to_string()
                } else {
                    let key = self.parse_field_key();
                    self.determine_next_element()?;
                    key
                };

                self.set_prev_element(Element::Fields);
                key
            }

            // As with measurement there is no keys to parse and the key will always just be
            // timestamp
            Element::Timestamp => "timestamp".to_string(),
        };

        Ok(key)
    }

    /// Fetch the next element in the current element to deserialize
    #[doc(hidden)]
    fn get_next_value(&mut self) -> Result<String> {
        let value = match self.get_next_element() {
            Element::Measurement => self.parse_measurement(),
            Element::Tags => self.parse_tag_value(),
            Element::Fields => self.parse_field_value(),
            Element::Timestamp => self.parse_timestamp(),
        };

        self.determine_next_element()?;
        Ok(value)
    }

    /// Discard the next value
    #[doc(hidden)]
    fn discard_next_value(&mut self) -> Result<()> {
        self.get_next_value()?;
        Ok(())
    }
}
