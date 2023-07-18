use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

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

impl ToString for VarOrBool {
    fn to_string(&self) -> String {
        match self {
            Self::Var(r) => r.to_string(),
            Self::Bool(v) => v.to_string(),
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

impl YaSerialize for VarOrBool {
    fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
        match self {
            VarOrBool::Var(variable) => {
                let _ret = writer.write(xml::writer::XmlEvent::characters(variable));
            }
            VarOrBool::Bool(value) => {
                let _ret = writer.write(xml::writer::XmlEvent::characters(&value.to_string()));
            }
        }
        Ok(())
    }

    fn serialize_attributes(
        &self,
        attributes: Vec<xml::attribute::OwnedAttribute>,
        namespace: xml::namespace::Namespace,
    ) -> Result<
        (
            Vec<xml::attribute::OwnedAttribute>,
            xml::namespace::Namespace,
        ),
        String,
    > {
        Ok((attributes, namespace))
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
        if v.starts_with("${") && v.ends_with("}") {
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
        deserializer.deserialize_string(VarOrBoolVisitor)
    }
}

impl YaDeserialize for VarOrBool {
    fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        loop {
            match reader.next_event()? {
                xml::reader::XmlEvent::StartElement { .. } => {}
                xml::reader::XmlEvent::Characters(text_content) => {
                    if text_content.is_empty() {
                        return Err("value of attribute is empty".to_string());
                    };
                    if text_content.starts_with("${") && text_content.ends_with("}") {
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
