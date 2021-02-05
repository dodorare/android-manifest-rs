use android_manifest;
mod common;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::Event::*;
use quick_xml::Reader;


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
