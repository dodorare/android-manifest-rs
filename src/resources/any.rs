use super::{
    DrawableResource, Resource, ResourceType, StringResource, StyleResource, XmlResource,
    parse_resource,
};
use crate::xml::{XmlDeserialize, XmlSerialize};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Visitor},
};
use std::fmt;
use std::io::{Read, Write};

/// Enum used when the value can be any of available resources.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AnyResource {
    String(Resource<StringResource>),
    Drawable(Resource<DrawableResource>),
    Xml(Resource<XmlResource>),
    Style(Resource<StyleResource>),
}

impl Serialize for AnyResource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            AnyResource::String(r) => Serialize::serialize(&r, serializer),
            AnyResource::Drawable(r) => Serialize::serialize(&r, serializer),
            AnyResource::Xml(r) => Serialize::serialize(&r, serializer),
            AnyResource::Style(r) => Serialize::serialize(&r, serializer),
        }
    }
}

impl XmlSerialize for AnyResource {
    fn serialize<W: Write>(
        &self,
        writer: &mut crate::xml::ser::Serializer<W>,
    ) -> Result<(), String> {
        match self {
            AnyResource::String(r) => XmlSerialize::serialize(r, writer),
            AnyResource::Drawable(r) => XmlSerialize::serialize(r, writer),
            AnyResource::Xml(r) => XmlSerialize::serialize(r, writer),
            AnyResource::Style(r) => XmlSerialize::serialize(r, writer),
        }
    }
}

fn parse_any_resource(v: &str) -> Result<AnyResource, String> {
    if v.is_empty() {
        return Err("value of attribute is empty".to_string());
    };
    let (package, resource_type, name) = parse_resource(v)?;
    let any = if StringResource::resource_type() == resource_type {
        AnyResource::String(Resource::<StringResource>::new_with_package(&name, package))
    } else if DrawableResource::resource_type() == resource_type {
        AnyResource::Drawable(Resource::<DrawableResource>::new_with_package(
            &name, package,
        ))
    } else if XmlResource::resource_type() == resource_type {
        AnyResource::Xml(Resource::<XmlResource>::new_with_package(&name, package))
    } else if StyleResource::resource_type() == resource_type {
        AnyResource::Style(Resource::<StyleResource>::new_with_package(&name, package))
    } else {
        return Err(format!("unsuported resource type: {}", resource_type));
    };
    Ok(any)
}

struct AnyResourceVisitor;

impl<'de> Visitor<'de> for AnyResourceVisitor {
    type Value = AnyResource;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an resource in format @resource_type/resource_name")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        parse_any_resource(v).map_err(|e| E::custom(e))
    }
}

impl<'de> Deserialize<'de> for AnyResource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(AnyResourceVisitor)
    }
}

impl XmlDeserialize for AnyResource {
    fn deserialize<R: Read>(reader: &mut crate::xml::de::Deserializer<R>) -> Result<Self, String> {
        loop {
            match reader.next_event()? {
                xml::reader::XmlEvent::StartElement { .. } => {}
                xml::reader::XmlEvent::Characters(ref v) => {
                    return parse_any_resource(v);
                }
                _ => {
                    break;
                }
            }
        }
        Err("Unable to parse attribute".to_string())
    }
}
