use serde::{Deserialize, Serialize};

use crate::VarOrBool;

/// Specifies a [`vendor-provided shared native library`] that the application must be
/// linked against.
///
/// This element tells the system to make the native library accessible for the package.
///
/// NDK libraries are by default accessible and therefore don't require the
/// `<uses-native-library>` tag.
///
/// Non-NDK native shared libraries that are provided by silicon vendors or device
/// manufacturers are not accessible by default if the app is targeting Android 12
/// or higher. The libraries are accessible only when they are explicitly requested
///  using the `<uses-native-library>` tag.
///
/// If the app is targeting Android 11 or lower, the `<uses-native-library>` tag is
/// not required. n that case, any native shared library is accessible regardless
/// of whether it is an NDK library.
///
/// This element also affects the installation of the application on a particular device:
///
/// ## Installation
/// If this element is present and its android:required attribute is set to true,
/// the [`PackageManager`] framework won't let the user install the application unless
/// the library is present on the user's device.
///
/// The android:required attribute is described in detail in the following section.
///
/// ## XML Syntax
/// ```xml
///  <uses-native-library
///     android:name="string"
///     android:required=["true" | "false"] />
/// ```
///
/// ## Contained in
/// * [`<application>`]
///
/// ## Introduced in
/// API Level S
///
/// [`vendor-provided shared native library`]: https://source.android.com/devices/tech/config/namespaces_libraries#adding-additional-native-libraries
/// [`PackageManager`]: https://developer.android.com/reference/android/content/pm/PackageManager
/// [`<application>`]: crate::Application
#[derive(
    Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Default, Clone,
)]
pub struct UsesNativeLibrary {
    /// The name of the library file.
    #[yaserde(attribute, prefix = "android")]
    pub name: String,
    /// Boolean value that indicates whether the application requires the library
    /// specified by android:name:
    ///
    /// * `"true"`: The application does not function without this library. The system
    ///   will
    /// not allow the application on a device that does not have the library.
    ///
    /// * `"false"`: The application can use the library if present, but is designed to
    ///   function
    /// without it if necessary. The system will allow the application to be installed,
    /// even if the library is not present. If you use `"false"`, you are responsible
    /// for gracefully handling the absence of the library.
    ///
    /// The default is `"true"`.
    #[yaserde(attribute, prefix = "android")]
    pub required: Option<VarOrBool>,
}
