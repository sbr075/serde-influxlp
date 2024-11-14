use std::fmt;

use serde::{
    de::{self, DeserializeOwned, Visitor},
    Deserialize,
};

use crate::error::Error;

use super::datatypes::{Number, Value};

impl<'de> Deserialize<'de> for Number {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct NumberVisitor;

        impl<'de> Visitor<'de> for NumberVisitor {
            type Value = Number;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid influxdb v2 line protocol number")
            }

            fn visit_f64<E>(self, n: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Number::Float(n))
            }

            fn visit_i64<E>(self, n: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Number::Integer(n))
            }

            fn visit_u64<E>(self, n: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Number::UInteger(n))
            }
        }

        deserializer.deserialize_any(NumberVisitor)
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid influxdb v2 line protocol data type")
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::None)
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Number(Number::Float(v)))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Number(Number::Integer(v)))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Number(Number::UInteger(v)))
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::String(s.to_string()))
            }

            fn visit_bool<E>(self, b: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Boolean(b))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

macro_rules! deserialize_value {
    ($method:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value, Error>
        where
            V: de::Visitor<'de>,
        {
            self.visit(visitor)
        }
    };
}

impl<'de> de::Deserializer<'de> for Value {
    type Error = Error;

    deserialize_value!(deserialize_any);
    deserialize_value!(deserialize_bool);
    deserialize_value!(deserialize_i8);
    deserialize_value!(deserialize_i16);
    deserialize_value!(deserialize_i32);
    deserialize_value!(deserialize_i64);
    deserialize_value!(deserialize_u8);
    deserialize_value!(deserialize_u16);
    deserialize_value!(deserialize_u32);
    deserialize_value!(deserialize_u64);
    deserialize_value!(deserialize_f32);
    deserialize_value!(deserialize_f64);
    deserialize_value!(deserialize_char);
    deserialize_value!(deserialize_str);
    deserialize_value!(deserialize_string);

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::unsupported("byte deserialization"))
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::unsupported("byte buffer deserialization"))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::unsupported("newtype struct deserialization"))
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::unsupported("sequence deserialization"))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::unsupported("tuple deserialization"))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::unsupported("tuple struct deserialization"))
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::unsupported("map deserialization"))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::unsupported("struct deserialization"))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::unsupported("enum deserialization"))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

/// Attempt to deserialize a Value into type `T`. Can only convert to values
/// which are supported by InfluxDB v2 Line protocol
///
/// **Supported values**
/// 1. Numbers
/// 2. Strings
/// 3. Booleans
///
/// # Example
///
/// ```rust
/// let number = Value::Integer(123);
///
/// let value: f64 = serde_influxlp::from_value(number).unwrap();
/// println!("{value}");
/// // Output: 123
/// ```
pub fn from_value<T>(value: Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    T::deserialize(value)
}
