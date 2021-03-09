use serde::{Deserialize, Serialize};

/// The root element of the AndroidManifest.xml file.
/// It must contain an `<application>` element and specify `xmlns:android` and `package` attributes.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename = "android:{camelCase}")]
pub struct Manifest {
    /// Defines the Android namespace.
    /// This attribute should always be set to `http://schemas.android.com/apk/res/android`.
    #[serde(rename = "xmlns:android")]
    pub xmlns: String,
    /// A full Java-language-style package name for the Android app.
    /// The name may contain uppercase or lowercase letters ('A' through 'Z'), numbers, and underscores ('_').
    /// However, individual package name parts may only start with letters.
    pub package: String,
    /// The name of a Linux user ID that will be shared with other apps. By default, Android assigns each app its own unique user ID.
    /// However, if this attribute is set to the same value for two or more apps, they will all share the same ID — provided that their certificate sets are identical.
    /// Apps with the same user ID can access each other's data and, if desired, run in the same process.
    #[serde(rename = "android:sharedUserId")]
    pub shared_user_id: Option<String>,
    /// The higher the sandbox version number, the higher the level of security.
    /// Its default value is 1; you can also set it to 2. Setting this attribute to 2 switches the app to a different SELinux sandbox.
    /// The following restrictions apply to a level 2 sandbox:
    /// The default value of `usesCleartextTraffic` in the Network Security Config is false.
    /// Uid sharing is not permitted.
    #[serde(rename = "android:targetSandboxVersion")]
    pub target_sandbox_version: Option<String>,
    /// A user-readable label for the shared user ID. The label must be set as a reference to a string resource; it cannot be a raw string.
    /// This attribute was introduced in API Level 3. It is meaningful only if the sharedUserId attribute is also set.
    #[serde(rename = "android:sharedUserLabel")]
    pub shared_user_label: Option<String>,
    /// An internal version number. This number is used only to determine whether one version is more recent than another, with higher numbers indicating more recent versions.
    /// This is not the version number shown to users; that number is set by the versionName attribute.
    /// The value must be set as an integer, such as "100". You can define it however you want, as long as each successive version has a higher number.
    #[serde(rename = "android:versionCode")]
    pub version_code: Option<i32>,
    /// The version number shown to users. This attribute can be set as a raw string or as a reference to a string resource.
    /// The string has no other purpose than to be displayed to users. The versionCode attribute holds the significant version number used internally.
    #[serde(rename = "android:versionName")]
    pub version_name: Option<String>,
    /// Note: By default, your app will be installed on the internal storage and cannot be installed on the external
    /// storage unless you define this attribute to be either `auto` or `preferExternal`.
    #[serde(rename = "android:installLocation")]
    pub install_location: Option<InstallLocation>,
}

/// The default install location for the app.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum InstallLocation {
    /// The app may be installed on the external storage, but the system will install
    /// the app on the internal storage by default. If the internal storage is full,
    /// then the system will install it on the external storage.
    /// Once installed, the user can move the app to either internal or external storage through the system settings.
    Auto,
    /// The app must be installed on the internal device storage only.
    /// If this is set, the app will never be installed on the external storage.
    /// If the internal storage is full, then the system will not install the app.
    /// This is also the default behavior if you do not define android:installLocation.
    InternalOnly,
    /// The app prefers to be installed on the external storage (SD card).
    /// There is no guarantee that the system will honor this request.
    /// The app might be installed on internal storage if the external media is unavailable or full.
    /// Once installed, the user can move the app to either internal or external storage through the system settings.
    PreferExternal,
}
