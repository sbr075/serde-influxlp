use std::fmt::Display;

use conv::*;
use regex::Regex;
use serde::de;

use crate::error::Error;

/// Represents any supported InfluxDB v2 Line protocol value
#[derive(Debug, Clone)]
pub enum Value {
    /// Represent a floating point number field value
    Float(f64),

    /// Represent a signed integer number field value
    Integer(i64),

    /// Represent an unsigned integer number field value
    UInteger(u64),

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

                        Value::Integer(number)
                    }
                    false => {
                        let number = match value.parse() {
                            Ok(number) => number,
                            Err(_) => return None,
                        };

                        Value::UInteger(number)
                    }
                }
            }
            false => match value.parse::<f64>() {
                Ok(value) => Value::Float(value),
                Err(_) => return None,
            },
        };

        Some(number)
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
            Value::Float(n) => visitor.visit_f64(n),
            Value::Integer(n) => visitor.visit_i64(n),
            Value::UInteger(n) => visitor.visit_u64(n),
            Value::String(s) => visitor.visit_string(s),
            Value::Boolean(b) => visitor.visit_bool(b),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Value::Float(n) => format!("{n}"),
            Value::Integer(n) => format!("{n}i"),
            Value::UInteger(n) => format!("{n}i"),
            Value::String(s) => format!("{s}"),
            Value::Boolean(b) => format!("{b}"),
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
        Value::Float(n.into())
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Float(n)
    }
}

impl From<i8> for Value {
    fn from(n: i8) -> Self {
        Value::Integer(n.into())
    }
}

impl From<i16> for Value {
    fn from(n: i16) -> Self {
        Value::Integer(n.into())
    }
}

impl From<i32> for Value {
    fn from(n: i32) -> Self {
        Value::Integer(n.into())
    }
}

impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Value::Integer(n)
    }
}

impl From<u8> for Value {
    fn from(n: u8) -> Self {
        Value::UInteger(n.into())
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::UInteger(value.into())
    }
}

impl From<u32> for Value {
    fn from(n: u32) -> Self {
        Value::UInteger(n.into())
    }
}

impl From<u64> for Value {
    fn from(n: u64) -> Self {
        Value::UInteger(n)
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

    /// Checks if value is a float
    pub fn is_float(&self) -> bool {
        matches!(self, Value::Float(_))
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
            Value::Float(n) => Some(*n),
            Value::Integer(n) => f64::value_from(*n).ok(),
            Value::UInteger(n) => f64::value_from(*n).ok(),
            Value::String(s) => s.parse::<f64>().ok(),
            _ => None,
        }
    }

    /// Checks if value is a signed integer
    pub fn is_int(&self) -> bool {
        matches!(self, Value::Integer(_))
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
    /// let value = Value::Float(9.5);
    ///
    /// println!("{}", value.as_int());
    /// // Output: 10
    /// ```
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Float(v) => {
                // Ensure `f64` fits within `i64` range
                if *v >= i64::MIN as f64 && *v <= i64::MAX as f64 {
                    Some(v.round() as i64)
                } else {
                    None
                }
            }
            Value::Integer(v) => Some(*v),
            Value::UInteger(v) => i64::value_from(*v).ok(),
            Value::String(v) => v.parse::<i64>().ok(),
            _ => None,
        }
    }

    /// Checks if value is an unsigned integer
    pub fn is_uint(&self) -> bool {
        matches!(self, Value::UInteger(_))
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
    /// let value = Value::Integer(10);
    ///
    /// println!("{}", value.as_uint());
    /// // Output: 10
    /// ```
    pub fn as_uint(&self) -> Option<u64> {
        match self {
            Value::Float(v) => {
                // Ensure `f64` fits within `u64` range
                if *v >= u64::MIN as f64 && *v <= u64::MAX as f64 {
                    Some(v.round() as u64)
                } else {
                    None
                }
            }
            Value::Integer(v) => u64::value_from(*v).ok(),
            Value::UInteger(v) => Some(*v),
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
    /// let value = Value::Integer(123);
    ///
    /// println!("{}", value.as_string());
    /// // Output: 123
    ///
    /// println!("{}", value.to_string());
    /// // Output: 123i
    /// ```
    ///
    /// This function is here mainly to offer an alternative
    pub fn as_string(&self) -> String {
        match self {
            Value::Float(n) => n.to_string(),
            Value::Integer(n) => n.to_string(),
            Value::UInteger(n) => n.to_string(),
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
