# Serde InfluxDB V2 Line Protocol

[![Build and release workflow][build-image]][build]
[![Influx line protocol tools on crates.io][crates.io-image]][crates.io]

*Note! This crate is still in its early stages*

*A thank you to the team behind [serde_json](https://crates.io/crates/serde_json). It is no secret that this crate has taken inspiration from serde_json. Creating a serializer-/deserializer without any knowledge about serde, except for using its derive traits and creating simple serializer-/deserializers for structs and enums, has been challenging at times. It has been a much welcome guiding rod!*

---

[build-image]: https://github.com/sbr075/serde-influxlp/actions/workflows/release.yml/badge.svg
[build]: https://github.com/sbr075/serde-influxlp/actions/workflows/release.yml
[crates.io-image]: https://img.shields.io/badge/crates.io-serde--influxlp-orange
[crates.io]: https://crates.io/crates/serde-influxlp

</br>

**Serde Influxlp is a *ser*ializer and *de*serializer for InfluxDB v2 Line Protocol**

InfluxDB's line protocol is a text-based format used to represent data points. It includes the measurement, optional tag set, field set, and an optional timestamp.

```
measurement         tag set             field set              timestamp
----------- ------------------- ------------------------- -------------------
measurement,tag1=val1,tag2=val2 field1="val1",field2=true 1729270461612452700
```

## Documentation  
- [Influxlp API](https://docs.rs/serde_influxlp)
- [Line protocol](https://docs.influxdata.com/influxdb/v2/reference/syntax/line-protocol/)
- [Best practices](https://docs.influxdata.com/influxdb/v2/write-data/best-practices/)

## Usage
Add these two crates to your `Cargo.toml`
```toml
[dependencies]
serde = "1.0.215"
serde_influxlp = "0.1.1"
```

### Limitations
Unfortunately due to the required format of the line protocol (as seen above) the struct, although created by the user, has a required format that must be followed closely. Some serde features are also currently unsupported, e.g., tagging. Below are different examples of how to setup and customize your struct.

This crate does not support any type which is not [supported](https://docs.influxdata.com/influxdb/v2/reference/syntax/line-protocol/#data-types-and-format) by InfluxDB v2 Line Protocol. These types are: 
1. Any number (int, uint, float)
2. Strings
3. Booleans

In the line protocol tag keys-/values and field keys-/values also has [restricted types](https://docs.influxdata.com/influxdb/v2/reference/syntax/line-protocol/#elements-of-line-protocol). This crate supports serializing and deserializing back-and-forth from types. Bare in mind however that InfluxDB will treat the element value as its expected type.

**Examples**

Below is the bare minimum required in a struct to be serialized and deserialized successfully. At most one entry is required in the fields map. `Value` is a custom enum for this crate.

```rust
use serde_influxlp::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Metric {
    pub measurement: String,

    pub fields: HashMap<String, Value>,
}
```

</br>

Tags and a timestamp field can also be added to the struct. If in some cases its uncertain whether the fields are present they can be marked as an `Option`. Timestamp should always be an i64 as in the line protocol timestamp is a unix timestamp with the same range as an i64.

</br>

```rust
use serde_influxlp::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tags {
    pub tag1: i32,

    pub tag2: Option<String>,
    ...
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metric {
    pub measurement: String,

    pub tags: Option<Tags>,

    pub fields: HashMap<String, Value>,

    pub timestamp: Option<i64>,
}
```

</br>

Tags and fields can also be defined as structs of their own if the values are known beforehand, or if you only want to serialize/deserialize a few fields. As with the tags and timestamp these can be marked as an `Option`

</br>

```rust
use serde_influxlp::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tags {
    pub tag1: i32,

    pub tag2: Option<String>,
    ...
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fields {
    pub field1: String,

    pub field2: Option<bool>,
    ...
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metric {
    pub measurement: String,

    pub tags: Tags,

    pub fields: Fields,

    pub timestamp: i64,
}
```

</br>

Enums are also supported

</br>

```rust
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Measurement {
    Metric1,
    Metric2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metric {
    pub measurement: Measurement,

    pub tags: Tags,

    pub fields: Fields,

    pub timestamp: i64,
}
```

</br>

As said previously we require the field names to be as they are because they are used to ensure that the serialization and deserialization is done properly. If you want to rename the fields it can be done with serdes rename feature

</br>

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Metric {
    #[serde(rename = "measurement")]
    pub name: String,

    #[serde(rename = "tags")]
    pub tag_set: Tags,

    #[serde(rename = "fields")]
    pub field_set: Fields,

    #[serde(rename = "timestamp")]
    pub time: i64,
}
```

</br>

### Serialization and deserialization
```rust
use serde_influxlp::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Measurement {
    Metric1,
    Metric2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tags {
    pub tag1: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fields {
    pub field1: String,

    pub field2: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metric {
    pub measurement: Measurement,

    pub tags: HashMap<String, Value>,

    pub fields: Fields,

    pub timestamp: Option<i64>,
}

fn main() {
    let metric = Metric {
        measurement: Measurement::Metric1,
        tags: HashMap::from([
            ("tag1".to_string(), Value::from(12.5)),
            ("tag2".to_string(), Value::from(25)),
        ]),
        fields: Fields {
            field1: "{\"hello\": \"world\"}".to_string(),
            field2: None,
        },
        timestamp: Some(1577836800),
    };

    let string = serde_influxlp::to_string(&metric).unwrap();

    // Output:
    // metric1,tag1=10.5 field1="{\"hello\": \"world\"}" 1577836800

    let metric: Metric = serde_influxlp::from_str(&string).unwrap()

    // Output:
    // Metric {
    //     measurement: Metric1,
    //     tags: {
    //         "tag1": Float(
    //             10.5,
    //         ),
    //     },
    //     fields: Fields {
    //         field1: "{\"hello\": \"world\"}",
    //         field2: None,
    //     },
    //     timestamp: Some(1577836800),
    // }

    // A line protocol can also be made by hand
    let string = "metric2 field1=\"Hello, reader!\",field2=t";
    let metric: Metric = from_str(&string).unwrap();

    // Output:
    // Metric {
    //     measurement: Metric2,
    //     tags: None,
    //     fields: Fields {
    //         field1: "Hello, reader!",
    //         field2: Some(
    //             true,
    //         ),
    //     },
    //     timestamp: None,
    // }
}
```

Tip: You can deserialize a line protocol string to a struct, then add, remove, or edit its values before serializing again to change the line protocol.

## Issues, new features, or contributions
If you discover any issues, find missing features that would make the crate better, or would like to contribute to the crate yourself go to the projects [GitHub](https://github.com/sbr075/serde-influxlp) and open a new issue or pull request. In advance, thank you!

## License

This project is licensed under either the [APACHE License](LICENSE-APACHE) or the [MIT License](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.