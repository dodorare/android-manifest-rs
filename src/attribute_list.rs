use serde::{
    de::{self, DeserializeOwned, Visitor},
    ser::Error,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;
use std::io::{Read, Write};
use std::marker::PhantomData;
use yaserde::{YaDeserialize, YaSerialize};

pub trait Delimiter {
    fn delimiter_symbol() -> &'static str;
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Semicolon;

impl Delimiter for Semicolon {
    fn delimiter_symbol() -> &'static str {
        ";"
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct VerticalBar;

impl Delimiter for VerticalBar {
    fn delimiter_symbol() -> &'static str {
        "|"
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct AttributeList<D: Delimiter, T: Serialize + DeserializeOwned> {
    vec: Vec<T>,
    phantom: PhantomData<D>,
}

impl<D: Delimiter, T: Serialize + DeserializeOwned> AttributeList<D, T> {
    pub fn new() -> Self {
        AttributeList {
            vec: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        AttributeList {
            vec,
            phantom: PhantomData,
        }
    }

    pub fn vec(&self) -> &Vec<T> {
        &self.vec
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
}

impl<D: Delimiter, T: Serialize + DeserializeOwned> From<Vec<T>> for AttributeList<D, T> {
    fn from(vec: Vec<T>) -> Self {
        AttributeList {
            vec,
            phantom: PhantomData,
        }
    }
}

impl<D: Delimiter, T: Serialize + DeserializeOwned> Serialize for AttributeList<D, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.is_empty() {
            return Err(S::Error::custom("a value list can't be empty"));
        };
        serializer.serialize_str(
            &self
                .vec()
                .iter()
                .map(|v| serde_plain::to_string(v).unwrap())
                .collect::<Vec<String>>()
                .join(D::delimiter_symbol()),
        )
    }
}

impl<D: Delimiter, T: Serialize + DeserializeOwned> YaSerialize for AttributeList<D, T> {
    fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
        // if self.is_empty() {
        //     println!("1");
        //     return Err("a value list can't be empty".to_string());
        // };
        let _ret = writer.write(xml::writer::XmlEvent::characters(
            &self
                .vec()
                .iter()
                .map(|v| serde_plain::to_string(v).unwrap())
                .collect::<Vec<String>>()
                .join(D::delimiter_symbol()),
        ));
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

fn parse_list_with_delimiter<D: Delimiter, T: Serialize + DeserializeOwned>(
    v: &str,
) -> Result<AttributeList<D, T>, String> {
    if v.is_empty() {
        return Err(
            "there is no default value list. at least one value must be specified".to_string(),
        );
    };
    let values = v
        .replace(' ', "")
        .split(D::delimiter_symbol())
        .map(|s| serde_plain::from_str(s).unwrap())
        .collect();
    Ok(AttributeList::from_vec(values))
}

struct ListVisitor<D: Delimiter, T: Serialize + DeserializeOwned> {
    delimiter: PhantomData<D>,
    value_type: PhantomData<T>,
}

impl<D: Delimiter, T: Serialize + DeserializeOwned> ListVisitor<D, T> {
    pub fn new() -> Self {
        ListVisitor {
            delimiter: PhantomData,
            value_type: PhantomData,
        }
    }
}

impl<'de, D: Delimiter, T: Serialize + DeserializeOwned> Visitor<'de> for ListVisitor<D, T> {
    type Value = AttributeList<D, T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&format!(
            "an value list in format 'value1' or 'value1{}value2{}value3'",
            D::delimiter_symbol(),
            D::delimiter_symbol()
        ))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        parse_list_with_delimiter(v).map_err(|e| E::custom(e))
    }
}

impl<'de, D: Delimiter, T: Serialize + DeserializeOwned> Deserialize<'de> for AttributeList<D, T> {
    fn deserialize<De>(deserializer: De) -> Result<Self, De::Error>
    where
        De: Deserializer<'de>,
    {
        deserializer.deserialize_string(ListVisitor::new())
    }
}

impl<D: Delimiter, T: Serialize + DeserializeOwned> YaDeserialize for AttributeList<D, T> {
    fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        loop {
            match reader.next_event()? {
                xml::reader::XmlEvent::StartElement { .. } => {}
                xml::reader::XmlEvent::Characters(ref v) => {
                    return parse_list_with_delimiter(v);
                }
                _ => {
                    break;
                }
            }
        }
        Err("Unable to parse attribute".to_string())
    }
}
