use super::resources::{DrawableResource, Resource, StringResourceOrString};
use serde::{Deserialize, Serialize};

/// Declares an [`Instrumentation`](https://developer.android.com/reference/android/app/Instrumentation)
/// class that enables you to monitor an application's interaction with the
/// system.
///
/// The `Instrumentation` object is instantiated before any of the
/// application's components.
///
/// ## Contained in:
/// [`<manifest>`](crate::Manifest)
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "instrumentation")]
pub struct Instrumentation {
    /// Whether or not the Instrumentation class should run as a functional test
    /// — `"true"` if it should, and `"false"` if not. The default value is
    /// `"false"`.
    #[serde(rename = "android:functionalTest")]
    pub functional_test: Option<bool>,
    /// Whether or not the Instrumentation object will turn profiling on and off
    /// — `"true"` if it determines when profiling starts and stops, and
    /// `"false"` if profiling continues the entire time it is running.
    /// A value of `"true"` enables the object to target profiling at a specific
    /// set of operations. The default value is `"false"`.
    #[serde(rename = "android:handleProfiling")]
    pub handle_profiling: Option<bool>,
    /// An icon that represents the Instrumentation class. This attribute must
    /// be set as a reference to a drawable resource.
    #[serde(rename = "android:icon")]
    pub icon: Option<Resource<DrawableResource>>,
    /// A user-readable label for the Instrumentation class. The label can be
    /// set as a raw string or a reference to a string resource.
    #[serde(rename = "android:label")]
    pub label: Option<StringResourceOrString>,
    /// The name of the Instrumentation subclass. This should be a fully
    /// qualified class name (such as,
    /// `"com.example.project.StringInstrumentation"`). However, as a
    /// shorthand, if the first character of the name is a period, it is
    /// appended to the package name specified in the `<manifest>` element.
    /// There is no default. The name must be specified.
    #[serde(rename = "android:name")]
    pub name: String,
    /// The application that the Instrumentation object will run against.
    /// An application is identified by the package name assigned in its
    /// manifest file by the <manifest> element.
    #[serde(rename = "android:targetPackage")]
    pub target_package: Option<String>,
    /// The processes that the `Instrumentation` object will run against. A
    /// comma-separated list indicates that the instrumentation will run against
    /// those specific processes. A value of `"*"` indicates that the
    /// instrumentation will run against all processes of the app defined in
    /// `android:targetPackage`. If this value isn't provided in the
    /// manifest, the instrumentation will run only against the main process of
    /// the app defined in `android:targetPackage`.
    #[serde(rename = "android:targetProcesses")]
    pub target_processes: Option<String>,
}
