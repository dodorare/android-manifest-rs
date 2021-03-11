mod resources;

pub use resources::*;
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
    pub config_changes: Option<ColorMode>,
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
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum DocumentLaunchMode{
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
    /// Note: For values other than `"none"` and `"never"` the activity must be defined with `launchMode="standard"`. If this attribute is not specified, `documentLaunchMode="none"` is used.
}