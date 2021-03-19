use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "layout")]
pub struct Layout {
    /// Default width of the activity when launched in freeform mode.
    #[serde(rename = "android:defaultWidth")]
    pub default_width: Option<i32>,
    /// Default height of the activity when launched in freeform mode.
    #[serde(rename = "android:defaultHeight")]
    pub default_height: Option<i32>,
    ///Initial placement of the activity when launched in freeform mode. See the Gravity reference for suitable values.
    #[serde(rename = "android:gravity")]
    pub gravity: Option<Gravity>,
    /// Minimum height and minimum width for the activity in both split-screen and freeform modes. If the user moves the divider in split-screen mode to make an activity
    /// smaller than the specified minimum, the system crops the activity to the size the user requests.
    #[serde(rename = "android:minHeight")]
    pub min_height: Option<i32>,
    #[serde(rename = "android:minWidth")]
    pub min_width: Option<i32>,
}

pub enum Gravity {
    /// Raw bit controlling whether the right/bottom edge is clipped to its container, based on the gravity direction being applied.
    AXIS_CLIP,
    /// Raw bit controlling how the right/bottom edge is placed.
    AXIS_PULL_AFTER,
    /// Raw bit controlling how the left/top edge is placed.
    AXIS_PULL_BEFORE,
    /// Raw bit indicating the gravity for an axis has been specified.
    AXIS_SPECIFIED,
    /// Bits defining the horizontal axis.
    AXIS_X_SHIFT,
    /// Bits defining the vertical axis.
    AXIS_Y_SHIFT,
    /// Push object to the bottom of its container, not changing its size.
    BOTTOM,
    /// Place the object in the center of its container in both the vertical and horizontal axis, not changing its size.
    CENTER,
    /// Place object in the horizontal center of its container, not changing its size.
    CENTER_HORIZONTAL,
    /// Place object in the vertical center of its container, not changing its size.
    CENTER_VERTICAL,
    /// Flag to clip the edges of the object to its container along the horizontal axis.
    CLIP_HORIZONTAL,
    /// Flag to clip the edges of the object to its container along the vertical axis.
    CLIP_VERTICAL,
    /// Special constant to enable clipping to an overall display along the horizontal dimension.
    DISPLAY_CLIP_HORIZONTAL,
    /// Special constant to enable clipping to an overall display along the vertical dimension.
    DISPLAY_CLIP_VERTICAL,
    /// Push object to x-axis position at the end of its container, not changing its size.
    END,
    /// Grow the horizontal and vertical size of the object if needed so it completely fills its container.
    FILL,
    /// Grow the horizontal size of the object if needed so it completely fills its container.
    FILL_HORIZONTAL,
    /// Grow the vertical size of the object if needed so it completely fills its container.
    FILL_VERTICAL,
    /// Binary mask to get the absolute horizontal gravity of a gravity.
    HORIZONTAL_GRAVITY_MASK,
    /// Push object to the left of its container, not changing its size.
    LEFT,
    /// Constant indicating that no gravity has been set *
    NO_GRAVITY,
    /// Binary mask for the horizontal gravity and script specific direction bit.
    RELATIVE_HORIZONTAL_GRAVITY_MASK,
    /// Raw bit controlling whether the layout direction is relative or not (START/END instead of absolute LEFT/RIGHT).
    RELATIVE_LAYOUT_DIRECTION,
    /// Push object to the right of its container, not changing its size.
    RIGHT,
    /// Push object to x-axis position at the start of its container, not changing its size.
    START,
    /// Push object to the top of its container, not changing its size.
    TOP,
    /// Binary mask to get the vertical gravity of a gravity.
    VERTICAL_GRAVITY_MASK,
}