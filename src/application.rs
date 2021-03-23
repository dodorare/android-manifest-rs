use super::activity::Activity;
use super::activity_alias::ActivityAlias;
use super::meta_data::MetaData;
use super::provider::Provider;
use super::receiver::Receiver;
use super::resources::{
    DrawableResource, Resource, StringResource, StringResourceOrString, StyleResource, XmlResource,
};
use super::service::Service;
use super::uses_library::UsesLibrary;
use serde::{Deserialize, Serialize};

/// The declaration of the application.
///
/// This element contains subelements that declare each of the application's
/// components and has attributes that can affect all the components.
/// Many of these attributes (such as `icon`, `label`, `permission`, `process`,
/// `taskAffinity`, and `allowTaskReparenting`) set default values for
/// corresponding attributes of the component elements. Others (such as
/// `debuggable`, `enabled`, `description`, and `allowClearUserData`) set values
/// for the application as a whole and cannot be overridden by the components.
///
/// ## Contained in:
/// [`<manifest>`](crate::Manifest)
///
/// ## Can contain:
/// * [`<activity>`](crate::Activity)
/// * [`<activity-alias>`](crate::ActivityAlias)
/// * [`<meta-data>`](crate::MetaData)
/// * [`<service>`](crate::Service)
/// * [`<receiver>`](crate::Receiver)
/// * [`<provider>`](crate::Provider)
/// * [`<uses-library>`](crate::UsesLibrary)
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "application")]
pub struct Application {
    /// Whether or not activities that the application defines can move from the
    /// task that started them to the task they have an affinity for when that
    /// task is next brought to the front — `"true"` if they can move, and
    /// `"false"` if they must remain with the task where they started. The
    /// default value is `"false"`. The `<activity>` element has its own
    /// `allowTaskReparenting` attribute that can override the value set here.
    /// See that attribute for more information.
    ///
    /// ## Note
    /// If your app targets Android 11 (API level 30) or higher, you cannot
    /// disable device-to-device migration of your app's files. The system
    /// automatically allows this functionality. You can still disable
    /// cloud-based backup and restore of your app's files by setting this
    /// attribute to false, even if your app targets Android 11 (API level 30)
    /// or higher.
    #[serde(rename = "android:allowTaskReparenting")]
    pub allow_task_reparenting: Option<bool>,
    /// Whether to allow the application to participate in the backup and
    /// restore infrastructure. If this attribute is set to `false`, no
    /// backup or restore of the application will ever be performed, even by a
    /// full-system backup that would otherwise cause all application data to be
    /// saved via adb. The default value of this attribute is `true`.
    #[serde(rename = "android:allowBackup")]
    pub allow_backup: Option<bool>,
    /// Whether to allow the application to reset user data.
    /// This data includes flags—such as whether the user has seen introductory
    /// tooltips—as well as user-customizable settings and preferences.
    /// The default value of this attribute is true
    ///
    /// `Note:` Only apps that are part of the system image can declare this
    /// attribute explicitly. Third-party apps cannot include this attribute in
    /// their manifest files.
    #[serde(rename = "android:allowClearUserData")]
    pub allow_clear_user_data: Option<bool>,
    /// Whether or not the app has the Heap pointer tagging feature enabled. The
    /// default value of this attribute is `true`.
    ///
    /// `Note:` Disabling this feature does not address the underlying code
    /// health issue. Future hardware devices may not support this manifest tag.
    #[serde(rename = "android:allowNativeHeapPointerTagging")]
    pub allow_native_heap_pointer_tagging: Option<bool>,
    /// The name of the class that implements the application's backup agent, a
    /// subclass of BackupAgent. The attribute value should be a fully
    /// qualified class name (such as, `"com.example.project.MyBackupAgent"`).
    /// However, as a shorthand, if the first character of the name is a period
    /// (for example, `".MyBackupAgent"`), it is appended to the package
    /// name specified in the `<manifest>` element. There is no default. The
    /// name must be specified.
    #[serde(rename = "android:backupAgent")]
    pub backup_agent: Option<String>,
    /// Indicates that Auto Backup operations may be performed on this app even
    /// if the app is in a foreground-equivalent state. The system shuts
    /// down an app during auto backup operation, so use this attribute with
    /// caution. Setting this flag to true can impact app behavior while the
    /// app is active. The default value is `false`, which means that the OS
    /// will avoid backing up the app while it is running in the foreground
    /// (such as a music app that is actively playing music via a service in the
    /// `startForeground()` state).
    #[serde(rename = "android:backupInForeground")]
    pub backup_in_foreground: Option<bool>,
    /// A drawable resource providing an extended graphical banner for its
    /// associated item. Use with the `<application>` tag to supply a
    /// default banner for all application activities, or with the `<activity>`
    /// tag to supply a banner for a specific activity. The system uses the
    /// banner to represent an app in the Android TV home screen. Since the
    /// banner is displayed only in the home screen, it should only be
    /// specified by applications with an activity that handles the
    /// `CATEGORY_LEANBACK_LAUNCHER` intent. This attribute must be set as a
    /// reference to a drawable resource containing the image (for example
    /// `"@drawable/banner"`). There is no default banner.
    #[serde(rename = "android:banner")]
    pub banner: Option<Resource<DrawableResource>>,
    /// Whether or not the application can be debugged, even when running on a
    /// device in user mode — `"true"` if it can be, and `"false"` if not. The
    /// default value is `"false"`.
    #[serde(rename = "android:debuggable")]
    pub debuggable: Option<bool>,
    /// User-readable text about the application, longer and more descriptive
    /// than the application label. The value must be set as a reference to a
    /// string resource. Unlike the label, it cannot be a raw string. There
    /// is no default value.
    #[serde(rename = "android:description")]
    pub description: Option<Resource<StringResource>>,
    /// Whether or not the application is direct-boot aware; that is, whether or
    /// not it can run before the user unlocks the device. If you're using a
    /// custom subclass of `Application`, and if any component inside your
    /// application is direct-boot aware, then your entire custom
    /// applicationis considered to be direct-boot aware.
    ///
    /// `Note:` During Direct Boot, your application can only access the data
    /// that is stored in device protected storage.
    #[serde(rename = "android:directBootAware")]
    pub direct_boot_aware: Option<bool>,
    /// Whether or not the Android system can instantiate components of the
    /// application — `"true"` if it can, and `"false"` if not. If the value
    /// is `"true"`, each component's enabled attribute determines whether that
    /// component is enabled or not. If the value is `"false"`, it overrides the
    /// component-specific values; all components are disabled. The default
    /// value is `"true"`.
    #[serde(rename = "android:enabled")]
    pub enabled: Option<bool>,
    /// Whether or not the package installer extracts native libraries from the
    /// APK to the filesystem. If set to `"false"`, then your native
    /// libraries must be page aligned and stored uncompressed in the APK.
    /// Although your APK might be larger, your application should load
    /// faster because the libraries are directly loaded from the APK at
    /// runtime. On the other hand, if set to `"true"`, native libraries in
    /// the APK can be compressed. During installation, the installer
    /// decompresses the libraries, and the linker loads the decompressed
    /// libraries at runtime; in this case, the APK would be smaller, but
    /// installation time might be slightly longe The default value is
    /// `"true"` if extractNativeLibs is not configured in
    /// `AndroidManifest.xml`.
    #[serde(rename = "android:extractNativeLibs")]
    pub extract_native_libs: Option<bool>,
    /// This attribute points to an XML file that contains full backup rules for
    /// Auto Backup. These rules determine what files get backed up. For
    /// more information, see XML Config Syntax for Auto Backup.
    /// This attribute is optional. If it is not specified, by default, Auto
    /// Backup includes most of your app's files. For more information, see
    /// Files that are backed up.
    #[serde(rename = "android:fullBackupContent")]
    pub full_backup_content: Option<String>,
    /// This attribute indicates whether or not to use Auto Backup on devices
    /// where it is available. If set to true, then your app performs Auto
    /// Backup when installed on a device running Android 6.0 (API level 23) or
    /// higher. On older devices, your app ignores this attribute and
    /// performs Key/Value Backups. The default value is `"false"`.
    #[serde(rename = "android:fullBackupOnly")]
    pub full_backup_only: Option<bool>,
    /// This attribute indicates whether or not to use GWP-ASan, which is a
    /// native memory allocator feature that helps find use-after-free and
    /// heap-buffer-overflow bugs. The default value is "never".
    #[serde(rename = "android:gwpAsanMode")]
    pub gwp_asan_mode: Option<GwpAsanMode>,
    /// Whether or not the application contains any code — `"true"` if it does,
    /// and `"false"` if not. When the value is `"false"`, the system does
    /// not try to load any application code when launching components. The
    /// default value is `"true"`.
    #[serde(rename = "android:hasCode")]
    pub has_code: Option<bool>,
    /// When the user uninstalls an app, whether or not to show the user a
    /// prompt to keep the app's data. The default value is `"false"`.
    #[serde(rename = "android:hasFragileUserData")]
    pub has_fragile_user_data: Option<bool>,
    /// Whether or not hardware-accelerated rendering should be enabled for all
    /// activities and views in this application — `"true"` if it should be
    /// enabled, and `"false"` if not. The default value is `"true"` if
    /// you've set either minSdkVersion or targetSdkVersion to "14" or higher;
    /// otherwise, it's `"false"`.
    ///
    /// `Note` that not all of the OpenGL 2D operations are accelerated. If you
    /// enable the hardware-accelerated renderer, test your application to
    /// ensure that it can make use of the renderer without errors.
    #[serde(rename = "android:hardwareAccelerated")]
    pub hardware_accelerateda: Option<bool>,
    /// An icon for the application as whole, and the default icon for each of
    /// the application's components. See the individual icon attributes for
    /// `<activity>`, `<activity-alias>`, `<service>`, `<receiver>`, and
    /// `<provider>` elements. This attribute must be set as a reference to
    /// a drawable resource containing the image (for example
    /// `"@drawable/icon"`). There is no default icon.
    #[serde(rename = "android:icon")]
    pub icon: Option<Resource<DrawableResource>>,
    /// Whether or not the application is a game. The system may group together
    /// applications classifed as games or display them separately from other
    /// applications. The default is `false`.
    #[serde(rename = "android:isGame")]
    pub is_game: Option<bool>,
    /// Whether the application in question should be terminated after its
    /// settings have been restored during a full-system restore operation.
    /// Single-package restore operations will never cause the application to be
    /// shut down. Full-system restore operations typically only occur once,
    /// when the phone is first set up. Third-party applications will not
    /// normally need to use this attribute. The default is `true`, which
    /// means that after the application has finished processing its data during
    /// a full-system restore, it will be terminated.
    #[serde(rename = "android:killAfterRestore")]
    pub kill_after_restore: Option<bool>,
    /// Whether your application's processes should be created with a large
    /// Dalvik heap. This applies to all processes created for the application.
    /// It only applies to the first application loaded into a process; if
    /// you're using a shared user ID to allow multiple applications to
    /// use a process, they all must use this option consistently or they will
    /// have unpredictable results. Most apps should not need this and
    /// should instead focus on reducing their overall memory usage for improved
    /// performance. Enabling this also does not guarantee a fixed increase
    /// in available memory, because some devices are constrained by their total
    /// available memory. To query the available memory size at runtime, use
    /// the methods `getMemoryClass()` or `getLargeMemoryClass()`.
    #[serde(rename = "android:largeHeap")]
    pub large_heap: Option<bool>,
    /// A user-readable label for the application as a whole, and a default
    /// label for each of the application's components. See the individual
    /// `label` attributes for `<activity>`, `<activity-alias>`, `<service>`,
    /// `<receiver>`, and `<provider>` elements. The label should be set as
    /// a reference to a string resource, so that it can be localized like other
    /// strings in the user interface. However, as a convenience while
    /// you're developing the application, it can also be set as a raw string.
    #[serde(rename = "android:label")]
    pub label: Option<StringResourceOrString>,
    /// A logo for the application as whole, and the default logo for
    /// activities. This attribute must be set as a reference to a drawable
    /// resource containing the image (for example `"@drawable/logo"`). There is
    /// no default logo.
    #[serde(rename = "android:logo")]
    pub logo: Option<Resource<DrawableResource>>,
    /// The fully qualified name of an Activity subclass that the system can
    /// launch to let users manage the memory occupied by the application on the
    /// device. The activity should also be declared with an <activity>
    /// element.
    #[serde(rename = "android:manageSpaceActivity")]
    pub manage_space_activity: Option<String>,
    /// The fully qualified name of an Application subclass implemented for the
    /// application. When the application process is started, this class is
    /// instantiated before any of the application's components.
    /// The subclass is optional; most applications won't need one. In the
    /// absence of a subclass, Android uses an instance of the base Application
    /// class.
    #[serde(rename = "android:name")]
    pub name: Option<String>,
    /// Specifies the name of the XML file that contains your application's
    /// Network Security Configuration. The value must be a reference to the
    /// XML resource file containing the configuration. This attribute was
    /// added in API level 24.
    #[serde(rename = "android:networkSecurityConfig")]
    pub network_security_config: Option<Resource<XmlResource>>,
    /// The name of a permission that clients must have in order to interact
    /// with the application. This attribute is a convenient way to set a
    /// permission that applies to all of the application's components.
    /// It can be overwritten by setting the permission attributes of individual
    /// components. For more information on permissions, see the Permissions
    /// section in the introduction and another document, Security and
    /// Permissions.
    #[serde(rename = "android:permission")]
    pub permission: Option<String>,
    /// Whether or not the application should remain running at all times —
    /// `"true"` if it should, and `"false"` if not. The default value is
    /// `"false"`. Applications should not normally set this flag;
    /// persistence mode is intended only for certain system applications.
    #[serde(rename = "android:persistent")]
    pub persistent: Option<bool>,
    /// The name of a process where all components of the application should
    /// run. Each component can override this default by setting its own process
    /// attribute. By default, Android creates a process for an application
    /// when the first of its components needs to run. All components then run
    /// in that process. The name of the default process matches the package
    /// name set by the `<manifest>` element.
    #[serde(rename = "android:process")]
    pub process: Option<String>,
    /// Indicates that the application is prepared to attempt a restore of any
    /// backed-up data set, even if the backup was stored by a newer version
    /// of the application than is currently installed on the device. Setting
    /// this attribute to `true` will permit the Backup Manager to attempt
    /// restore even when a version mismatch suggests that the data are
    /// incompatible. Use with caution! The default value of this attribute
    /// is `false`.
    #[serde(rename = "android:restoreAnyVersion")]
    pub restore_any_version: Option<bool>,
    /// Whether or not the application wants to opt out of scoped storage.
    ///
    /// `Note:` Depending on changes related to policy or app compatibility, the
    /// system might not honor this opt-out request.
    #[serde(rename = "android:requestLegacyExternalStorage")]
    pub request_legacy_external_storage: Option<bool>,
    /// Specifies the account type required by the application in order to
    /// function. If your app requires an `Account`, the value for this
    /// attribute must correspond to the account authenticator type used by your
    /// app (as defined by `AuthenticatorDescription`), such as "com.google".
    /// The default value is null and indicates that the application can work
    /// without any accounts. Because restricted profiles currently cannot
    /// add accounts, specifying this attribute makes your app unavailable from
    /// a restricted profile unless you also declare `android:
    /// restrictedAccountType` with the same value.
    #[serde(rename = "android:requiredAccountType")]
    pub required_account_type: Option<String>,
    /// Specifies whether the app supports multi-window display. You can set
    /// this attribute in either the `<activity>` or `<application>` element.
    /// If you set this attribute to true, the user can launch the activity in
    /// split-screen and freeform modes. If you set the attribute to false,
    /// the activity does not support multi-window mode. If this value is
    /// false, and the user attempts to launch the activity in multi-window
    /// mode, the activity takes over the full screen. If your app targets
    /// API level 24 or higher, but you do not specify a value for this
    /// attribute, the attribute's value defaults to true. This attribute
    /// was added in API level 24
    #[serde(rename = "android:resizeableActivity")]
    pub resizeable_activity: Option<bool>,
    /// Specifies the account type required by this application and indicates
    /// that restricted profiles are allowed to access such accounts that belong
    /// to the owner user. If your app requires an `Account` and restricted
    /// profiles are allowed to access the primary user's accounts,the value for
    /// this attribute must correspond to the account authenticator type
    /// used by your app (as defined by `AuthenticatorDescription`), such as
    /// `"com.google"`. The default value is null and indicates that the
    /// application can work without any accounts.
    ///
    /// `Caution:` Specifying this attribute allows restricted profiles to use
    /// your app with accounts that belong to the owner user, which may reveal
    /// personally identifiable information. If the account may reveal
    /// personal details, you should not use this attribute and you should
    /// instead declare the android:requiredAccountType attribute to make
    /// your app unavailable to restricted profiles. This attribute was added in
    /// API level 18.
    #[serde(rename = "android:restrictedAccountType")]
    pub restricted_account_type: Option<String>,
    /// Declares whether your application is willing to support right-to-left
    /// (RTL) layouts. If set to true and targetSdkVersion is set to 17 or
    /// higher, various RTL APIs will be activated and used by the system so
    /// your app can display RTL layouts. if set to false or if
    /// targetSdkVersion is set to 16 or lower, the RTL APIs will be ignored or
    /// will have no effect and your app will behave the same regardless of the
    /// layout direction associated to the user's Locale choice (your layouts
    /// will always be left-to-right). The default value of this attribute is
    /// false This attribute was added in API level 17.
    #[serde(rename = "android:supportsRtl")]
    pub supports_rtl: Option<bool>,
    /// An affinity name that applies to all activities within the application,
    /// except for those that set a different affinity with their own
    /// `taskAffinity` attributes. See that attribute for more information.
    /// By default, all activities within an application share the same
    /// affinity. The name of that affinity is the same as the package name
    /// set by the `<manifest>` element.
    #[serde(rename = "android:taskAffinity")]
    pub task_affinity: Option<String>,
    /// Indicates whether this application is only for testing purposes. For
    /// example, it may expose functionality or data outside of itself that
    /// would cause a security hole, but is useful for testing. This kind of
    /// APK can be installed only through `adb`—you cannot publish it to Google
    /// Play.
    #[serde(rename = "android:testOnly")]
    pub test_only: Option<bool>,
    /// A reference to a style resource defining a default theme for all
    /// activities in the application. Individual activities can override the
    /// default by setting their own theme attributes. For more information,
    /// see the Styles and Themes developer guide.
    #[serde(rename = "android:theme")]
    pub theme: Option<Resource<StyleResource>>,
    /// Extra options for an activity's UI.
    /// Must be one of the following values.
    #[serde(rename = "android:uiOptions")]
    pub ui_options: Option<UiOptions>,
    /// Indicates whether the app intends to use cleartext network traffic, such
    /// as cleartext HTTP. The default value for apps that target API level 27
    /// or lower is `"true"`. Apps that target API level 28 or higher
    /// default to `"false"`. When the attribute is set to `"false"`,
    /// platform components (for example, HTTP and FTP stacks,
    /// `DownloadManager`, and `MediaPlayer`)  will refuse the app's requests to
    /// use cleartext traffic. Third-party libraries are strongly encouraged
    /// to honor this setting as well. The key reason for avoiding cleartext
    /// traffic is the lack of confidentiality, authenticity, and protections
    /// against tampering; a network attacker can eavesdrop on transmitted
    /// data and also modify it without being detected. `Note`: WebView
    /// honors this attribute for applications targeting API level 26 and
    /// higher.
    #[serde(rename = "android:usesCleartextTraffic")]
    pub uses_cleartext_traffic: Option<bool>,
    /// Indicates whether the app would like the virtual machine (VM) to operate
    /// in safe mode. The default value is `"false"`. This attribute was
    /// added in API level 8 where a value of "true" disabled the Dalvik
    /// just-in-time (JIT) compiler. This attribute was adapted in API level
    /// 22 where a value of "true" disabled the ART ahead-of-time (AOT)
    /// compiler.
    #[serde(rename = "android:vmSafeMode")]
    pub vm_safe_mode: Option<bool>,
    #[serde(rename = "activity", skip_serializing_if = "Vec::is_empty", default)]
    pub activities: Vec<Activity>,
    #[serde(rename = "service", skip_serializing_if = "Vec::is_empty", default)]
    pub services: Vec<Service>,
    #[serde(rename = "receiver", skip_serializing_if = "Vec::is_empty", default)]
    pub receivers: Vec<Receiver>,
    #[serde(rename = "provider", skip_serializing_if = "Vec::is_empty", default)]
    pub providers: Vec<Provider>,
    pub activity_alias: Option<ActivityAlias>,
    #[serde(rename = "meta-data", skip_serializing_if = "Vec::is_empty", default)]
    pub meta_datas: Vec<MetaData>,
    pub uses_library: Option<UsesLibrary>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GwpAsanMode {
    Always,
    Never,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum UiOptions {
    /// No extra UI options. This is the default.
    None,
    /// Add a bar at the bottom of the screen to display action items in the app
    /// bar (also known as the action bar), when constrained for horizontal
    /// space (such as when in portrait mode on a handset). Instead of a
    /// small number of action items appearing in the app bar at the top of the
    /// screen, the app bar is split into the top navigation section and the
    /// bottom bar for action items. This ensures a reasonable amount of space
    /// is made available not only for the action items, but also for navigation
    /// and title elements at the top. Menu items are not split across the
    /// two bars; they always appear together.
    SplitActionBarWhenNarrow,
}
