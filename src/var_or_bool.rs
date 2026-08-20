use crate::xml::{XmlDeserialize, XmlSerialize};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Visitor},
};
use std::fmt;
use std::io::{Read, Write};

/// Enum used when the value can be string resource or just a row string.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VarOrBool {
    Var(String),
    Bool(bool),
}

impl Default for VarOrBool {
    fn default() -> Self {
        Self::bool(false)
    }
}

impl From<bool> for VarOrBool {
    fn from(value: bool) -> Self {
        Self::bool(value)
    }
}

impl From<&str> for VarOrBool {
    fn from(value: &str) -> Self {
        Self::var(value)
    }
}

impl VarOrBool {
    pub fn var(name: impl Into<String>) -> VarOrBool {
        Self::Var(name.into())
    }

    pub fn bool(s: bool) -> VarOrBool {
        Self::Bool(s)
    }
}

impl fmt::Display for VarOrBool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Var(r) => write!(f, "{}", r),
            Self::Bool(v) => write!(f, "{}", v),
        }
    }
}

impl Serialize for VarOrBool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            VarOrBool::Var(variable) => Serialize::serialize(&variable, serializer),
            VarOrBool::Bool(value) => serializer.serialize_bool(*value),
        }
    }
}

impl XmlSerialize for VarOrBool {
    fn serialize<W: Write>(
        &self,
        writer: &mut crate::xml::ser::Serializer<W>,
    ) -> Result<(), String> {
        writer
            .write(xml::writer::XmlEvent::characters(&self.to_string()))
            .map_err(|error| error.to_string())
    }
}

struct VarOrBoolVisitor;

impl<'de> Visitor<'de> for VarOrBoolVisitor {
    type Value = VarOrBool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a boolean value or a variable in the \"${variable}\" format")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.into())
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.is_empty() {
            return Err(E::custom("value of attribute is empty"));
        };
        if v.starts_with("${") && v.ends_with('}') {
            Ok(VarOrBool::var(v))
        } else {
            Ok(VarOrBool::Bool(v.parse().map_err(|_| {
                E::custom(format!("value `{v}` is not a valid boolean"))
            })?))
        }
    }
}

impl<'de> Deserialize<'de> for VarOrBool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(VarOrBoolVisitor)
    }
}

impl XmlDeserialize for VarOrBool {
    fn deserialize<R: Read>(reader: &mut crate::xml::de::Deserializer<R>) -> Result<Self, String> {
        loop {
            match reader.next_event()? {
                xml::reader::XmlEvent::StartElement { .. } => {}
                xml::reader::XmlEvent::Characters(text_content) => {
                    if text_content.is_empty() {
                        return Err("value of attribute is empty".to_string());
                    };
                    if text_content.starts_with("${") && text_content.ends_with('}') {
                        return Ok(VarOrBool::Var(text_content));
                    } else {
                        return Ok(VarOrBool::Bool(text_content.parse().map_err(|_| {
                            format!("value {text_content} is not a valid boolean")
                        })?));
                    }
                }
                _ => {
                    break;
                }
            }
        }
        Err("Unable to parse attribute".to_string())
    }
}
