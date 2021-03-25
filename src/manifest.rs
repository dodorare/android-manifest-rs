use super::application::Application;
use super::compatible_screens::CompatibleScreens;
use super::instrumentation::Instrumentation;
use super::permission::Permission;
use super::permission_group::PermissionGroup;
use super::permission_tree::PermissionTree;
use super::resources::{Resource, StringResource};
use super::supports_gl_texture::SupportsGlTexture;
use super::supports_screens::SupportsScreens;
use super::uses_configuration::UsesConfiguration;
use super::uses_feature::UsesFeature;
use super::uses_permission::UsesPermission;
use super::uses_permission_sdk_23::UsesPermissionSdk23;
use super::uses_sdk::UsesSdk;
use serde::{Deserialize, Serialize};

/// The root element of the `AndroidManifest.xml` file.
///
/// It must contain an [`<application>`] element and specify `xmlns:android` and
/// `package` attributes.
///
/// ## XML Syntax
/// ```xml
/// <manifest xmlns:android="http://schemas.android.com/apk/res/android"
///           package="string"
///           android:sharedUserId="string"
///           android:sharedUserLabel="string resource"
///           android:versionCode="integer"
///           android:versionName="string"
///           android:installLocation=["auto" | "internalOnly" | "preferExternal"] >
///     ...
/// </manifest>
/// ```
///
/// ## Must contain
/// * [`<application>`]
///
/// ## Can contain
/// * [`<compatible-screens>`]
/// * [`<instrumentation>`]
/// * [`<permission>`]
/// * [`<permission-group>`]
/// * [`<permission-tree>`]
/// * [`<supports-gl-texture>`]
/// * [`<supports-screens>`]
/// * [`<uses-configuration>`]
/// * [`<uses-feature>`]
/// * [`<uses-permission>`]
/// * [`<uses-permission-sdk-23>`]
/// * [`<uses-sdk>`]
///
/// ## Introduced in
/// API Level 1 for all attributes, unless noted otherwise in the attribute description.
///
/// [`<application>`]: crate::Application
/// [`<compatible-screens>`]: crate::CompatibleScreens
/// [`<instrumentation>`]: crate::Instrumentation
/// [`<permission>`]: crate::Permission
/// [`<permission-group>`]: crate::PermissionGroup
/// [`<permission-tree>`]: crate::PermissionTree
/// [`<supports-gl-texture>`]: crate::SupportsGlTexture
/// [`<supports-screens>`]: crate::SupportsScreens
/// [`<uses-configuration>`]: crate::UsesConfiguration
/// [`<uses-feature>`]: crate::UsesFeature
/// [`<uses-permission>`]: crate::UsesPermission
/// [`<uses-permission-sdk-23>`]: crate::UsesPermissionSdk23
/// [`<uses-sdk>`]: crate::UsesSdk
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "manifest")]
pub struct Manifest {
    /// Defines the Android namespace. This attribute should always be set to
    ///
    /// `http://schemas.android.com/apk/res/android`.
    #[serde(rename = "xmlns:android")]
    pub xmlns: String,
    /// A full Java-language-style package name for the Android app. The name may contain
    /// uppercase or lowercase letters ('A' through 'Z'), numbers, and underscores
    /// ('_'). However, individual package name parts may only start with letters.
    ///
    /// While building your app into the an application package (APK), the build system
    /// uses the package attribute for two things:
    ///
    /// * It applies this name as the namespace for your app's generated R.java class
    ///   (used to access your [`app resources`]).
    ///
    /// For example, if package is set to `"com.example.myapp"`, the R class is created at
    /// `com.example.myapp.R`.
    ///
    /// * It uses this name to resolve any relative class names that are declared in the
    ///   manifest file. For example, if package is set to `"com.example.myapp"`, an
    ///   activity declared as
    /// `<activity android:name=".MainActivity">` is resolved to be
    /// `com.example.myapp.MainActivity`.
    ///
    /// This name is also the default name for your app process (see the `<application>`
    /// element's [`process`] attribute). And it's the default task affinity for your
    /// activities (see the `<activity>` element's [`taskAffinity`] attribute).
    ///
    /// This name also represents the application ID, which must be universally unique in
    /// order to publish your app in [`Google Play`]. However, toward the end of the APK
    /// build process, the build tools override the package name using the
    /// applicationId property from the build.gradle file (used by Android Studio
    /// projects). As long as you keep the manifest's package name the same as the
    /// build file's applicationId, this won't be a concern. But if these two values
    /// differ, you should understand the differences between the `"package name"` and
    /// `"application ID"` by reading [`how to set the application ID`].
    ///
    /// To avoid conflicts with other developers, you should use Internet domain ownership
    /// as the basis for your package names (in reverse). For example, apps published by
    /// Google start with com.google.
    ///
    /// ## Note
    /// Both the `com.example` and `com.android` namespaces are forbidden by Google Play.
    ///
    /// ## Caution
    /// `If you want to change your package name` after you publish your app, you can, but
    /// `you must keep the applicationId the same`. The applicationId defines the unique
    /// identity for your app on Google Play. So if you change it, the APK is considered
    /// to be a different app and users of the previous version will not receive an
    /// update. For more information, see [`how to set the application ID`].
    ///
    /// [`app resources`]: https://developer.android.com/guide/topics/resources/providing-resources
    /// [`process`]: crate::Application#structfield.process
    /// [`taskAffinity`]: crate::Activity#structfield.task_affinitys
    /// [`Google Play`]: https://developer.android.com/distribute/google-play
    /// [`how to set the application ID`]: https://developer.android.com/studio/build/application-id
    pub package: String,
    /// ## Caution
    /// `This constant was deprecated in API level 29.`
    /// Shared user IDs cause non-deterministic behavior within the package manager. As
    /// such, its use is strongly discouraged and may be removed in a future version of
    /// Android. Instead, apps should use proper communication mechanisms, such as
    /// services and content providers, to facilitate interoperability between shared
    /// components. Note that existing apps cannot remove this value, as migrating off a
    /// shared user ID is not supported.
    ///
    /// The name of a Linux user ID that will be shared with other apps. By default,
    /// Android assigns each app its own unique user ID. However, if this attribute is
    /// set to the same value for two or more apps, they will all share the same ID —
    /// provided that their certificate sets are identical. Apps with the same user ID
    /// can access each other's data and, if desired, run in the same process.
    #[serde(rename = "android:sharedUserId")]
    pub shared_user_id: Option<String>,
    /// The higher the sandbox version number, the higher the level of security. Its
    /// default value is 1; you can also set it to 2. Setting this attribute
    /// to 2 switches the app to a different SELinux sandbox.
    ///
    /// The following restrictions apply to a level 2 sandbox:
    ///
    /// * The default value of `usesCleartextTraffic` in the Network Security Config is
    ///   false.
    /// * Uid sharing is not permitted.
    ///
    /// For Android Instant Apps targeting Android 8.0 (API level 26) or higher, this
    /// attribute must be set to 2. You can set the sandbox level in the installed version
    /// of your app to the less restrictive level 1, but if you do so, your app does not
    /// persist app data from the instant app to the installed version of your app. You
    /// must set the installed app's sandbox value to 2 in order for the data to persist
    /// from the instant app to the installed version.
    ///
    /// Once an app is installed, you can only update its target sandbox value to a higher
    /// value. To downgrade the target sandbox value, you must uninstall the app and
    /// replace it with a version whose manifest contains a lower value for this
    /// attribute.
    #[serde(rename = "android:targetSandboxVersion")]
    pub target_sandbox_version: Option<String>,
    /// ## Caution
    /// `This constant was deprecated in API level 29`. Shared user IDs cause
    /// non-deterministic behavior within the package manager. As such, its use is
    /// strongly discouraged and may be removed in a future version of Android. Instead,
    /// apps should use proper communication mechanisms, such as services and content
    /// providers, to facilitate interoperability between shared components. Note that
    /// existing apps cannot remove this value, as migrating off a shared user ID is not
    /// supported
    ///
    /// A user-readable label for the shared user ID. The label must be set as a reference
    /// to a string resource; it cannot be a raw string.
    ///
    /// This attribute was introduced in API Level 3. It is meaningful only if the
    /// [`sharedUserId`] attribute is also set.
    ///
    /// [`sharedUserId`]: crate::Manifest#structfield.shared_user_id
    #[serde(rename = "android:sharedUserLabel")]
    pub shared_user_label: Option<Resource<StringResource>>,
    /// An internal version number. This number is used only to determine whether one
    /// version is more recent than another, with higher numbers indicating more
    /// recent versions. This is not the version number shown to users; that number is
    /// set by the `versionName` attribute.
    ///
    /// The value must be set as an integer, such as
    /// "100". You can define it however you want, as long as each successive version
    /// has a higher number. For example, it could be a build number. Or you could
    /// translate a version number in "x.y" format to an integer by encoding the "x" and
    /// "y" separately in the lower and upper 16 bits. Or you could simply increase the
    /// number by one each time a new version is released.
    #[serde(rename = "android:versionCode")]
    pub version_code: Option<i32>,
    /// The version number shown to users. This attribute can be set as a raw string or as
    /// a reference to a string resource. The string has no other purpose than to be
    /// displayed to users. The `versionCode` attribute holds the significant version
    /// number used internally.
    #[serde(rename = "android:versionName")]
    pub version_name: Option<String>,
    /// When an app is installed on the external storage:  
    ///
    /// * The `.apk` file is saved to the external storage, but any app data (such as
    ///   databases) is still saved on the internal device memory.
    /// * The container in which the `.apk` file is saved is encrypted with a key that
    ///   allows the app to operate only on the device that installed it. (A user cannot
    ///   transfer the SD card to another device and use apps installed on the card.)
    ///   Though, multiple SD cards can be used with the same device.
    /// * At the user's request, the app can be moved to the internal storage.
    ///
    /// The user may also request to move an app from the internal storage to the external
    /// storage. However, the system will not allow the user to move the app to external
    /// storage if this attribute is set to internalOnly, which is the default setting.
    ///
    /// Read [`App Install Location`] for more information about using this attribute
    /// (including how to maintain backward compatibility).
    ///
    /// Introduced in: API Level 8.
    ///
    /// [`App Install Location`]: https://developer.android.com/guide/topics/data/install-location
    #[serde(rename = "android:installLocation")]
    pub install_location: Option<InstallLocation>,
    pub application: Application,
    #[serde(
        rename = "compatible-screens",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub compatible_screens: Vec<CompatibleScreens>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub instrumentation: Vec<Instrumentation>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub permission: Vec<Permission>,
    #[serde(
        rename = "permission-group",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub permission_group: Vec<PermissionGroup>,
    #[serde(
        rename = "permission-tree",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub permission_tree: Vec<PermissionTree>,
    #[serde(
        rename = "supports-gl-texture",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub supports_gl_texture: Vec<SupportsGlTexture>,
    #[serde(
        rename = "supports-screens",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub supports_screens: Vec<SupportsScreens>,
    #[serde(
        rename = "uses-configuration",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub uses_configuration: Vec<UsesConfiguration>,
    #[serde(
        rename = "uses-feature",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub uses_feature: Vec<UsesFeature>,
    #[serde(
        rename = "uses-permission",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub uses_permission: Vec<UsesPermission>,
    pub uses_permission_sdk_23: Option<UsesPermissionSdk23>,
    pub uses_sdk: Option<UsesSdk>,
}

/// The default install location for the app.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum InstallLocation {
    /// The app may be installed on the external storage, but the system will install the
    /// app on the internal storage by default. If the internal storage is full, then
    /// the system will install it on the external storage. Once installed, the user
    /// can move the app to either internal or external storage through the system
    /// settings.
    Auto,
    /// The app must be installed on the internal device storage only. If this is set, the
    /// app will never be installed on the external storage. If the internal storage
    /// is full, then the system will not install the app. This is also the default
    /// behavior if you do not define android:installLocation.
    InternalOnly,
    /// The app prefers to be installed on the external storage (SD card). There is no
    /// guarantee that the system will honor this request. The app might be installed
    /// on internal storage if the external media is unavailable or full. Once
    /// installed, the user can move the app to either internal or external storage
    /// through the system settings.
    PreferExternal,
}
