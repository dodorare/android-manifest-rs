use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

pub trait ResourceType: FromStr {
    fn new(name: &str) -> Resource<Self> {
        Resource {
            name: name.to_owned(),
            phantom: PhantomData,
        }
    }
    fn resource_type() -> &'static str;
}

#[derive(Debug, PartialEq)]
pub struct Resource<T: ResourceType> {
    name: String,
    phantom: PhantomData<T>,
}

impl<T: ResourceType> Resource<T> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            phantom: PhantomData,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn resource_type(&self) -> &'static str {
        T::resource_type()
    }
}

impl<T: ResourceType> Serialize for Resource<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("@{}/{}", T::resource_type(), self.name))
    }
}

struct ResourceVisitor<T: ResourceType> {
    phantom: PhantomData<T>,
}

impl<'de, T: ResourceType> Visitor<'de> for ResourceVisitor<T> {
    type Value = Resource<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&format!(
            "an {} resource in format @{}/resource_name",
            T::resource_type(),
            T::resource_type()
        ))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let split_str: Vec<_> = v.split('/').collect();
        if split_str.len() != 2 {
            return Err(E::custom(format!(
                "an {} resource in format @{}/resource_name",
                T::resource_type(),
                T::resource_type()
            )));
        };
        let first_part = split_str.get(0).unwrap();
        let resource_type = &first_part[1..];
        let resource_name = split_str.get(1).unwrap();
        if resource_type != T::resource_type() {
            return Err(E::custom(format!(
                "a wrong resource type, expected @{}/{} found {}",
                T::resource_type(),
                resource_name,
                v
            )));
        };
        Ok(Resource {
            name: resource_name.to_string(),
            phantom: PhantomData,
        })
    }
}

impl<'de, T: ResourceType> Deserialize<'de> for Resource<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(ResourceVisitor {
            phantom: PhantomData,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct StringResource;

impl FromStr for StringResource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "string" {
            Ok(StringResource)
        } else {
            Err(format!("failed to convert {} to string recource type", s))
        }
    }
}

impl ResourceType for StringResource {
    fn resource_type() -> &'static str {
        "string"
    }
}

#[derive(Debug, PartialEq)]
pub struct DrawableResource;

impl FromStr for DrawableResource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "drawable" {
            Ok(DrawableResource)
        } else {
            Err(format!("failed to convert {} to drawable resource type", s))
        }
    }
}

impl ResourceType for DrawableResource {
    fn resource_type() -> &'static str {
        "drawable"
    }
}

#[derive(Debug, PartialEq)]
pub struct XmlResource;

impl FromStr for XmlResource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "xml" {
            Ok(XmlResource)
        } else {
            Err(format!("failed to convert {} to xml resource type", s))
        }
    }
}

impl ResourceType for XmlResource {
    fn resource_type() -> &'static str {
        "xml"
    }
}

#[derive(Debug, PartialEq)]
pub struct StyleResource;

impl FromStr for StyleResource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "style" {
            Ok(StyleResource)
        } else {
            Err(format!("failed to convert {} to style resource type", s))
        }
    }
}

impl ResourceType for StyleResource {
    fn resource_type() -> &'static str {
        "style"
    }
}
