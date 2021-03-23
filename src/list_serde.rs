use quick_xml::{de::from_str, se::to_string};
use serde::{
    de::{self, DeserializeOwned, Visitor},
    ser::Error,
    Deserializer, Serialize, Serializer,
};
use std::fmt;
use std::marker::PhantomData;

fn serialize_list<S, T>(delimiter: &str, values: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    if values.is_empty() {
        return Err(S::Error::custom("a value list can't be empty"));
    };
    serializer.serialize_str(
        &values
            .into_iter()
            .map(|v| to_string(v).unwrap())
            .collect::<Vec<String>>()
            .join(delimiter),
    )
}

struct ListVisitor<'a, T> {
    pub delimiter: &'a str,
    pub phantom: PhantomData<T>,
}

impl<'de, 'a, T> Visitor<'de> for ListVisitor<'a, T>
where
    T: DeserializeOwned,
{
    type Value = Vec<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&format!(
            "an value list in format 'value1' or 'value1{}value2{}value3'",
            self.delimiter, self.delimiter
        ))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.is_empty() {
            return Err(E::custom(
                "there is no default value list. at least one value must be specified",
            ));
        };
        let values = v
            .replace(" ", "")
            .split(self.delimiter)
            .map(|s| from_str(s).unwrap())
            .collect();
        Ok(values)
    }
}

fn deserialize_list<'de, D, T>(delimiter: &str, deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    deserializer.deserialize_string(ListVisitor {
        delimiter,
        phantom: PhantomData,
    })
}

pub mod semicolon_list {
    use super::{deserialize_list, serialize_list};
    use serde::{de::DeserializeOwned, Deserializer, Serialize, Serializer};

    pub fn serialize<S, T>(values: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        serialize_list(";", values, serializer)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: DeserializeOwned,
    {
        deserialize_list(";", deserializer)
    }
}

pub mod vertical_bar_list {
    use super::{deserialize_list, serialize_list};
    use serde::{de::DeserializeOwned, Deserializer, Serialize, Serializer};

    pub fn serialize<S, T>(values: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        serialize_list("|", values, serializer)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: DeserializeOwned,
    {
        deserialize_list("|", deserializer)
    }
}
