use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "uses-permission-sdk-23")]
pub struct UsesPermissionSdk23 {
    /// The name of the permission. This permission can be defined by the app
    /// with the <permission> element, it can be a permission defined by another
    /// app, or it can be one of the standard system permissions, such as
    /// "android.permission.CAMERA" or "android.permission.READ_CONTACTS".
    #[serde(rename = "android:name")]
    pub name: Option<String>,
    /// The highest API level at which this permission should be granted to your
    /// app. If the app is installed on a device with a later API level, the
    /// app is not granted the permission and cannot use any related
    /// functionality.
    #[serde(rename = "android:maxSdkVersion")]
    pub max_sdk_version: Option<i32>,
}
