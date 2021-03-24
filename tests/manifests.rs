use quick_xml::de::from_reader;
use quick_xml::se::to_writer;
use std::fs::File;
use std::io::{BufReader, BufWriter};

use android_manifest::*;

#[test]
fn test_simple_android_manifest_deserialize() {
    let file = File::open("docs/AndroidManifest_simple.xml").unwrap();
    let reader = BufReader::new(file);
    let manifest: Manifest = from_reader(reader).unwrap();
    println!("{:#?}", manifest);
}

#[test]
fn test_simple_android_manifest_serialize() {
    let file = File::create("docs/AndroidManifest_generated.xml").unwrap();
    let writer = BufWriter::new(file);
    let note = Manifest {
        xmlns: "http://schemas.android.com/apk/res/android".to_owned(),
        package: "rust.myapp".to_owned(),
        shared_user_label: Some(StringResource::new("resource_name", None)),
        application: Application {
            activity: Vec::from([Activity::default()]),
            ..Default::default()
        },
        ..Default::default()
    };
    to_writer(writer, &note).unwrap();
}
