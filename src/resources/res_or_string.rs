use super::{Resource, ResourceVisitor, StringResource};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum StringResourceOrString {
    StringResource(Resource<StringResource>),
    String(String),
}

impl Serialize for StringResourceOrString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            StringResourceOrString::StringResource(resource) => resource.serialize(serializer),
            StringResourceOrString::String(value) => serializer.serialize_str(value),
        }
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
            return Err(E::custom("value is empty"));
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
