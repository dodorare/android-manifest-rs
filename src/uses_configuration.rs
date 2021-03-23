use serde::{Deserialize, Serialize};

/// Indicates what hardware and software features the application requires.
///
/// For example, an application might specify that it requires a physical keyboard or a
/// particular navigation device, like a trackball. The specification is used to avoid
/// installing the application on devices where it will not work.
///
/// ## `Note:`
/// Most apps should not use this manifest tag. You should always support input with a
/// directional pad (d-pad) in order to assist sight-impaired users and support devices
/// that provide d-pad input in addition to or instead of touch. information about how to
/// support d-pad input in your app, read Enabling Focus Navigation. If your app
/// absolutely cannot function without a touchscreen, then instead use the
/// [`<uses-feature>`] tag to declare the required touchscreen type, ranging from
/// `"android.hardware.faketouch"` for basic touch-style events to more advanced touch
/// types such as `"android.hardware.touchscreen.multitouch.jazzhand"` for distinct input
/// from multiple fingers.
///
/// ## introduced in:
/// API Level 3
///
/// ## Contained in:
/// [`<manifest>`]
///
/// [`<uses-feature>`]: crate::UsesFeature
/// [`<manifest>`]: crate::Manifest
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "uses-configuration")]
pub struct UsesConfiguration {
    /// Whether or not the application requires a five-way navigation control — `"true"`
    /// if it does, and `"false"` if not. A five-way control is one that can move the
    /// selection up, down, right, or left, and also provides a way of invoking the
    /// current selection. It could be a D-pad (directional pad), trackball, or other
    /// device. If an application requires a directional control, but not a control of
    /// a particular type, it can set this attribute to "true" and ignore the
    /// [`reqNavigation`] attribute. However, if it requires a particular type
    /// of directional control, it can ignore this attribute and set
    /// `reqNavigation` instead.
    ///
    /// [`reqNavigation`]: crate::UsesConfiguration#structfield.req_navigation
    #[serde(rename = "android:reqFiveWayNav")]
    pub req_five_way_nav: Option<bool>,
    /// Whether or not the application requires a hardware keyboard — `"true"` if it does,
    /// and `"false"` if not.
    #[serde(rename = "android:reqHardKeyboard")]
    pub req_hard_keyboard: Option<bool>,
    /// The type of keyboard the application requires, if any at all. This attribute does
    /// not distinguish between hardware and software keyboards. If a hardware
    /// keyboard of a certain type is required, specify the type here and also set the
    /// reqHardKeyboard attribute to `"true"`.
    #[serde(rename = "android:reqKeyboardType")]
    pub req_keyboard_type: Option<ReqKeyboardType>,
    /// The navigation device required by the application, if any.
    #[serde(rename = "android:reqNavigation")]
    pub req_navigation: Option<ReqNavigation>,
    /// The type of touch screen the application requires, if any at all.
    #[serde(rename = "android:reqTouchScreen")]
    pub req_touch_screen: Option<ReqTouchScreen>,
}

/// The value must be one of the following strings:
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ReqKeyboardType {
    /// The application does not require a keyboard. (A keyboard requirement is
    /// not defined.) This is the default value.
    Undefined,
    /// The application does not require a keyboard.
    Nokeys,
    /// The application requires a standard QWERTY keyboard.
    Qwerty,
    /// The application requires a twelve-key keypad, like those on most phones
    /// — with keys for the digits from 0 through 9 plus star (*) and pound (#)
    /// keys.
    Twelvekey,
}

/// If an application requires a navigational control, but the exact type of
/// control doesn't matter, it can set the [`reqFiveWayNav`] attribute to "true"
/// rather than set this one.
///
/// [`reqFiveWayNav`]: crate::UsesConfiguration#structfield.req_five_way_nav
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ReqNavigation {
    /// The application does not require any type of navigation control. (The
    /// navigation requirement is not defined.) This is the default value.
    Undefined,
    /// The application does not require a navigation control.
    Nonav,
    /// The application requires a D-pad (directional pad) for navigation.
    Dpad,
    /// The application requires a trackball for navigation.
    Trackball,
    /// The application requires a navigation wheel.
    Wheel,
}

/// ## `Note:`
/// If some type of touch input is required for your app, you should
/// instead use the [`<uses-feature>`] tag to declare the required touchscreen
/// type, beginning with `"android.hardware.faketouch"` for basic touch-style
/// events.
///
/// [`<uses-feature>`]: crate::UsesFeature
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ReqTouchScreen {
    /// The application doesn't require a touch screen. (The touch screen
    /// requirement is undefined.) This is the default value.
    Undefined,
    ///	The application doesn't require a touch screen.
    Notouch,
    /// The application requires a touch screen that's operated with a stylus.
    Stylus,
    /// The application requires a touch screen that can be operated with a
    /// finger.
    Finger,
}
