use std::{assert_eq, iter};
use std::io::Cursor;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

use quick_xml::Writer;
use quick_xml::events::{Event, BytesEnd, BytesStart};
use quick_xml::Reader;

use simple_xml_builder::XMLElement;

use android_manifest;
mod common;

#[test]
fn test_setup_module() {
    let test_string = common::test_function();

    assert_eq!(test_string, "Setup is working");
}


#[test]
fn test_sample_xml() {
    let src: &[u8] = std::include_bytes!("documents/sample_rss.xml");
    let mut buf = Vec::new();
    let mut r = Reader::from_reader(src);
    let mut count = 0;
    loop {
        match r.read_event(&mut buf).unwrap() {
            Event::Start(_) => count += 1,
            Event::Decl(e) => println!("{:?}", e.version()),
            Eof => break,
            _ => (),
        }
        buf.clear();
    }
    println!("{}", count);
}


#[test]
fn test_transforming_xml() {
    let xml = r#"<this_tag k1="v1" k2="v2"><child>text</child></this_tag>"#;
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) if e.name() == b"this_tag" => {
    
                // crates a new element ... alternatively we could reuse `e` by calling
                // `e.into_owned()`
                let mut elem = BytesStart::owned(b"my_elem".to_vec(), "my_elem".len());
    
                // collect existing attributes
                elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()));
    
                // copy existing attributes, adds a new my-key="some value" attribute
                elem.push_attribute(("my-key", "some value"));
    
                // writes the event to the writer
                assert!(writer.write_event(Event::Start(elem)).is_ok());
            },
            Ok(Event::End(ref e)) if e.name() == b"this_tag" => {
                assert!(writer.write_event(Event::End(BytesEnd::borrowed(b"my_elem"))).is_ok());
            },
            Ok(Event::Eof) => break,
            Ok(e) => assert!(writer.write_event(e).is_ok()),
            // or using the buffer
            // Ok(e) => assert!(writer.write(&buf).is_ok()),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        }
        buf.clear();
    }
    
    let result = writer.into_inner().into_inner();
    let expected = r#"<my_elem k1="v1" k2="v2" my-key="some value"><child>text</child></my_elem>"#;
    assert_eq!(result, expected.as_bytes());
}


#[test]
fn simple_xml() {
    let path = Path::new("../../lol.xml");

    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let mut manifest = XMLElement::new("manifest");

    let mut map: HashMap<String, String> = HashMap::new();

    map.insert("xmlns:android".to_string(), "http://schemas.android.com/apk/res/android".to_string());
    map.insert("package".to_string(), "com.github.sample".to_string());
    map.insert("android:versionName".to_string(), "Lollipop".to_string());
    map.insert("android:versionCode".to_string(), "5.1".to_string());    

    for (key, value) in map {
        manifest.add_attribute(&key, &value);
    }

    let mut application = XMLElement::new("application");
    application.add_attribute("android:label", "SampleApplication");

    manifest.add_child(application);
    
    manifest.write(file).unwrap();    

    // #############################################

    let txt = include_str!("./documents/test_write.xml").trim();

    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut buf: Vec<u8> = Vec::new();



    /*
    let mut file = File::create("./documents/sample.xml").unwrap();

    let mut person = XMLElement::new("person");
    person.add_attribute("id", "232");
    let mut name = XMLElement::new("name");
    name.add_text("Joe Schmoe");
    person.add_child(name);
    let mut age = XMLElement::new("age");
    age.add_text("24");
    person.add_child(age);
    let hobbies = XMLElement::new("hobbies");
    person.add_child(hobbies);
    
    person.write(file);
    */
    // assert_eq!("", txt);
}

