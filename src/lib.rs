#[macro_use]
extern crate yaserde_derive;

mod action;
mod activity;
mod activity_alias;
mod application;
mod attribute_list;
mod category;
mod compatible_screens;
mod data;
pub mod error;
mod grant_uri_permission;
mod instrumentation;
mod intent_filter;
mod layout;
mod manifest;
mod meta_data;
mod path_permission;
mod permission;
mod permission_group;
mod permission_tree;
mod profileable;
mod provider;
mod queries;
mod receiver;
mod resources;
mod service;
mod supports_gl_texture;
mod supports_screens;
mod ui_options;
mod uses_configuration;
mod uses_feature;
mod uses_library;
mod uses_native_library;
mod uses_permission;
mod uses_permission_sdk_23;
mod uses_sdk;
mod var_or_bool;

pub use action::*;
pub use activity::*;
pub use activity_alias::*;
pub use application::*;
pub use attribute_list::*;
pub use category::*;
pub use compatible_screens::*;
pub use data::*;
use error::{Error, Result};
pub use grant_uri_permission::*;
pub use instrumentation::*;
pub use intent_filter::*;
pub use layout::*;
pub use manifest::*;
pub use meta_data::*;
pub use path_permission::*;
pub use permission::*;
pub use permission_group::*;
pub use permission_tree::*;
pub use profileable::*;
pub use provider::*;
pub use queries::*;
pub use receiver::*;
pub use resources::*;
pub use service::*;
pub use supports_gl_texture::*;
pub use supports_screens::*;
pub use ui_options::*;
pub use uses_configuration::*;
pub use uses_feature::*;
pub use uses_library::*;
pub use uses_native_library::UsesNativeLibrary;
pub use uses_permission::*;
pub use uses_permission_sdk_23::*;
pub use uses_sdk::*;
pub use var_or_bool::*;

/// Deserialize an instance of type [`AndroidManifest`](crate::AndroidManifest) from a
/// string of XML text.
pub fn from_str(s: &str) -> Result<AndroidManifest> {
    yaserde::de::from_str(s).map_err(Error::FailedToDeserialize)
}

/// Deserialize an instance of type [`AndroidManifest`](crate::AndroidManifest) from an IO
/// stream of XML text.
pub fn from_reader<R: std::io::Read>(reader: R) -> Result<AndroidManifest> {
    yaserde::de::from_reader(reader).map_err(Error::FailedToDeserialize)
}

/// Serialize the given [`AndroidManifest`](crate::AndroidManifest) structure as a String
/// of XML text.
pub fn to_string(manifest: &AndroidManifest) -> Result<String> {
    yaserde::ser::to_string(manifest).map_err(Error::FailedToSerialize)
}

/// Serialize the given [`AndroidManifest`](crate::AndroidManifest) structure as a
/// pretty-printed String of XML text.
pub fn to_string_pretty(manifest: &AndroidManifest) -> Result<String> {
    let config = yaserde::ser::Config {
        perform_indent: true,
        write_document_declaration: true,
        indent_string: None,
    };
    yaserde::ser::to_string_with_config(manifest, &config).map_err(Error::FailedToSerialize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_manifest_deserialize() {
        let given_xml = r#"<?xml version="1.0" encoding="utf-8"?>
    <manifest xmlns:android="http://schemas.android.com/apk/res/android"
              package="org.domokit.gcm"
              android:versionCode="4"
              android:versionName="0.0.4">
        <uses-sdk android:minSdkVersion="14" android:targetSdkVersion="21" />
        <uses-permission android:name="android.permission.INTERNET" />

        <uses-permission android:name="android.permission.WAKE_LOCK" />
        <uses-permission android:name="com.google.android.c2dm.permission.RECEIVE" />

        <permission android:name="org.domokit.gcm.permission.C2D_MESSAGE"
            android:protectionLevel="signature" />
        <uses-permission android:name="org.domokit.gcm.permission.C2D_MESSAGE" />

        <application android:label="gcm" android:name="org.domokit.sky.shell.SkyApplication" android:usesCleartextTraffic="${usesCleartextTraffic}">
            <activity android:configChanges="orientation|keyboardHidden|keyboard|screenSize"
                      android:hardwareAccelerated="true"
                      android:launchMode="singleTask"
                      android:name="org.domokit.sky.shell.SkyActivity"
                      android:theme="@android:style/Theme.Black.NoTitleBar">
                <intent-filter>
                    <action android:name="android.intent.action.MAIN" />
                    <category android:name="android.intent.category.LAUNCHER" />
                </intent-filter>
            </activity>
            <service
                android:name="org.domokit.sky.shell.UpdateService"
                android:exported="false"
                android:process=":remote"/>
            <receiver
                android:name="com.google.android.gms.gcm.GcmReceiver"
                android:exported="true"
                android:permission="com.google.android.c2dm.permission.SEND" >
                <intent-filter>
                    <action android:name="com.google.android.c2dm.intent.RECEIVE" />
                    <category android:name="org.domokit.sky.shell" />
                </intent-filter>
            </receiver>
            <service
                android:name="org.domokit.gcm.GcmListenerService"
                android:exported="false" >
                <intent-filter>
                    <action android:name="com.google.android.c2dm.intent.RECEIVE" />
                </intent-filter>
            </service>
            <service
                android:name="org.domokit.gcm.InstanceIDListenerService"
                android:exported="false">
                <intent-filter>
                    <action android:name="com.google.android.gms.iid.InstanceID"/>
                </intent-filter>
            </service>
            <service
                android:name="org.domokit.gcm.RegistrationIntentService"
                android:exported="false">
            </service>
        </application>
    </manifest>"#;
        let expected_manifest = AndroidManifest {
            package: Some("org.domokit.gcm".to_string()),
            version_code: Some(4),
            version_name: Some("0.0.4".to_string()),
            application: Application {
                label: Some(StringResourceOrString::string("gcm")),
                name: Some("org.domokit.sky.shell.SkyApplication".to_string()),
                uses_cleartext_traffic: Some("${usesCleartextTraffic}".into()),
                activity: vec![Activity {
                    config_changes: AttributeList::from_vec(vec![
                        ConfigChanges::Orientation,
                        ConfigChanges::KeyboardHidden,
                        ConfigChanges::Keyboard,
                        ConfigChanges::ScreenSize,
                    ]),
                    hardware_accelerated: Some(true.into()),
                    launch_mode: Some(LaunchMode::SingleTask),
                    name: "org.domokit.sky.shell.SkyActivity".to_string(),
                    theme: Some(StyleResource::new(
                        "Theme.Black.NoTitleBar",
                        Some("android".to_string()),
                    )),
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
                service: vec![
                    Service {
                        exported: Some(false.into()),
                        name: "org.domokit.sky.shell.UpdateService".to_string(),
                        process: Some(":remote".to_string()),
                        ..Default::default()
                    },
                    Service {
                        exported: Some(false.into()),
                        name: "org.domokit.gcm.GcmListenerService".to_string(),
                        intent_filter: vec![IntentFilter {
                            action: vec![Action {
                                name: Some("com.google.android.c2dm.intent.RECEIVE".to_string()),
                            }],
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                    Service {
                        exported: Some(false.into()),
                        name: "org.domokit.gcm.InstanceIDListenerService".to_string(),
                        intent_filter: vec![IntentFilter {
                            action: vec![Action {
                                name: Some("com.google.android.gms.iid.InstanceID".to_string()),
                            }],
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                    Service {
                        exported: Some(false.into()),
                        name: "org.domokit.gcm.RegistrationIntentService".to_string(),
                        ..Default::default()
                    },
                ],
                receiver: vec![Receiver {
                    exported: Some(true.into()),
                    name: "com.google.android.gms.gcm.GcmReceiver".to_string(),
                    permission: Some("com.google.android.c2dm.permission.SEND".to_string()),
                    intent_filter: vec![IntentFilter {
                        action: vec![Action {
                            name: Some("com.google.android.c2dm.intent.RECEIVE".to_string()),
                        }],
                        category: vec![Category {
                            name: Some("org.domokit.sky.shell".to_string()),
                        }],
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            },
            uses_sdk: Some(UsesSdk {
                min_sdk_version: Some(14),
                target_sdk_version: Some(21),
                ..Default::default()
            }),
            permission: vec![Permission {
                name: Some("org.domokit.gcm.permission.C2D_MESSAGE".to_string()),
                protection_level: Some(ProtectionLevel::Signature),
                ..Default::default()
            }],
            uses_permission: vec![
                UsesPermission {
                    name: Some("android.permission.INTERNET".to_string()),
                    ..Default::default()
                },
                UsesPermission {
                    name: Some("android.permission.WAKE_LOCK".to_string()),
                    ..Default::default()
                },
                UsesPermission {
                    name: Some("com.google.android.c2dm.permission.RECEIVE".to_string()),
                    ..Default::default()
                },
                UsesPermission {
                    name: Some("org.domokit.gcm.permission.C2D_MESSAGE".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        let deserialized_xml_manifest: AndroidManifest = from_str(given_xml).unwrap();
        assert_eq!(expected_manifest, deserialized_xml_manifest);
        let serialized_toml_manifest = toml::to_string_pretty(&deserialized_xml_manifest).unwrap();
        let deserialized_toml_manifest: AndroidManifest =
            toml::from_str(&serialized_toml_manifest).unwrap();
        assert_eq!(deserialized_xml_manifest, deserialized_toml_manifest);
    }
}
