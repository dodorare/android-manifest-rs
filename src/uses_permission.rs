use serde::{Deserialize, Serialize};

/// Specifies a system permission that the user must grant in order for the app
/// to operate correctly.
///
/// Permissions are granted by the user when the application is installed (on devices
/// running Android 5.1 and lower) or while the app is running (on devices running Android
/// 6.0 and higher). For more information on permissions, see the [`Permissions`] section
/// in the introduction and the separate [`System Permissions`] API guide. A list of
/// permissions defined by the base platform can be found at
/// [`android.Manifest.permission`].
///
/// ## Note
/// In some cases, the permissions that you request through
/// `<uses-permission>` can affect how your application is filtered by Google Play.
///
/// If you request a hardware-related permission — `CAMERA`, for example — Google Play
/// assumes that your application requires the underlying hardware feature and filters the
/// application from devices that do not offer it.
///
/// To control filtering, always explicitly declare hardware features in `<uses-feature>`
/// elements, rather than relying on Google Play to "discover" the requirements in
/// `<uses-permission>` elements. Then, if you want to disable filtering for a particular
/// feature, you can add a `android:required`="`false`" attribute to the <uses-feature>
/// declaration.
///
/// For a list of permissions that imply hardware features, see the documentation for the
/// [`<uses-feature>`] element.
///
/// ## Contained in
/// * [`<manifest>`]
///
/// ## Introduced in
/// API Level 1
///
/// [`Permissions`]: https://developer.android.com/guide/topics/manifest/manifest-intro#perms
/// [`System Permissions`]: https://developer.android.com/guide/topics/permissions/overview
/// [`android.Manifest.permission`]: https://developer.android.com/reference/android/Manifest.permission
/// [`<uses-feature>`]: https://developer.android.com/guide/topics/manifest/uses-feature-element#permissions-features
/// [`<manifest>`]: crate::Manifest
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "uses-permission")]
pub struct UsesPermission {
    /// The name of the permission. It can be a permission defined by theapplication with
    /// the [`<permission>`] element, a permission defined by another application, or
    /// one of the standard system permissions (such as [`android.permission.CAMERA`]
    /// or [`android.permission.READ_CONTACTS`]). As these examples show, a
    /// permission name typically includes the package name as a prefix.
    ///
    /// [`<permission>`]: crate::Permission
    /// [`android.permission.CAMERA`]: https://developer.android.com/reference/android/Manifest.permission#CAMERA
    /// [`android.permission.READ_CONTACTS`]: https://developer.android.com/reference/android/Manifest.permission#READ_CONTACTS
    #[serde(rename = "android:name")]
    pub name: Option<String>,
    /// The highest API level at which this permission should be granted to your app.
    /// Setting this attribute is useful if the permission your app requires is no
    /// longer needed beginning at a certain API level.
    ///
    /// ## XML Examples
    /// Beginning with Android 4.4 (API level 19), it's no longer necessary for your app
    /// to request the [`WRITE_EXTERNAL_STORAGE`] permission when your app wants to
    /// write to its own application-specific directories on external storage (the
    /// directories provided by [`getExternalFilesDir()`]). However, the permission is
    /// required for API level 18 and lower. So you can declare that this permission
    /// is needed only up to API level 18 with a declaration such as this:
    /// ```xml
    /// <uses-permission
    ///     android:name="android.permission.WRITE_EXTERNAL_STORAGE"
    ///     android:maxSdkVersion="18" />
    /// ```
    /// This way, beginning with API level 19, the system will no longer grant your app
    /// the [`WRITE_EXTERNAL_STORAGE`] permission.
    ///
    /// [`WRITE_EXTERNAL_STORAGE`]: https://developer.android.com/reference/android/Manifest.permission#WRITE_EXTERNAL_STORAGE
    /// [`getExternalFilesDir()`]: https://developer.android.com/reference/android/content/Context#getExternalFilesDir(java.lang.String)
    #[serde(rename = "android:maxSdkVersion")]
    pub max_sdk_version: Option<i32>,
}
