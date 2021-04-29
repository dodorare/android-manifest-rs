# Android Manifest

An [AndroidManifest] serializer and deserializer for Rust. This library will also likely continue to stay up to date with the official AndroidManifest specification as changes happen.

[AndroidManifest]: https://developer.android.com/guide/topics/manifest/manifest-intro

```toml
# Cargo.toml
[dependencies]
android_manifest = "1.0"
```

```rust
let xml = r#"
<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android" 
          package="com.example.toggletest" 
          android:versionCode="1" 
          android:versionName="1.0">
    <application android:allowBackup="true" 
               android:icon="@drawable/ic_launcher" 
               android:label="@string/app_name" 
               android:theme="@style/AppTheme">
        <activity android:label="@string/app_name" 
                  android:name="com.example.toggletest.MainActivity">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
    </application>
</manifest>"#;

let manifest: AndroidManifest = android_manifest::from_str(xml).unwrap();
```

# License

This project is licensed under Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in toml-rs by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
