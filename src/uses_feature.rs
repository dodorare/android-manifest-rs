use serde::{Deserialize, Serialize};

/// Declares a single hardware or software feature that is used by the
/// application.
///
/// The purpose of a <uses-feature> declaration is to inform any external entity
/// of the set of hardware and software features on which your application
/// depends. The element offers a required attribute that lets you specify
/// whether your application requires and cannot function without the declared
/// feature, or whether it prefers to have the feature but can function without
/// it. Because feature support can vary across Android devices, the
/// <uses-feature> element serves an important role in letting an application
/// describe the device-variable features that it uses.

/// The set of available features that your application declares corresponds to
/// the set of feature constants made available by the Android `PackageManager`,
/// which are listed for convenience in the `Features Reference` sections at the
/// bottom of this document.

/// You must specify each feature in a separate `<uses-feature>` element, so if
/// your application requires multiple features, it would declare multiple
/// `<uses-feature>` elements. For example, an application that requires both
///
/// Bluetooth and camera features in the device would declare these two
/// elements:
/// ```xml
/// <uses-feature android:name="android.hardware.bluetooth" />
/// <uses-feature android:name="android.hardware.camera" />
/// ```
/// In general, you should always make sure to declare `<uses-feature>` elements
/// for all of the features that your application requires.

/// Declared `<uses-feature>` elements are informational only, meaning that the
/// Android system itself does not check for matching feature support on the
/// device before installing an application. However, other services (such as
/// Google Play) or applications may check your application's `<uses-feature>`
/// declarations as part of handling or interacting with your application. For
/// this reason, it's very important that you declare all of the features (from
/// the list below) that your application uses.

/// For some features, there may exist a specific attribute that allows you to
/// define a version of the feature, such as the version of Open GL used
/// (declared with `glEsVersion`). Other features that either do or do not exist
/// for a device, such as a camera, are declared using the `name` attribute.

/// Although the <uses-feature> element is only activated for devices running
/// API Level 4 or higher, it is recommended to include these elements for all
/// applications, even if the `minSdkVersion` is "3" or lower. Devices running
/// older versions of the platform will simply ignore the element.
///
/// ## `Note:`
/// When declaring a feature, remember that you must also request permissions as
/// appropriate. For example, you must still request the `CAMERA` permission
/// before your application can access the camera API. Requesting the permission
/// grants your application access to the appropriate hardware and software,
/// while declaring the features used by your application ensures proper device
/// compatibility.
///
/// ## Contained in:
/// [`<manifest>`](crate::Manifest)
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
    /// feature specified in `android:name`.
    /// * When you declare `android:required="true"` for a feature, you are
    ///   specifying that the
    /// application cannot function, or is not designed to function, when the
    /// specified feature is not present on the device.
    /// * When you declare `android:required="false"` for a feature, it means
    ///   that the application
    /// prefers to use the feature if present on the device, but that it is
    /// designed to function without the specified feature, if necessary.
    ///
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