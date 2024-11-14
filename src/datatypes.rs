use std::str::FromStr;

#[derive(Debug, Clone)]
pub(crate) enum Element {
    Measurement,

    Tags,

    Fields,

    Timestamp,
}

impl FromStr for Element {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let element = match s {
            "measurement" => Element::Measurement,
            "tags" => Element::Tags,
            "fields" => Element::Fields,
            "timestamp" => Element::Timestamp,
            _ => return Err(()),
        };

        Ok(element)
    }
}

impl Element {
    pub(crate) fn is_measurement(&self) -> bool {
        matches!(self, Element::Measurement)
    }

    pub(crate) fn is_tags(&self) -> bool {
        matches!(self, Element::Tags)
    }
}
