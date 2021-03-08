use serde::{Deserialize, Serialize};

/// The root element of the AndroidManifest.xml file.
/// It must contain an `<application>` element and specify `xmlns:android` and `package` attributes.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Manifest {
    /// Defines the Android namespace.
    /// This attribute should always be set to `http://schemas.android.com/apk/res/android`.
    #[serde(rename = "xmlns:android")]
    pub xmlns: String,
    /// A full Java-language-style package name for the Android app.
    /// The name may contain uppercase or lowercase letters ('A' through 'Z'), numbers, and underscores ('_').
    /// However, individual package name parts may only start with letters.
    pub package: String,
    #[serde(rename = "android:versionCode")]
    pub version_code: Option<i32>,
    #[serde(rename = "android:versionName")]
    pub version_name: Option<String>,
    #[serde(rename = "android:installLocation")]
    pub install_location: Option<InstallLocation>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum InstallLocation {
    Auto,
    InternalOnly,
    PreferExternal,
}
