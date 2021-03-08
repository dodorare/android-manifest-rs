use quick_xml::de::{from_reader, from_str, DeError};
use quick_xml::se::to_writer;
use serde::{Deserialize, Serialize};
use std::io::{BufReader, BufWriter};
use std::{fs::File, i32};
extern crate quick_xml;
extern crate serde;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "xml")]
struct Xml {
    to: String,
    from: String,
    heading: String,
    body: String,
}

#[test]
fn test_simple_xml_read() {
    let file = File::open("docs/xml_example.xml").unwrap();
    let reader = BufReader::new(file);
    let note: Xml = from_reader(reader).unwrap();
    println!("{:?}", note);
}

#[test]
#[should_panic]
fn test_simple_xml_fail_read() {
    let file = File::open("docs/xml_example_error.xml").unwrap();
    let reader = BufReader::new(file);
    let _note: Xml = from_reader(reader).unwrap();
}

#[test]
fn test_simple_xml_write() {
    let file = File::create("docs/xml_example_write.xml").unwrap();
    let writer = BufWriter::new(file);
    let note = Xml {
        to: "Adil".to_string(),
        from: "Alex".to_string(),
        heading: "Massage".to_string(),
        body: "Hello".to_string(),
    };
    to_writer(writer, &note).unwrap();
}

#[derive(Debug, Deserialize, PartialEq)]
struct Manifest {
    #[serde(rename = "xmlns:android")]
    xmlns: Option<String>,
    package: String,
    #[serde(rename = "android:versionCode")]
    version_code: Option<i32>,
    #[serde(rename = "android:versionName")]
    version_name: Option<String>,
}

#[test]
fn test_simple_android_manifest_deserialize() {
    let file = File::open("docs/AndroidManifest.xml").unwrap();
    let reader = BufReader::new(file);
    let manifest: Manifest = from_reader(reader).unwrap();
    println!("{:?}", manifest);
}
