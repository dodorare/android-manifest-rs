mod resources;

use resources::{DrawableResource, Resource, StringResource};
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
    pub shared_user_label: Option<Resource<StringResource>>,
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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Application {
    /// The `<activity>` element has its own `allowTaskReparenting` attribute that can override the value set here. See that attribute for more information.
    #[serde(rename = "android:allowTaskReparenting")]
    pub allow_task_reparenting: Option<bool>,
    /// Whether to allow the application to participate in the backup and restore infrastructure. If this attribute is set
    /// to false, no backup or restore of the application will ever be performed, even by a full-system backup that would otherwise cause all application data to be saved via adb.
    /// The default value of this attribute is true.
    #[serde(rename = "android:allowBackup")]
    pub allow_backup: Option<bool>,
    /// Whether to allow the application to reset user data.
    /// This data includes flags—such as whether the user has seen introductory tooltips—as well as user-customizable settings and preferences.
    /// The default value of this attribute is true
    /// Note: Only apps that are part of the system image can declare this attribute explicitly. Third-party apps cannot include this attribute in their manifest files.
    #[serde(rename = "android:allowClearUserData")]
    pub allow_clear_user_data: Option<bool>,
    /// Whether or not the app has the Heap pointer tagging feature enabled. The default value of this attribute is `true`.
    /// `Note:` Disabling this feature does not address the underlying code health issue. Future hardware devices may not support this manifest tag.
    #[serde(rename = "android:allowNativeHeapPointerTagging")]
    pub allow_native_heap_pointer_tagging: Option<bool>,
    /// The name of the class that implements the application's backup agent, a subclass of BackupAgent.
    /// The attribute value should be a fully qualified class name (such as, `"com.example.project.MyBackupAgent"`).
    /// However, as a shorthand, if the first character of the name is a period (for example, `".MyBackupAgent"`),
    /// it is appended to the package name specified in the `<manifest>` element.
    #[serde(rename = "android:backupAgent")]
    pub backup_agent: Option<String>,
    /// Indicates that Auto Backup operations may be performed on this app even if the app is in a foreground-equivalent state.
    /// The system shuts down an app during auto backup operation, so use this attribute with caution.
    /// Setting this flag to true can impact app behavior while the app is active.
    /// The default value is false, which means that the OS will avoid backing up the app while it is running in the foreground
    /// (such as a music app that is actively playing music via a service in the startForeground() state).
    #[serde(rename = "android:backupInForeground")]
    pub backup_in_foreground: Option<bool>,
    /// A drawable resource providing an extended graphical banner for its associated item. Use with the
    /// `<application>` tag to supply a default banner for all application activities, or with the `<activity>` tag to supply a banner for a specific activity.
    #[serde(rename = "android:banner")]
    pub banner: Option<Resource<DrawableResource>>,
    /// Whether or not the application can be debugged, even when running on a device in user mode — `"true"` if it can be, and `"false"` if not. The default value is `"false"`.
    #[serde(rename = "android:debuggable")]
    pub debuggable: Option<bool>,
    /// User-readable text about the application, longer and more descriptive than the application label. The value must be set as a reference to a string resource.
    /// Unlike the label, it cannot be a raw string. There is no default value.
    #[serde(rename = "android:description")]
    pub description: Option<Resource<StringResource>>,
    /// Whether or not the application is direct-boot aware; that is, whether or not it can run before the user unlocks the device.
    /// If you're using a custom subclass of `Application`, and if any component inside your application
    /// is direct-boot aware, then your entire custom applicationis considered to be direct-boot aware.
    /// Note: During Direct Boot, your application can only access the data that is stored in device protected storage.
    #[serde(rename = "android:directBootAware")]
    pub direct_boot_aware: Option<bool>,
    /// Whether or not the Android system can instantiate components of the application — `"true"` if it can, and
    /// `"false"` if not. If the value is `"true"`, each component's enabled attribute determines whether that
    /// component is enabled or not. If the value is `"false"`, it overrides the component-specific values; all components are disabled.
    /// The default value is `"true"`.
    #[serde(rename = "android:enabled")]
    pub enabled: Option<bool>,
    /// Whether or not the package installer extracts native libraries from the APK to the filesystem. If set to
    /// "false", then your native libraries must be page aligned and stored uncompressed in the APK. Although
    /// your APK might be larger, your application should load faster because the libraries are directly loaded from the APK at runtime. On the other hand, if set to
    /// "true", native libraries in the APK can be compressed. During installation, the installer decompresses the libraries, and
    /// the linker loads the decompressed libraries at runtime; in this case, the APK would be smaller, but installation time might be slightly longe
    /// The default value is "true" if extractNativeLibs is not configured in AndroidManifest.xml.
    #[serde(rename = "android:extractNativeLibs")]
    pub extract_native_libs: Option<bool>,
    /// This attribute points to an XML file that contains full backup rules for Auto Backup.
    /// These rules determine what files get backed up. For more information, see XML Config Syntax for Auto Backup.
    /// This attribute is optional. If it is not specified, by default, Auto Backup includes most of your app's files. For more information, see Files that are backed up.
    #[serde(rename = "android:fullBackupContent")]
    pub full_backup_content: Option<String>,
    /// This attribute indicates whether or not to use Auto Backup on devices where it is available.
    /// If set to true, then your app performs Auto Backup when installed on a device running Android 6.0 (API level 23) or higher.
    /// On older devices, your app ignores this attribute and performs Key/Value Backups.
    /// The default value is `"false"`.
    #[serde(rename = "android:fullBackupOnly")]
    pub full_backup_only: Option<bool>,
    /// This attribute indicates whether or not to use GWP-ASan, which is a native memory allocator feature that helps find use-after-free and heap-buffer-overflow bugs.
    /// The default value is "never".
    #[serde(rename = "android:gwpAsanMode")]
    pub gwp_asan_mode: Option<GwpAsanMode>,
    /// Whether or not the application contains any code — `"true"` if it does, and `"false"` if not.
    /// When the value is `"false"`, the system does not try to load any application code when launching components. The default value is `"true"`.
    #[serde(rename = "android:hasCode")]
    pub has_code: Option<bool>,
    /// When the user uninstalls an app, whether or not to show the user a prompt to keep the app's data. The default value is `"false"`.
    #[serde(rename = "android:hasFragileUserData")]
    pub has_fragile_user_data: Option<bool>,
    /// Whether or not hardware-accelerated rendering should be enabled for all activities and views in this application — "true" if it should be enabled, and "false" if not.
    /// The default value is "true" if you've set either minSdkVersion or targetSdkVersion to "14" or higher; otherwise, it's "false".
    /// Note that not all of the OpenGL 2D operations are accelerated. If you enable the hardware-accelerated renderer, test your application to ensure that it
    /// can make use of the renderer without errors.
    #[serde(rename = "android:hardwareAccelerated")]
    pub hardware_accelerateda: Option<bool>,
    /// An icon for the application as whole, and the default icon for each of the application's components.
    /// See the individual icon attributes for `<activity>`, `<activity-alias>`, `<service>`, `<receiver>`, and `<provider>` elements.
    /// This attribute must be set as a reference to a drawable resource containing the image (for example `"@drawable/icon"`). There is no default icon.
    #[serde(rename = "android:icon")]
    pub icon: Option<Resource<DrawableResource>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
enum GwpAsanMode {
    Always,
    Never,
}
