use crate::{datatypes::Element, error::Result, Error};

use super::{datatypes::Position, Reader};

pub struct SliceReader<'a> {
    input: &'a [u8],

    /// Previously parrsed element
    prev: Element,

    /// Next expected element to parse
    next: Element,

    include_tags: bool,

    position: Position,
}

impl<'a> SliceReader<'a> {
    pub fn new(s: &'a [u8]) -> Self {
        let mut reader = Self {
            input: s,
            prev: Element::Measurement,
            next: Element::Measurement,
            include_tags: false,
            position: Position::new(),
        };
        let _ = reader.skip_until_valid_line();

        reader
    }
}

impl<'de> Reader<'de> for SliceReader<'de> {
    fn get_position(&self) -> Position {
        self.position.clone()
    }

    fn include_tags(&mut self) {
        self.include_tags = true;
    }

    fn tags_included(&self) -> bool {
        self.include_tags
    }

    fn peek_char(&mut self) -> Result<u8> {
        let idx = self.position.column + self.position.previous_columns;
        match idx < self.input.len() {
            true => {
                let c = self.input[idx];
                Ok(c)
            }
            false => Err(Error::unexpected_eof()),
        }
    }

    fn skip_char(&mut self) {
        self.position.column += 1;
    }

    fn get_prev_element(&self) -> &Element {
        &self.prev
    }

    fn set_prev_element(&mut self, prev: Element) {
        self.prev = prev;
    }

    fn get_next_element(&self) -> &Element {
        &self.next
    }

    fn set_next_element(&mut self, next: Element) {
        self.next = next;
    }

    fn has_next_line(&mut self) -> bool {
        if self.skip_until_valid_line().is_err() {
            return false;
        }

        match self.peek_char() {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn set_next_line(&mut self) {
        self.position.next_line();

        self.prev = Element::Measurement;
        self.next = Element::Measurement;
        self.include_tags = false;
    }
}
