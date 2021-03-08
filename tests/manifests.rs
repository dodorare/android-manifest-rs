// libraries for reading
use quick_xml::de::from_reader;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

// libraries for writing

#[derive(Debug, Deserialize, PartialEq)]
struct Xml {
    to: String,
    from: String,
    heading: String,
    body: String,
}
#[test]
fn test_simple_xml_read() {
    // Need to test read XML file
    let file = File::open("docs/xml_example.xml").unwrap();
    let reader = BufReader::new(file);
    let note: Xml = from_reader(reader).unwrap();
}

#[test]
#[should_panic]
fn test_simple_xml_fail_read() {
    // Need to test fail read XML file
    let file = File::open("docs/xml_example_error.xml").unwrap();
    let reader = BufReader::new(file);
    let note: Xml = from_reader(reader).unwrap();
    panic!("ERROR");
}

#[test]
fn test_simple_xml_write() {
    // Need to test write XML file
}

#[test]
#[should_panic]
fn test_simple_xml_fail_write() {
    // Need to test fail write XML file
    panic!();
}
