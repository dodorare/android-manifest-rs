use crate::VarOrBool;

use super::resources::{MipmapOrDrawableResource, StringResourceOrString};
use serde::{Deserialize, Serialize};

/// Declares an [`Instrumentation`] class that enables you to monitor an application's
/// interaction with the system.
///
/// The Instrumentation object is instantiated before any of the application's
/// components.
///
/// ## XML Syntax
/// ```xml
/// <instrumentation android:functionalTest=["true" | "false"]
///                  android:handleProfiling=["true" | "false"]
///                  android:icon="drawable resource"
///                  android:label="string resource"
///                  android:name="string"
///                  android:targetPackage="string"
///                  android:targetProcesses="string" />
/// ```
///
/// ## Contained in:
/// * [`<manifest>`]
///
/// ## Introduced in
/// API Level 1
///
/// [`Instrumentation`]: https://developer.android.com/reference/android/app/Instrumentation
/// [`<manifest>`]: crate::AndroidManifest
#[derive(
    Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Default, Clone,
)]
pub struct Instrumentation {
    /// Whether or not the Instrumentation class should run as a functional test —
    /// `"true"` if it should, and `"false"` if not. The default value is `"false"`.
    #[yaserde(attribute, prefix = "android", rename = "functionalTest")]
    pub functional_test: Option<VarOrBool>,
    /// Whether or not the Instrumentation object will turn profiling on and off —
    /// `"true"` if it determines when profiling starts and stops, and `"false"` if
    /// profiling continues the entire time it is running. A value of `"true"` enables
    /// the object to target profiling at a specific set of operations. The default
    /// value is `"false"`.
    #[yaserde(attribute, prefix = "android", rename = "handleProfiling")]
    pub handle_profiling: Option<VarOrBool>,
    /// An icon that represents the Instrumentation class. This attribute must be set as a
    /// reference to a drawable resource.
    #[yaserde(attribute, prefix = "android")]
    pub icon: Option<MipmapOrDrawableResource>,
    /// A user-readable label for the Instrumentation class. The label can be set as a raw
    /// string or a reference to a string resource.
    #[yaserde(attribute, prefix = "android")]
    pub label: Option<StringResourceOrString>,
    /// The name of the [`Instrumentation`] subclass. This should be a fully qualified
    /// class name (such as, `"com.example.project.StringInstrumentation"`). However,
    /// as a shorthand, if the first character of the name is a period, it is
    /// appended to the package name specified in the [`<manifest>`] element.
    /// There is no default. The name must be specified.
    ///
    /// [`Instrumentation`]: https://developer.android.com/reference/android/app/Instrumentation
    /// [`<manifest>`]: crate::AndroidManifest
    #[yaserde(attribute, prefix = "android")]
    pub name: String,
    /// The application that the [`Instrumentation`] object will run against. An
    /// application is identified by the package name assigned in its manifest file by
    /// the [`<manifest>`] element.
    ///
    /// [`Instrumentation`]: https://developer.android.com/reference/android/app/Instrumentation
    /// [`<manifest>`]: crate::AndroidManifest
    #[yaserde(attribute, prefix = "android", rename = "targetPackage")]
    pub target_package: Option<String>,
    /// The processes that the [`Instrumentation`] object will run against. A
    /// comma-separated list indicates that the instrumentation will run against those
    /// specific processes. A value of `"*"` indicates that the instrumentation will
    /// run against all processes of the app defined in [`android:targetPackage`]. If
    /// this value isn't provided in the manifest, the instrumentation will run only
    /// against the main process of the app defined in [`android:targetPackage`].
    ///
    /// [`Instrumentation`]: https://developer.android.com/reference/android/app/Instrumentation
    /// [`android:targetPackage`]: crate::Instrumentation#structfield.target_package
    #[yaserde(attribute, prefix = "android", rename = "targetProcesses")]
    pub target_processes: Option<String>,
}
