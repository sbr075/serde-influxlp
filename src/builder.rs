use crate::{
    datatypes::Element,
    error::{Error, Result},
    Value,
};

#[derive(Debug, Clone, Default)]
struct LineBuilder {
    measurement: Option<Value>,

    tags: Option<Vec<Value>>,

    fields: Option<Vec<Value>>,

    timestamp: Option<Value>,
}

impl LineBuilder {
    fn set_measurement(&mut self, measurement: Value) {
        self.measurement = Some(measurement)
    }

    fn add_tag(&mut self, tag: Value) {
        self.tags.get_or_insert(Vec::new()).push(tag);
    }

    fn remove_tag(&mut self) {
        if let Some(tags) = &mut self.tags {
            tags.pop();
        }
    }

    fn add_field(&mut self, field: Value) {
        self.fields.get_or_insert(Vec::new()).push(field);
    }

    fn remove_field(&mut self) {
        if let Some(fields) = &mut self.fields {
            fields.pop();
        }
    }

    fn set_timestamp(&mut self, timestamp: Value) {
        self.timestamp = Some(timestamp)
    }

    fn reset(&mut self) {
        *self = LineBuilder::default();
    }

    fn escape_key(&self, value: &Value) -> String {
        match value {
            Value::String(s) => s
                .replace(r"\=", "=")
                .replace(r"\,", ",")
                .replace(r"\ ", " "),
            _ => value.as_string(),
        }
    }

    fn escape_tag(&self, value: &Value) -> String {
        match value {
            Value::String(s) => s
                .replace(r"\=", "=")
                .replace(r"\,", ",")
                .replace(r"\ ", " "),
            _ => value.to_string(),
        }
    }

    fn escape_field_value(&self, value: &Value) -> String {
        match value {
            Value::String(s) => {
                let escaped = s.replace("\\", "\\\\").replace("\"", "\\\"");
                format!("\"{escaped}\"")
            }
            _ => value.to_string(),
        }
    }

    fn build(&mut self) -> Result<String> {
        let mut line = String::new();
        match self.measurement {
            Some(ref measurement) => line.push_str(&measurement.to_string()),
            None => return Err(Error::missing_element("measurement")),
        }

        if let Some(tags) = self
            .tags
            .as_ref()
            .and_then(|v| if !v.is_empty() { Some(v) } else { None })
        {
            // We should not reach a state where the tag set is uneven but I am untrusting
            let tag_set: Vec<&[Value]> = tags.chunks(2).collect();
            if !tag_set.iter().all(|c| c.len() == 2) {
                return Err(Error::uneven_set("tag"));
            }

            let tags: Vec<String> = tag_set
                .into_iter()
                .map(|t| {
                    let key = self.escape_key(t.get(0).unwrap());
                    let value = self.escape_tag(t.get(1).unwrap());

                    format!("{key}={value}")
                })
                .collect();

            line = format!("{line},{}", tags.join(","))
        }

        match self.fields {
            Some(ref fields) => {
                if fields.is_empty() {
                    return Err(Error::missing_element("fields"));
                }

                // We should not reach a state where the tag set is uneven but I am untrusting
                let field_set: Vec<&[Value]> = fields.chunks(2).collect();
                if !field_set.iter().all(|c| c.len() == 2) {
                    return Err(Error::uneven_set("field"));
                }

                let fields: Vec<String> = field_set
                    .into_iter()
                    .map(|f| {
                        let key = self.escape_key(f.get(0).unwrap());
                        let value = self.escape_field_value(f.get(1).unwrap());

                        format!("{key}={value}")
                    })
                    .collect();

                line = format!("{line} {}", fields.join(","))
            }
            None => return Err(Error::missing_element("fields")),
        }

        if let Some(ref timestamp) = self.timestamp {
            line = format!("{line} {}", timestamp.as_string())
        }

        self.reset();
        Ok(line)
    }
}

pub(crate) struct Builder {
    builder: LineBuilder,

    lines: Vec<String>,

    curr: Element,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            builder: LineBuilder::default(),
            lines: Vec::new(),
            curr: Element::Measurement,
        }
    }

    pub fn output(&self) -> String {
        self.lines.join("\n")
    }

    pub fn build_line(&mut self) -> Result<()> {
        let line = self.builder.build()?;
        self.lines.push(line);

        Ok(())
    }

    pub fn set_element(&mut self, element: Element) {
        self.curr = element;
    }

    pub fn add_value<T>(&mut self, value: T)
    where
        T: Into<Value>,
    {
        let value = value.into();
        if value.is_none() {
            return;
        }

        match self.curr {
            Element::Measurement => self.builder.set_measurement(value),
            Element::Tags => self.builder.add_tag(value),
            Element::Fields => self.builder.add_field(value),
            Element::Timestamp => self.builder.set_timestamp(value),
        }
    }

    pub fn remove_value(&mut self) {
        // Measurement and timestamp does not have keys that can be added before finding
        // out the value was None
        match self.curr {
            Element::Tags => self.builder.remove_tag(),
            Element::Fields => self.builder.remove_field(),
            _ => (),
        }
    }
}
