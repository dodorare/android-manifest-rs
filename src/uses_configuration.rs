use serde::{Deserialize, Serialize};

use crate::VarOrBool;

/// Indicates what hardware and software features the application requires.
///
/// For example, an application might specify that it requires a physical keyboard or a
/// particular navigation device, like a trackball. The specification is used to avoid
/// installing the application on devices where it will not work.
///
/// ## Note
/// `Most apps should not use this manifest tag`    . You should always support input with
/// a directional pad (d-pad) in order to assist sight-impaired users and support devices
/// that provide d-pad input in addition to or instead of touch. information about how to
/// support d-pad input in your app, read [`Enabling Focus Navigation`]. If your app
/// absolutely cannot function without a touchscreen, then instead use the
/// [`<uses-feature>`] tag to declare the required touchscreen type, ranging from
/// `"android.hardware.faketouch"` for basic touch-style events to more advanced touch
/// types such as `"android.hardware.touchscreen.multitouch.jazzhand"` for distinct input
/// from multiple fingers.
///
/// ## XML Syntax
/// ```xml
/// <uses-configuration android:reqFiveWayNav=["true" | "false"]
///                     android:reqHardKeyboard=["true" | "false"]
///                     android:reqKeyboardType=["undefined" | "nokeys" | "qwerty" | "twelvekey"]
///                     android:reqNavigation=["undefined" | "nonav" | "dpad" | "trackball" | "wheel"]
///                     android:reqTouchScreen=["undefined" | "notouch" | "stylus" | "finger"] />
/// ```
///
/// ## Contained in
/// * [`<manifest>`]
///
/// ## Introduced in
/// API Level 3
///
/// [`Enabling Focus Navigation`]: https://developer.android.com/guide/topics/ui/accessibility/apps#focus-nav
/// [`<uses-feature>`]: crate::UsesFeature
/// [`<manifest>`]: crate::AndroidManifest
#[derive(
    Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Default, Clone,
)]
pub struct UsesConfiguration {
    /// Whether or not the application requires a five-way navigation control — `"true"`
    /// if it does, and `"false"` if not. A five-way control is one that can move the
    /// selection up, down, right, or left, and also provides a way of invoking the
    /// current selection. It could be a D-pad (directional pad), trackball, or other
    /// device.
    ///
    /// If an application requires a directional control, but not a control of
    /// a particular type, it can set this attribute to "true" and ignore the
    /// [`reqNavigation`] attribute. However, if it requires a particular type
    /// of directional control, it can ignore this attribute and set
    /// `reqNavigation` instead.
    ///
    /// [`reqNavigation`]: crate::UsesConfiguration#structfield.req_navigation
    #[yaserde(attribute, prefix = "android", rename = "reqFiveWayNav")]
    pub req_five_way_nav: Option<VarOrBool>,
    /// Whether or not the application requires a hardware keyboard — `"true"` if it does,
    /// and `"false"` if not.
    #[yaserde(attribute, prefix = "android", rename = "reqHardKeyboard")]
    pub req_hard_keyboard: Option<VarOrBool>,
    /// The type of keyboard the application requires, if any at all. This attribute does
    /// not distinguish between hardware and software keyboards. If a hardware
    /// keyboard of a certain type is required, specify the type here and also set the
    /// reqHardKeyboard attribute to `"true"`.
    #[yaserde(attribute, prefix = "android", rename = "reqKeyboardType")]
    pub req_keyboard_type: Option<ReqKeyboardType>,
    /// The navigation device required by the application, if any.
    ///
    /// If an application requires a navigational control, but the exact type of
    /// control doesn't matter, it can set the [`reqFiveWayNav`] attribute to "true"
    /// rather than set this one.
    ///
    /// [`reqFiveWayNav`]: crate::UsesConfiguration#structfield.req_five_way_nav
    #[yaserde(attribute, prefix = "android", rename = "reqNavigation")]
    pub req_navigation: Option<ReqNavigation>,
    /// The type of touch screen the application requires, if any at all.
    #[yaserde(attribute, prefix = "android", rename = "reqTouchScreen")]
    pub req_touch_screen: Option<ReqTouchScreen>,
}

/// The type of keyboard the application requires, if any at all.
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ReqKeyboardType {
    /// The application does not require a keyboard. (A keyboard requirement is
    /// not defined.) This is the default value.
    #[yaserde(rename = "undefined")]
    Undefined,
    /// The application does not require a keyboard.
    #[yaserde(rename = "nokeys")]
    Nokeys,
    /// The application requires a standard QWERTY keyboard.
    #[yaserde(rename = "qwerty")]
    Qwerty,
    /// The application requires a twelve-key keypad, like those on most phones
    /// — with keys for the digits from 0 through 9 plus star (*) and pound (#)
    /// keys.
    #[yaserde(rename = "twelvekey")]
    Twelvekey,
}

impl Default for ReqKeyboardType {
    fn default() -> Self {
        ReqKeyboardType::Undefined
    }
}

/// The navigation device required by the application, if any.
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ReqNavigation {
    /// The application does not require any type of navigation control. (The
    /// navigation requirement is not defined.) This is the default value.
    #[yaserde(rename = "undefined")]
    Undefined,
    /// The application does not require a navigation control.
    #[yaserde(rename = "nonav")]
    Nonav,
    /// The application requires a D-pad (directional pad) for navigation.
    #[yaserde(rename = "dpad")]
    Dpad,
    /// The application requires a trackball for navigation.
    #[yaserde(rename = "trackball")]
    Trackball,
    /// The application requires a navigation wheel.
    #[yaserde(rename = "wheel")]
    Wheel,
}

impl Default for ReqNavigation {
    fn default() -> Self {
        ReqNavigation::Undefined
    }
}

/// The type of touch screen the application requires, if any at all.
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ReqTouchScreen {
    /// The application doesn't require a touch screen. (The touch screen
    /// requirement is undefined.) This is the default value.
    #[yaserde(rename = "undefined")]
    Undefined,
    /// The application doesn't require a touch screen.
    #[yaserde(rename = "notouch")]
    Notouch,
    /// The application requires a touch screen that's operated with a stylus.
    #[yaserde(rename = "stylus")]
    Stylus,
    /// The application requires a touch screen that can be operated with a
    /// finger.
    ///
    /// ## Node
    /// If some type of touch input is required for your app, you should
    /// instead use the [`<uses-feature>`] tag to declare the required touchscreen
    /// type, beginning with `"android.hardware.faketouch"` for basic touch-style
    /// events.
    ///
    /// [`<uses-feature>`]: crate::UsesFeature
    #[yaserde(rename = "finger")]
    Finger,
}

impl Default for ReqTouchScreen {
    fn default() -> Self {
        ReqTouchScreen::Undefined
    }
}
