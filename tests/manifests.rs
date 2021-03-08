use quick_xml::de::from_reader;
use quick_xml::se::to_writer;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};

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
    let _note: Xml = from_reader(reader).unwrap();
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
