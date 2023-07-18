use crate::VarOrBool;

use super::activity::Activity;
use super::activity_alias::ActivityAlias;
use super::meta_data::MetaData;
use super::profileable::Profileable;
use super::provider::Provider;
use super::receiver::Receiver;
use super::resources::{
    DrawableResource, MipmapOrDrawableResource, Resource, StringResource, StringResourceOrString,
    StyleResource, XmlResource,
};
use super::service::Service;
use super::ui_options::UiOptions;
use super::uses_library::UsesLibrary;
use super::uses_native_library::UsesNativeLibrary;
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
/// ## XML Syntax
/// ```xml
/// <application android:allowTaskReparenting=["true" | "false"]
///              android:allowBackup=["true" | "false"]
///              android:allowClearUserData=["true" | "false"]
///              android:allowNativeHeapPointerTagging=["true" | "false"]
///              android:backupAgent="string"
///              android:backupInForeground=["true" | "false"]
///              android:banner="drawable resource"
///              android:debuggable=["true" | "false"]
///              android:description="string resource"
///              android:directBootAware=["true" | "false"]
///              android:enabled=["true" | "false"]
///              android:extractNativeLibs=["true" | "false"]
///              android:fullBackupContent="xml resource"
///              android:fullBackupOnly=["true" | "false"]
///              android:gwpAsanMode=["always" | "never"]
///              android:hasCode=["true" | "false"]
///              android:hasFragileUserData=["true" | "false"]
///              android:hardwareAccelerated=["true" | "false"]
///              android:icon="drawable resource"
///              android:isGame=["true" | "false"]
///              android:killAfterRestore=["true" | "false"]
///              android:largeHeap=["true" | "false"]
///              android:label="string resource"
///              android:logo="drawable resource"
///              android:manageSpaceActivity="string"
///              android:name="string"
///              android:networkSecurityConfig="xml resource"
///              android:permission="string"
///              android:persistent=["true" | "false"]
///              android:process="string"
///              android:restoreAnyVersion=["true" | "false"]
///              android:requestLegacyExternalStorage=["true" | "false"]
///              android:requiredAccountType="string"
///              android:resizeableActivity=["true" | "false"]
///              android:restrictedAccountType="string"
///              android:supportsRtl=["true" | "false"]
///              android:taskAffinity="string"
///              android:testOnly=["true" | "false"]
///              android:theme="resource or theme"
///              android:uiOptions=["none" | "splitActionBarWhenNarrow"]
///              android:usesCleartextTraffic=["true" | "false"]
///              android:vmSafeMode=["true" | "false"] >
///       ...
/// </application>
/// ```
///
/// ## Contained in
/// * [`<manifest>`]
///
/// ## Can contain
/// * [`<activity>`]
/// * [`<activity-alias>`]
/// * [`<meta-data>`]
/// * [`<service>`]
/// * [`<receiver>`]
/// * [`<provider>`]
/// * [`<uses-library>`]
///
/// ## Introduced in
/// API Level 1
///
/// [`<manifest>`]: crate::AndroidManifest
/// [`<activity>`]: crate::Activity
/// [`<activity-alias>`]: crate::ActivityAlias
/// [`<meta-data>`]: crate::MetaData
/// [`<service>`]: crate::Service
/// [`<receiver>`]: crate::Receiver
/// [`<provider>`]: crate::Provider
/// [`<uses-library>`]: crate::UsesLibrary
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Default, Clone)]
pub struct Application {
    /// Whether or not activities that the application defines can move from the task that
    /// started them to the task they have an affinity for when that task is next
    /// brought to the front — "`true`" if they can move, and "`false`" if they must
    /// remain with the task where they started.
    ///
    /// The default value is "`false`".
    ///
    /// The [`<activity>`] element has its own [`allowTaskReparenting`] attribute that can
    /// override the value set here. See that attribute for more information.
    ///
    /// [`<activity>`]: crate::Activity
    /// [`allowTaskReparenting`]: crate::Activity#structfield.allow_task_reparenting
    #[yaserde(attribute, prefix = "android", rename = "allowTaskReparenting")]
    pub allow_task_reparenting: Option<VarOrBool>,
    /// Whether to allow the application to participate in the backup and restore
    /// infrastructure. If this attribute is set to false, no backup or restore of the
    /// application will ever be performed, even by a full-system backup that would
    /// otherwise cause all application data to be saved via adb.
    ///
    /// The default value of this attribute is "`true`".
    ///
    /// ## Note
    /// If your app targets Android 11 (API level 30) or higher, you cannot disable
    /// device-to-device migration of your app's files. The system automatically
    /// allows this functionality.
    ///
    /// You can still disable cloud-based backup and restore of your app's files by
    /// setting this attribute to "`false`", even if your app targets Android 11 (API
    /// level 30) or higher.
    #[yaserde(attribute, prefix = "android", rename = "allowBackup")]
    pub allow_backup: Option<VarOrBool>,
    /// Whether to allow the application to reset user data. This data includes flags—such
    /// as whether the user has seen introductory tooltips—as well as user-customizable
    /// settings and preferences.
    ///
    /// The default value of this attribute is "`true`".
    ///
    /// For more information, see [`Restoring User Data on New Devices`].
    ///
    /// ## Note
    /// Only apps that are part of the system image can declare this attribute explicitly.
    /// Third-party apps cannot include this attribute in their manifest files.
    ///
    /// [`Restoring User Data on New Devices`]: https://developer.android.com/guide/topics/data/backup
    #[yaserde(attribute, prefix = "android", rename = "allowClearUserData")]
    pub allow_clear_user_data: Option<VarOrBool>,
    /// Whether or not the app has the Heap pointer tagging feature enabled.
    ///
    /// The default value of this attribute is `true`.
    ///
    /// ## Note
    /// Disabling this feature does `not` address the underlying code health issue.
    /// Future hardware devices may not support this manifest tag.
    ///
    /// For more information, see [`Tagged Pointers`].
    ///
    /// [`Tagged Pointers`]: https://source.android.com/devices/tech/debug/tagged-pointers
    #[yaserde(
        attribute,
        prefix = "android",
        rename = "allowNativeHeapPointerTagging"
    )]
    pub allow_native_heap_pointer_tagging: Option<VarOrBool>,
    /// The name of the class that implements the application's backup agent, a subclass
    /// of [`BackupAgent`]. The attribute value should be a fully qualified class name
    /// (such as, `"com.example.project.MyBackupAgent"`). However, as a shorthand, if
    /// the first character of the name is a period (for example, `".MyBackupAgent"`),
    /// it is appended to the package name specified in the [`<manifest>`] element.
    ///
    /// There is no default. The name must be specified.
    ///
    /// [`BackupAgent`]: https://developer.android.com/reference/android/app/backup/BackupAgent
    /// [`<manifest>`]: crate::AndroidManifest
    #[yaserde(attribute, prefix = "android", rename = "backupAgent")]
    pub backup_agent: Option<String>,
    /// Indicates that [`Auto Backup`] operations may be performed on this app even if the
    /// app is in a foreground-equivalent state. The system shuts down an app during
    /// auto backup operation, so use this attribute with caution. Setting this flag
    /// to true can impact app behavior while the app is active.
    ///
    /// The default value is "`false`", which means that the OS will avoid backing up the
    /// app while it is running in the foreground (such as a music app that is
    /// actively playing music via a service in the [`startForeground()`] state).
    ///
    /// [`Auto Backup`]: https://developer.android.com/guide/topics/data/autobackup
    /// [`startForeground()`]: https://developer.android.com/reference/android/app/Service#startForeground(int,%20android.app.Notification)
    #[yaserde(attribute, prefix = "android", rename = "backupInForeground")]
    pub backup_in_foreground: Option<VarOrBool>,
    /// A [`drawable resource`] providing an extended graphical banner for its associated
    /// item. Use with the `<application>` tag to supply a default banner for all
    /// application activities, or with the [`<activity>`] tag to supply a banner for
    /// a specific activity.
    ///
    /// The system uses the banner to represent an app in the Android TV home screen.
    /// Since the banner is displayed only in the home screen, it should only be
    /// specified by applications with an activity that handles the
    /// [`CATEGORY_LEANBACK_LAUNCHER`] intent.
    ///
    /// This attribute must be set as a reference to a drawable resource containing the
    /// image (for example `"@drawable/banner"`). There is no default banner.
    ///
    /// See [`Provide a home screen banner`] in Get Started with TV Apps for more
    /// information.
    ///
    /// [`drawable resource`]: https://developer.android.com/guide/topics/resources/drawable-resource
    /// [`<activity>`]: crate::Activity
    /// [`CATEGORY_LEANBACK_LAUNCHER`]: https://developer.android.com/reference/android/content/Intent#CATEGORY_LEANBACK_LAUNCHER
    /// [`Provide a home screen banner`]: https://developer.android.com/training/tv/start/start#banner
    #[yaserde(attribute, prefix = "android")]
    pub banner: Option<Resource<DrawableResource>>,
    /// Whether or not the application can be debugged, even when running on a device in
    /// user mode — "`true`" if it can be, and "`false`" if not. The default value is
    /// "`false`".
    #[yaserde(attribute, prefix = "android")]
    pub debuggable: Option<VarOrBool>,
    /// User-readable text about the application, longer and more descriptive than the
    /// application label. The value must be set as a reference to a string resource.
    /// Unlike the label, it cannot be a raw string.
    ///
    /// There is no default value.
    #[yaserde(attribute, prefix = "android")]
    pub description: Option<Resource<StringResource>>,
    /// Whether or not the application is direct-boot aware; that is, whether or
    /// not it can run before the user unlocks the device. If you're using a
    /// custom subclass of [`Application`], and if any component inside your
    /// application is direct-boot aware, then your entire custom
    /// applicationis considered to be direct-boot aware.
    ///
    /// The default value is "`false`".
    ///
    /// ## Note
    /// During [`Direct Boot`], your application can only access the data that is stored
    /// in device protected storage.
    ///
    /// [`Application`]: https://developer.android.com/reference/android/app/Application
    /// [`Direct Boot`]: https://developer.android.com/training/articles/direct-boot
    #[yaserde(attribute, prefix = "android", rename = "directBootAware")]
    pub direct_boot_aware: Option<VarOrBool>,
    /// Whether or not the Android system can instantiate components of the
    /// application — "`true`" if it can, and "`false`" if not. If the value
    /// is "`true`", each component's enabled attribute determines whether that
    /// component is enabled or not. If the value is "`false`", it overrides the
    /// component-specific values; all components are disabled.
    ///
    /// The default value is "`true`".
    #[yaserde(attribute, prefix = "android")]
    pub enabled: Option<VarOrBool>,
    /// Whether or not the package installer extracts native libraries from the APK to the
    /// filesystem. If set to "`false`", then your native libraries must be page aligned
    /// and stored uncompressed in the APK. Although your APK might be larger, your
    /// application should load faster because the libraries are directly loaded from the
    /// APK at runtime. On the other hand, if set to "`true`", native libraries in the APK
    /// can be compressed. During installation, the installer decompresses the libraries,
    /// and the linker loads the decompressed libraries at runtime; in this case, the APK
    /// would be smaller, but installation time might be slightly longer.
    ///
    /// The default value is "`true`" if extractNativeLibs is not configured in
    /// `AndroidManifest.xml`. However, when building your app using [`Android Gradle
    /// plugin 3.6.0`] or higher, this property is reset to "`false`" if it is `NOT`
    /// configured in `AndroidManifest.xml`; so if your native libraries in the APK
    /// are compressed, you must explicitly set it to "`true`" in
    /// `AndroidManifest.xml`.
    ///
    /// [`Android Gradle plugin 3.6.0`]: https://developer.android.com/studio/releases/gradle-plugin#3-6-0
    #[yaserde(attribute, prefix = "android", rename = "extractNativeLibs")]
    pub extract_native_libs: Option<VarOrBool>,
    /// This attribute points to an XML file that contains full backup rules for [`Auto
    /// Backup`]. These rules determine what files get backed up. For more information,
    /// see [`XML Config Syntax`] for Auto Backup.
    ///
    /// This attribute is optional. If it is not specified, by default, Auto Backup
    /// includes most of your app's files. For more information, see [`Files that are
    /// backed`] up.
    ///
    /// [`Auto Backup`]: https://developer.android.com/guide/topics/data/autobackup
    /// [`XML Config Syntax`]: https://developer.android.com/guide/topics/data/autobackup#XMLSyntax
    /// [`Files that are backed`]: https://developer.android.com/guide/topics/data/autobackup#Files
    #[yaserde(attribute, prefix = "android", rename = "fullBackupContent")]
    pub full_backup_content: Option<Resource<XmlResource>>,
    /// This attribute indicates whether or not to use [`Auto Backup`] on devices where it
    /// is available. If set to "`true`", then your app performs Auto Backup when
    /// installed on a device running Android 6.0 (API level 23) or higher. On older
    /// devices, your app ignores this attribute and performs [`Key/Value Backups`].
    ///
    /// The default value is "`false`".
    ///
    /// [`Auto Backup`]: https://developer.android.com/guide/topics/data/autobackup
    /// [`Key/Value Backups`]: https://developer.android.com/guide/topics/data/keyvaluebackup
    #[yaserde(attribute, prefix = "android", rename = "fullBackupOnly")]
    pub full_backup_only: Option<VarOrBool>,
    /// This attribute indicates whether or not to use [`GWP-ASan`], which is a native
    /// memory allocator feature that helps find use-after-free and
    /// heap-buffer-overflow bugs.
    ///
    /// The default value is "`never`".
    ///
    /// [`GWP-ASan`]: https://developer.android.com/ndk/guides/gwp-asan
    #[yaserde(attribute, prefix = "android", rename = "gwpAsanMode")]
    pub gwp_asan_mode: Option<GwpAsanMode>,
    /// Whether or not the application contains any code — "`true`" if it does, and
    /// "`false`" if not. When the value is "`false`", the system does not try to load
    /// any application code when launching components.
    ///
    /// The default value is "`true`".
    ///
    /// For example, if your app supports [`Play Feature Delivery`] and includes feature
    /// modules that do not generate any DEX files—which is bytecode optimized for the
    /// Android platform—you need to set this property to "`false`" in the module's
    /// manifest file. Otherwise, you may get runtime errors.
    ///
    /// [`Play Feature Delivery`]: https://developer.android.com/platform/technology/app-bundle
    #[yaserde(attribute, prefix = "android", rename = "hasCode")]
    pub has_code: Option<VarOrBool>,
    /// When the user uninstalls an app, whether or not to show the user a prompt to keep
    /// the app's data.
    ///
    /// The default value is "`false`".
    #[yaserde(attribute, prefix = "android", rename = "hasFragileUserData")]
    pub has_fragile_user_data: Option<VarOrBool>,
    /// Whether or not hardware-accelerated rendering should be enabled for all activities
    /// and views in this application — "`true`" if it should be enabled, and
    /// "`false`" if not. The default value is "`true`" if you've set either
    /// [`minSdkVersion`] or [`targetSdkVersion`] to "14" or higher; otherwise, it's
    /// "`false`".
    ///
    /// Starting from Android 3.0 (API level 11), a hardware-accelerated OpenGL renderer
    /// is available to applications, to improve performance for many common 2D graphics
    /// operations. When the hardware-accelerated renderer is enabled, most operations in
    /// Canvas, Paint, Xfermode, ColorFilter, Shader, and Camera are accelerated. This
    /// results in smoother animations, smoother scrolling, and improved responsiveness
    /// overall, even for applications that do not explicitly make use the framework's
    /// OpenGL libraries.
    ///
    /// For more information, read the [`Hardware Acceleration`] guide.
    ///
    /// ## Note
    /// Not all of the OpenGL 2D operations are accelerated. If you enable the
    /// hardware-accelerated renderer, test your application to ensure that it can
    /// make use of the renderer without errors.
    ///
    /// [`minSdkVersion`]: crate::UsesSdk#structfield.min_sdk_version
    /// [`targetSdkVersion`]: crate::UsesSdk#structfield.target_sdk_version
    /// [`Hardware Acceleration`]: https://developer.android.com/guide/topics/graphics/hardware-accel
    #[yaserde(attribute, prefix = "android", rename = "hardwareAccelerated")]
    pub hardware_accelerated: Option<VarOrBool>,
    /// An icon for the application as whole, and the default icon for each of the
    /// application's components. See the individual icon attributes for [`<activity>`],
    /// [`<activity-alias>`], [`<service>`], [`<receiver>`], and [`<provider>`] elements.
    ///
    /// This attribute must be set as a reference to a drawable resource containing the
    /// image (for example `"@drawable/icon"`).
    ///
    /// There is no default icon.
    ///
    /// [`<activity>`]: crate::Activity
    /// [`<activity-alias>`]: crate::ActivityAlias
    /// [`<service>`]: crate::Service
    /// [`<receiver>`]: crate::Receiver
    /// [`<provider>`]: crate::Provider
    #[yaserde(attribute, prefix = "android")]
    pub icon: Option<MipmapOrDrawableResource>,
    /// Whether or not the application is a game. The system may group together
    /// applications classifed as games or display them separately from other
    /// applications.
    ///
    /// The default is `false`.
    #[yaserde(attribute, prefix = "android", rename = "isGame")]
    pub is_game: Option<VarOrBool>,
    /// Whether the application in question should be terminated after its settings have
    /// been restored during a full-system restore operation. Single-package restore
    /// operations will never cause the application to be shut down. Full-system restore
    /// operations typically only occur once, when the phone is first set up. Third-party
    /// applications will not normally need to use this attribute.
    ///
    /// The default is "`true`", which means that after the application has finished
    /// processing its data during a full-system restore, it will be terminated.
    #[yaserde(attribute, prefix = "android", rename = "killAfterRestore")]
    pub kill_after_restore: Option<VarOrBool>,
    /// Whether your application's processes should be created with a large Dalvik heap.
    /// This applies to all processes created for the application. It only applies to the
    /// first application loaded into a process; if you're using a shared user ID to allow
    /// multiple applications to use a process, they all must use this option consistently
    /// or they will have unpredictable results.
    ///
    /// Most apps should not need this and should instead focus on reducing their overall
    /// memory usage for improved performance. Enabling this also does not guarantee a
    /// fixed increase in available memory, because some devices are constrained by their
    /// total available memory.
    ///
    /// To query the available memory size at runtime, use the methods
    /// [`getMemoryClass()`] or [`getLargeMemoryClass()`].
    ///
    /// [`getMemoryClass()`]: https://developer.android.com/reference/android/app/ActivityManager#getMemoryClass()
    /// [`getLargeMemoryClass()`]: https://developer.android.com/reference/android/app/ActivityManager#getLargeMemoryClass()
    #[yaserde(attribute, prefix = "android", rename = "largeHeap")]
    pub large_heap: Option<VarOrBool>,
    /// A user-readable label for the application as a whole, and a default label for each
    /// of the application's components. See the individual label attributes for
    /// [`<activity>`], [`<activity-alias>`], [`<service>`], [`<receiver>`], and
    /// [`<provider>`] elements.
    ///
    /// The label should be set as a reference to a string resource, so that it can be
    /// localized like other strings in the user interface. However, as a convenience
    /// while you're developing the application, it can also be set as a raw string.
    ///
    /// [`<activity>`]: crate::Activity
    /// [`<activity-alias>`]: crate::ActivityAlias
    /// [`<service>`]: crate::Service
    /// [`<receiver>`]: crate::Receiver
    /// [`<provider>`]: crate::Provider
    #[yaserde(attribute, prefix = "android")]
    pub label: Option<StringResourceOrString>,
    /// A logo for the application as whole, and the default logo for activities. This
    /// attribute must be set as a reference to a drawable resource containing the
    /// image (for example `"@drawable/logo"`).
    ///
    /// There is no default logo.
    #[yaserde(attribute, prefix = "android")]
    pub logo: Option<Resource<DrawableResource>>,
    /// The fully qualified name of an Activity subclass that the system can launch to let
    /// users manage the memory occupied by the application on the device. The
    /// activity should also be declared with an [`<activity>`] element.
    ///
    /// [`<activity>`]: crate::Activity
    #[yaserde(attribute, prefix = "android", rename = "manageSpaceActivity")]
    pub manage_space_activity: Option<String>,
    /// The fully qualified name of an [`Application`] subclass implemented for the
    /// application. When the application process is started, this class is instantiated
    /// before any of the application's components.
    ///
    /// The subclass is optional; most applications won't need one. In the absence of a
    /// subclass, Android uses an instance of the base Application class.
    ///
    /// [`Application`]: https://developer.android.com/reference/android/app/Application
    #[yaserde(attribute, prefix = "android")]
    pub name: Option<String>,
    /// Specifies the name of the XML file that contains your application's [`Network
    /// Security Configuration`]. The value must be a reference to the XML resource file
    /// containing the configuration.
    ///
    /// This attribute was added in API level 24.
    ///
    /// [`Network Security Configuration`]: https://developer.android.com/training/articles/security-config
    #[yaserde(attribute, prefix = "android", rename = "networkSecurityConfig")]
    pub network_security_config: Option<Resource<XmlResource>>,
    /// The name of a permission that clients must have in order to interact with the
    /// application. This attribute is a convenient way to set a permission that applies
    /// to all of the application's components. It can be overwritten by setting the
    /// `permission` attributes of individual components.
    ///
    /// For more information on permissions, see the [`Permissions`] section in the
    /// introduction and another document, [`Security and Permissions`].
    ///
    /// [`Permissions`]: https://developer.android.com/guide/topics/manifest/manifest-intro#perms
    /// [`Security and Permissions`]: https://developer.android.com/training/articles/security-tips
    #[yaserde(attribute, prefix = "android")]
    pub permission: Option<String>,
    /// Whether or not the application should remain running at all times — "`true`" if it
    /// should, and "`false`" if not. The default value is "`false`". Applications
    /// should not normally set this flag; persistence mode is intended only for
    /// certain system applications.
    #[yaserde(attribute, prefix = "android")]
    pub persistent: Option<VarOrBool>,
    /// The name of a process where all components of the application should run. Each
    /// component can override this default by setting its own `process` attribute.
    ///
    /// By default, Android creates a process for an application when the first of its
    /// components needs to run. All components then run in that process. The name of the
    /// default process matches the package name set by the [`<manifest>`] element.
    ///
    /// By setting this attribute to a process name that's shared with another
    /// application, you can arrange for components of both applications to run in the
    /// same process — but only if the two applications also share a user ID and be signed
    /// with the same certificate.
    ///
    /// If the name assigned to this attribute begins with a colon (':'), a new process,
    /// private to the application, is created when it's needed. If the process name
    /// begins with a lowercase character, a global process of that name is created. A
    /// global process can be shared with other applications, reducing resource usage.
    ///
    /// [`<manifest>`]: crate::AndroidManifest
    #[yaserde(attribute, prefix = "android")]
    pub process: Option<String>,
    /// Indicates that the application is prepared to attempt a restore of any backed-up
    /// data set, even if the backup was stored by a newer version of the application
    /// than is currently installed on the device. Setting this attribute to "`true`"
    /// will permit the Backup Manager to attempt restore even when a version mismatch
    /// suggests that the data are incompatible. Use with caution!
    ///
    /// The default value of this attribute is `false`.
    #[yaserde(attribute, prefix = "android", rename = "restoreAnyVersion")]
    pub restore_any_version: Option<VarOrBool>,
    /// Whether or not the application wants to opt out of [`scoped storage`].
    ///
    /// ## Note
    /// Depending on changes related to policy or app compatibility, the system might not
    /// honor this opt-out request.
    ///
    /// [`scoped storage`]: https://developer.android.com/training/data-storage#scoped-storage
    #[yaserde(attribute, prefix = "android", rename = "requestLegacyExternalStorage")]
    pub request_legacy_external_storage: Option<VarOrBool>,
    /// Specifies the account type required by the application in order to function. If
    /// your app requires an [`Account`], the value for this attribute must correspond to
    /// the account authenticator type used by your app (as defined by
    /// [`AuthenticatorDescription`]), such as "com.google".
    ///
    /// The default value is null and indicates that the application can work without any
    /// accounts.
    ///
    /// Because restricted profiles currently cannot add accounts, specifying this
    /// attribute `makes your app unavailable from a restricted profile` unless you also
    /// declare [`android:restrictedAccountType`] with the same value.
    ///
    /// This attribute was added in API level 18.
    ///
    /// ## Caution
    /// If the account data may reveal personally identifiable information, it's important
    /// that you declare this attribute and leave [`android:restrictedAccountType`] null,
    /// so that restricted profiles cannot use your app to access personal information
    /// that belongs to the owner user.
    ///
    /// [`Account`]: https://developer.android.com/reference/android/accounts/Account
    /// [`AuthenticatorDescription`]: https://developer.android.com/reference/android/accounts/AuthenticatorDescription
    /// [`android:restrictedAccountType`]:
    /// crate::Application#structfield.restricted_account_type
    #[yaserde(attribute, prefix = "android", rename = "requiredAccountType")]
    pub required_account_type: Option<String>,
    /// Specifies whether the app supports [`multi-window display`]. You can set this
    /// attribute in either the [`<activity>`] or `<application>` element.
    ///
    /// If you set this attribute to true, the user can launch the activity in
    /// split-screen and freeform modes. If you set the attribute to false, the activity
    /// does not support multi-window mode. If this value is false, and the user attempts
    /// to launch the activity in multi-window mode, the activity takes over the full
    /// screen.
    ///
    /// If your app targets API level 24 or higher, but you do not specify a value for
    /// this attribute, the attribute's value defaults to true.
    ///
    /// This attribute was added in API level 24.
    ///
    /// ## Note
    /// A task's root activity value is applied to all additional activities launched in
    /// the task. That is, if the root activity of a task is resizable then the system
    /// treats all other activities in the task as resizable. If the root activity is not
    /// resizable, the other activities in the task are not resizable
    ///
    /// [`multi-window display`]: https://developer.android.com/guide/topics/ui/multi-window
    /// [`<activity>`]: crate::Activity
    #[yaserde(attribute, prefix = "android", rename = "resizeableActivity")]
    pub resizeable_activity: Option<VarOrBool>,
    /// Specifies the account type required by this application and indicates that
    /// restricted profiles are allowed to access such accounts that belong to the owner
    /// user. If your app requires an [`Account`] and restricted profiles `are allowed to
    /// access` the primary user's accounts, the value for this attribute must correspond
    /// to the account authenticator type used by your app (as defined by
    /// [`AuthenticatorDescription`]), such as "com.google".
    ///
    /// The default value is null and indicates that the application can work without any
    /// accounts.
    ///
    /// ## Caution
    /// Specifying this attribute allows restricted profiles to use your app with accounts
    /// that belong to the owner user, which may reveal personally identifiable
    /// information. If the account may reveal personal details, you `should not` use this
    /// attribute and you should instead declare the [`android:requiredAccountType`]
    /// attribute to make your app unavailable to restricted profiles.
    ///
    /// This attribute was added in API level 18.
    ///
    /// [`Account`]: https://developer.android.com/reference/android/accounts/Account
    /// [`AuthenticatorDescription`]: https://developer.android.com/reference/android/accounts/AuthenticatorDescription
    /// [`android:requiredAccountType`]:
    /// crate::Application#structfield.required_account_type
    #[yaserde(attribute, prefix = "android", rename = "restrictedAccountType")]
    pub restricted_account_type: Option<String>,
    /// Declares whether your application is willing to support right-to-left (RTL)
    /// layouts. If set to "`true`" and [`targetSdkVersion`] is set to 17 or higher,
    /// various RTL APIs will be activated and used by the system so your app can
    /// display RTL layouts. If set to "`false`" or if [`targetSdkVersion`] is set to
    /// 16 or lower, the RTL APIs will be ignored or will have no effect and your app
    /// will behave the same regardless of the layout direction associated to the
    /// user's Locale choice (your layouts will always be left-to-right).
    ///
    /// The default value of this attribute is "`false`".
    ///
    /// This attribute was added in API level 17.
    ///
    /// [`targetSdkVersion`]: crate::UsesSdk#structfield.target_sdk_version
    #[yaserde(attribute, prefix = "android", rename = "supportsRtl")]
    pub supports_rtl: Option<VarOrBool>,
    /// An affinity name that applies to all activities within the application, except for
    /// those that set a different affinity with their own [`taskAffinity`] attributes.
    /// See that attribute for more information.
    ///
    /// By default, all activities within an application share the same affinity. The name
    /// of that affinity is the same as the package name set by the [`<manifest>`]
    /// element.
    ///
    /// [`taskAffinity`]: crate::Activity#structfield.task_affinity
    /// [`<manifest>`]: crate::AndroidManifest
    #[yaserde(attribute, prefix = "android", rename = "taskAffinity")]
    pub task_affinity: Option<String>,
    /// Indicates whether this application is only for testing purposes. For example, it
    /// may expose functionality or data outside of itself that would cause a security
    /// hole, but is useful for testing. This kind of APK can be installed only through
    /// [`adb`] — you cannot publish it to Google Play.
    ///
    /// Android Studio automatically adds this attribute when you click `Run`.
    ///
    /// [`adb`]: https://developer.android.com/studio/command-line/adb
    #[yaserde(attribute, prefix = "android", rename = "testOnly")]
    pub test_only: Option<VarOrBool>,
    /// A reference to a style resource defining a default theme for all activities in the
    /// application. Individual activities can override the default by setting their own
    /// [`theme`] attributes. For more information, see the [`Styles and Themes`]
    /// developer guide.
    ///
    /// [`theme`]: crate::Activity#structfield.theme
    /// [`Styles and Themes`]: https://developer.android.com/guide/topics/ui/look-and-feel/themes
    #[yaserde(attribute, prefix = "android")]
    pub theme: Option<Resource<StyleResource>>,
    /// Extra options for an activity's UI.
    ///
    /// For more information about the app bar, see the [`Adding the App Bar`] training
    /// class.
    ///
    /// This attribute was added in API level 14.
    ///
    /// [`Adding the App Bar`]: https://developer.android.com/training/appbar
    #[yaserde(attribute, prefix = "android", rename = "uiOptions")]
    pub ui_options: Option<UiOptions>,
    /// Indicates whether the app intends to use cleartext network traffic, such as
    /// cleartext HTTP. The default value for apps that target API level 27 or lower is
    /// "`true`". Apps that target API level 28 or higher default to "`false`".
    ///
    /// When the attribute is set to "`false`", platform components (for example, HTTP and
    /// FTP stacks, [`DownloadManager`], and [`MediaPlayer`]) will refuse the app's
    /// requests to use cleartext traffic. Third-party libraries are strongly
    /// encouraged to honor this setting as well. The key reason for avoiding
    /// cleartext traffic is the lack of confidentiality, authenticity, and
    /// protections against tampering; a network attacker can eavesdrop on transmitted
    /// data and also modify it without being detected.
    ///
    /// This flag is honored on a best-effort basis because it's impossible to prevent all
    /// cleartext traffic from Android applications given the level of access provided to
    /// them. For example, there's no expectation that the [`Socket`] API will honor this
    /// flag because it cannot determine whether its traffic is in cleartext. However,
    /// most network traffic from applications is handled by higher-level network
    /// stacks/components, which can honor this flag by either reading it from
    /// [`ApplicationInfo.flags`] or
    /// [`NetworkSecurityPolicy.isCleartextTrafficPermitted()`].
    ///
    /// ## Note
    /// [`WebView`] honors this attribute for applications targeting API level 26 and
    /// higher.
    ///
    /// During app development, StrictMode can be used to identify any cleartext traffic
    /// from the app. See [`StrictMode.VmPolicy.Builder.detectCleartextNetwork()`] for
    /// more information.
    ///
    /// This attribute was added in API level 23.
    ///
    /// This flag is ignored on Android 7.0 (API level 24) and above if an Android Network
    /// Security Config is present.
    ///
    /// [`DownloadManager`]: https://developer.android.com/reference/android/app/DownloadManager
    /// [`MediaPlayer`]: https://developer.android.com/reference/android/media/MediaPlayer
    /// [`Socket`]: https://developer.android.com/reference/java/net/Socket
    /// [`ApplicationInfo.flags`]: https://developer.android.com/reference/android/content/pm/ApplicationInfo#flags
    /// [`NetworkSecurityPolicy.isCleartextTrafficPermitted()`]: https://developer.android.com/reference/android/security/NetworkSecurityPolicy#isCleartextTrafficPermitted()
    /// [`WebView`]: https://developer.android.com/reference/android/webkit/WebView
    /// [`StrictMode.VmPolicy.Builder.detectCleartextNetwork()`]: https://developer.android.com/reference/android/os/StrictMode.VmPolicy.Builder#detectCleartextNetwork()
    #[yaserde(attribute, prefix = "android", rename = "usesCleartextTraffic")]
    pub uses_cleartext_traffic: Option<VarOrBool>,
    /// Indicates whether the app would like the virtual machine (VM) to operate in safe
    /// mode. The default value is "`false`".
    ///
    /// This attribute was added in API level 8 where a value of "`true`" disabled the
    /// Dalvik just-in-time (JIT) compiler.
    ///
    /// This attribute was adapted in API level 22 where a value of "`true`" disabled the
    /// ART ahead-of-time (AOT) compiler.
    #[yaserde(attribute, prefix = "android", rename = "vmSafeMode")]
    pub vm_safe_mode: Option<VarOrBool>,
    /// Optional `<profileable>` tag.
    pub profileable: Option<Profileable>,
    /// List of `<activity>` tags.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub activity: Vec<Activity>,
    /// List of `<service>` tags.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service: Vec<Service>,
    /// List of `<receiver>` tags.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub receiver: Vec<Receiver>,
    /// List of `<provider>` tags.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provider: Vec<Provider>,
    /// List of `<activity-alias>` tags.
    #[yaserde(rename = "activity-alias")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub activity_alias: Vec<ActivityAlias>,
    /// List of `<meta-data>` tags.
    #[yaserde(rename = "meta-data")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub meta_data: Vec<MetaData>,
    /// List of `<uses-library>` tags.
    #[yaserde(rename = "uses-library")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub uses_library: Vec<UsesLibrary>,
    /// List of `<uses-native-library>` tags.
    #[yaserde(rename = "uses-native-library")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub uses_native_library: Vec<UsesNativeLibrary>,
}

impl Application {
    pub fn is_default(&self) -> bool {
        self == &Application::default()
    }
}

/// GWP-ASan is a native memory allocator feature that helps find [`use-after-free`] and
/// [`heap-buffer-overflow`] bugs.
///
/// [`use-after-free`]: https://cwe.mitre.org/data/definitions/416.html
/// [`heap-buffer-overflow`]: https://cwe.mitre.org/data/definitions/122.html
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GwpAsanMode {
    /// Always disabled: This setting completely disables GWP-ASan in your app and is the
    /// default for non-system apps.
    #[yaserde(rename = "never")]
    Never,
    /// Always enabled: This setting enables GWP-ASan in your app, which includes the
    /// following:
    /// 1. The operating system reserves a fixed amount of RAM for GWP-ASan operations,
    ///   approximately ~70KiB for each affected process. (Enable GWP-ASan if your app is
    ///   not critically sensitive to increases in memory usage.)
    /// 2. GWP-ASan intercepts a randomly-chosen subset of heap allocations and places
    /// them   into a special region that reliably detects memory safety violations.
    /// 3. When a memory safety violation occurs in the special region, GWP-ASan
    /// terminates   the process.
    /// 4. GWP-ASan provides additional information about the fault in the crash report.
    #[yaserde(rename = "always")]
    Always,
}

impl Default for GwpAsanMode {
    fn default() -> GwpAsanMode {
        GwpAsanMode::Never
    }
}
