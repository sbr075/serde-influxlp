use std::{
    error::Error as StdError,
    fmt::{self, Debug, Display},
    io,
};

use serde::{de, ser};

use crate::reader::datatypes::Position;

pub(crate) type Result<T> = std::result::Result<T, Error>;

pub enum ErrorCode {
    /// A custom error message
    Message(String),

    Io(io::Error),

    EmptyInput,

    /// Reached end of line earlier than expected
    UnexpectedEof,

    /// Met an unexpacted character while parsing line
    UnexpectedChar(String),

    /// Tried to deserialize from an unsupported type
    InvalidType {
        got: String,
        expected: String,
    },

    /// Failed to deserialize value as it is not recognized
    InvalidValue(String),

    /// Field type was defined as char but value was not a valid char
    InvalidChar {
        got: String,
        len: usize,
    },

    /// Tried to serialize an infinite float to a string
    InfiniteFloat,

    /// Unsupported key type
    InvalidKey,

    /// Set field creates an invalid structure
    InvalidFieldType(String),

    /// Required element missing
    MissingElement(String),

    /// Tag-/field set has an uneven amount of key and values
    UnevenSet(String),

    /// Feature is not supported by this crate although it might be in the
    /// future!
    UnsupportedFeature(String),
}

/// Custom Error for serde_influxlp
///
/// # Example
///
/// ```rust
/// #[derive(Debug, Serialize)]
/// pub struct Metric {
///     pub measurment: String,
///
///     pub timestamp: i64,
/// };
///
/// fn main() {
///     let metric = Metric {
///         measurement: "measurement".to_string(),
///         timestamp: 123456,
///     };
///
///     if let Err(e) = serde_influxlp::to_string(&metric) {
///         println!("{e}");
///         // Output: missing element: `fields`
///     }
/// }
/// ```
pub struct Error {
    /// Error code indicating what went wrong
    pub code: ErrorCode,

    /// Column and line error occured
    ///
    /// *For serialization position will always be (0, 0)*
    pub position: Position,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err = match &self.code {
            ErrorCode::Message(v) => v.to_string(),
            ErrorCode::Io(v) => v.to_string(),
            ErrorCode::EmptyInput => "empty input".to_string(),
            ErrorCode::UnexpectedEof => "unexpected eof".to_string(),
            ErrorCode::InvalidType { got, expected } => {
                format!(
                    "invalid type: value `{got}` is not of correct type, expected type {expected} \
                     at column {}, line {}",
                    self.position.column, self.position.line
                )
            }
            ErrorCode::InvalidValue(v) => {
                format!(
                    "invalid value `{v}` at column {}, line {}",
                    self.position.column, self.position.line
                )
            }
            ErrorCode::InvalidChar { got: char, len } => {
                format!(
                    "invalid char: `{char}` of length {len} at column {}, line {}",
                    self.position.column, self.position.line
                )
            }
            ErrorCode::UnexpectedChar(v) => {
                format!(
                    "unexpected char: `{v}` at column {}, line {}",
                    self.position.column, self.position.line
                )
            }
            ErrorCode::InfiniteFloat => "invalid float: floats must be finite".to_string(),
            ErrorCode::InvalidKey => "invalid key: keys must be of type string".to_string(),
            ErrorCode::InvalidFieldType(v) => format!(
                "invalid field type `{v}`, expected any of: float, int, uint, string, or bool"
            ),
            ErrorCode::MissingElement(v) => format!("missing element: `{v}`"),
            ErrorCode::UnevenSet(v) => {
                format!("invalid set: {v} set contains an uneven amount of key- and values")
            }
            ErrorCode::UnsupportedFeature(v) => {
                format!("attempted to use a unsupported feature: {v}")
            }
        };

        write!(f, "an error occured: {err}")
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {self}")
    }
}

impl StdError for Error {}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error {
            code: ErrorCode::Message(msg.to_string()),
            position: Position::new(),
        }
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Error {
        Error {
            code: ErrorCode::Message(msg.to_string()),
            position: Position::new(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error {
            code: ErrorCode::Io(value),
            position: Position::new(),
        }
    }
}

impl Error {
    pub(crate) fn unexpected_eof() -> Self {
        Error {
            code: ErrorCode::UnexpectedEof,
            position: Position::new(),
        }
    }

    pub(crate) fn invalid_type(
        got: impl ToString,
        expected: impl ToString,
        mut position: Position,
    ) -> Self {
        // We've actually parsed to the end of this value so we adjust position to show
        // it correctly in the error mesage
        let got = got.to_string();
        position.column -= got.len();

        Error {
            code: ErrorCode::InvalidType {
                got,
                expected: expected.to_string(),
            },
            position,
        }
    }

    pub(crate) fn invalid_value(value: impl ToString, mut position: Position) -> Self {
        // We've actually parsed to the end of this value so we adjust position to show
        // it correctly in the error mesage
        let value = value.to_string();
        position.column -= value.len();

        Error {
            code: ErrorCode::InvalidValue(value),
            position,
        }
    }

    pub(crate) fn invalid_char(char: impl ToString, len: usize, mut position: Position) -> Self {
        // We've actually parsed to the end of this value so we adjust position to show
        // it correctly in the error mesage
        position.column -= len;

        Error {
            code: ErrorCode::InvalidChar {
                got: char.to_string(),
                len,
            },
            position,
        }
    }

    pub(crate) fn unexpected_char(char: impl ToString, mut position: Position) -> Self {
        // We've actually parsed to the end of this value so we adjust position to show
        // it correctly in the error mesage
        position.column -= 1;

        Error {
            code: ErrorCode::UnexpectedChar(char.to_string()),
            position,
        }
    }

    pub(crate) fn infinite_float() -> Self {
        Error {
            code: ErrorCode::InfiniteFloat,
            position: Position::new(),
        }
    }

    pub(crate) fn invalid_key() -> Self {
        Error {
            code: ErrorCode::InvalidKey,
            position: Position::new(),
        }
    }

    pub(crate) fn invalid_field_type(typ: impl ToString) -> Self {
        Error {
            code: ErrorCode::InvalidFieldType(typ.to_string()),
            position: Position::new(),
        }
    }

    pub(crate) fn missing_element(element: impl ToString) -> Self {
        Error {
            code: ErrorCode::MissingElement(element.to_string()),
            position: Position::new(),
        }
    }

    pub(crate) fn uneven_set(set: impl ToString) -> Self {
        Error {
            code: ErrorCode::UnevenSet(set.to_string()),
            position: Position::new(),
        }
    }

    pub(crate) fn unsupported(feature: impl ToString) -> Self {
        Error {
            code: ErrorCode::UnsupportedFeature(feature.to_string()),
            position: Position::new(),
        }
    }
}
