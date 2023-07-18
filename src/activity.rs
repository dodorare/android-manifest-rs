use crate::VarOrBool;

use super::attribute_list::{AttributeList, VerticalBar};
use super::intent_filter::IntentFilter;
use super::layout::Layout;
use super::meta_data::MetaData;
use super::resources::{
    DrawableResource, MipmapOrDrawableResource, Resource, StringResourceOrString, StyleResource,
};
use super::ui_options::UiOptions;
use serde::{Deserialize, Serialize};

/// Declares an activity (an [`Activity`] subclass) that implements part of the
/// application's visual user interface.
///
/// All activities must be represented by `<activity>` elements in the manifest file. Any
/// that are not declared there will not be seen by the system and will never be run.
///
/// ## XML Syntax
/// ```xml
/// <activity android:allowEmbedded=["true" | "false"]
///           android:allowTaskReparenting=["true" | "false"]
///           android:alwaysRetainTaskState=["true" | "false"]
///           android:autoRemoveFromRecents=["true" | "false"]
///           android:banner="drawable resource"
///           android:clearTaskOnLaunch=["true" | "false"]
///           android:colorMode=["hdr" | "wideColorGamut"]
///           android:configChanges=["mcc", "mnc", "locale",
///                                  "touchscreen", "keyboard", "keyboardHidden",
///                                  "navigation", "screenLayout", "fontScale",
///                                  "uiMode", "orientation", "density",
///                                  "screenSize", "smallestScreenSize"]
///           android:directBootAware=["true" | "false"]
///           android:documentLaunchMode=["intoExisting" | "always" |
///                                       "none" | "never"]
///           android:enabled=["true" | "false"]
///           android:excludeFromRecents=["true" | "false"]
///           android:exported=["true" | "false"]
///           android:finishOnTaskLaunch=["true" | "false"]
///           android:hardwareAccelerated=["true" | "false"]
///           android:icon="drawable resource"
///           android:immersive=["true" | "false"]
///           android:label="string resource"
///           android:launchMode=["standard" | "singleTop"|
///                               "singleTask" | "singleInstance"]
///           android:lockTaskMode=["normal" | "never" |
///                                 "if_whitelisted" | "always"]
///           android:maxRecents="integer"
///           android:maxAspectRatio="float"
///           android:multiprocess=["true" | "false"]
///           android:name="string"
///           android:noHistory=["true" | "false"]
///           android:parentActivityName="string"
///           android:persistableMode=["persistRootOnly" |
///                                    "persistAcrossReboots" | "persistNever"]
///           android:permission="string"
///           android:process="string"
///           android:relinquishTaskIdentity=["true" | "false"]
///           android:resizeableActivity=["true" | "false"]
///           android:screenOrientation=["unspecified" | "behind" |
///                                      "landscape" | "portrait" |
///                                      "reverseLandscape" | "reversePortrait" |
///                                      "sensorLandscape" | "sensorPortrait" |
///                                      "userLandscape" | "userPortrait" |
///                                      "sensor" | "fullSensor"|"nosensor" |
///                                      "user" | "fullUser" | "locked"]
///           android:showForAllUsers=["true" | "false"]
///           android:stateNotNeeded=["true" | "false"]
///           android:supportsPictureInPicture=["true" | "false"]
///           android:taskAffinity="string"
///           android:theme="resource or theme"
///           android:uiOptions=["none" | "splitActionBarWhenNarrow"]
///           android:windowSoftInputMode=["stateUnspecified",
///                                        "stateUnchanged", "stateHidden",
///                                        "stateAlwaysHidden", "stateVisible",
///                                        "stateAlwaysVisible", "adjustUnspecified",
///                                        "adjustResize", "adjustPan"] >
///     ...
/// </activity>
/// ```
///
/// ## Contained in
/// * [`<application>`]
///
/// ## Can contain
/// * [`<intent-filter>`]
/// * [`<meta-data>`]
/// * [`<layout>`]
///
/// ## Introduced in
/// API Level 1 for all attributes except for [`noHistory`] and [`windowSoftInputMode`],
/// which were added in API Level 3.
///
/// [`Activity`]: https://developer.android.com/reference/android/app/Activity
/// [`<application>`]: crate::Application
/// [`<intent-filter>`]: crate::IntentFilter
/// [`<meta-data>`]: crate::MetaData
/// [`<layout>`]: crate::Layout
/// [`noHistory`]: crate::Activity#structfield.no_history
/// [`windowSoftInputMode`]: crate::Activity#structfield.window_soft_input_mode
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Default, Clone)]
pub struct Activity {
    /// Indicate that the activity can be launched as the embedded child of another
    /// activity. Particularly in the case where the child lives in a container such
    /// as a Display owned by another activity. For example, activities that are used
    /// for Wear custom notifications must declare this so Wear can display the
    /// activity in it's context stream, which resides in another process. The default
    /// value of this attribute is "`false`".
    #[yaserde(attribute, prefix = "android", rename = "allowEmbedded")]
    pub allow_embedded: Option<VarOrBool>,
    /// Whether or not the activity can move from the task that started it to the task it
    /// has an affinity for when that task is next brought to the front — "`true`" if
    /// it can move, and "`false`" if it must remain with the task where it started.
    ///
    /// If this attribute is not set, the value set by the corresponding
    /// [`allowTaskReparenting`] attribute of the [`<application>`] element
    /// applies to the activity. The default value is "`false`".
    ///
    /// Normally when an activity is started, it's associated with the task of the
    /// activity that started it and it stays there for its entire lifetime. You can use
    /// this attribute to force it to be re-parented to the task it has an affinity for
    /// when its current task is no longer displayed. Typically, it's used to cause the
    /// activities of an application to move to the main task associated with that
    /// application.
    ///
    /// For example, if an e-mail message contains a link to a web page, clicking the link
    /// brings up an activity that can display the page. That activity is defined by the
    /// browser application, but is launched as part of the e-mail task. If it's
    /// reparented to the browser task, it will be shown when the browser next comes to
    /// the front, and will be absent when the e-mail task again comes forward.
    ///
    /// The affinity of an activity is defined by the [`taskAffinity`] attribute. The
    /// affinity of a task is determined by reading the affinity of its root activity.
    /// Therefore, by definition, a root activity is always in a task with the same
    /// affinity. Since activities with "`singleTask`" or "`singleInstance`" launch
    /// modes can only be at the root of a task, re-parenting is limited to the
    /// "`standard`" and "`singleTop`" modes. (See also the [`launchMode`] attribute.)
    ///
    /// [`allowTaskReparenting`]: crate::Application#structfield.allow_task_reparenting
    /// [`<application>`]: crate::Application
    /// [`taskAffinity`]: crate::Activity#structfield.task_affinity
    /// [`launchMode`]: crate::Activity#structfield.launch_mode
    #[yaserde(attribute, prefix = "android", rename = "allowTaskReparenting")]
    pub allow_task_reparenting: Option<VarOrBool>,
    /// Whether or not the state of the task that the activity is in will always be
    /// maintained by the system — "`true`" if it will be, and "`false`" if the system
    /// is allowed to reset the task to its initial state in certain situations. The
    /// default value is "`false`". This attribute is meaningful only for the root
    /// activity of a task; it's ignored for all other activities.
    ///
    /// Normally, the system clears a task (removes all activities from the stack above
    /// the root activity) in certain situations when the user re-selects that task
    /// from the home screen. Typically, this is done if the user hasn't visited the
    /// task for a certain amount of time, such as 30 minutes.
    ///
    /// However, when this attribute is "`true`", users will always return to the task in
    /// its last state, regardless of how they get there. This is useful, for example, in
    /// an application like the web browser where there is a lot of state (such as
    /// multiple open tabs) that users would not like to lose.
    #[yaserde(attribute, prefix = "android", rename = "alwaysRetainTaskState")]
    pub always_retain_task_state: Option<VarOrBool>,
    /// Whether or not tasks launched by activities with this attribute remains in the
    /// [`overview screen`] until the last activity in the task is completed. If true, the
    /// task is automatically removed from the `overview screen.` This overrides the
    /// caller's use of [`FLAG_ACTIVITY_RETAIN_IN_RECENTS`]. It must be a boolean value,
    /// either "`true`" or "`false`".
    ///
    /// [`overview screen`]: https://developer.android.com/guide/components/activities/recents
    /// [`FLAG_ACTIVITY_RETAIN_IN_RECENTS`]: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_RETAIN_IN_RECENTS
    #[yaserde(attribute, prefix = "android", rename = "autoRemoveFromRecents")]
    pub auto_remove_from_recents: Option<VarOrBool>,
    /// A [`drawable resource`] providing an extended graphical banner for its associated
    /// item. Use with the `<activity>` tag to supply a default banner for a specific
    /// activity, or with the [`<application>`] tag to supply a banner for all
    /// application activities.
    ///
    /// The system uses the banner to represent an app in the Android TV home screen.
    /// Since the banner is displayed only in the home screen, it should only be
    /// specified by applications with an activity that handles the
    /// [`CATEGORY_LEANBACK_LAUNCHER`] intent.
    ///
    /// This attribute must be set as a reference to a drawable resource containing the
    /// image (for example "`@drawable/banner`"). There is no default banner.
    ///
    /// See [`Provide a home screen banner`] in Get Started with TV Apps for more
    /// information.
    ///
    /// [`drawable resource`]: https://developer.android.com/guide/topics/resources/drawable-resource
    /// [`<application>`]: crate::Application
    /// [`CATEGORY_LEANBACK_LAUNCHER`]: https://developer.android.com/reference/android/content/Intent#CATEGORY_LEANBACK_LAUNCHER
    /// [`Provide a home screen banner`]: https://developer.android.com/training/tv/start/start#banner
    #[yaserde(attribute, prefix = "android")]
    pub banner: Option<Resource<DrawableResource>>,
    /// Whether or not all activities will be removed from the task, except for the root
    /// activity, whenever it is re-launched from the home screen — "`true`" if the
    /// task is always stripped down to its root activity, and "`false`" if not. The
    /// default value is "`false`". This attribute is meaningful only for activities
    /// that start a new task (the root activity); it's ignored for all other
    /// activities in the task.
    ///
    /// When the value is "`true`", every time users start the task again, they are
    /// brought to its root activity regardless of what they were last doing in the
    /// task and regardless of whether they used the Back or Home button to leave it.
    /// When the value is "`false`", the task may be cleared of activities in some
    /// situations (see the [`alwaysRetainTaskState`] attribute), but not always.
    ///
    /// Suppose, for example, that someone launches activity P from the home screen, and
    /// from there goes to activity Q. The user next presses Home, and then returns to
    /// activity P. Normally, the user would see activity Q, since that is what they were
    /// last doing in P's task. However, if P set this flag to "`true`", all of the
    /// activities on top of it (Q in this case) would be removed when the user launched
    /// activity P from the home screen. So the user would see only P when returning to
    /// the task.
    ///
    /// If this attribute and [`allowTaskReparenting`] are both "`true`", any activities
    /// that can be re-parented are moved to the task they share an affinity with; the
    /// remaining activities are then dropped, as described above.
    ///
    /// This attribute is ignored if [`FLAG_ACTIVITY_RESET_TASK_IF_NEEDED`] is not set.
    ///
    /// [`alwaysRetainTaskState`]: crate::Activity#structfield.always_retain_task_state
    /// [`allowTaskReparenting`]: crate::Activity#structfield.allow_task_reparenting
    /// [`FLAG_ACTIVITY_RESET_TASK_IF_NEEDED`]: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_RESET_TASK_IF_NEEDED
    #[yaserde(attribute, prefix = "android", rename = "clearTaskOnLaunch")]
    pub clear_task_on_launch: Option<VarOrBool>,
    /// Requests the activity to be displayed in wide color gamut mode on compatible
    /// devices. In wide color gamut mode, a window can render outside of the [`SRGB`]
    /// gamut to display more vibrant colors. If the device doesn't support wide color
    /// gamut rendering, this attribute has no effect. For more information about
    /// rendering in wide color mode, see [`Enhancing Graphics with Wide Color Content`].
    ///
    /// [`Enhancing Graphics with Wide Color Content`]: https://developer.android.com/training/wide-color-gamut
    /// [`SRGB`]: https://developer.android.com/reference/android/graphics/ColorSpace.Named#SRGB
    #[yaserde(attribute, prefix = "android", rename = "colorMode")]
    pub color_mode: Option<ColorMode>,
    /// Lists configuration changes that the activity will handle itself. When a
    /// configuration change occurs at runtime, the activity is shut down and
    /// restarted by default, but declaring a configuration with this
    /// attribute will prevent the activity from being restarted. Instead, the
    /// activity remains running and its [`onConfigurationChanged()`] method is
    /// called.
    ///
    /// Any or all of the following strings are valid values for this attribute. Multiple
    /// values are separated by '`|`' — for example: "`locale|navigation|orientation`".
    ///
    /// ## Note
    /// Using this attribute should be avoided and used only as a last resort. Please read
    /// [`Handling Runtime Changes`] for more information about how to properly handle a
    /// restart due to a configuration change.
    ///
    /// [`Handling Runtime Changes`]: https://developer.android.com/guide/topics/resources/runtime-changes
    /// [`onConfigurationChanged()`]: https://developer.android.com/reference/android/app/Activity#onConfigurationChanged(android.content.res.Configuration)
    #[yaserde(
        attribute,
        prefix = "android",
        rename = "configChanges",
        skip_serializing_if = "check_config_changes"
    )]
    #[serde(default, skip_serializing_if = "AttributeList::is_empty")]
    pub config_changes: AttributeList<VerticalBar, ConfigChanges>,
    /// Whether or not the activity is direct-boot aware; that is, whether or  not it can
    /// run before the user unlocks the device.
    ///
    /// The default value is "`false`".
    ///
    /// ## Note
    /// During [`Direct Boot`], an activity in your application can only access the data
    /// that is stored in device protected storage.
    ///
    /// [`Direct Boot`]: https://developer.android.com/training/articles/direct-boot
    #[yaserde(attribute, prefix = "android", rename = "directBootAware")]
    pub direct_boot_aware: Option<VarOrBool>,
    /// Specifies how a new instance of an activity should be added to a task
    /// each time it is launched. This attribute permits the user to have
    /// multiple documents from the same application appear in the [`overview
    /// screen`].
    ///
    /// ## Note
    /// For values other than "`none`" and "`never`" the activity must be defined with
    /// `launchMode`="`standard`". If this attribute is not specified,
    /// `documentLaunchMode`="`none`" is used.
    ///
    /// [`overview screen`]: https://developer.android.com/guide/components/activities/recents
    #[yaserde(attribute, prefix = "android", rename = "documentLaunchMode")]
    pub document_launch_mode: Option<DocumentLaunchMode>,
    /// Whether or not the activity can be instantiated by the system — "`true`" if it can
    /// be, and "`false`" if not.
    ///
    /// The default value is "`true`".
    ///
    /// The [`<application>`] element has its own [`enabled`] attribute that applies to
    /// all application components, including activities. The [`<application>`] and
    /// `<activity>` attributes must both be "`true`" (as they both are by default)
    /// for the system to be able to instantiate the activity. If either is "`false`",
    /// it cannot be instantiated.
    ///
    /// [`<application>`]: crate::Application
    /// [`enabled`]: crate::Application#structfield.enabled
    #[yaserde(attribute, prefix = "android")]
    pub enabled: Option<VarOrBool>,
    /// Whether or not the task initiated by this activity should be excluded
    /// from the list of recently used applications, the [`overview screen`].
    /// That is, when this activity is the root activity of a new task, this
    /// attribute determines whether the task should not appear in the list of
    /// recent apps. Set "`true`" if the task should be excluded from the
    /// list; set "`false`" if it should be included.
    ///
    /// The default value is "`false`".
    ///
    /// [`overview screen`]: https://developer.android.com/guide/components/activities/recents
    #[yaserde(attribute, prefix = "android", rename = "excludeFromRecents")]
    pub exclude_from_recents: Option<VarOrBool>,
    /// This element sets whether the activity can be launched by components of other
    /// applications — "`true`" if it can be, and "`false`" if not. If "`false`", the
    /// activity can be launched only by components of the same application or
    /// applications with the same user ID.
    ///
    /// If you are using intent filters, you should not set this element "`false`". If you
    /// do so, and an app tries to call the activity, system throws an
    /// [`ActivityNotFoundException`]. Instead, you should prevent other apps
    /// from calling the activity by not setting intent filters for it.
    ///
    /// If you do not have intent filters, the default value for this element is
    /// "false". If you set the element "true", the activity is accessible to
    /// any app that knows its exact class name, but does not resolve when
    /// the system tries to match an implicit intent.
    ///
    /// This attribute is not the only way to limit an activity's exposure to other
    /// applications. You can also use a permission to limit the external entities
    /// that can invoke the activity (see the [`permission`] attribute).
    ///
    /// [`ActivityNotFoundException`]: https://developer.android.com/reference/android/content/ActivityNotFoundException
    /// [`permission`]: crate::Activity#structfield.permission
    #[yaserde(attribute, prefix = "android")]
    pub exported: Option<VarOrBool>,
    /// Whether or not an existing instance of the activity should be shut down (finished)
    /// whenever the user again launches its task (chooses the task on the home
    /// screen) — "`true`" if it should be shut down, and "`false`" if not.
    ///
    /// The default value is "`false`".
    ///
    /// If this attribute and [`allowTaskReparenting`] are both "`true`", this attribute
    /// trumps the other. The affinity of the activity is ignored. The activity is not
    /// re-parented, but destroyed.
    ///
    /// [`allowTaskReparenting`]: https://developer.android.com/guide/topics/manifest/activity-element#reparent
    #[yaserde(attribute, prefix = "android", rename = "finishOnTaskLaunch")]
    pub finish_on_task_launch: Option<VarOrBool>,
    /// Whether or not hardware-accelerated rendering should be enabled for this Activity
    /// — "`true`" if it should be enabled, and "`false`" if not.
    ///
    /// The default value is "`false`".
    ///
    /// Starting from Android 3.0, a hardware-accelerated OpenGL renderer is available to
    /// applications, to improve performance for many common 2D graphics operations.
    /// When the hardware-accelerated renderer is enabled, most operations in Canvas,
    /// Paint, Xfermode, ColorFilter, Shader, and Camera are accelerated.This results in
    /// smoother animations, smoother scrolling, and improved responsiveness overall, even
    /// for applications that do not explicitly make use the framework's OpenGL libraries.
    /// Because of the increased resources required to enable hardware acceleration, your
    /// app will consume more RAM.
    ///
    /// Note that not all of the OpenGL 2D operations are accelerated. If you enable the
    /// hardware-accelerated renderer, test your application to ensure that it can
    /// make use of the renderer without errors.
    #[yaserde(attribute, prefix = "android", rename = "hardwareAccelerated")]
    pub hardware_accelerated: Option<VarOrBool>,
    /// An icon representing the activity. The icon is displayed to users when a
    /// representation of the activity is required on-screen. For example,
    /// icons for activities that initiate tasks are displayed in the launcher
    /// window. The icon is often accompanied by a label (see the
    /// [`android:label`] attribute).
    ///
    /// This attribute must be set as a reference to a drawable resource containing the
    /// image definition. If it is not set, the icon specified for the application as
    /// a whole is used instead (see the [`<application>`] element's
    /// [`icon`](crate::Application#structfield.icon) attribute).
    ///
    /// The activity's icon — whether set here or by the [`<application>`] element — is
    /// also the default icon for all the activity's intent filters (see the
    /// [`<intent-filter>`] element's [`icon`](crate::IntentFilter#structfield.icon)
    /// attribute).
    ///
    /// [`android:label`]: crate::Activity#structfield.label
    /// [`<application>`]: crate::Application
    /// [`<intent-filter>`]: crate::IntentFilter
    #[yaserde(attribute, prefix = "android")]
    pub icon: Option<MipmapOrDrawableResource>,
    /// Sets the immersive mode setting for the current activity. If the
    /// `android:immersive` attribute is set to true in the app's manifest entry
    /// for this activity, the [`ActivityInfo.flags`] member always has its
    /// [`FLAG_IMMERSIVE`] bit set, even if the immersive mode is changed at
    /// runtime using the [`setImmersive()`] method.
    ///
    /// [`ActivityInfo.flags`]: https://developer.android.com/reference/android/content/pm/ActivityInfo#flags
    /// [`FLAG_IMMERSIVE`]: https://developer.android.com/reference/android/content/pm/ActivityInfo#FLAG_IMMERSIVE
    /// [`setImmersive()`]: https://developer.android.com/reference/android/app/Activity#setImmersive(boolean)
    #[yaserde(attribute, prefix = "android")]
    pub immersive: Option<VarOrBool>,
    /// A user-readable label for the activity. The label is displayed on-screen when the
    /// activity must be represented to the user. It's often displayed along with the
    /// activity icon. If this attribute is not set, the label set for the application
    /// as a whole is used instead (see the [`<application>`] element's
    /// [`label`](crate::Application#structfield.label) attribute).
    ///
    /// The activity's label — whether set here or by the [`<application>`] element — is
    /// also the default label for all the activity's intent filters (see the
    /// [`<intent-filter>`] element's [`label`](crate::IntentFilter#structfield.label)
    /// attribute).
    ///
    /// The label should be set as a reference  to a string resource, so that it can be
    /// localized like other strings in the user interface. However, as a convenience
    /// while you're developing the application, it can also be set as a raw string.
    ///
    /// [`<application>`]: crate::Application
    /// [`<intent-filter>`]: crate::IntentFilter
    #[yaserde(attribute, prefix = "android")]
    pub label: Option<StringResourceOrString>,
    /// An instruction on how the activity should be launched. There are four modes that
    /// work in conjunction with activity flags (`FLAG_ACTIVITY_*` constants) in
    /// [`Intent`] objects to determine what should happen when the activity is called
    /// upon to handle an intent.
    ///
    /// The default mode is `"standard"`.
    ///
    /// [`Intent`]: https://developer.android.com/reference/android/content/Intent
    #[yaserde(attribute, prefix = "android", rename = "launchMode")]
    pub launch_mode: Option<LaunchMode>,
    /// Determines how the system presents this activity when the device is running in
    /// [`lock task mode`].
    ///
    /// Android can run tasks in an immersive, kiosk-like fashion called lock task mode.
    /// When the system runs in lock task mode, device users typically can’t see
    /// notifications, access non-allowlisted apps, or return to the home screen
    /// (unless the Home app is allowlisted). Only apps that have been allowlisted by
    /// a device policy controller (DPC) can run when the system is in lock task mode.
    /// System and [`privileged apps`], however, can run in lock task mode without
    /// being allowlisted.
    ///
    /// This attribute was introduced in API Level 23.
    ///
    /// [`lock task mode`]: https://developer.android.com/work/dpc/dedicated-devices/lock-task-mode
    /// [`privileged apps`]: https://source.android.com/devices/tech/config/perms-allowlist
    #[yaserde(attribute, prefix = "android", rename = "lockTaskMode")]
    pub lock_task_mode: Option<LockTaskMode>,
    /// The maximum number of tasks rooted at this activity in the [`overview screen`].
    /// When this number of entries is reached, the system removes the least-recently
    /// used instance from the overview screen. Valid values are 1 through 50 (25 on
    /// low memory devices); zero is invalid. This must be an integer value, such as
    /// 50.
    ///
    /// The default value is 16.
    ///
    /// [`overview screen`]: https://developer.android.com/guide/components/activities/recents
    #[yaserde(attribute, prefix = "android", rename = "maxRecents")]
    pub max_recents: Option<u32>,
    /// The maximum aspect ratio the activity supports. If the app runs on a device with a
    /// wider aspect ratio, the system automatically letterboxes the app, leaving
    /// portions of the screen unused so the app can run at its specified maximum
    /// aspect ratio. Maximum aspect ratio is expressed as the decimal form of the
    /// quotient of the device's longer dimension divided by its shorter dimension.
    /// For example, if the maximum aspect ratio is 7:3, set the value of this
    /// attribute to 2.33. On non-wearable devices, the value of this attribute needs
    /// to be 1.33 or greater. On wearable devices, it must be 1.0 or greater.
    /// Otherwise, the system ignores the set value.
    ///
    /// ## Note
    /// This attribute is ignored if the activity has [`resizeableActivity`] set to true,
    /// since that means your activity supports any size.
    ///
    /// For more information about this attribute, see [`Supporting Multiple Screens`].
    ///
    /// [`Supporting Multiple Screens`]: https://developer.android.com/guide/practices/screens_support
    /// [`resizeableActivity`]: crate::Activity#structfield.resizeable_activity
    #[yaserde(attribute, prefix = "android", rename = "maxAspectRatio")]
    pub max_aspect_ratio: Option<f32>,
    /// Whether an instance of the activity can be launched into the process of the
    /// component that started it — "`true`" if it can be, and "`false`" if not.
    ///
    /// The default value is "`false`".
    ///
    /// Normally, a new instance of an activity is launched into the process of the
    /// application that defined it, so all instances of the activity run in the same
    /// process. However, if this flag is set to "`true`", instances of the activity
    /// can run in multiple processes, allowing the system to create instances
    /// wherever they are used (provided permissions allow it), something that is
    /// almost never necessary or desirable.
    #[yaserde(attribute, prefix = "android")]
    pub multiprocess: Option<VarOrBool>,
    /// The name of the class that implements the activity, a subclass of [`Activity`].
    /// The attribute value should be a fully qualified class name (such as, "`com.
    /// example.project.ExtracurricularActivity`"). However, as a shorthand, if the
    /// first character of the name is a period (for example, "`.
    /// ExtracurricularActivity`"), it is appended to the package name specified in
    /// the [`<manifest>`] element.
    ///
    /// Once you publish your application, you [`should not change this name`] (unless
    /// you've set [`android:exported`]="`false`").
    ///
    /// There is no default. The name must be specified.
    ///
    /// [`Activity`]: https://developer.android.com/reference/android/app/Activity
    /// [`<manifest>`]: crate::AndroidManifest
    /// [`should not change this name`]: https://android-developers.googleblog.com/2011/06/things-that-cannot-change.html
    /// [`android:exported`]: crate::Activity#structfield.exported
    #[yaserde(attribute, prefix = "android")]
    pub name: String,
    /// Whether or not the activity should be removed from the activity stack and finished
    /// (its [`finish()`] method called) when the user navigates away from it and it's
    /// no longer visible on screen — "`true`" if it should be finished, and "`false`"
    /// if not.
    ///
    /// The default value is "`false`".
    ///
    /// A value of "`true`" means that the activity will not leave a historical
    /// trace. It will not remain in the activity stack for the task, so the
    /// user will not be able to return to it. In this case,
    /// [`onActivityResult()`] is never called if you start another activity for a
    /// result from this activity.
    ///
    /// This attribute was introduced in API Level 3.
    ///
    /// [`finish()`]: https://developer.android.com/reference/android/app/Activity#finish()
    /// [`onActivityResult()`]: https://developer.android.com/reference/android/app/Activity#onActivityResult(int,%20int,%20android.content.Intent)
    #[yaserde(attribute, prefix = "android", rename = "noHistory")]
    pub no_history: Option<VarOrBool>,
    /// The class name of the logical parent of the activity. The name here must match the
    /// class name given to the corresponding `<activity>` element's [`android:name`]
    /// attribute.
    ///
    /// The system reads this attribute to determine which activity should be started when
    /// the user presses the Up button in the action bar. The system can also use this
    /// information to synthesize a back stack of activities with
    /// [`TaskStackBuilder`].
    ///
    /// To support API levels 4 - 16, you can also declare the parent activity
    /// with a `<meta-data>` element that specifies a value for
    /// `"android.support.PARENT_ACTIVITY"`.
    ///
    /// For more information about declaring the parent activity to support Up navigation,
    /// read [`Providing Up Navigation`].
    ///
    /// This attribute was introduced in API Level 16.
    ///
    /// ## XML Examples
    /// To support API levels 4 - 16:
    /// ```xml
    /// <activity android:name="com.example.app.ChildActivity"
    ///           android:label="@string/title_child_activity"
    ///           android:parentActivityName="com.example.app.MainActivity" >
    ///      <!-- Parent activity meta-data to support API level 4+ -->
    ///      <meta-data android:name="android.support.PARENT_ACTIVITY"
    ///                 android:value="com.example.app.MainActivity" />
    /// </activity>
    /// ```
    ///
    /// [`android:name`]: crate::Activity#structfield.name
    /// [`TaskStackBuilder`]: https://developer.android.com/reference/android/app/TaskStackBuilder
    /// [`Providing Up Navigation`]: https://developer.android.com/guide/navigation
    #[yaserde(attribute, prefix = "android", rename = "parentActivityName")]
    pub parent_activity_name: Option<String>,
    /// Defines how an instance of an activity is preserved within a containing task
    /// across device restarts.
    ///
    /// If the root activity of a task sets this attribute's value to `persistRootOnly`,
    /// then only the root activity is preserved. Otherwise, the activities that are
    /// higher up the task's [`back stack`] are examined; any of these activities that
    /// set this attribute's value to `persistAcrossReboots` are preserved.
    ///
    /// This attribute was introduced in API level 21.
    ///
    /// [`back stack`]: https://developer.android.com/guide/components/activities/tasks-and-back-stack
    #[yaserde(attribute, prefix = "android", rename = "persistableMode")]
    pub persistable_mode: Option<PersistableMode>,
    /// The name of a permission that clients must have to launch the activity or
    /// otherwise get it to respond to an intent. If a caller of [`startActivity()`]
    /// or [`startActivityForResult()`] has not been granted the specified permission,
    /// its intent will not be delivered to the activity.
    ///
    /// If this attribute is not set, the permission set by the [`<application>`]
    /// element's [`permission`] attribute applies to the activity. If neither
    /// attribute is set, the activity is not protected by a permission.
    ///
    /// For more information on permissions, see the [`Permissions`] section in the
    /// introduction and another document, [`Security and Permissions`].
    ///
    /// [`startActivity()`]: https://developer.android.com/reference/android/content/Context#startActivity(android.content.Intent)
    /// [`startActivityForResult()`]: https://developer.android.com/reference/android/app/Activity#startActivityForResult(android.content.Intent,%20int)
    /// [`<application>`]: crate::Application
    /// [`permission`]: crate::Application#structfield.permission
    /// [`Permissions`]: https://developer.android.com/guide/topics/manifest/manifest-intro#perms
    /// [`Security and Permissions`]: https://developer.android.com/training/articles/security-tips
    #[yaserde(attribute, prefix = "android")]
    pub permission: Option<String>,
    /// The name of the process in which the activity should run. Normally, all components
    /// of an application run in a default process name created for the application
    /// and you do not need to use this attribute. But if necessary, you can override
    /// the default process name with this attribute, allowing you to spread your app
    /// components across multiple processes.
    ///
    /// If the name assigned to this attribute begins with a colon (':'), a new process,
    /// private to the application, is created when it's needed and the activity runs
    /// in that process. If the process name begins with a lowercase character, the
    /// activity will run in a global process of that name, provided that it has
    /// permission to do so. This allows components in different applications to share
    /// a process, reducing resource usage.
    ///
    /// The [`<application>`] element's [`process`] attribute can set a different default
    /// process name for all components.
    ///
    /// [`<application>`]: crate::Application
    /// [`process`]: crate::Application#structfield.process
    #[yaserde(attribute, prefix = "android")]
    pub process: Option<String>,
    /// Whether or not the activity relinquishes its task identifiers to an activity above
    /// it in the task stack. A task whose root activity has this attribute set to
    /// "`true`" replaces the base Intent with that of the next activity in the task.
    /// If the next activity also has this attribute set to "`true`" then it will
    /// yield the base Intent to any activity that it launches in the same task. This
    /// continues for each activity until an activity is encountered which has this
    /// attribute set to "`false`".
    ///
    /// The default value is "`false`".
    ///
    /// This attribute set to "`true`" also permits the activity's use of the
    /// [`ActivityManager.TaskDescription`] to change labels, colors and icons in the
    /// [`overview screen`].
    ///
    /// [`ActivityManager.TaskDescription`]: https://developer.android.com/reference/android/app/ActivityManager.TaskDescription
    /// [`overview screen`]: https://developer.android.com/guide/components/activities/recents
    #[yaserde(attribute, prefix = "android", rename = "relinquishTaskIdentity")]
    pub relinquish_task_identity: Option<VarOrBool>,
    /// Specifies whether the app supports [`multi-window display`]. You can set
    /// this attribute in either the `<activity>` or [`<application>`] element.
    ///
    /// If you set this attribute to true, the user can launch the activity in
    /// split-screen and freeform modes. If you set the attribute to false, the
    /// activity does not support multi-window mode. If this value is false,
    /// and the user attempts to launch the activity in multi-window mode, the
    /// activity takes over the full screen.
    ///
    /// If your app targets API level 24 or higher, but you do not specify a value for
    /// this attribute, the attribute's value defaults to true.
    ///
    /// This attribute was added in API level 24.
    ///
    /// ## Note
    /// A task's root activity value is applied to all additional activities launched
    /// in the task. That is, if the root activity of a task is resizable then the system
    /// treats all other activities in the task as resizable. If the root activity is not
    /// resizable, the other activities in the task are not resizable.
    ///
    /// [`multi-window display`]: https://developer.android.com/guide/topics/ui/multi-window
    /// [`<application>`]: crate::Application
    #[yaserde(attribute, prefix = "android", rename = "resizeableActivity")]
    pub resizeable_activity: Option<VarOrBool>,
    /// The orientation of the activity's display on the device. The system ignores this
    /// attribute if the activity is running in [`multi-window mode`].
    ///
    /// ## Note
    /// When you declare one of the landscape or portrait values, it is considered a hard
    /// requirement for the orientation in which the activity runs. As such, the value
    /// you declare enables filtering by services such as Google Play so your
    /// application is available only to devices that support the orientation required
    /// by your activities. For example, if you declare either `"landscape"`,
    /// `"reverseLandscape"`, or `"sensorLandscape"`, then your application will be
    /// available only to devices that support landscape orientation. However, you
    /// should also explicitly declare that your application requires either portrait
    /// or landscape orientation with the [`<uses-feature>`] element.
    ///
    /// ```xml
    /// <uses-feature android:name="android.hardware.screen.portrait"/>.
    /// ```
    ///
    /// This is purely a filtering behavior provided by Google Play (and other services
    /// that support it) and the platform itself does not control whether your app can
    /// be installed when a device supports only certain orientations.
    ///
    /// [`multi-window mode`]: https://developer.android.com/guide/topics/ui/multi-window
    /// [`<uses-feature>`]: crate::UsesFeature
    #[yaserde(attribute, prefix = "android", rename = "screenOrientation")]
    pub screen_orientation: Option<ScreenOrientation>,
    /// Whether or not the activity is shown when the device's current user is
    /// different than the user who launched the activity. You can set this
    /// attribute to a literal value — "`true`" or "`false`" — or you can set the
    /// attribute to a resource or theme attribute that contains a boolean
    /// value.
    ///
    /// This attribute was added in API level 23.
    #[yaserde(attribute, prefix = "android", rename = "showForAllUsers")]
    pub show_for_all_users: Option<VarOrBool>,
    /// Whether or not the activity can be killed and successfully restarted without
    /// having saved its state — "`true`" if it can be restarted without reference to
    /// its previous state, and "`false`" if its previous state is required. The
    /// default value is "`false`".
    ///
    /// Normally, before an activity is temporarily shut down to save resources, its
    /// [`onSaveInstanceState()`] method is called. This method stores the current
    /// state of the activity in a [`Bundle`] object, which is then passed to
    /// [`onCreate()`] when the activity is restarted. If this attribute is set to
    /// "`true`", `onSaveInstanceState()` may not be called and `onCreate()`
    /// will be passed null instead of the Bundle — just as it was when the
    /// activity started for the first time.
    ///
    /// A "`true`" setting ensures that the activity can be restarted in the absence of
    /// retained state. For example, the activity that displays the home screen uses
    /// this setting to make sure that it does not get removed if it crashes for some
    /// reason.
    ///
    /// [`onSaveInstanceState()`]: https://developer.android.com/reference/android/app/Activity#onSaveInstanceState(android.os.Bundle)
    /// [`Bundle`]: https://developer.android.com/reference/android/os/Bundle
    /// [`onCreate()`]: https://developer.android.com/reference/android/app/Activity#onCreate(android.os.Bundle)
    #[yaserde(attribute, prefix = "android", rename = "stateNotNeeded")]
    pub state_not_needed: Option<VarOrBool>,
    /// Specifies whether the activity supports [`Picture-in-Picture`] display.
    ///
    /// This attribute was added in API level 24.
    ///
    /// [`Picture-in-Picture`]: https://developer.android.com/guide/topics/ui/picture-in-picture
    #[yaserde(attribute, prefix = "android", rename = "supportsPictureInPicture")]
    pub supports_picture_in_picture: Option<VarOrBool>,
    /// The task that the activity has an affinity for. Activities with the same affinity
    /// conceptually belong to the same task (to the same `"application"` from the
    /// user's perspective). The affinity of a task is determined by the affinity of
    /// its root activity.
    ///
    /// The affinity determines two things — the task that the activity is re-parented to
    /// (see the [`allowTaskReparenting`] attribute) and the task that will house
    /// the activity when it is launched with the [`FLAG_ACTIVITY_NEW_TASK`] flag.
    ///
    /// By default, all activities in an application have the same affinity. You can set
    /// this attribute to group them differently, and even place activities defined in
    /// different applications within the same task. To specify that the activity does
    /// not have an affinity for any task, set it to an empty string.
    ///
    /// If this attribute is not set, the activity inherits the affinity set for the
    /// application (see the [`<application>`] element's [`taskAffinity`] attribute). The
    /// name of the default affinity for an application is the package name set by the
    /// [`<manifest>`] element.
    ///
    /// [`allowTaskReparenting`]: crate::Activity#structfield.allow_task_reparenting
    /// [`FLAG_ACTIVITY_NEW_TASK`]: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_NEW_TASK
    /// [`<application>`]: crate::Application
    /// [`taskAffinity`]: crate::Application#structfield.task_affinity
    /// [`<manifest>`]: crate::AndroidManifest
    #[yaserde(attribute, prefix = "android", rename = "taskAffinity")]
    pub task_affinity: Option<String>,
    /// A reference to a style resource defining an overall theme for the activity. This
    /// automatically sets the activity's context to use this theme (see
    /// [`setTheme()`] and may also cause "starting" animations prior to the
    /// activity being launched (to better match what the activity actually looks
    /// like).
    ///
    /// If this attribute is not set, the activity inherits the theme set for the
    /// application as a whole — from the [`<application>`] element's [`theme`]
    /// attribute. If that attribute is also not set, the default system theme is
    /// used.
    ///
    /// For more information, see the [`Styles and Themes`] developer guide.
    ///
    /// [`setTheme()`]: https://developer.android.com/reference/android/content/Context#setTheme(int)
    /// [`<application>`]: crate::Application
    /// [`theme`]: crate::Application#structfield.theme
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
    /// How the main window of the activity interacts with the window containing the
    /// on-screen soft keyboard. The setting for this attribute affects two things:
    ///
    /// * The state of the soft keyboard — whether it is hidden or visible — when the
    ///   activity becomes the focus of user attention.
    /// * The adjustment made to the activity's main window — whether it is resized
    ///   smaller to make room for the soft keyboard or whether its contents pan to make
    ///   the current focus visible when part of the window is covered by the soft
    ///   keyboard.
    ///
    /// The setting must be one of the values listed in the following table, or a
    /// combination of one "`state...`" value plus one "`adjust...`" value.
    /// Setting multiple values in either group — multiple "`state...`" values,
    /// for example — has undefined results. Individual values are separated
    /// by a vertical bar (`|`).
    ///
    /// ## XML Examples
    /// ```xml
    /// <activity android:windowSoftInputMode="stateVisible|adjustResize" ... >
    /// ```
    ///
    /// Values set here (other than "`stateUnspecified`" and "`adjustUnspecified`")
    /// override values set in the theme.
    #[yaserde(
        attribute,
        prefix = "android",
        rename = "windowSoftInputMode",
        skip_serializing_if = "check_window_soft_input_mode"
    )]
    #[serde(default, skip_serializing_if = "AttributeList::is_empty")]
    pub window_soft_input_mode: AttributeList<VerticalBar, WindowSoftInputMode>,
    /// A `<layout>` tag.
    pub layout: Option<Layout>,
    /// List of `<intent-filter>` tags.
    #[yaserde(rename = "intent-filter")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub intent_filter: Vec<IntentFilter>,
    /// List of `<meta-data>` tags.
    #[yaserde(rename = "meta-data")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub meta_data: Vec<MetaData>,
}

impl Activity {
    fn check_config_changes(&self, value: &AttributeList<VerticalBar, ConfigChanges>) -> bool {
        value.is_empty()
    }

    fn check_window_soft_input_mode(
        &self,
        value: &AttributeList<VerticalBar, WindowSoftInputMode>,
    ) -> bool {
        value.is_empty()
    }
}

/// Requests the activity to be displayed in wide color gamut mode on compatible
/// devices.
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ColorMode {
    /// Indicating that the activity should use a high dynamic range if the presentation
    /// display supports it.
    #[yaserde(rename = "hdr")]
    Hdr,
    /// Indicating that the activity should use a wide color gamut if the presentation
    /// display supports it. To render wide color gamut content, your app must load a
    /// wide color bitmap, that is a bitmap with a color profile containing a color
    /// space wider than sRGB.
    #[yaserde(rename = "wideColorGamut")]
    WideColorGamut,
}

impl Default for ColorMode {
    fn default() -> Self {
        ColorMode::Hdr
    }
}

/// Lists configuration changes that the `activity` will handle itself.
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ConfigChanges {
    /// The display density has changed — the user might have specified a different
    /// display scale, or a different display might now be active. Added in API level
    /// 24.
    #[yaserde(rename = "density")]
    Density,
    /// The font scaling factor has changed — the user has selected a new global font
    /// size.
    #[yaserde(rename = "fontScale")]
    FontScale,
    /// The keyboard type has changed — for example, the user has plugged in an external
    /// keyboard.
    #[yaserde(rename = "keyboard")]
    Keyboard,
    /// The keyboard accessibility has changed — for example, the user hasrevealed the
    /// hardware keyboard.
    #[yaserde(rename = "keyboardHidden")]
    KeyboardHidden,
    /// The layout direction has changed — for example, changing from left-to-right (LTR)
    /// to right-to-left (RTL). Added in API level 17.
    #[yaserde(rename = "layoutDirection")]
    LayoutDirection,
    /// The locale has changed — the user has selected a new language that text should be
    /// displayed in.
    #[yaserde(rename = "locale")]
    Locale,
    /// The IMSI mobile country code (MCC) has changed — a SIM has been detected and
    /// updated the MCC.
    #[yaserde(rename = "mcc")]
    Mcc,
    /// The IMSI mobile network code (MNC) has changed — a SIM has been detected and
    /// updated the MNC.
    #[yaserde(rename = "mnc")]
    Mnc,
    /// The navigation type (trackball/dpad) has changed. (This should never
    /// normally happen.)
    #[yaserde(rename = "navigation")]
    Navigation,
    /// The screen orientation has changed — the user has rotated the device.
    ///
    /// ## Note
    /// If your application targets Android 3.2 (API level 13) or higher, then you should
    /// also declare the "`screenSize`" and "`screenLayout`" configurations, because
    /// they might also change when a device switches between portrait and landscape
    /// orientations.
    #[yaserde(rename = "orientation")]
    Orientation,
    /// The screen layout has changed — a different display might now be active.
    #[yaserde(rename = "screenLayout")]
    ScreenLayout,
    /// The current available screen size has changed. This represents a change in the
    /// currently available size, relative to the current aspect ratio, so will change
    /// when the user switches between landscape and portrait. Added in API level 13.
    #[yaserde(rename = "screenSize")]
    ScreenSize,
    /// The physical screen size has changed. This represents a change in size regardless
    /// of orientation, so will only change when the actual physical screen size has
    /// changed such as switching to an external display. A change to this
    /// configuration corresponds to a change in the [`smallestWidth configuration`].
    /// Added in API level 13
    ///
    /// [`smallestWidth configuration`]: https://developer.android.com/guide/topics/resources/providing-resources#SmallestScreenWidthQualifier
    #[yaserde(rename = "smallestScreenSize")]
    SmallestScreenSize,
    /// The touchscreen has changed. (This should never normally happen.)
    #[yaserde(rename = "touchscreen")]
    Touchscreen,
    /// The user interface mode has changed — the user has placed the device into a desk
    /// or car dock, or the night mode has changed. For more information about the
    /// different UI modes, see [`UiModeManager`]. Added in API level 8.
    ///
    /// [`UiModeManager`]: https://developer.android.com/reference/android/app/UiModeManager
    #[yaserde(rename = "uiMode")]
    UiMode,
}

impl Default for ConfigChanges {
    fn default() -> Self {
        ConfigChanges::Density
    }
}

/// Four values which produce the following effects when the user opens a document with
/// the application
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum DocumentLaunchMode {
    /// The system searches for a task whose base intent's `ComponentName` and data URI
    /// match those of the launching intent. If the system finds such a task, the
    /// system clears the task, and restarts with the root activity receiving a call
    /// to [`onNewIntent(android.content.Intent)`]. If the system does not find such a
    /// task, the system creates a new task.
    ///
    /// [`onNewIntent(android.content.Intent)`]: https://developer.android.com/reference/android/app/Activity#onNewIntent(android.content.Intent)
    #[yaserde(rename = "intoExisting")]
    IntoExisting,
    /// The activity creates a new task for the document, even if the document is already
    /// opened. This is the same as setting both the [`FLAG_ACTIVITY_NEW_DOCUMENT`]
    /// and [`FLAG_ACTIVITY_MULTIPLE_TASK`] flags.
    ///
    /// [`FLAG_ACTIVITY_NEW_DOCUMENT`]: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_NEW_DOCUMENT
    /// [`FLAG_ACTIVITY_MULTIPLE_TASK`]: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_MULTIPLE_TASK
    #[yaserde(rename = "always")]
    Always,
    /// The activity does not create a new task for the activity. This is the default
    /// value, which creates a new task only when [`FLAG_ACTIVITY_NEW_TASK`]
    /// is set. The overview screen treats the activity as it would by
    /// default: it displays a single task for the app, which resumes from
    /// whatever activity the user last invoked.
    ///
    /// [`FLAG_ACTIVITY_NEW_TASK`]: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_NEW_TASK
    #[yaserde(rename = "none")]
    None,
    /// This activity is not launched into a new document even if the Intent contains
    /// [`FLAG_ACTIVITY_NEW_DOCUMENT`]. Setting this overrides the behavior of the
    /// [`FLAG_ACTIVITY_NEW_DOCUMENT`] and [`FLAG_ACTIVITY_MULTIPLE_TASK`] flags, if
    /// either of these are set in the activity, and the overview screen displays a
    /// single task for the app, which resumes from whatever activity the user last
    /// invoked.
    ///
    /// [`FLAG_ACTIVITY_NEW_DOCUMENT`]: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_NEW_DOCUMENT
    /// [`FLAG_ACTIVITY_MULTIPLE_TASK`]: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_MULTIPLE_TASK
    #[yaserde(rename = "never")]
    Never,
}

impl Default for DocumentLaunchMode {
    fn default() -> Self {
        DocumentLaunchMode::None
    }
}

/// An instruction on how the activity should be launched.
///
/// As shown in the enum variant description, the modes fall into two main groups, with
/// `"standard"` and `"singleTop"`activities on one side, and "singleTask"
/// and "singleInstance" activities on the other. An activity with the
/// `"standard"` or "singleTop" launch mode can be instantiated multiple
/// times. The instances can belong to any task and can be located
/// anywhere in the activity stack. Typically, they're launched into the
/// task that called [`startActivity()`] (unless the Intent object contains a
/// [`FLAG_ACTIVITY_NEW_TASK`] instruction, in which case a different task is
/// chosen — see the [`taskAffinity`] attribute).
///
/// In contrast, `"singleTask"` and `"singleInstance"` activities can only begin a
/// task. They are always at the root of the activity stack. Moreover, the
/// device can hold only one instance of the activity at a time — only one
/// such task.
///
/// The `"standard"` and `"singleTop"` modes differ from each other in just one
/// respect: Every time there's a new intent for a `"standard"` activity, a new
/// instance of the class is created to respond to that intent. Each instance
/// handles a single intent. Similarly, a new instance of a `"singleTop"` activity
/// may also be created to handle a new intent. However, if the target task
/// already has an existing instance of the activity at the top of its stack, that
/// instance will receive the new intent (in an [`onNewIntent()`] call); a new
/// instance is not created. In other circumstances — for example, if an existing
/// instance of the `"singleTop"` activity is in the target task, but not at
/// the top of the stack, or if it's at the top of a stack, but not in
/// the target task — a new instance would be created and pushed on the
/// stack.
///
/// Similarly, if you [`navigate up`] to an activity on the current stack, the behavior is
/// determined by the parent activity's launch mode. If the parent activity has
/// launch mode `singleTop` (or the `up` intent contains [`FLAG_ACTIVITY_CLEAR_TOP`]),
/// the parent is brought to the top of the stack, and its state is preserved. The
/// navigation intent is received by the parent activity's [`onNewIntent()`] method.
/// If the parent activity has launch mode standard (and the up intent does not
/// contain [`FLAG_ACTIVITY_CLEAR_TOP`]), the current activity and its parent
/// are both popped off the stack, and a new instance of the parent activity
/// is created to receive the navigation intent.
///
/// The "singleTask" and "singleInstance" modes also differ from each other in only
/// one respect: A "singleTask" activity allows other activities to be part of its
/// task. It's always at the root of its task, but other activities (necessarily
/// "standard" and "singleTop" activities) can be launched into that task. A
/// "singleInstance" activity, on the other hand, permits no other activities to be
/// part of its task. It's the only activity in the task. If it starts another
/// activity, that activity is assigned to a different task — as if
/// FLAG_ACTIVITY_NEW_TASK was in the intent.
///
/// As shown in the enum variant description, `standard` is the default mode and is
/// appropriate for most types of activities. `SingleTop` is also a common and useful
/// launch mode for many types of activities. The other modes — `singleTask` and
/// `singleInstance` — are `not appropriate for most applications`, since they result in
/// an interaction model that is likely to be unfamiliar to users and is very different
/// from most other applications.
///
/// Regardless of the launch mode that you choose, make sure to test the usability of the
/// activity during launch and when navigating back to it from other activities and tasks
/// using the Back button.
///
/// For more information on launch modes and their interaction with Intent flags, see the
/// [`Tasks and Back Stack`] document.
///
/// [`startActivity()`]: https://developer.android.com/reference/android/content/Context#startActivity(android.content.Intent)
/// [`FLAG_ACTIVITY_NEW_TASK`]: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_NEW_TASK
/// [`taskAffinity`]: crate::Activity#structfield.task_affinity
/// [`onNewIntent()`]: https://developer.android.com/reference/android/app/Activity#onNewIntent(android.content.Intent)
/// [`navigate up`]: https://developer.android.com/guide/navigation
/// [`FLAG_ACTIVITY_CLEAR_TOP`]: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_CLEAR_TOP
/// [`Tasks and Back Stack`]: https://developer.android.com/guide/components/activities/tasks-and-back-stack
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum LaunchMode {
    /// Default. The system always creates a new instance of the activity in the target
    /// task and routes the intent to it.
    ///
    /// Use Cases: Normal launches for most activities
    ///
    /// Multiple Instances?: Yes
    #[yaserde(rename = "standard")]
    Standard,
    /// If an instance of the activity already exists at the top of the target task, the
    /// system routes the intent to that instance through a call to
    /// its [`onNewIntent()`] method, rather than creating a new instance of the
    /// activity.
    ///
    /// Use Cases: Normal launches for most activities
    ///
    /// Multiple Instances?: Conditionally
    ///
    /// [`onNewIntent()`]: https://developer.android.com/reference/android/app/Activity#onNewIntent(android.content.Intent)
    #[yaserde(rename = "singleTop")]
    SingleTop,
    /// The system creates the activity at the root of a new task and routes the intent to
    /// it. However, if an instance of the activity already exists, the system routes
    /// the intent to existing instance through a call to its [`onNewIntent()`]
    /// method, rather than creating a new one.
    ///
    /// Use Cases: Specialized launches (not recommended for general use)
    ///
    /// Multiple Instances?: No
    ///
    /// [`onNewIntent()`]: https://developer.android.com/reference/android/app/Activity#onNewIntent(android.content.Intent)
    #[yaserde(rename = "singleTask")]
    SingleTask,
    /// Same as "`singleTask`", except that the system doesn't launch any other activities
    /// into the task holding the instance. The activity is always the single and only
    /// member of its task.
    ///
    /// Use Cases: Specialized launches (not recommended for general use)
    ///
    /// Multiple Instances?: No
    #[yaserde(rename = "singleInstance")]
    SingleInstance,
}

impl Default for LaunchMode {
    fn default() -> Self {
        LaunchMode::Standard
    }
}

/// This value indicates how tasks rooted at this activity will behave in lockTask mode.
/// The value can be any one of the following [`R.attr.lockTaskMode`] string values:
///
/// [`R.attr.lockTaskMode`]: https://developer.android.com/reference/android/R.attr#lockTaskMode
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum LockTaskMode {
    /// `Default value`. This is the default value. Tasks don't launch into lock task mode
    /// but can be placed there by calling [`startLockTask()`].
    ///
    /// [`startLockTask()`]: https://developer.android.com/reference/android/app/Activity#startLockTask()
    #[yaserde(rename = "normal")]
    Normal,
    /// Tasks don't launch into lockTask mode, and the device user can't pin these tasks
    /// from the overview screen.
    ///
    /// ## Note
    /// This mode is only available to system and privileged applications.
    /// Non-privileged apps with this value are treated as `normal`.
    #[yaserde(rename = "never")]
    Never,
    /// If the DPC authorizes this package using
    /// [`DevicePolicyManager.setLockTaskPackages()`], then this mode is identical to
    /// always, except that the activity needs to call [`stopLockTask()`] before being
    /// able to finish if it is the last locked task. If the DPC does not authorize
    /// this package then this mode is identical to normal.
    ///
    /// [`DevicePolicyManager.setLockTaskPackages()`]: https://developer.android.com/reference/android/app/admin/DevicePolicyManager#setLockTaskPackages(android.content.ComponentName,%20java.lang.String[])
    /// [`stopLockTask()`]: https://developer.android.com/reference/android/app/Activity#stopLockTask()
    #[serde(rename = "if_whitelisted")]
    #[yaserde(rename = "if_whitelisted")]
    IfWhitelisted,
    /// Tasks rooted at this activity always launch into lock task mode. If the
    /// system is already in lock task mode when this task is launched then the
    /// new task are launched on top of the current task. Tasks launched in
    /// this mode can exit lock task mode by calling [`finish()`].
    ///
    /// ## Note
    /// This mode is only available to system and privileged
    /// applications. Non-privileged apps with this value are treated as `normal`.
    ///
    /// [`finish()`]: https://developer.android.com/reference/android/app/Activity#finish()
    #[yaserde(rename = "always")]
    Always,
}

impl Default for LockTaskMode {
    fn default() -> Self {
        LockTaskMode::Normal
    }
}

/// Defines how an instance of an activity is preserved within a containing task
/// across device restarts.
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PersistableMode {
    /// `Default value`. When the system restarts, the activity task is preserved, but
    /// only the root activity's launching intent is used.
    ///
    /// When your app's launching intent loads your app's root activity, the activity
    /// doesn't receive a [`PersistableBundle`] object. Therefore,
    /// don't use [`onSaveInstanceState()`] to preserve the state of your app's
    /// root activity across a device restart.
    ///
    /// ## Note
    /// This attribute value affects your app's behavior only if it's set on your app's
    /// root activity.
    ///
    /// [`PersistableBundle`]: https://developer.android.com/reference/android/os/PersistableBundle
    /// [`onSaveInstanceState()`]: https://developer.android.com/reference/android/app/Activity#onSaveInstanceState(android.os.Bundle,%20android.os.PersistableBundle)
    #[yaserde(rename = "persistRootOnly")]
    PersistRootOnly,
    /// This activity's state is preserved, along with the state of each activity higher
    /// up the [`back stack`] that has its own persistableMode attribute set to
    /// persistAcrossReboots. If an activity doesn't have a persistableMode attribute
    /// that is set to persistAcrossReboots, or if it's launched using the
    /// [`Intent.FLAG_ACTIVITY_NEW_DOCUMENT`] flag, then that activity, along with all
    /// activities higher up the back stack, aren't preserved.
    ///
    /// When an intent loads an activity whose persistableMode attribute is set to
    /// `persistAcrossReboots` in your app, the activity receives a
    /// [`PersistableBundle`] object in its [`onCreate()`] method. Therefore, you can
    /// use [`onSaveInstanceState()`] to preserve the state of an activity across a
    /// device restart as long as its persistableMode attribute is set to
    /// `persistAcrossReboots`.
    ///
    /// ## Note
    /// This attribute value affects your app's behavior even if it's
    /// set on an activity other than your app's root activity
    ///
    /// [`back stack`]: https://developer.android.com/guide/components/activities/tasks-and-back-stack
    /// [`Intent.FLAG_ACTIVITY_NEW_DOCUMENT`]: https://developer.android.com/reference/android/content/Intent#FLAG_ACTIVITY_NEW_DOCUMENT
    /// [`PersistableBundle`]: https://developer.android.com/reference/android/os/PersistableBundle
    /// [`onCreate()`]: https://developer.android.com/reference/android/app/Activity#onCreate(android.os.Bundle,%20android.os.PersistableBundle)
    /// [`onSaveInstanceState()`]: https://developer.android.com/reference/android/app/Activity#onSaveInstanceState(android.os.Bundle,%20android.os.PersistableBundle)
    #[yaserde(rename = "persistAcrossReboots")]
    PersistAcrossReboots,
    /// The activity's state isn't preserved.
    ///
    /// ## Note
    /// This attribute value affects your app's behavior only if it's
    /// set on your app's root activity.
    #[yaserde(rename = "persistNever")]
    PersistNever,
}

impl Default for PersistableMode {
    fn default() -> Self {
        PersistableMode::PersistRootOnly
    }
}

/// The orientation of the activity's display on the device.
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ScreenOrientation {
    /// `The default value`. The system chooses the orientation. The policy it uses, and
    /// therefore the choices made in specific contexts, may differ from device to
    /// device.
    #[yaserde(rename = "unspecified")]
    Unspecified,
    /// The same orientation as the activity that's immediately beneath it in the activity
    /// stack.
    #[yaserde(rename = "behind")]
    Behind,
    /// Landscape orientation (the display is wider than it is tall).
    #[yaserde(rename = "landscape")]
    Landscape,
    /// Portrait orientation (the display is taller than it is wide).
    #[yaserde(rename = "portrait")]
    Portrait,
    /// Landscape orientation in the opposite direction from normal landscape.
    ///
    /// Added in API level 9.
    #[yaserde(rename = "reverseLandscape")]
    ReverseLandscape,
    /// Portrait orientation in the opposite direction from normal portrait.
    ///
    /// Added in API level 9.
    #[yaserde(rename = "reversePortrait")]
    ReversePortrait,
    /// Landscape orientation, but can be either normal or reverse landscape based on the
    /// device sensor. The sensor is used even if the user has locked sensor-based
    /// rotation.
    ///
    /// Added in API level 9.
    #[yaserde(rename = "sensorLandscape")]
    SensorLandscape,
    /// Portrait orientation, but can be either normal or reverse portrait based on the
    /// device sensor. The sensor is used even if the user has locked sensor-based
    /// rotation.
    ///
    /// Added in API level 9.
    #[yaserde(rename = "sensorPortrait")]
    SensorPortrait,
    /// Landscape orientation, but can be either normal or reverse landscape based on the
    /// device sensor and the user's preference.
    ///
    /// Added in API level 18.
    #[yaserde(rename = "userLandscape")]
    UserLandscape,
    /// Portrait orientation, but can be either normal or reverse portrait based on the
    /// device sensor and the user's preference.
    ///
    /// Added in API level 18.
    #[yaserde(rename = "userPortrait")]
    UserPortrait,
    /// The orientation is determined by the device orientation sensor. The orientation of
    /// the display depends on how the user is holding the device; it changes when the
    /// user rotates the device. Some devices, though, will not rotate to all four
    /// possible orientations, by default. To allow all four orientations, use
    /// "`fullSensor`" The sensor is used even if the user locked sensor-based rotation.
    #[yaserde(rename = "sensor")]
    Sensor,
    /// The orientation is determined by the device orientation sensor for any of the 4
    /// orientations. This is similar to "`sensor`" except this allows any of the 4
    /// possible screen orientations, regardless of what the device will normally do
    /// (for example, some devices won't normally use reverse portrait or reverse
    /// landscape, but this enables those).
    ///
    /// Added in API level 9.
    #[yaserde(rename = "fullSensor")]
    FullSensor,
    /// The orientation is determined without reference to a physical orientation sensor.
    /// The sensor is ignored, so the display will not rotate based on how the user
    /// moves the device.
    #[yaserde(rename = "nosensor")]
    Nosensor,
    /// The user's current preferred orientation.
    #[yaserde(rename = "user")]
    User,
    /// If the user has locked sensor-based rotation, this behaves the same as user,
    /// otherwise it behaves the same as `fullSensor` and allows any of the 4 possible
    /// screen orientations.
    ///
    /// Added in API level 18.
    #[yaserde(rename = "fullUser")]
    FullUser,
    /// Locks the orientation to its current rotation, whatever that is.
    ///
    /// Added in API level 18.
    #[yaserde(rename = "locked")]
    Locked,
}

impl Default for ScreenOrientation {
    fn default() -> Self {
        ScreenOrientation::Unspecified
    }
}

/// How the main window of the activity interacts with the window containing the on-screen
/// soft keyboard.
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum WindowSoftInputMode {
    /// The state of the soft keyboard (whether it is hidden or visible) is not
    /// specified. The system will choose an appropriate state or rely on the
    /// setting in the theme. This is the default setting for the behavior
    /// of the soft keyboard.
    #[yaserde(rename = "stateUnspecified")]
    StateUnspecified,
    /// The soft keyboard is kept in whatever state it was last in, whether
    /// visible or hidden, when the activity comes to the fore.
    #[yaserde(rename = "stateUnchanged")]
    StateUnchanged,
    /// The soft keyboard is hidden when the user chooses the activity — that
    /// is, when the user affirmatively navigates forward to the activity,
    /// rather than backs into it because of leaving another activity.
    #[yaserde(rename = "stateHidden")]
    StateHidden,
    /// The soft keyboard is always hidden when the activity's main window has
    /// input focus.
    #[yaserde(rename = "stateAlwaysHidden")]
    StateAlwaysHidden,
    /// The soft keyboard is made visible when the user chooses the activity —
    /// that is, when the user affirmatively navigates forward to the activity,
    /// rather than backs into it because of leaving another activity.
    #[yaserde(rename = "stateVisible")]
    StateVisible,
    /// The soft keyboard is visible when the window receives input focus.
    #[yaserde(rename = "stateAlwaysVisible")]
    StateAlwaysVisible,
    /// It is unspecified whether the activity's main window resizes to make
    /// room for the soft keyboard, or whether the contents of the window pan to
    /// make the current focus visible on-screen. The system will
    /// automatically select one of these modes depending on whether the content
    /// of the window has any layout views that can scroll their contents.
    /// If there is such a view, the window will be resized, on the assumption
    /// that scrolling can make all of the window's contents visible within a
    /// smaller area.
    ///
    /// This is the default setting for the behavior of the main window.
    #[yaserde(rename = "adjustUnspecified")]
    AdjustUnspecified,
    /// The activity's main window is always resized to make room for the soft
    /// keyboard on screen.
    #[yaserde(rename = "adjustResize")]
    AdjustResize,
    /// The activity's main window is not resized to make room for the soft
    /// keyboard. Rather, the contents of the window are automatically panned so
    /// that the current focus is never obscured by the keyboard and users can
    /// always see what they are typing. This is generally less desirable
    /// than resizing, because the user may need to close the soft keyboard to
    /// get at and interact with obscured parts of the window.
    #[yaserde(rename = "adjustPan")]
    AdjustPan,
}

impl Default for WindowSoftInputMode {
    fn default() -> Self {
        WindowSoftInputMode::StateUnspecified
    }
}
