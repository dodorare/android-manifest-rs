<div align="center">
<h1>Android Manifest</h1>

<a href="https://github.com/dodorare/android-manifest-rs/actions"><img alt="CI Info" src="https://github.com/dodorare/android-manifest-rs/workflows/CI/badge.svg"/></a>
<a href="https://crates.io/crates/android-manifest"><img alt="Crate Info" src="https://img.shields.io/crates/v/android-manifest.svg"/></a>
<a href="https://docs.rs/android-manifest/"><img alt="API Docs" src="https://img.shields.io/badge/docs.rs-android-manifest"/></a>
<a href="https://crates.io/crates/android-manifest"><img alt="Crate" src="https://img.shields.io/crates/d/android-manifest?label=cargo%20installs"/></a>
</div>

[AndroidManifest] serializer and deserializer for Rust. This library will also likely continue to stay up to date with the official AndroidManifest specification as changes happen.

[AndroidManifest]: https://developer.android.com/guide/topics/manifest/manifest-intro

Install [android-manifest](https://crates.io/crates/android-manifest):

```toml
# Cargo.toml
[dependencies]
android-manifest = "*"
```

Create `AndroidManifest.xml` by yourself:
```rust
let manifest = AndroidManifest {
    package: "com.example.toggletest".to_string(),
    version_code: Some("1".to_string()),
    version_name: Some("1.0".to_string()),
    application: Application {
        allow_backup: Some(true),
        icon: Some(DrawableResource::new("ic_launcher", None)),
        label: Some(StringResourceOrString::resource("app_name", None)),
        theme: Some(StyleResource::new("AppTheme", None)),
        activity: vec![Activity {
            label: Some(StringResourceOrString::resource("app_name", None)),
            name: "com.example.toggletest.MainActivity".to_string(),
            intent_filter: vec![IntentFilter {
                action: vec![Action {
                    name: Some("android.intent.action.MAIN".to_string()),
                }],
                category: vec![Category {
                    name: Some("android.intent.category.LAUNCHER".to_string()),
                }],
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    },
    ..Default::default()
};
let serialized_manifest = android_manifest::to_string_pretty(&manifest).unwrap();
```

Or parse any `AndroidManifest.xml` file:
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

This project is licensed under Apache License, Version 2.0, ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in toml-rs by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
