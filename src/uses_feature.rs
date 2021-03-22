use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "uses-feature")]
pub struct UsesFeature {
    /// Specifies a single hardware or software feature used by the application,
    /// as a descriptor string. Valid attribute values are listed in the
    /// Hardware features and Software features sections. These attribute
    /// values are case-sensitive.
    #[serde(rename = "android:name")]
    pub name: Option<String>,
    /// Boolean value that indicates whether the application requires the
    /// feature specified in `android:name`. When you declare
    /// `android:required="true"` for a feature, you are specifying that the
    /// application cannot function, or is not designed to function, when the
    /// specified feature is not present on the device. When you declare
    /// `android:required="false"` for a feature, it means that the application
    /// prefers to use the feature if present on the device, but that it is
    /// designed to function without the specified feature, if necessary.
    /// The default value for android:required if not declared is `"true"`.
    #[serde(rename = "android:required")]
    pub required: Option<bool>,
    /// The OpenGL ES version required by the application. The higher 16 bits
    /// represent the major number and the lower 16 bits represent the minor
    /// number. For example, to specify OpenGL ES version 2.0, you would set
    /// the value as "0x00020000", or to specify OpenGL ES 3.2, you would set
    /// the value as "0x00030002". An application should specify at most one
    /// android:glEsVersion attribute in its manifest. If it specifies more than
    /// one, the android:glEsVersion with the numerically highest value is used
    /// and any other values are ignored. If an application does not specify
    /// an android:glEsVersion attribute, then it is assumed that the
    /// application requires only OpenGL ES 1.0, which is supported by all
    /// Android-powered devices. An application can assume that if a
    /// platform supports a given OpenGL ES version, it also supports all
    /// numerically lower OpenGL ES versions. Therefore, an application that
    /// requires both OpenGL ES 1.0 and OpenGL ES 2.0 must specify that it
    /// requires OpenGL ES 2.0. An application that can work with any of
    /// several OpenGL ES versions should only specify the numerically lowest
    /// version of OpenGL ES that it requires.
    #[serde(rename = "android:glEsVersion")]
    pub gl_es_version: Option<i32>,
}
