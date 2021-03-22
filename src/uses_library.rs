use serde::{Deserialize, Serialize};
///
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "uses-library")]
pub struct UsesLibrary {
    /// The name of the library. The name is provided by the documentation for
    /// the package you are using. An example of this is
    /// "android.test.runner", a package that contains Android test classes.
    #[serde(rename = "android:name")]
    pub name: Option<String>,
    /// Boolean value that indicates whether the application requires the
    /// library specified by android:name: "true": The application does not
    /// function without this library. The system will not allow the application
    /// on a device that does not have the library.
    #[serde(rename = "android:required")]
    pub required: Option<bool>,
}
