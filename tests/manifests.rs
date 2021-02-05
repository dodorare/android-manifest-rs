use std::io::Cursor;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::Event::*;
use quick_xml::Reader;
use quick_xml::Writer;

use android_manifest;
mod common;


#[test]
fn test_sample_xml() {
    common::setup();
    let src: &[u8] = std::include_bytes!("documents/sample_rss.xml");
    let mut buf = Vec::new();
    let mut r = Reader::from_reader(src);
    let mut count = 0;
    loop {
        match r.read_event(&mut buf).unwrap() {
            Start(_) => count += 1,
            Decl(e) => println!("{:?}", e.version()),
            Eof => break,
            _ => (),
        }
        buf.clear();
    }
    println!("{}", count);
}


#[test]
fn test_writer() {
    let txt = std::include_str!("documents/test_write.xml").trim();
    let mut reader = Reader::from_str(txt);
    reader.trim_text(true);
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Eof) => break,
            Ok(e) => assert!(writer.write_event(e).is_ok()),
            Err(e) => panic!(e),
        }
    }

    let result = writer.into_inner().into_inner();
    assert_eq!(result, txt.as_bytes());
}
