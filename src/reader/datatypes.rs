pub(crate) const BACKSLASH: u8 = b'\\';
pub(crate) const NEWLINE: u8 = b'\n';
pub(crate) const WHITESPACE: u8 = b' ';
pub(crate) const DOUBLEQUOTE: u8 = b'"';
pub(crate) const COMMA: u8 = b',';
pub(crate) const EQUALSIGN: u8 = b'=';

#[derive(Debug, Clone)]
pub struct Position {
    /// Total number of columns in previous lines
    ///
    /// Does not include the number of columns in the current line
    pub previous_columns: usize,

    /// Total number of columns parsed in current line
    pub column: usize,

    /// Number of line currently being worked on
    pub line: usize,
}

impl Position {
    pub(crate) fn new() -> Self {
        Position {
            previous_columns: 0,
            column: 0,
            line: 1,
        }
    }

    pub(crate) fn next_line(&mut self) {
        self.previous_columns += self.column;
        self.column = 0;
        self.line += 1;
    }
}
