use super::ResourceType;
use std::str::FromStr;

/// String resource type.
#[derive(Debug, PartialEq, Clone)]
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

/// Drawable resource type.
#[derive(Debug, PartialEq, Clone)]
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

/// Xml resource type.
#[derive(Debug, PartialEq, Clone)]
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

/// Style resource type.
#[derive(Debug, PartialEq, Clone)]
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
