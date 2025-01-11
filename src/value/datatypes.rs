use std::{fmt::Display, hash::Hash};

use conv::*;
use regex::Regex;
use serde::de;

use crate::error::Error;

#[derive(Debug, Clone)]
pub enum Number {
    /// Represent a floating point number field value
    Float(f64),

    /// Represent a signed integer number field value
    Integer(i64),

    /// Represent an unsigned integer number field value
    UInteger(u64),
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Number::Float(n1), Number::Float(n2)) => n1 == n2,
            (Number::Integer(n1), Number::Integer(n2)) => n1 == n2,
            (Number::UInteger(n1), Number::UInteger(n2)) => n1 == n2,
            _ => false,
        }
    }
}

impl Eq for Number {}

impl Hash for Number {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match *self {
            Number::Float(n) => {
                match n == 0.0 {
                    true => 0.0f64.to_bits(),
                    false => n.to_bits(),
                }
                .hash(state);
            }
            Number::Integer(n) => n.hash(state),
            Number::UInteger(n) => n.hash(state),
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let number = match self {
            Number::Float(n) => format!("{n}"),
            Number::Integer(n) => format!("{n}i"),
            Number::UInteger(n) => format!("{n}i"),
        };

        write!(f, "{number}")
    }
}

impl Number {
    fn visit<'de, V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Number::Float(n) => visitor.visit_f64(n),
            Number::Integer(n) => visitor.visit_i64(n),
            Number::UInteger(n) => visitor.visit_u64(n),
        }
    }

    /// Checks if number is a float
    pub fn is_float(&self) -> bool {
        matches!(self, Number::Float(_))
    }

    /// Attempts to convert the inner value of self into a f64. If the
    /// conversion fails None is returned instead
    ///
    /// # Example
    ///
    /// ```rust
    /// let number = Number::Integer(123);
    ///
    /// println!("{}", value.as_float());
    /// // Output: 123.0
    /// ```
    pub fn as_float(&self) -> Option<f64> {
        match *self {
            Number::Float(n) => Some(n),
            Number::Integer(n) => f64::value_from(n).ok(),
            Number::UInteger(n) => f64::value_from(n).ok(),
        }
    }

    /// Checks if number is a signed integer
    pub fn is_int(&self) -> bool {
        matches!(self, Number::Integer(_))
    }

    /// Attempts to convert the inner value of self into a i64. If the
    /// conversion fails None is returned instead
    ///
    /// # Example
    ///
    /// ```rust
    /// let number = Number::Float(123.5);
    ///
    /// println!("{}", value.as_int());
    /// // Output: 124
    /// ```
    pub fn as_int(&self) -> Option<i64> {
        match *self {
            Number::Float(v) => {
                // Ensure `f64` fits within `i64` range
                let v = v.round();
                if v >= i64::MIN as f64 && v <= i64::MAX as f64 {
                    Some(v as i64)
                } else {
                    None
                }
            }
            Number::Integer(v) => Some(v),
            Number::UInteger(v) => i64::value_from(v).ok(),
        }
    }

    /// Checks if number is an unsigned integer
    pub fn is_uint(&self) -> bool {
        matches!(self, Number::UInteger(_))
    }

    /// Attempts to convert the inner value of self into a u64. If the
    /// conversion fails None is returned instead
    ///
    /// # Example
    ///
    /// ```rust
    /// let number = Number::Float(123.4);
    ///
    /// println!("{}", value.as_int());
    /// // Output: 123
    /// ```
    pub fn as_uint(&self) -> Option<u64> {
        match *self {
            Number::Float(v) => {
                // Ensure `f64` fits within `u64` range
                let v = v.round();
                if v >= u64::MIN as f64 && v <= u64::MAX as f64 {
                    Some(v as u64)
                } else {
                    None
                }
            }
            Number::Integer(v) => u64::value_from(v).ok(),
            Number::UInteger(v) => Some(v),
        }
    }

    /// An alternative to [Values](Value) `to_string` function. Instead uses the
    /// inner values `to_string` function to convert self to a string.
    pub fn as_string(&self) -> String {
        match *self {
            Number::Float(n) => {
                // Use ryu if n is finite else use normal to_string conversion
                if n.is_finite() {
                    ryu::Buffer::new().format_finite(n).to_owned()
                } else {
                    n.to_string()
                }
            }
            Number::Integer(n) => itoa::Buffer::new().format(n).to_owned(),
            Number::UInteger(n) => itoa::Buffer::new().format(n).to_owned(),
        }
    }
}

/// Represents any supported InfluxDB v2 Line protocol value
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    /// Represents a value which is not set
    ///
    /// Although not a valid line protocol datatype this is added to add support
    /// for formats which allow nullable values. When serialized or deserialized
    /// it will output nothing same as Rust's None
    None,

    Number(Number),

    /// Represent a string field value
    String(String),

    /// Represent a boolean field value
    Boolean(bool),
}

impl Value {
    pub(crate) fn from_number_str(s: &str) -> Option<Self> {
        let mut value = s.to_string();

        // Check if string is a number that ends with an i
        let re = Regex::new(r"^-?\d+i$").unwrap();
        let number = match re.is_match(&value) {
            true => {
                // Remove the `i`
                value.pop();

                match value.starts_with("-") {
                    true => {
                        let number = match value.parse() {
                            Ok(number) => number,
                            Err(_) => return None,
                        };

                        Number::Integer(number)
                    }
                    false => {
                        let number = match value.parse() {
                            Ok(number) => number,
                            Err(_) => return None,
                        };

                        Number::UInteger(number)
                    }
                }
            }
            false => match value.parse::<f64>() {
                Ok(value) => Number::Float(value),
                Err(_) => return None,
            },
        };

        Some(Value::Number(number))
    }

    pub(crate) fn from_bool_str(s: &str) -> Option<Self> {
        let bool = match s {
            "t" | "T" | "true" | "True" | "TRUE" => true,
            "f" | "F" | "false" | "False" | "FALSE" => false,
            _ => return None,
        };

        Some(Value::Boolean(bool))
    }

    pub(crate) fn from_any_str(s: &str) -> Value {
        let mut char = s.chars();
        let char = match char.next() {
            Some(c) => c,
            None => return Value::String(s.to_owned()),
        };

        let value = match char.to_ascii_lowercase() {
            '-' | '0'..='9' => Value::from_number_str(&s),
            't' | 'f' => Value::from_bool_str(&s),
            _ => None,
        };

        value.unwrap_or(Value::String(s.to_owned()))
    }

    pub(crate) fn visit<'de, V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Value::None => visitor.visit_none(),
            Value::Number(n) => n.visit(visitor),
            Value::String(s) => visitor.visit_string(s),
            Value::Boolean(b) => visitor.visit_bool(b),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Value::None => format!(""),
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.to_string(),
            Value::Boolean(b) => b.to_string(),
        };

        write!(f, "{}", value)
    }
}

impl From<char> for Value {
    fn from(s: char) -> Self {
        Value::String(s.to_string())
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<&String> for Value {
    fn from(s: &String) -> Self {
        Value::String(s.to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<f32> for Value {
    fn from(n: f32) -> Self {
        Value::Number(Number::Float(n.into()))
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(Number::Float(n))
    }
}

impl From<i8> for Value {
    fn from(n: i8) -> Self {
        Value::Number(Number::Integer(n.into()))
    }
}

impl From<i16> for Value {
    fn from(n: i16) -> Self {
        Value::Number(Number::Integer(n.into()))
    }
}

impl From<i32> for Value {
    fn from(n: i32) -> Self {
        Value::Number(Number::Integer(n.into()))
    }
}

impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Value::Number(Number::Integer(n))
    }
}

impl From<u8> for Value {
    fn from(n: u8) -> Self {
        Value::Number(Number::UInteger(n.into()))
    }
}

impl From<u16> for Value {
    fn from(n: u16) -> Self {
        Value::Number(Number::UInteger(n.into()))
    }
}

impl From<u32> for Value {
    fn from(n: u32) -> Self {
        Value::Number(Number::UInteger(n.into()))
    }
}

impl From<u64> for Value {
    fn from(n: u64) -> Self {
        Value::Number(Number::UInteger(n))
    }
}

impl From<bool> for Value {
    fn from(n: bool) -> Self {
        Value::Boolean(n)
    }
}

impl Value {
    /// Converts this type into a shared reference of itself
    pub fn as_ref(&self) -> &Self {
        &self
    }

    /// Returns a mutable version of itself
    pub fn get_mut(&mut self) -> &mut Self {
        self
    }

    /// Checks if value is a none
    pub fn is_none(&self) -> bool {
        matches!(self, Value::None)
    }

    /// Checks if value is a float
    pub fn is_float(&self) -> bool {
        match self {
            Value::Number(n) => n.is_float(),
            _ => false,
        }
    }

    /// Attempts to convert the inner value of self into a f64. If the
    /// conversion fails None is returned instead
    ///
    /// Booleans can never be converted to a f64
    ///
    /// # Example
    ///
    /// ```rust
    /// let value = Value::String("123".to_string());
    ///
    /// println!("{}", value.as_float());
    /// // Output: 123.0
    /// ```
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Number(n) => n.as_float(),
            Value::String(s) => s.parse::<f64>().ok(),
            _ => None,
        }
    }

    /// Checks if value is a signed integer
    pub fn is_int(&self) -> bool {
        match self {
            Value::Number(n) => n.is_int(),
            _ => false,
        }
    }

    /// Attempts to convert the inner value of self into a i64. If the
    /// conversion fails None is returned instead
    ///
    /// Booleans can never be converted to a i64 and floats are rounded to the
    /// neareast whole number before converting
    ///
    /// # Example
    ///
    /// ```rust
    /// let value = Value::from(9.5);
    ///
    /// println!("{}", value.as_int());
    /// // Output: 10
    /// ```
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Number(n) => n.as_int(),
            Value::String(v) => v.parse::<i64>().ok(),
            _ => None,
        }
    }

    /// Checks if value is an unsigned integer
    pub fn is_uint(&self) -> bool {
        match self {
            Value::Number(n) => n.is_uint(),
            _ => false,
        }
    }

    /// Attempts to convert the inner value of self into a u64. If the
    /// conversion fails None is returned instead
    ///
    /// Booleans can never be converted to a u64 and floats are rounded to the
    /// neareast whole number before converting
    ///
    /// # Example
    ///
    /// ```rust
    /// let value = Value::from(10);
    ///
    /// println!("{}", value.as_uint());
    /// // Output: 10
    /// ```
    pub fn as_uint(&self) -> Option<u64> {
        match self {
            Value::Number(v) => v.as_uint(),
            Value::String(v) => v.parse::<u64>().ok(),
            _ => None,
        }
    }

    /// Checks if value is a string
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    /// Converts the inner value of self into a string and returns it
    ///
    /// This should not be confused with the Values' `to_string`
    /// function as this function returns the inner values to_string value while
    /// Values to_string function returns the InfluxDB line protocol
    /// representation of the inner value
    ///
    /// # Example
    ///
    /// ```rust
    /// let value = Value::from(123);
    ///
    /// println!("{}", value.as_string());
    /// // Output: 123
    ///
    /// println!("{}", value.to_string());
    /// // Output: 123i
    /// ```
    ///
    /// This function is here mainly to offer an alternative to the line
    /// protocol formatted datatypes
    pub fn as_string(&self) -> String {
        match self {
            Value::None => self.to_string(),
            Value::Number(n) => n.as_string(),
            Value::String(s) => s.to_string(),
            Value::Boolean(b) => b.to_string(),
        }
    }

    /// Checks if value is a float
    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }

    /// Attempts to convert the inner value of self into a boolean. Only a
    /// boolean can be converted into a boolean so using this function when
    /// Value is any other value will only return a None
    ///
    /// # Example
    ///
    /// ```rust
    /// let value = Value::Boolean(false);
    ///
    /// println!("{}", value.as_bool());
    /// // Output: false
    /// ```
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }
}
