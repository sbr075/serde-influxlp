use std::io;

use regex::Regex;
use serde::{
    de::{self, value::StringDeserializer, DeserializeOwned, IntoDeserializer},
    Deserialize,
};

use crate::{
    reader::{self, Reader},
    Value,
};

use self::reader::datatypes::Position;

use super::error::{Error, Result};

struct Deserializer<R> {
    reader: R,
}

impl<'de, R> Deserializer<R>
where
    R: Reader<'de>,
{
    fn from_reader(reader: R) -> Self {
        Deserializer { reader }
    }

    fn reader_position(&self) -> Position {
        self.reader.get_position()
    }

    fn include_tags(&mut self) {
        self.reader.include_tags();
    }

    fn has_next_line(&mut self) -> bool {
        self.reader.has_next_line()
    }

    fn set_next_line(&mut self) {
        self.reader.set_next_line()
    }

    fn has_next_key(&mut self) -> Result<bool> {
        self.reader.has_next_key()
    }

    fn get_next_key(&mut self) -> Result<String> {
        self.reader.get_next_key()
    }

    fn get_next_value(&mut self) -> Result<String> {
        self.reader.get_next_value()
    }

    fn discard_next_value(&mut self) -> Result<()> {
        self.reader.discard_next_value()
    }
}

macro_rules! deserialize_integer {
    ($method:ident, $visit:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            let mut value = self.get_next_value()?;

            // Check if element is a valid number
            let re = Regex::new(r"^-?\d+i?$").unwrap();
            let result = match re.is_match(&value) {
                true => {
                    // Remove integer indicator
                    if value.ends_with("i") {
                        value.pop();
                    }

                    value.parse()
                }
                false => return Err(Error::invalid_value(value, self.reader_position())),
            };

            match result {
                Ok(value) => visitor.$visit(value),
                Err(_) => Err(Error::invalid_value(value, self.reader_position())),
            }
        }
    };
}

impl<'de, R: Reader<'de>> de::Deserializer<'de> for &mut Deserializer<R> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let value = self.get_next_value()?;
        let result = Value::from_any_str(&value).visit(visitor);

        match result {
            Ok(value) => Ok(value),
            Err(_) => Err(Error::invalid_value(value, self.reader_position())),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let element = self.get_next_value()?;
        let value = match Value::from_bool_str(&element) {
            Some(value) => value.visit(visitor),
            None => {
                return Err(Error::invalid_type(
                    &element,
                    "bool",
                    self.reader_position(),
                ))
            }
        };

        match value {
            Ok(value) => Ok(value),
            Err(_) => Err(Error::invalid_type(
                &element,
                "bool",
                self.reader_position(),
            )),
        }
    }

    deserialize_integer!(deserialize_i8, visit_i8);
    deserialize_integer!(deserialize_i16, visit_i16);
    deserialize_integer!(deserialize_i32, visit_i32);
    deserialize_integer!(deserialize_i64, visit_i64);
    deserialize_integer!(deserialize_u8, visit_u8);
    deserialize_integer!(deserialize_u16, visit_u16);
    deserialize_integer!(deserialize_u32, visit_u32);
    deserialize_integer!(deserialize_u64, visit_u64);

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let element = self.get_next_value()?;
        let value = match element.parse() {
            Ok(value) => value,
            Err(_) => return Err(Error::invalid_type(&element, "f32", self.reader_position())),
        };

        visitor.visit_f32(value)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let element = self.get_next_value()?;
        let value = match element.parse() {
            Ok(value) => value,
            Err(_) => return Err(Error::invalid_type(&element, "f64", self.reader_position())),
        };

        visitor.visit_f64(value)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let element = self.get_next_value()?;
        let len = element.chars().count();
        if len != 1 {
            return Err(Error::invalid_char(element, len, self.reader_position()));
        }
        visitor.visit_char(element.chars().next().unwrap())
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.get_next_value().and_then(|e| visitor.visit_str(&e))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.get_next_value().and_then(|e| visitor.visit_str(&e))
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::unsupported("byte deserialization"))
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::unsupported("byte buffer deserialization"))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::unsupported("newtype struct deserialization"))
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(SeqDeserializer::new(self))
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::unsupported("tuple struct deserialization"))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if fields.contains(&"tags") {
            self.include_tags();
        };

        visitor.visit_map(self)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::unsupported("identifier deserialization"))
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.discard_next_value()?;
        visitor.visit_unit()
    }
}

impl<'a, R: Reader<'a>> de::MapAccess<'a> for Deserializer<R> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'a>,
    {
        if !self.has_next_key()? {
            return Ok(None);
        }

        let key = self.get_next_key()?;
        seed.deserialize(StringDeserializer::new(key)).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'a>,
    {
        seed.deserialize(self)
    }
}

struct SeqDeserializer<'a, R: 'a> {
    de: &'a mut Deserializer<R>,

    first: bool,
}

impl<'a, R: 'a> SeqDeserializer<'a, R> {
    fn new(de: &'a mut Deserializer<R>) -> Self {
        SeqDeserializer { de, first: true }
    }
}

impl<'de, 'a, R: Reader<'de> + 'a> de::SeqAccess<'de> for SeqDeserializer<'a, R> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        // Skip the check and next line fetching if this is the first access
        match !self.first {
            true => {
                if !self.de.has_next_line() {
                    return Ok(None);
                }

                self.de.set_next_line();
            }
            false => self.first = false,
        }

        seed.deserialize(&mut *self.de).map(Some)
    }
}

impl<'a, R: Reader<'a>> de::EnumAccess<'a> for &mut Deserializer<R> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'a>,
    {
        let variant_name = self.get_next_value()?;
        seed.deserialize(variant_name.into_deserializer())
            .map(|v| (v, self))
    }
}

impl<'a, R: Reader<'a>> de::VariantAccess<'a> for &mut Deserializer<R> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'a>,
    {
        seed.deserialize(self)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'a>,
    {
        de::Deserializer::deserialize_seq(self, visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'a>,
    {
        de::Deserializer::deserialize_struct(self, "", fields, visitor)
    }
}

pub fn from_reader<T>(r: impl io::Read) -> Result<T>
where
    T: DeserializeOwned,
{
    let mut deserializer = Deserializer::from_reader(reader::IoReader::new(r));
    let value = T::deserialize(&mut deserializer)?;

    Ok(value)
}

pub fn from_slice<'a, T>(s: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_reader(reader::SliceReader::new(s));
    let value = T::deserialize(&mut deserializer)?;

    Ok(value)
}

/// Deserialize a valid line protocol string into a struct T
///
/// # Example
///
/// Below is an example of the least required for serialization to succeed
///
/// ```rust
/// use serde_influxlp::Value;
///
/// #[derive(Debug, Serialize, Deserialize)]
/// pub struct Fields {
///     pub field1: i32,
/// }
///
/// #[derive(Debug, Serialize, Deserialize)]
/// pub struct Metric {
///     pub measurement: String,
///
///     pub fields: Fields,
/// }
///
/// fn main() {
///     let line = "measurement field1=123i";
///
///     let metric: Metric = serde_influxlp::from_str(line).unwrap();
///     println!("{metric:#?}");
///     // Output Metric {
///     //     measurement: "measurement",
///     //     fields: Fields {
///     //         field1: 123,
///     //     },
///     // }
/// }
/// ```
pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    from_slice(s.as_bytes())
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, serde::Deserialize)]
    #[serde(rename_all = "lowercase")]
    enum Exposure {
        Public,
        Private,
    }

    #[derive(Debug, serde::Deserialize)]
    #[allow(dead_code)]
    struct Tags {
        pub tag1: i32,

        pub tag2: Option<String>,

        pub tag3: Exposure,
    }

    #[derive(Debug, serde::Deserialize)]
    #[allow(dead_code)]
    struct Fields {
        pub field1: i32,

        pub field2: bool,
    }

    #[derive(Debug, serde::Deserialize)]
    #[allow(dead_code)]
    struct Metric {
        pub measurement: String,

        pub tags: Tags,

        pub fields: Fields,

        pub timestamp: i64,
    }

    #[test]
    fn test_de_from_str() {
        let line = "metric1,tag1=123,tag3=private field1=321,field2=t 123456789";
        let result = from_str::<Metric>(line);
        assert!(result.is_ok());

        let lines = r#"
        metric1,tag1=123,tag3=public field1=321,field2=t 123456789
        #comment line

        metric2,tag1=321,tag2=hello\ world,tag3=private field1=123,field2=True 123456789

        #another comment line

        "#;
        let result = from_str::<Vec<Metric>>(lines);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.len(), 2);

        let line = "metric1,tag1=123,tag3=private field1=321,field2=t 123456789";
        let result = from_str::<Vec<Metric>>(line);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.len(), 1);

        let line = r#"
                
        "#;
        let result = from_str::<Metric>(line);
        assert!(result.is_err());
    }

    #[test]
    fn test_de_from_reader() {
        let line = "metric1,tag1=123,tag3=private field1=321,field2=t 123456789".as_bytes();
        let result = from_reader::<Metric>(line);
        assert!(result.is_ok());

        let lines = r#"
        metric1,tag1=123,tag3=public field1=321,field2=t 123456789
        #comment line

        metric2,tag1=321,tag2=hello\ world,tag3=private field1=123,field2=True 123456789

        #another comment line

        "#
        .as_bytes();
        let result = from_reader::<Vec<Metric>>(lines);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.len(), 2);

        let line = "metric1,tag1=123,tag3=private field1=321,field2=t 123456789".as_bytes();
        let result = from_reader::<Vec<Metric>>(line);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.len(), 1);

        let line = r#"
                
        "#
        .as_bytes();
        let result = from_reader::<Metric>(line);
        assert!(result.is_err());
    }
}
