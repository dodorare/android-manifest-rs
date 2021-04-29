use super::{parse_resource_with_type, Resource, ResourceType, ResourceVisitor, StringResource};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

/// Enum used when the value can be string resource or just a row string.
#[derive(Debug, PartialEq)]
pub enum StringResourceOrString {
    StringResource(Resource<StringResource>),
    String(String),
}

impl StringResourceOrString {
    pub fn resource(name: &str, package: Option<String>) -> StringResourceOrString {
        Self::StringResource(StringResource::new(name, package))
    }

    pub fn string(s: &str) -> StringResourceOrString {
        Self::String(s.to_string())
    }
}

impl Serialize for StringResourceOrString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            StringResourceOrString::StringResource(resource) => {
                Serialize::serialize(&resource, serializer)
            }
            StringResourceOrString::String(value) => serializer.serialize_str(value),
        }
    }
}

impl YaSerialize for StringResourceOrString {
    fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
        match self {
            StringResourceOrString::StringResource(resource) => {
                YaSerialize::serialize(resource, writer)?;
            }
            StringResourceOrString::String(value) => {
                let _ret = writer.write(xml::writer::XmlEvent::characters(value));
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

struct StringResourceOrStringVisitor;

impl<'de> Visitor<'de> for StringResourceOrStringVisitor {
    type Value = StringResourceOrString;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an string resource in format @string/resource_name or string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.is_empty() {
            return Err(E::custom("value of attribute is empty"));
        };
        if v.chars().next().unwrap() == '@' {
            Ok(StringResourceOrString::StringResource(
                ResourceVisitor::<StringResource>::new().visit_str(v)?,
            ))
        } else {
            Ok(StringResourceOrString::String(v.to_owned()))
        }
    }
}

impl<'de> Deserialize<'de> for StringResourceOrString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(StringResourceOrStringVisitor)
    }
}

impl YaDeserialize for StringResourceOrString {
    fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        loop {
            match reader.next_event()? {
                xml::reader::XmlEvent::StartElement { .. } => {}
                xml::reader::XmlEvent::Characters(text_content) => {
                    if text_content.is_empty() {
                        return Err("value of attribute is empty".to_string());
                    };
                    if text_content.chars().next().unwrap() == '@' {
                        return Ok(StringResourceOrString::StringResource(
                            parse_resource_with_type(&text_content)?,
                        ));
                    } else {
                        return Ok(StringResourceOrString::String(text_content));
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
