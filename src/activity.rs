use super::resources::{DrawableResource, Resource, StringResourceOrString, StyleResource};
use serde::{Deserialize, Serialize};

/// Declares an activity (an `Activity` subclass) that implements part of the application's visual user interface.
/// All activities must be represented by `<activity>` elements in the manifest file.
/// Any that are not declared there will not be seen by the system and will never be run.
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "activity")]
pub struct Activity {
    /// Indicate that the activity can be launched as the embedded child of another activity. Particularly in the case where the child lives in a container such as a
    /// Display owned by another activity. For example, activities that are used for Wear custom notifications must declare this so Wear can display the activity in it's context
    /// stream, which resides in another process.
    /// The default value of this attribute is `false`.
    #[serde(rename = "android:allowEmbedded")]
    pub allow_embedded: Option<bool>,
    /// Whether or not the activity can move from the task that started it to the task it has an affinity for when that task is next brought to the front — `"true"`
    /// f it can move, and `"false"` if it must remain with the task where it started.
    /// If this attribute is not set, the value set by the corresponding `allowTaskReparenting` attribute of the `<application>` element applies to the activity.
    /// The default value is `"false"`.
    #[serde(rename = "android:allowTaskReparenting")]
    pub allow_task_reparenting: Option<bool>,
    /// Whether or not the state of the task that the activity is in will always be maintained by the system — `"true"` if it will be, and `"false"`
    /// if the system is allowed to reset the task to its initial state in certain situations. The default value is `"false"`.
    /// This attribute is meaningful only for the root activity of a task; it's ignored for all other activities.
    /// Normally, the system clears a task (removes all activities from the stack above the root activity) in certain situations when the user re-selects that task from the home screen.
    /// Typically, this is done if the user hasn't visited the task for a certain amount of time, such as 30 minutes.
    #[serde(rename = "android:alwaysRetainTaskState")]
    pub always_retain_task_state: Option<bool>,
    /// Whether or not tasks launched by activities with this attribute remains in the overview screen until the last activity in the task is completed.
    /// If true, the task is automatically removed from the `overview screen.` This overrides the caller's use of `FLAG_ACTIVITY_RETAIN_IN_RECENTS`.
    /// It must be a boolean value, either `"true"` or `"false"`.
    #[serde(rename = "android:autoRemoveFromRecents")]
    pub auto_remove_from_recents: Option<bool>,
    /// A drawable resource providing an extended graphical banner for its associated item. Use with the `<activity>` tag to supply a default banner for a specific activity,
    /// or with the `<application>` tag to supply a banner for all application activities.
    /// The system uses the banner to represent an app in the Android TV home screen. Since the banner is displayed only in the home screen,
    /// it should only be specified by applications with an activity that handles the `CATEGORY_LEANBACK_LAUNCHER` intent.
    /// This attribute must be set as a reference to a drawable resource containing the image (for example `"@drawable/banner"`). There is no default banner.
    /// See Provide a home screen banner in Get Started with TV Apps for more information.
    #[serde(rename = "android:banner")]
    pub banner: Option<Resource<DrawableResource>>,
    /// Whether or not all activities will be removed from the task, except for the root activity, whenever it is re-launched from the home screen — `"true"`
    /// if the task is always stripped down to its root activity, and `"false"` if not. The default value is `"false"`.
    /// This attribute is meaningful only for activities that start a new task (the root activity); it's ignored for all other activities in the task.
    /// If this attribute and allowTaskReparenting are both `"true"`, any activities that can be re-parented are moved to the task they share an affinity with; the remaining activities are then dropped, as described above.
    /// This attribute is ignored if `FLAG_ACTIVITY_RESET_TASK_IF_NEEDED` is not set.  
    #[serde(rename = "android:clearTaskOnLaunch")]
    pub clear_task_on_launch: Option<bool>,
    /// Requests the activity to be displayed in wide color gamut mode on compatible devices. In wide color gamut mode, a window can render outside of the `SRGB` gamut to display more vibrant colors.
    /// If the device doesn't support wide color gamut rendering, this attribute has no effect.
    /// For more information about rendering in wide color mode, see `Enhancing Graphics with Wide Color Content`.
    #[serde(rename = "android:colorMode")]
    pub color_mode: Option<ColorMode>,
    /// Lists configuration changes that the activity will handle itself. When a configuration change occurs at runtime, the activity is shut down and restarted by default,
    /// but declaring a configuration with this attribute will prevent the activity from being restarted. Instead, the activity remains running and its `onConfigurationChanged()` method is called.
    /// `Note:` Using this attribute should be avoided and used only as a last resort. Please read Handling Runtime Changes for more information about how to properly handle a restart due to a configuration change.
    #[serde(rename = "android:configChanges")]
    pub config_changes: Option<ConfigChanges>,
    /// Whether or not the activity is direct-boot aware; that is, whether or not it can run before the user unlocks the device.
    /// Note: During Direct Boot, an activity in your application can only access the data that is stored in device protected storage.
    /// The default value is "false".
    #[serde(rename = "android:directBootAware")]
    pub direct_boot_aware: Option<bool>,
    /// Specifies how a new instance of an activity should be added to a task each time it is launched. This attribute permits the user to have multiple documents from the same application appear in the overview screen.
    #[serde(rename = "android:documentLaunchMode")]
    pub document_launch_mode: Option<DocumentLaunchMode>,
    /// Whether or not the activity can be instantiated by the system — `"true"` if it can be, and `"false"` if not. The default value is `"true"`.
    /// The `<application>` element has its own `enabled` attribute that applies to all application components, including activities.
    /// The `<application>` and `<activity>` attributes must both be `"true"` (as they both are by default) for the system to be able to instantiate the activity. If either is `"false"`, it cannot be instantiated.
    #[serde(rename = "android:enabled")]
    pub enabled: Option<bool>,
    /// Whether or not the task initiated by this activity should be excluded from the list of recently used applications, the `overview screen`.
    /// That is, when this activity is the root activity of a new task, this attribute determines whether the task should not appear in the list of recent apps.
    /// Set `"true"` if the task should be excluded from the list; set `"false"` if it should be included. The default value is `"false"`.
    #[serde(rename = "android:excludeFromRecents")]
    pub exclude_from_recents: Option<bool>,
    /// This element sets whether the activity can be launched by components of other applications — "true" if it can be, and "false" if not.
    /// If "false", the activity can be launched only by components of the same application or applications with the same user ID.
    /// If you are using intent filters, you should not set this element "false". If you do so, and an app tries to call the activity, system throws an ActivityNotFoundException.
    /// Instead, you should prevent other apps from calling the activity by not setting intent filters for it.
    /// If you do not have intent filters, the default value for this element is "false". If you set the element "true", the activity is accessible to any app that knows its exact class name,
    /// but does not resolve when the system tries to match an implicit intent. This attribute is not the only way to limit an activity's exposure to other applications.
    /// You can also use a permission to limit the external entities that can invoke the activity (see the permission attribute).
    #[serde(rename = "android:exported")]
    pub exported: Option<bool>,
    /// Whether or not an existing instance of the activity should be shut down (finished) whenever the user again launches its task (chooses the task on the home screen) — "true"
    /// if it should be shut down, and "false" if not. The default value is "false".
    /// If this attribute and allowTaskReparenting are both "true", this attribute trumps the other. The affinity of the activity is ignored. The activity is not re-parented, but destroyed.
    #[serde(rename = "android:finishOnTaskLaunch")]
    pub finish_on_task_launch: Option<bool>,
    /// Whether or not hardware-accelerated rendering should be enabled for this Activity — `"true"` if it should be enabled, and "false" if not. The default value is `"false".
    /// Starting from Android 3.0, a hardware-accelerated OpenGL renderer is available to applications, to improve performance for many common 2D graphics operations.
    /// When the hardware-accelerated renderer is enabled, most operations in Canvas, Paint, Xfermode, ColorFilter, Shader, and Camera are accelerated.
    /// Note that not all of the OpenGL 2D operations are accelerated. If you enable the hardware-accelerated renderer, test your application to ensure that it can make use of the renderer without errors.
    #[serde(rename = "android:hardwareAccelerated")]
    pub hardware_accelerated: Option<bool>,
    /// An icon representing the activity. The icon is displayed to users when a representation of the activity is required on-screen.
    /// For example, icons for activities that initiate tasks are displayed in the launcher window. The icon is often accompanied by a label (see the android:label attribute).
    /// This attribute must be set as a reference to a drawable resource containing the image definition. If it is not set, the icon specified for the application
    /// as a whole is used instead (see the <application> element's icon attribute). The activity's icon — whether set here or by the <application> element — is also the default icon for all
    /// the activity's intent filters (see the `<intent-filter>` element's icon attribute).
    #[serde(rename = "android:icon")]
    pub icon: Option<Resource<DrawableResource>>,
    /// Sets the immersive mode setting for the current activity. If the android:immersive attribute is set to true in the app's manifest entry for this activity, the ActivityInfo.flags
    /// member always has its `FLAG_IMMERSIVE` bit set, even if the immersive mode is changed at runtime using the setImmersive() method.
    #[serde(rename = "android:immersive")]
    pub immersive: Option<bool>,
    /// A user-readable label for the activity. The label is displayed on-screen when the activity must be represented to the user. It's often displayed along with the activity icon.
    /// If this attribute is not set, the label set for the application as a whole is used instead (see the <application> element's label attribute).
    /// The activity's label — whether set here or by the <application> element — is also the default label for all the activity's intent filters (see the <intent-filter> element's label attribute).
    /// The label should be set as a reference to a string resource, so that it can be localized like other strings in the user interface.
    /// However, as a convenience while you're developing the application, it can also be set as a raw string.
    #[serde(rename = "android:label")]
    pub label: Option<StringResourceOrString>,
    /// An instruction on how the activity should be launched. There are four modes that work in conjunction
    /// with activity flags (`FLAG_ACTIVITY_*` constants) in `Intent` objects to determine what should happen when the activity is called upon to handle an intent.
    /// The default mode is "standard".
    /// As shown in the table below, the modes fall into two main groups, with "standard" and "singleTop" activities on one side, and "singleTask" and "singleInstance" activities on the other.
    /// An activity with the "standard" or "singleTop" launch mode can be instantiated multiple times.
    /// The instances can belong to any task and can be located anywhere in the activity stack.
    /// Typically, they're launched into the task that called startActivity() (unless the Intent object contains a FLAG_ACTIVITY_NEW_TASK instruction, in which case a different task is chosen — see the taskAffinity attribute).
    #[serde(rename = "android:launchMode")]
    pub launch_mode: Option<LaunchMode>,
    /// Determines how the system presents this activity when the device is running in lock task mode.
    /// Android can run tasks in an immersive, kiosk-like fashion called lock task mode. When the system runs in lock task mode, device users typically can’t see notifications, access non-allowlisted apps, or return to the
    /// home screen (unless the Home app is allowlisted). Only apps that have been allowlisted by a device policy controller (DPC) can run when the system is in lock task mode.
    /// System and privileged apps, however, can run in lock task mode without being allowlisted.
    #[serde(rename = "android:lockTaskMode")]
    pub lock_task_mode: Option<LockTaskMode>,
    /// The maximum number of tasks rooted at this activity in the overview screen. When this number of entries is reached, the system removes the least-recently used instance from the overview screen.
    /// Valid values are 1 through 50 (25 on low memory devices); zero is invalid. This must be an integer value, such as 50. The default value is 16.
    #[serde(rename = "android:maxRecents")]
    pub max_recents: Option<i32>,
    /// The maximum aspect ratio the activity supports. If the app runs on a device with a wider aspect ratio, the system automatically letterboxes the app, leaving portions of the screen unused so the app can run at its
    /// specified maximum aspect ratio. Maximum aspect ratio is expressed as the decimal form of the quotient of the device's longer dimension divided by its shorter dimension. For example, if the maximum aspect ratio is
    /// 7:3, set the value of this attribute to 2.33. On non-wearable devices, the value of this attribute needs to be 1.33 or greater. On wearable devices, it must be 1.0 or greater. Otherwise, the system ignores the set value.
    /// `Note:` This attribute is ignored if the activity has `resizeableActivity` set to true, since that means your activity supports any size.
    #[serde(rename = "android:maxAspectRatio")]
    pub max_aspect_ratio: Option<f32>,
    /// Whether an instance of the activity can be launched into the process of the component that started it — "true" if it can be, and "false" if not. The default value is "false".
    /// Normally, a new instance of an activity is launched into the process of the application that defined it, so all instances of the activity run in the same process. However, if this flag is set to "true", instances of the
    /// activity can run in multiple processes, allowing the system to create instances wherever they are used (provided permissions allow it), something that is almost never necessary or desirable.
    #[serde(rename = "android:multiprocess")]
    pub multiprocess: Option<bool>,
    /// The name of the class that implements the activity, a subclass of Activity. The attribute value should be a fully qualified class name (such as, "com.example.project.ExtracurricularActivity"). However, as a
    /// shorthand, if the first character of the name is a period (for example, ".ExtracurricularActivity"), it is appended to the package name specified in the <manifest> element.
    /// Once you publish your application, you should not change this name (unless you've set android:exported="false").
    /// There is no default. The name must be specified.
    #[serde(rename = "android:name")]
    pub name: String,
    /// Whether or not the activity should be removed from the activity stack and finished (its finish() method called) when the user navigates away from
    /// it and it's no longer visible on screen — "true" if it should be finished, and "false" if not. The default value is "false".
    /// A value of "true" means that the activity will not leave a historical trace. It will not remain in the activity stack for the task, so the user will not be able to return to it.
    /// In this case, onActivityResult() is never called if you start another activity for a result from this activity.
    /// This attribute was introduced in API Level 3.
    #[serde(rename = "android:noHistory")]
    pub no_history: Option<bool>,
    /// The class name of the logical parent of the activity. The name here must match the class name given to the corresponding <activity> element's android:name attribute.
    /// The system reads this attribute to determine which activity should be started when the user presses the Up button in the action bar. The system can also use this information to synthesize a back stack of activities with TaskStackBuilder.
    /// To support API levels 4 - 16, you can also declare the parent activity with a <meta-data> element that specifies a value for "android.support.PARENT_ACTIVITY".
    #[serde(rename = "android:parentActivityName")]
    pub parent_activity_name: Option<String>,
    /// Defines how an instance of an activity is preserved within a containing task across device restarts.
    /// If the root activity of a task sets this attribute's value to persistRootOnly, then only the root activity is preserved.
    /// Otherwise, the activities that are higher up the task's back stack are examined; any of these activities that set this attribute's value to `persistAcrossReboots` are preserved
    #[serde(rename = "android:persistableMode")]
    pub persistable_mode: Option<PersistableMode>,
    /// The name of a permission that clients must have to launch the activity or otherwise get it to respond to an intent.
    /// If a caller of startActivity() or startActivityForResult() has not been granted the specified permission, its intent will not be delivered to the activity.
    /// If this attribute is not set, the permission set by the <application> element's permission attribute applies to the activity. If neither attribute is set, the activity is not protected by a permission.
    /// For more information on permissions, see the Permissions section in the introduction and another document, Security and Permissions.
    #[serde(rename = "android:permission")]
    pub permission: Option<String>,
    /// The name of the process in which the activity should run. Normally, all components of an application run in a default process name created for the application and you do not need to use this attribute. But if necessary,
    /// you can override the default process name with this attribute, allowing you to spread your app components across multiple processes.
    /// If the name assigned to this attribute begins with a colon (':'), a new process, private to the application, is created when it's needed and the activity runs in that process. If the process name begins with a lowercase
    /// character, the activity will run in a global process of that name, provided that it has permission to do so. This allows components in different applications to share a process, reducing resource usage.
    /// The `<application>` element's `process` attribute can set a different default process name for all components.
    #[serde(rename = "android:process")]
    pub process: Option<String>,
    /// Whether or not the activity relinquishes its task identifiers to an activity above it in the task stack. A task whose root activity has this attribute set to `"true"` replaces the base Intent with that of the next activity in the
    /// task. If the next activity also has this attribute set to `"true"` then it will yield the base Intent to any activity that it launches in the same task.
    /// This continues for each activity until an activity is encountered which has this attribute set to `"false"`. The default value is `"false"`.
    /// This attribute set to `"true"` also permits the activity's use of the `ActivityManager.TaskDescription` to change labels, colors and icons in the `overview screen`.
    #[serde(rename = "android:relinquishTaskIdentity")]
    pub relinquish_task_identity: Option<bool>,
    /// Specifies whether the app supports multi-window display. You can set this attribute in either the <activity> or <application> element.
    /// If you set this attribute to true, the user can launch the activity in split-screen and freeform modes. If you set the attribute to false, the activity does not support multi-window mode.
    /// If this value is false, and the user attempts to launch the activity in multi-window mode, the activity takes over the full screen.
    /// If your app targets API level 24 or higher, but you do not specify a value for this attribute, the attribute's value defaults to true.
    /// This attribute was added in API level 24.
    #[serde(rename = "android:resizeableActivity")]
    pub resizeable_activity: Option<bool>,
    /// The orientation of the activity's display on the device. The system ignores this attribute if the activity is running in multi-window mode.
    #[serde(rename = "android:screenOrientation")]
    pub screen_orientation: Option<ScreenOrientation>,
    /// Whether or not the activity is shown when the device's current user is different than the user who launched the activity.
    /// You can set this attribute to a literal value—"true" or "false"—or you can set the attribute to a resource or theme attribute that contains a boolean value.
    /// This attribute was added in API level 23.
    #[serde(rename = "android:showForAllUsers")]
    pub show_for_all_users: Option<bool>,
    /// Whether or not the activity can be killed and successfully restarted without having saved its state — "true" if
    /// it can be restarted without reference to its previous state, and "false" if its previous state is required. The default value is "false".
    /// Normally, before an activity is temporarily shut down to save resources, its onSaveInstanceState() method is called. This method stores the current state of the activity in a Bundle object, which is then passed to
    /// onCreate() when the activity is restarted. If this attribute is set to "true", onSaveInstanceState() may not be called and onCreate() will be passed null instead of the Bundle — just as it was when the activity started for the first time.
    /// A "true" setting ensures that the activity can be restarted in the absence of retained state. For example, the activity that displays the home screen uses this setting
    /// to make sure that it does not get removed if it crashes for some reason.
    #[serde(rename = "android:stateNotNeeded")]
    pub state_not_needed: Option<bool>,
    /// Specifies whether the activity supports Picture-in-Picture display.
    /// This attribute was added in API level 24.
    #[serde(rename = "android:supportsPictureInPicture")]
    pub supports_picture_in_picture: Option<bool>,
    /// The task that the activity has an affinity for. Activities with the same affinity conceptually belong to the same task (to the same "application" from the user's perspective).
    /// The affinity of a task is determined by the affinity of its root activity.
    /// The affinity determines two things — the task that the activity is re-parented to (see the allowTaskReparenting attribute) and the task that will house the activity when it is launched with the FLAG_ACTIVITY_NEW_TASK flag.
    /// By default, all activities in an application have the same affinity. You can set this attribute to group them differently, and even place activities defined in different applications within the same task.
    /// To specify that the activity does not have an affinity for any task, set it to an empty string.
    /// If this attribute is not set, the activity inherits the affinity set for the application (see the <application> element's taskAffinity attribute).
    /// The name of the default affinity for an application is the package name set by the <manifest> element.
    #[serde(rename = "android:taskAffinity")]
    pub task_affinity: Option<String>,
    /// A reference to a style resource defining an overall theme for the activity. This automatically sets the activity's context to use this theme (see setTheme()
    /// and may also cause "starting" animations prior to the activity being launched (to better match what the activity actually looks like).
    /// If this attribute is not set, the activity inherits the theme set for the application as a whole — from the <application> element's theme attribute.
    /// If that attribute is also not set, the default system theme is used. For more information, see the Styles and Themes developer guide.
    #[serde(rename = "android:theme")]
    pub theme: Option<Resource<StyleResource>>,
    /// Extra options for an activity's UI.
    /// Must be one of the following values.
    #[serde(rename = "android:uiOptions")]
    pub ui_options: Option<String>,
    /// How the main window of the activity interacts with the window containing the on-screen soft keyboard. The setting for this attribute affects two things:
    /// The state of the soft keyboard — whether it is hidden or visible — when the activity becomes the focus of user attention.
    /// The adjustment made to the activity's main window — whether it is resized smaller to make room for the soft keyboard or whether its contents pan to make the current focus visible when part of the window is covered by the soft keyboard.
    /// The setting must be one of the values listed in the following table, or a combination of one `"state..."` value plus one `"adjust..."` value.
    #[serde(rename = "android:windowSoftInputMode")]
    pub window_soft_input_mode: Option<WindowSoftInputMode>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ColorMode {
    Hdr,
    WideColorGamut,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ConfigChanges {
    /// The IMSI mobile country code (MCC) has changed — a SIM has been detected and updated the MCC.
    Mcc,
    /// The IMSI mobile network code (MNC) has changed — a SIM has been detected and updated the MNC.
    Mnc,
    /// The locale has changed — the user has selected a new language that text should be displayed in.
    Locale,
    /// The touchscreen has changed. (This should never normally happen.)
    Touchscreen,
    /// The keyboard type has changed — for example, the user has plugged in an external keyboard.
    Keyboard,
    /// The keyboard accessibility has changed — for example, the user has revealed the hardware keyboard.
    KeyboardHidden,
    /// The navigation type (trackball/dpad) has changed. (This should never normally happen.)
    Navigation,
    /// The screen layout has changed — a different display might now be active.
    ScreenLayout,
    /// The font scaling factor has changed — the user has selected a new global font size.
    FontScale,
    /// The user interface mode has changed — the user has placed the device into a desk or car dock, or the night mode has changed. For more information about the different UI modes, see UiModeManager.
    /// Added in API level 8.
    UiMode,
    /// The screen orientation has changed — the user has rotated the device.
    /// `Note:` If your application targets Android 3.2 (API level 13) or higher, then you should also declare the `"screenSize"` and `"screenLayout"` configurations,
    /// because they might also change when a device switches between portrait and landscape orientations.
    Orientation,
    /// The display density has changed — the user might have specified a different display scale, or a different display might now be active.
    /// Added in API level 24.
    Density,
    /// The current available screen size has changed.
    /// This represents a change in the currently available size, relative to the current aspect ratio, so will change when the user switches between landscape and portrait.
    /// Added in API level 13.
    ScreenSize,
    /// The physical screen size has changed.
    /// This represents a change in size regardless of orientation, so will only change when the actual physical screen size has changed such as switching to an external display.
    /// A change to this configuration corresponds to a change in the smallestWidth configuration.
    /// Added in API level 13
    SmallestScreenSize,
}

/// This attribute has four values which produce the following effects when the user opens a document with the application:
/// Note: For values other than `"none"` and `"never"` the activity must be defined with `launchMode="standard"`. If this attribute is not specified, `documentLaunchMode="none"` is used.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum DocumentLaunchMode {
    /// The system searches for a task whose base intent's ComponentName and data URI match those of the launching intent.
    /// If the system finds such a task, the system clears the task, and restarts with the root activity receiving a call to `onNewIntent(android.content.Intent)`.
    /// If the system does not find such a task, the system creates a new task.
    IntoExisting,
    /// The activity creates a new task for the document, even if the document is already opened.
    /// This is the same as setting both the `FLAG_ACTIVITY_NEW_DOCUMENT` and `FLAG_ACTIVITY_MULTIPLE_TASK` flags
    Always,
    /// The activity does not create a new task for the activity. This is the default value, which creates a new task only when FLAG_ACTIVITY_NEW_TASK is set.
    /// The overview screen treats the activity as it would by default: it displays a single task for the app, which resumes from whatever activity the user last invoked.
    None,
    /// This activity is not launched into a new document even if the Intent contains FLAG_ACTIVITY_NEW_DOCUMENT.
    /// Setting this overrides the behavior of the FLAG_ACTIVITY_NEW_DOCUMENT and FLAG_ACTIVITY_MULTIPLE_TASK flags, if either of these are set in the activity, and the overview screen
    /// displays a single task for the app, which resumes from whatever activity the user last invoked.
    Never,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum LaunchMode {
    /// Default. The system always creates a new instance of the activity in the target task and routes the intent to it.
    Standard,
    /// If an instance of the activity already exists at the top of the target task, the system routes the intent to that instance through a call to its onNewIntent() method, rather than creating a new instance of the activity.
    SingleTop,
    /// The system creates the activity at the root of a new task and routes the intent to it. However, if an instance of the activity already exists,
    /// the system routes the intent to existing instance through a call to its onNewIntent() method, rather than creating a new one.
    SingleTask,
    ///	Same as "singleTask", except that the system doesn't launch any other activities into the task holding the instance.
    /// The activity is always the single and only member of its task.
    SingleInstance,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum LockTaskMode {
    /// Default value. This is the default value. Tasks don't launch into lock task mode but can be placed there by calling `startLockTask()`.
    Normal,
    /// Tasks don't launch into lockTask mode, and the device user can't pin these tasks from the overview screen.
    /// `Note:` This mode is only available to system and privileged applications. Non-privileged apps with this value are treated as normal
    Never,
    /// Tasks rooted at this activity always launch into lock task mode. If the system is already in lock task mode when this task is launched then the new task are launched on top of the current task.
    /// Tasks launched in this mode can exit lock task mode by calling finish().
    /// Note: This mode is only available to system and privileged applications. Non-privileged apps with this value are treated as normal.
    Always,
    /// If the DPC authorizes this package using `DevicePolicyManager`.`setLockTaskPackages()`, then this mode is identical to always, except that the activity needs to call `stopLockTask()` before being
    /// able to finish if it is the last locked task. If the DPC does not authorize this package then this mode is identical to normal.
    #[serde(rename = "if_whitelisted")]
    IfWhitelisted,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PersistableMode {
    ///	Default value. When the system restarts, the activity task is preserved, but only the root activity's launching intent is used.
    /// When your app's launching intent loads your app's root activity, the activity doesn't receive a PersistableBundle object.
    /// Therefore, don't use onSaveInstanceState() to preserve the state of your app's root activity across a device restart.
    /// Note: This attribute value affects your app's behavior only if it's set on your app's root activity.
    PersistRootOnly,
    /// This activity's state is preserved, along with the state of each activity higher up the back stack that has its own persistableMode attribute set to persistAcrossReboots. If an
    /// activity doesn't have a persistableMode attribute that is set to persistAcrossReboots, or if it's launched using the
    /// Intent.FLAG_ACTIVITY_NEW_DOCUMENT flag, then that activity, along with all activities higher up the back stack, aren't preserved.
    /// When an intent loads an activity whose persistableMode attribute is set to persistAcrossReboots in your app, the activity receives a PersistableBundle object
    /// in its onCreate() method. Therefore, you can use onSaveInstanceState() to preserve the state of an activity across a device restart as long as its persistableMode attribute is set to persistAcrossReboots.
    /// Note: This attribute value affects your app's behavior even if it's set on an activity other than your app's root activity
    PersistAcrossReboots,
    /// The activity's state isn't preserved.
    /// `Note:` This attribute value affects your app's behavior only if it's set on your app's root activity.
    PersistNever,
}

/// Note: When you declare one of the landscape or portrait values, it is considered a hard requirement for the orientation in which the activity runs. As such, the value you declare enables filtering by services such as
/// Google Play so your application is available only to devices that support the orientation required by your activities. For example, if you declare either "landscape", "reverseLandscape", or "sensorLandscape", then
/// your application will be available only to devices that support landscape orientation. However, you should also explicitly declare that your application requires either portrait or landscape orientation with the <uses-feature> element.
/// For example, <uses-feature android:name="android.hardware.screen.portrait"/>. This is purely a filtering behavior provided by Google Play (and other services that support it) and the platform
/// itself does not control whether your app can be installed when a device supports only certain orientations.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ScreenOrientation {
    ///	The default value. The system chooses the orientation. The policy it uses, and therefore the choices made in specific contexts, may differ from device to device.
    Unspecified,
    /// The same orientation as the activity that's immediately beneath it in the activity stack.
    Behind,
    /// Landscape orientation (the display is wider than it is tall).
    Landscape,
    /// Portrait orientation (the display is taller than it is wide).
    Portrait,
    /// Landscape orientation in the opposite direction from normal landscape. Added in API level 9.
    ReverseLandscape,
    /// Portrait orientation in the opposite direction from normal portrait. Added in API level 9.
    ReversePortrait,
    /// Landscape orientation, but can be either normal or reverse landscape based on the device sensor. The sensor is used even if the user has locked sensor-based rotation. Added in API level 9.
    SensorLandscape,
    /// Portrait orientation, but can be either normal or reverse portrait based on the device sensor. The sensor is used even if the user has locked sensor-based rotation. Added in API level 9.
    SensorPortrait,
    /// Landscape orientation, but can be either normal or reverse landscape based on the device sensor and the user's preference. Added in API level 18.
    UserLandscape,
    /// Portrait orientation, but can be either normal or reverse portrait based on the device sensor and the user's preference. Added in API level 18.
    UserPortrait,
    /// The orientation is determined by the device orientation sensor. The orientation of the display depends on how the user is holding the device; it changes when the user rotates the device.
    /// Some devices, though, will not rotate to all four possible orientations, by default. To allow all four orientations, use "fullSensor" The sensor is used even if the user locked sensor-based rotation.
    Sensor,
    /// The orientation is determined by the device orientation sensor for any of the 4 orientations. This is similar to "sensor" except this allows any of the 4 possible screen orientations, regardless
    /// of what the device will normally do (for example, some devices won't normally use reverse portrait or reverse landscape, but this enables those). Added in API level 9.
    FullSensor,
    /// The orientation is determined without reference to a physical orientation sensor. The sensor is ignored, so the display will not rotate based on how the user moves the device.
    Nosensor,
    /// The user's current preferred orientation.
    User,
    /// If the user has locked sensor-based rotation, this behaves the same as user, otherwise it behaves the same as fullSensor and allows any of the 4 possible screen orientations. Added in API level 18.
    FullUser,
    /// Locks the orientation to its current rotation, whatever that is. Added in API level 18.
    Locked,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum UiOptions {
    ///	No extra UI options. This is the default
    None,
    /// Add a bar at the bottom of the screen to display action items in the app bar (also known as the action bar), when constrained for horizontal space (such as when in portrait mode on a handset).
    /// Instead of a small number of action items appearing in the app bar at the top of the screen, the app bar is split into the top navigation section and the bottom bar for action items.
    /// This ensures a reasonable amount of space is made available not only for the action items, but also for navigation and title elements at the top. Menu items are not split across the two bars; they always appear together.
    SplitActionBarWhenNarrow,
}

/// Values set here (other than "stateUnspecified" and "adjustUnspecified") override values set in the theme.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum WindowSoftInputMode {
    /// The state of the soft keyboard (whether it is hidden or visible) is not specified. The system will choose an appropriate state or rely on the setting in the theme.
    /// This is the default setting for the behavior of the soft keyboard.
    StateUnspecified,
    /// The soft keyboard is kept in whatever state it was last in, whether visible or hidden, when the activity comes to the fore.
    StateUnchanged,
    /// The soft keyboard is hidden when the user chooses the activity — that is, when the user affirmatively navigates forward to the activity, rather than backs into it because of leaving another activity.
    StateHidden,
    /// The soft keyboard is always hidden when the activity's main window has input focus.
    StateAlwaysHidden,
    /// The soft keyboard is made visible when the user chooses the activity — that is, when the user affirmatively navigates forward to the activity, rather than backs into it because of leaving another activity.
    StateVisible,
    /// The soft keyboard is visible when the window receives input focus.
    StateAlwaysVisible,
    /// It is unspecified whether the activity's main window resizes to make room for the soft keyboard, or whether the contents of the window pan to make the current focus visible on-screen.
    /// The system will automatically select one of these modes depending on whether the content of the window has any layout views that can scroll their contents.
    /// If there is such a view, the window will be resized, on the assumption that scrolling can make all of the window's contents visible within a smaller area.
    /// This is the default setting for the behavior of the main window.
    AdjustUnspecified,
    ///	The activity's main window is always resized to make room for the soft keyboard on screen.
    AdjustResize,
    /// The activity's main window is not resized to make room for the soft keyboard. Rather, the contents of the window are automatically panned so that the current focus is never obscured by the keyboard and users can always see what they are typing.
    /// This is generally less desirable than resizing, because the user may need to close the soft keyboard to get at and interact with obscured parts of the window.
    AdjustPan,
}
