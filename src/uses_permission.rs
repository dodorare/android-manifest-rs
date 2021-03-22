use serde::{Deserialize, Serialize};

/// Specifies a system permission that the user must grant in order for the app
/// to operate correctly.
///
/// Permissions are granted by the user when the application is installed (on
/// devices running Android 5.1 and lower) or while the app is running (on
/// devices running Android 6.0 and higher). For more information on
/// permissions, see the Permissions section in the introduction and the
/// separate System Permissions API guide. A list of permissions defined by the
/// base platform can be found at `android.Manifest.permission`.
///
/// ## Contained in:
/// [`<manifest>`](crate::Manifest)
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "uses-permission")]
pub struct UsesPermission {
    /// The name of the permission. It can be a permission defined by the
    /// application with the `<permission>` element, a permission defined by
    /// another application, or one of the standard system permissions (such
    /// as `"android.permission.CAMERA"` or
    /// `"android.permission.READ_CONTACTS"`). As these examples show, a
    /// permission name typically includes the package name as a prefix.
    #[serde(rename = "android:name")]
    pub name: Option<String>,
    /// The highest API level at which this permission should be granted to your
    /// app. Setting this attribute is useful if the permission your app
    /// requires is no longer needed beginning at a certain API level.
    /// For example, beginning with Android 4.4 (API level 19), it's no longer
    /// necessary for your app to request the WRITE_EXTERNAL_STORAGE permission
    /// when your app wants to write to its own application-specific
    /// directories on external storage (the directories provided by
    /// `getExternalFilesDir()`). However, the permission is required for API
    /// level 18 and lower. So you can declare that this permission is
    /// needed only up to API level 18 with a declaration such as this:
    /// ```xml
    /// <uses-permission
    /// android:name="android.permission.WRITE_EXTERNAL_STORAGE"
    /// android:maxSdkVersion="18" />
    /// ```
    /// This way, beginning with API level 19, the system will no longer grant
    /// your app the `WRITE_EXTERNAL_STORAGE` permission.
    #[serde(rename = "android:maxSdkVersion")]
    pub max_sdk_version: Option<i32>,
}