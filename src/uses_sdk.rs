use serde::{Deserialize, Serialize};

/// Lets you express an application's compatibility with one or more versions of the
/// Android platform, by means of an API Level integer.
///
/// The API Level expressed by an application will be compared to the API Level of a given
/// Android system, which may vary among different Android devices.
///
/// Despite its name, this element is used to specify the API Level, not the version
/// number of the SDK (software development kit) or Android platform. The API Level is
/// always a single integer. You cannot derive the API Level from its associated Android
/// version number (for example, it is not the same as the major version or the sum of the
/// major and minor versions). [`Versioning Your Applications.`]
///
/// ## Contained in:
/// [`<manifest>`]
///
/// ## Introduced in:
/// API Level 1
///
/// [`Versioning Your Applications.`]: https://developer.android.com/studio/publish/versioning
/// [`<manifest>`]: crate::Manifest
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "uses-sdk")]
pub struct UsesSdk {
    /// An integer designating the minimum API Level required for the application to run.
    /// The Android system will prevent the user from installing the application if
    /// the system's API Level is lower than the value specified in this attribute.
    /// You should always declare this attribute.
    ///
    /// ## Caution:
    /// If you do not declare this attribute, the system assumes a default value of "1",
    /// which indicates that your application is compatible with all versions of
    /// Android. If your application is not compatible with all versions (for
    /// instance, it uses APIs introduced in API Level 3) and you have not declared
    /// the proper `minSdkVersion`, then when installed on a system with an API Level
    /// less than 3, the application will crash during runtime when attempting to
    /// access the unavailable APIs. For this reason, be certain to declare the
    /// appropriate API Level in the minSdkVersion attribute.
    #[serde(rename = "android:minSdkVersion")]
    pub min_sdk_version: Option<i32>,
    /// An integer designating the API Level that the application targets. If not set, the
    /// default value equals that given to `minSdkVersion`.
    ///
    /// This attribute informs the system that you have tested against the target version
    /// and the system should not enable any compatibility behaviors to maintain your
    /// app's forward-compatibility with the target version. The application is still
    /// able to run on older versions (down to `minSdkVersion`).
    ///
    /// As Android evolves with each new version, some behaviors and even appearances
    /// might change. However, if the API level of the platform is higher than the
    /// version declared by your app's targetSdkVersion the system may enable
    /// compatibility behaviors to ensure that your app continues to work the way you
    /// expect.You can disable such compatibility behaviors by specifying
    /// targetSdkVersion to match the API level of the platform on which it's running.
    /// For example, setting this value to "11" or higher allows the system to apply
    /// a new default theme (Holo) to your app when running on Android 3.0
    /// or higher and also disables [`screen compatibility mode`] when running on
    /// larger screens (because support for API level 11 implicitly supports
    /// larger screens).
    ///
    /// There are many compatibility behaviors that the system may enable based on the
    /// value you set for this attribute. Several of these behaviors are described by
    /// the corresponding platform versions in the [`Build.VERSION_CODES`] reference.
    ///
    /// To maintain your application along with each Android release, you should increase
    /// the value of this attribute to match the latest API level, then thoroughly
    /// test your application on the corresponding platform version.
    ///
    /// Introduced in: API Level 4
    ///
    /// [`screen compatibility mode`]: https://developer.android.com/guide/topics/manifest/supports-screens-element#compat-mode
    /// [`Build.VERSION_CODES`]: https://developer.android.com/reference/android/os/Build.VERSION_CODES
    #[serde(rename = "android:targetSdkVersion")]
    pub target_sdk_version: Option<i32>,
    /// An integer designating the maximum API Level on which the application is designed
    /// to run. In Android 1.5, 1.6, 2.0, and 2.0.1, the system checks the value of
    /// this attribute when installing an application and when re-validating the
    /// application after a system update. In either case, if the application's
    /// maxSdkVersion attribute is lower than the API Level used by the system itself,
    /// then the system will not allow the application to be installed. In the case of
    /// re-validation after system update, this effectively removes your application
    /// from the device. To illustrate how this attribute can affect your application
    /// after system updates, consider the following example: An application
    /// declaring maxSdkVersion="5" in its manifest is published on Google Play.
    /// A user whose device is running Android 1.6 (API Level 4) downloads and
    /// installs the app. After a few weeks, the user receives an
    /// over-the-air system update to Android 2.0 (API Level 5). After the
    /// update is installed, the system checks the application's maxSdkVersion
    /// and successfully re-validates it. The application functions as
    /// normal. However, some time later, the device receives another system
    /// update, this time to Android 2.0.1 (API Level 6). After the update, the
    /// system can no longer re-validate the application because the system's
    /// own API Level (6) is now higher than the maximum supported by the
    /// application (5). The system prevents the application from being
    /// visible to the user, in effect removing it from the device.
    /// ## `Warning:`
    /// Declaring this attribute is not recommended. First, there is
    /// no need to set the attribute as means of blocking deployment of your
    /// application onto new versions of the Android platform as they are
    /// released. By design, new versions of the platform are fully
    /// backward-compatible. Your application should work properly on new
    /// versions, provided it uses only standard APIs and follows
    /// development best practices. Second, note that in some cases, declaring
    /// the attribute can result in your application being removed from users'
    /// devices after a system update to a higher API Level. Most devices on
    /// which your application is likely to be installed will receive periodic
    /// system updates over the air, so you should consider their effect on your
    /// application before setting this attribute.
    #[serde(rename = "android:maxSdkVersion")]
    pub max_sdk_version: Option<i32>,
}
