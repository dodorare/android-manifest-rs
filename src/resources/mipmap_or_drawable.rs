use super::{
    DrawableResource, MipmapResource, Resource, ResourceType, ResourceVisitor,
    parse_resource_with_type,
};
use crate::xml::{XmlDeserialize, XmlSerialize};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Visitor},
};
use std::fmt;
use std::io::{Read, Write};

/// Enum used when the value can be string resource or just a row string.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MipmapOrDrawableResource {
    Mipmap(Resource<MipmapResource>),
    Drawable(Resource<DrawableResource>),
}

impl MipmapOrDrawableResource {
    pub fn mipmap(name: &str, package: Option<String>) -> MipmapOrDrawableResource {
        Self::Mipmap(MipmapResource::new(name, package))
    }

    pub fn drawable(name: &str, package: Option<String>) -> MipmapOrDrawableResource {
        Self::Drawable(DrawableResource::new(name, package))
    }
}

impl fmt::Display for MipmapOrDrawableResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mipmap(r) => write!(f, "{}", r),
            Self::Drawable(r) => write!(f, "{}", r),
        }
    }
}

impl Serialize for MipmapOrDrawableResource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Mipmap(r) => Serialize::serialize(&r, serializer),
            Self::Drawable(r) => Serialize::serialize(&r, serializer),
        }
    }
}

impl XmlSerialize for MipmapOrDrawableResource {
    fn serialize<W: Write>(
        &self,
        writer: &mut crate::xml::ser::Serializer<W>,
    ) -> Result<(), String> {
        match self {
            Self::Mipmap(r) => XmlSerialize::serialize(r, writer),
            Self::Drawable(r) => XmlSerialize::serialize(r, writer),
        }
    }
}

struct MipmapOrDrawableResourceVisitor;

impl<'de> Visitor<'de> for MipmapOrDrawableResourceVisitor {
    type Value = MipmapOrDrawableResource;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .write_str("an resource in format @mipmap/resource_name or @drawable/resource_name")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.is_empty() {
            return Err(E::custom("value of attribute is empty"));
        };
        if v.starts_with("@mipmap") {
            Ok(MipmapOrDrawableResource::Mipmap(
                ResourceVisitor::<MipmapResource>::new().visit_str(v)?,
            ))
        } else if v.starts_with("@drawable") {
            Ok(MipmapOrDrawableResource::Drawable(
                ResourceVisitor::<DrawableResource>::new().visit_str(v)?,
            ))
        } else {
            Err(E::custom(format!("wrong resource type: {}", v)))
        }
    }
}

impl<'de> Deserialize<'de> for MipmapOrDrawableResource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(MipmapOrDrawableResourceVisitor)
    }
}

impl XmlDeserialize for MipmapOrDrawableResource {
    fn deserialize<R: Read>(reader: &mut crate::xml::de::Deserializer<R>) -> Result<Self, String> {
        loop {
            match reader.next_event()? {
                xml::reader::XmlEvent::StartElement { .. } => {}
                xml::reader::XmlEvent::Characters(text_content) => {
                    if text_content.is_empty() {
                        return Err("value of attribute is empty".to_string());
                    };
                    if text_content.starts_with("@mipmap") {
                        return Ok(MipmapOrDrawableResource::Mipmap(parse_resource_with_type(
                            &text_content,
                        )?));
                    } else if text_content.starts_with("@drawable") {
                        return Ok(MipmapOrDrawableResource::Drawable(
                            parse_resource_with_type(&text_content)?,
                        ));
                    } else {
                        return Err(format!("wrong resource type: {}", text_content));
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
