// use quick_xml::de::from_reader;
// use quick_xml::se::to_writer;
// use std::fs::File;
// use std::io::{BufReader, BufWriter};

use android_manifest::*;

// #[test]
// fn test_simple_android_manifest_deserialize() {
//     let file = File::open("docs/AndroidManifest_simple.xml").unwrap();
//     let reader = BufReader::new(file);
//     let manifest: Manifest = from_reader(reader).unwrap();
//     println!("{:#?}", manifest);
// }

// #[test]
// fn test_simple_android_manifest_serialize() {
//     let file = File::create("docs/AndroidManifest_generated.xml").unwrap();
//     let writer = BufWriter::new(file);
//     let note = Manifest {
//         xmlns: "http://schemas.android.com/apk/res/android".to_owned(),
//         package: "rust.myapp".to_owned(),
//         shared_user_label: Some(StringResource::new("resource_name", None)),
//         application: Application {
//             activity: Vec::from([Activity::default()]),
//             ..Default::default()
//         },
//         ..Default::default()
//     };
//     to_writer(writer, &note).unwrap();
// }

#[test]
fn test_android_manifest_deserialize_1() {
    let xml = r#"
<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    package="com.example.toggletest"
    android:versionCode="1"
    android:versionName="1.0" >

    <uses-sdk
        android:minSdkVersion="8"
        android:targetSdkVersion="18" />
    <uses-permission android:name="android.permission.ACCESS_WIFI_STATE"/>
    <uses-permission android:name="android.permission.ACCESS_FINE_LOCATION"/>
    <uses-permission android:name="android.permission.ACCESS_COARSE_LOCATION"/>

    <application
        android:allowBackup="true"
        android:icon="@drawable/ic_launcher"
        android:label="@string/app_name"
        android:theme="@style/AppTheme" >
        <activity
            android:name="com.example.toggletest.MainActivity"
            android:label="@string/app_name" >
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />

                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
    </application>
    <uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />
    <uses-permission android:name="android.permission.ACCESS_WIFI_STATE" />

</manifest>"#;
    let manifest: Manifest = quick_xml::de::from_str(xml).unwrap();
    println!("{:#?}", manifest);
}
