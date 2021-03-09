mod string;

pub use string::*;

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

pub trait ResourceType: FromStr {
    fn resource_type() -> &'static str;
}

#[derive(Debug, PartialEq)]
pub struct Resource<T: ResourceType> {
    name: String,
    phantom: PhantomData<T>,
}

impl<T: ResourceType> Resource<T> {
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
        Ok(Resource {
            name: v.to_string(),
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
