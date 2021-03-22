use serde::{Deserialize, Serialize};

/// With Android 7.0, the `<layout>` manifest element supports several
/// attributes that affect how an activity behaves in multi-window mode:
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "layout")]
pub struct Layout {
    /// Default width of the activity when launched in freeform mode.
    #[serde(rename = "android:defaultWidth")]
    pub default_width: Option<i32>,
    /// Default height of the activity when launched in freeform mode.
    #[serde(rename = "android:defaultHeight")]
    pub default_height: Option<i32>,
    ///Initial placement of the activity when launched in freeform mode. See
    /// the Gravity reference for suitable values.
    #[serde(rename = "android:gravity")]
    pub gravity: Option<Gravity>,
    /// Minimum height and minimum width for the activity in both split-screen
    /// and freeform modes. If the user moves the divider in split-screen mode
    /// to make an activity smaller than the specified minimum, the system
    /// crops the activity to the size the user requests.
    #[serde(rename = "android:minHeight")]
    pub min_height: Option<i32>,
    #[serde(rename = "android:minWidth")]
    pub min_width: Option<i32>,
}
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Gravity {
    /// Raw bit controlling whether the right/bottom edge is clipped to its
    /// container, based on the gravity direction being applied.
    AxisClip,
    /// Raw bit controlling how the right/bottom edge is placed.
    AxisPullAfter,
    /// Raw bit controlling how the left/top edge is placed.
    AxisPullBefore,
    /// Raw bit indicating the gravity for an axis has been specified.
    AxisSpecified,
    /// Bits defining the horizontal axis.
    AxisXShift,
    /// Bits defining the vertical axis.
    AxisYShift,
    /// Push object to the bottom of its container, not changing its size.
    Bottom,
    /// Place the object in the center of its container in both the vertical and
    /// horizontal axis, not changing its size.
    Center,
    /// Place object in the horizontal center of its container, not changing its
    /// size.
    CenterHorizontal,
    /// Place object in the vertical center of its container, not changing its
    /// size.
    CenterVertical,
    /// Flag to clip the edges of the object to its container along the
    /// horizontal axis.
    ClipHorizontal,
    /// Flag to clip the edges of the object to its container along the vertical
    /// axis.
    ClipVertical,
    /// Special constant to enable clipping to an overall display along the
    /// horizontal dimension.
    DisplayClipHorizontal,
    /// Special constant to enable clipping to an overall display along the
    /// vertical dimension.
    DisplayClipVertical,
    /// Push object to x-axis position at the end of its container, not changing
    /// its size.
    End,
    /// Grow the horizontal and vertical size of the object if needed so it
    /// completely fills its container.
    Fill,
    /// Grow the horizontal size of the object if needed so it completely fills
    /// its container.
    FillHorizontal,
    /// Grow the vertical size of the object if needed so it completely fills
    /// its container.
    FillVertical,
    /// Binary mask to get the absolute horizontal gravity of a gravity.
    HorizontalGravityMask,
    /// Push object to the left of its container, not changing its size.
    Left,
    /// Constant indicating that no gravity has been set *
    NoGravity,
    /// Binary mask for the horizontal gravity and script specific direction
    /// bit.
    RelativeHorizontalGravityMask,
    /// Raw bit controlling whether the layout direction is relative or not
    /// (START/END instead of absolute LEFT/RIGHT).
    RelativeLayoutDirection,
    /// Push object to the right of its container, not changing its size.
    Right,
    /// Push object to x-axis position at the start of its container, not
    /// changing its size.
    Start,
    /// Push object to the top of its container, not changing its size.
    Top,
    /// Binary mask to get the vertical gravity of a gravity.
    VerticalGravityMask,
}
