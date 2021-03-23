mod any;
mod res_or_string;
mod types;

pub use any::*;
pub use res_or_string::*;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;
pub use types::*;

/// Trait implemented by types that can be used as resource.
pub trait ResourceType: FromStr {
    /// Creates new instance of [`Resource`](crate::Resource).
    fn new(name: &str, package: Option<String>) -> Resource<Self> {
        Resource {
            name: name.to_owned(),
            package,
            phantom: PhantomData,
        }
    }
    /// Returns string representation of the `resource_type`.
    fn resource_type() -> &'static str;
}

/// Generic resource type.
#[derive(Debug, PartialEq)]
pub struct Resource<T: ResourceType> {
    name: String,
    package: Option<String>,
    phantom: PhantomData<T>,
}

impl<T: ResourceType> Resource<T> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            package: None,
            phantom: PhantomData,
        }
    }

    pub fn new_with_package(name: String, package: Option<String>) -> Self {
        Self {
            name,
            package,
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
        if let Some(package) = &self.package {
            serializer.serialize_str(&format!(
                "@{}:{}/{}",
                package,
                T::resource_type(),
                self.name
            ))
        } else {
            serializer.serialize_str(&format!("@{}/{}", T::resource_type(), self.name))
        }
    }
}

struct ResourceVisitor<T: ResourceType> {
    phantom: PhantomData<T>,
}

impl<T: ResourceType> ResourceVisitor<T> {
    pub fn new() -> Self {
        ResourceVisitor {
            phantom: PhantomData,
        }
    }
}

impl<'de, T: ResourceType> Visitor<'de> for ResourceVisitor<T> {
    type Value = Resource<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&format!(
            "an {} resource in format @[package:]{}/resource_name",
            T::resource_type(),
            T::resource_type()
        ))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let (package, resource_type, resource_name) =
            parse_resource(v).map_err(|e| E::custom(e))?;
        if resource_type != T::resource_type() {
            return Err(E::custom(format!(
                "a wrong resource type, expected @[package:]{}/{}, found {}",
                T::resource_type(),
                resource_name,
                v
            )));
        };
        Ok(Resource {
            name: resource_name,
            package: package,
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

/// Parses a resource string in format
/// `@[package:]resource_type/resource_name` into three parts
fn parse_resource(resource: &str) -> Result<(Option<String>, String, String), &str> {
    let split_str: Vec<_> = resource.split('/').collect();
    if split_str.len() != 2 {
        return Err(
            "a wrong resource format, expected format @[package:]resource_type/resource_name",
        );
    };
    let first_part = split_str.get(0).unwrap();
    let resource_type = &first_part[1..];
    let split_type: Vec<_> = resource_type.split(':').collect();
    let (resource_type, package) = if split_type.len() == 2 {
        (split_type[1], Some(split_type[0].to_string()))
    } else {
        (split_type[0], None)
    };
    let resource_name = split_str.get(1).unwrap();
    Ok((
        package,
        resource_type.to_string(),
        resource_name.to_string(),
    ))
}
