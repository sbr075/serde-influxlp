pub(crate) const BACKSLASH: u8 = b'\\';
pub(crate) const NEWLINE: u8 = b'\n';
pub(crate) const WHITESPACE: u8 = b' ';
pub(crate) const DOUBLEQUOTE: u8 = b'"';
pub(crate) const COMMA: u8 = b',';
pub(crate) const EQUALSIGN: u8 = b'=';

#[derive(Debug, Clone)]
pub struct Position {
    pub column: usize,

    pub line: usize,
}

impl Position {
    pub(crate) fn new() -> Self {
        Position { column: 0, line: 0 }
    }

    pub(crate) fn next_line(&mut self) {
        self.line += 1;
    }
}
