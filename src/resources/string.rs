use super::ResourceType;
use std::str::FromStr;

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
