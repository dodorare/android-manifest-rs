use quick_xml::de::from_reader;
use std::fs::File;
use std::io::BufReader;

use android_manifest::Manifest;

#[test]
fn test_simple_android_manifest_deserialize() {
    let file = File::open("docs/AndroidManifest.xml").unwrap();
    let reader = BufReader::new(file);
    let manifest: Manifest = from_reader(reader).unwrap();
    println!("{:?}", manifest);
}
