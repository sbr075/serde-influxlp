use crate::{datatypes::Element, error::Result, Error};

use super::{
    datatypes::{Position, NEWLINE},
    Reader,
};

pub struct SliceReader<'a> {
    input: &'a [u8],

    lines: Vec<&'a [u8]>,

    /// Previously parrsed element
    prev: Element,

    /// Next expected element to parse
    next: Element,

    include_tags: bool,

    position: Position,
}

impl<'a> SliceReader<'a> {
    pub fn new(s: &'a [u8]) -> Self {
        // Remove comment- and empty lines
        let mut lines: Vec<&[u8]> = s
            .trim_ascii()
            .split(|c| *c == NEWLINE)
            .filter_map(|l| {
                let line = l.trim_ascii();
                match !(line.starts_with(b"#") || line.is_empty()) {
                    true => Some(line),
                    false => None,
                }
            })
            .collect();

        Self {
            input: lines.pop().unwrap_or(&[]),
            lines,
            prev: Element::Measurement,
            next: Element::Measurement,
            include_tags: false,
            position: Position::new(),
        }
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
        match self.position.column < self.input.len() {
            true => {
                let c = self.input[self.position.column];
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
        !self.lines.is_empty()
    }

    fn set_next_line(&mut self) {
        self.input = self.lines.remove(0);
        self.position.next_line();

        self.prev = Element::Measurement;
        self.next = Element::Measurement;
        self.include_tags = false;
    }
}
