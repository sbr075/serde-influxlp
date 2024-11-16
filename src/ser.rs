use std::{io, str::FromStr};

use serde::{
    ser::{self, Impossible, SerializeMap, SerializeSeq, SerializeStruct, SerializeTuple},
    Serialize,
};

use crate::{builder::Builder, datatypes::Element, Value};

use super::error::{Error, Result};

pub struct Serializer {
    builder: Builder,

    /// Current depth of the serialization
    ///
    /// Used to prevent map fields in tags / fields as they are not supported
    depth: usize,
}

impl Serializer {
    fn new() -> Self {
        Self {
            builder: Builder::new(),
            depth: 0,
        }
    }

    fn output(&mut self) -> String {
        self.builder.output()
    }

    fn build_line(&mut self) -> Result<()> {
        self.builder.build_line()
    }

    fn set_element(&mut self, element: Element) {
        self.builder.set_element(element);
    }

    fn add_key<T>(&mut self, key: T)
    where
        T: Into<Value>,
    {
        self.builder.add_value(key)
    }

    fn add_value<T>(&mut self, value: T) -> Result<()>
    where
        T: Into<Value>,
    {
        self.builder.add_value(value);
        Ok(())
    }

    fn remove_value(&mut self) -> Result<()> {
        self.builder.remove_value();
        Ok(())
    }
}

impl<'de> ser::Serializer for &'de mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = TypeSerializer<'de>;
    type SerializeTuple = TypeSerializer<'de>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = TypeSerializer<'de>;
    type SerializeStruct = TypeSerializer<'de>;
    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_bool(self, b: bool) -> Result<Self::Ok> {
        self.add_value(b)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.add_value(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.add_value(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.add_value(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        self.add_value(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.add_value(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.add_value(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.add_value(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        self.add_value(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        self.add_value(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        self.add_value(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.add_value(v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.add_value(v)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok> {
        Err(Error::unsupported("bytes serialization"))
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        self.remove_value()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.serialize_str(&variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::unsupported("newtype struct serialization"))
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::unsupported("newtype variant serialization"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(TypeSerializer { ser: self })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::unsupported("tuple struct serialization"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::unsupported("tuple variant serialization"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.depth += 1;
        if self.depth > 2 {
            return Err(Error::invalid_field_type("struct"));
        }

        Ok(TypeSerializer { ser: self })
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::unsupported("struct variant serialization"))
    }
}

pub struct TypeSerializer<'a> {
    ser: &'a mut Serializer,
}

impl<'a> SerializeSeq for TypeSerializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a> SerializeTuple for TypeSerializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

struct MapKeySerializer;

impl ser::Serializer for MapKeySerializer {
    type Ok = String;
    type Error = Error;

    type SerializeSeq = Impossible<String, Error>;
    type SerializeTuple = Impossible<String, Error>;
    type SerializeTupleStruct = Impossible<String, Error>;
    type SerializeTupleVariant = Impossible<String, Error>;
    type SerializeMap = Impossible<String, Error>;
    type SerializeStruct = Impossible<String, Error>;
    type SerializeStructVariant = Impossible<String, Error>;

    fn serialize_bool(self, v: bool) -> Result<String> {
        Ok(v.to_string())
    }

    fn serialize_i8(self, v: i8) -> Result<String> {
        Ok(itoa::Buffer::new().format(v).to_owned())
    }

    fn serialize_i16(self, v: i16) -> Result<String> {
        Ok(itoa::Buffer::new().format(v).to_owned())
    }

    fn serialize_i32(self, v: i32) -> Result<String> {
        Ok(itoa::Buffer::new().format(v).to_owned())
    }

    fn serialize_i64(self, v: i64) -> Result<String> {
        Ok(itoa::Buffer::new().format(v).to_owned())
    }

    fn serialize_u8(self, v: u8) -> Result<String> {
        Ok(itoa::Buffer::new().format(v).to_owned())
    }

    fn serialize_u16(self, v: u16) -> Result<String> {
        Ok(itoa::Buffer::new().format(v).to_owned())
    }

    fn serialize_u32(self, v: u32) -> Result<String> {
        Ok(itoa::Buffer::new().format(v).to_owned())
    }

    fn serialize_u64(self, v: u64) -> Result<String> {
        Ok(itoa::Buffer::new().format(v).to_owned())
    }

    fn serialize_f32(self, v: f32) -> Result<String> {
        if v.is_finite() {
            Ok(ryu::Buffer::new().format_finite(v).to_owned())
        } else {
            Err(Error::infinite_float())
        }
    }

    fn serialize_f64(self, v: f64) -> Result<String> {
        if v.is_finite() {
            Ok(ryu::Buffer::new().format_finite(v).to_owned())
        } else {
            Err(Error::infinite_float())
        }
    }

    fn serialize_char(self, v: char) -> Result<String> {
        Ok(v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<String> {
        Ok(v.to_string())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<String> {
        Err(Error::invalid_key())
    }

    fn serialize_none(self) -> Result<String> {
        Err(Error::invalid_key())
    }

    fn serialize_some<T>(self, _value: &T) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::invalid_key())
    }

    fn serialize_unit(self) -> Result<String> {
        Err(Error::invalid_key())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<String> {
        Err(Error::invalid_key())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<String> {
        Err(Error::invalid_key())
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::invalid_key())
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::invalid_key())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::invalid_key())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::invalid_key())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::invalid_key())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::invalid_key())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::invalid_key())
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::invalid_key())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::invalid_key())
    }
}

impl<'a> SerializeMap for TypeSerializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let key = key.serialize(MapKeySerializer)?;

        match Element::from_str(&key) {
            Ok(element) => {
                self.ser.set_element(element);
            }
            Err(_) => {
                self.ser.add_key(key);
            }
        }

        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok> {
        self.ser.depth -= 1;
        if self.ser.depth == 0 {
            self.ser.build_line()?;
        }

        Ok(())
    }
}

impl<'a> SerializeStruct for TypeSerializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<Self::Ok> {
        self.ser.depth -= 1;
        if self.ser.depth == 0 {
            self.ser.build_line()?;
        }

        Ok(())
    }
}

pub fn to_writer<W, T>(mut writer: W, value: &T) -> Result<()>
where
    W: io::Write,
    T: Serialize,
{
    let mut serializer = Serializer::new();
    value.serialize(&mut serializer)?;

    let output = serializer.output();
    writer.write_all(output.as_bytes())?;

    Ok(())
}

pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut writer = Vec::new();
    to_writer(&mut writer, value)?;
    Ok(writer)
}

/// Serialize a valid data structure to a InfluxDB V2 Line protocol
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
///     let metric = Metric {
///         measurement: "measurement".to_string(),
///         fields: Fields { field1: 123 },
///     };
///
///     let line = serde_influxlp::to_string(&metric).unwrap();
///     println!("{line}");
///     // Output: measurement field1=123i
/// }
/// ```
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let result = to_vec(value)?;
    let string = unsafe { String::from_utf8_unchecked(result) };

    Ok(string)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{de::from_str, Value};

    use super::*;

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "lowercase")]
    enum Measurement {
        Metric1,
        Metric2,
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct Tags {
        pub tag1: i32,
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct Fields {
        pub field1: String,

        pub field2: Option<bool>,
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct Metric {
        #[serde(rename = "measurement")]
        pub metric: Measurement,

        pub tags: Option<HashMap<String, Value>>,

        pub fields: Fields,

        pub timestamp: Option<i64>,
    }

    #[test]
    fn test_ser_to_string() {
        let metric = Metric {
            metric: Measurement::Metric1,
            tags: Some(HashMap::new()),
            fields: Fields {
                field1: "{\"hello\": \"world\"}".to_string(),
                field2: None,
            },
            timestamp: Some(1577836800),
        };

        let line = to_string(&metric);
        assert!(line.is_ok());
        let line = line.unwrap();

        let expected = "metric1 field1=\"{\\\"hello\\\": \\\"world\\\"}\" 1577836800";
        assert_eq!(line, expected);

        let metric = from_str::<Metric>(&line);
        assert!(metric.is_ok())
    }
}
