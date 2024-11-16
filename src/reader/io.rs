use std::io;

use crate::{datatypes::Element, error::Result, Error};

use super::{datatypes::Position, Reader};

pub struct IoReader<R>
where
    R: io::Read,
{
    iter: io::Bytes<R>,

    /// Temporary value stored by `peek_char`
    tmp: Option<u8>,

    /// Previously parrsed element
    prev: Element,

    /// Next expected element to parse
    next: Element,

    include_tags: bool,

    position: Position,
}

impl<R> IoReader<R>
where
    R: io::Read,
{
    pub fn new(reader: R) -> Self {
        let mut reader = Self {
            iter: reader.bytes(),
            tmp: None,
            prev: Element::Measurement,
            next: Element::Measurement,
            include_tags: false,
            position: Position::new(),
        };
        let _ = reader.skip_until_valid_line();

        reader
    }
}

impl<'de, R> Reader<'de> for IoReader<R>
where
    R: io::Read,
{
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
        if let Some(c) = self.tmp {
            return Ok(c);
        }

        match self.iter.next() {
            Some(c) => {
                let c = c.map_err(|_| Error::unexpected_eof())?;
                self.position.column += 1;
                self.tmp = Some(c);
                Ok(c)
            }
            None => Err(Error::unexpected_eof()),
        }
    }

    fn skip_char(&mut self) {
        self.tmp = None;
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
