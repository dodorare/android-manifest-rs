use serde::{Deserialize, Serialize};

/// Specifies that an app wants a particular permission.
///
/// But only if the app is installed on a device running Android 6.0 ([`API level`]
/// 23) or higher. If the device is running API level 22 or lower, the app does
/// not have the specified permission.
///
/// This element is useful when you update an app to include a new feature that
/// requires an additional permission. If a user updates an app on a device that
/// is running API level 22 or lower, the system prompts the user at install
/// time to grant all new permissions that are declared in that update. If a new
/// feature is minor enough, you may prefer to disable the feature altogether on
/// those devices, so the user does not have to grant additional permissions to
/// update the app. By using the `<uses-permission-sdk-23>` element instead of
/// [`<uses-permission>`], you can request the permission only if the app is running
/// on platforms that support the [`runtime permissions model`] in which the user
/// grants permissions to the app while it is running.
///
/// For more information on permissions, see the [`Permissions`] section in the
/// introduction and the separate [`System Permissions`] API guide. A list of
/// permissions defined by the base platform is available at
/// [`android.Manifest.permission`].
///
/// ## Introduced in:
/// API Level 23
///
/// ## Contained in:
/// * [`<manifest>`]
///
/// [`API level`]: https://developer.android.com/guide/topics/manifest/uses-sdk-element#ApiLevels
/// [`<uses-permission>`]: crate::UsesPermission
/// [`runtime permissions model`]: https://developer.android.com/training/permissions/requesting
/// [`Permissions`]: https://developer.android.com/guide/topics/manifest/manifest-intro#perms
/// [`System Permissions`]: https://developer.android.com/guide/topics/permissions/overview
/// [`android.Manifest.permission`]: https://developer.android.com/reference/android/Manifest.permission
/// [`<manifest>`]: crate::Manifest
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "uses-permission-sdk-23")]
pub struct UsesPermissionSdk23 {
    /// The name of the permission. This permission can be defined by the app
    /// with the [`<permission>`] element, it can be a permission defined by another
    /// app, or it can be one of the standard system permissions, such as
    /// [`android.permission.CAMERA`] or [`android.permission.READ_CONTACTS`].
    ///
    /// [`<permission>`]: crate::Permission
    /// [`android.permission.CAMERA`]: https://developer.android.com/reference/android/Manifest.permission#CAMERA
    /// [`android.permission.READ_CONTACTS`]: https://developer.android.com/reference/android/Manifest.permission#READ_CONTACTS
    #[serde(rename = "android:name")]
    pub name: Option<String>,
    /// The highest API level at which this permission should be granted to your
    /// app. If the app is installed on a device with a later API level, the
    /// app is not granted the permission and cannot use any related
    /// functionality.
    #[serde(rename = "android:maxSdkVersion")]
    pub max_sdk_version: Option<i32>,
}
