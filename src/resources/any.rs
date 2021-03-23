use super::{
    parse_resource, DrawableResource, Resource, ResourceType, StringResource, StyleResource,
    XmlResource,
};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;

/// Enum used when the value can be any of available resources.
#[derive(Debug, PartialEq)]
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
            AnyResource::String(r) => r.serialize(serializer),
            AnyResource::Drawable(r) => r.serialize(serializer),
            AnyResource::Xml(r) => r.serialize(serializer),
            AnyResource::Style(r) => r.serialize(serializer),
        }
    }
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
        if v.is_empty() {
            return Err(E::custom("value is empty"));
        };
        let (package, resource_type, name) = parse_resource(v).map_err(|e| E::custom(e))?;
        let any = if StringResource::resource_type() == resource_type {
            AnyResource::String(Resource::<StringResource>::new_with_package(name, package))
        } else if DrawableResource::resource_type() == resource_type {
            AnyResource::Drawable(Resource::<DrawableResource>::new_with_package(
                name, package,
            ))
        } else if XmlResource::resource_type() == resource_type {
            AnyResource::Xml(Resource::<XmlResource>::new_with_package(name, package))
        } else if StyleResource::resource_type() == resource_type {
            AnyResource::Style(Resource::<StyleResource>::new_with_package(name, package))
        } else {
            return Err(E::custom(format!(
                "unsuported resource type: {}",
                resource_type
            )))?;
        };
        Ok(any)
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
