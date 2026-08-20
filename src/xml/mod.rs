use std::io::{Read, Write};
use xml::writer::XmlEvent;

pub(crate) mod de;
pub(crate) mod ser;

pub(crate) trait XmlDeserialize: Sized {
    fn deserialize<R: Read>(reader: &mut de::Deserializer<R>) -> Result<Self, String>;
}

pub(crate) trait XmlSerialize: Sized {
    fn serialize<W: Write>(&self, writer: &mut ser::Serializer<W>) -> Result<(), String>;

    #[allow(dead_code)] // Invoked only by generated implementations that use XML flattening.
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

#[allow(dead_code)] // The derive emits only the visitor methods required by each model.
pub(crate) trait Visitor<'de>: Sized {
    type Value;

    fn visit_bool(self, value: &str) -> Result<Self::Value, String> {
        Err(format!("Unexpected bool {value:?}"))
    }

    fn visit_i8(self, value: &str) -> Result<Self::Value, String> {
        Err(format!("Unexpected i8 {value:?}"))
    }

    fn visit_u8(self, value: &str) -> Result<Self::Value, String> {
        Err(format!("Unexpected u8 {value:?}"))
    }

    fn visit_i16(self, value: &str) -> Result<Self::Value, String> {
        Err(format!("Unexpected i16 {value:?}"))
    }

    fn visit_u16(self, value: &str) -> Result<Self::Value, String> {
        Err(format!("Unexpected u16 {value:?}"))
    }

    fn visit_i32(self, value: &str) -> Result<Self::Value, String> {
        Err(format!("Unexpected i32 {value:?}"))
    }

    fn visit_u32(self, value: &str) -> Result<Self::Value, String> {
        Err(format!("Unexpected u32 {value:?}"))
    }

    fn visit_i64(self, value: &str) -> Result<Self::Value, String> {
        Err(format!("Unexpected i64 {value:?}"))
    }

    fn visit_u64(self, value: &str) -> Result<Self::Value, String> {
        Err(format!("Unexpected u64 {value:?}"))
    }

    fn visit_f32(self, value: &str) -> Result<Self::Value, String> {
        Err(format!("Unexpected f32 {value:?}"))
    }

    fn visit_f64(self, value: &str) -> Result<Self::Value, String> {
        Err(format!("Unexpected f64 {value:?}"))
    }

    fn visit_str(self, value: &str) -> Result<Self::Value, String> {
        Err(format!("Unexpected str {value:?}"))
    }
}

macro_rules! serialize_type {
    ($type:ty) => {
        impl XmlSerialize for $type {
            fn serialize<W: Write>(&self, writer: &mut ser::Serializer<W>) -> Result<(), String> {
                writer
                    .write(XmlEvent::characters(&self.to_string()))
                    .map_err(|error| error.to_string())
            }
        }
    };
}

serialize_type!(bool);
serialize_type!(char);
serialize_type!(usize);
serialize_type!(u8);
serialize_type!(u16);
serialize_type!(u32);
serialize_type!(u64);
serialize_type!(isize);
serialize_type!(i8);
serialize_type!(i16);
serialize_type!(i32);
serialize_type!(i64);
serialize_type!(f32);
serialize_type!(f64);

pub(crate) use log as __log;
pub(crate) use xml as __xml;

macro_rules! __derive_debug {
    ($($arg:tt)+) => {
        $crate::xml::__log::debug!(target: "android_manifest_derive", $($arg)+)
    };
}

macro_rules! __derive_trace {
    ($($arg:tt)+) => {
        $crate::xml::__log::trace!(target: "android_manifest_derive", $($arg)+)
    };
}

pub(crate) use __derive_debug;
pub(crate) use __derive_trace;
