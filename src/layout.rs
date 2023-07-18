use serde::{Deserialize, Serialize};

/// Affect how an activity behaves in multi-window mode.
///
/// With Android 7.0, the <layout> manifest element supports several attributes that
/// affect how an activity behaves in multi-window mode:
///
/// ## Contained in
/// * [`<activity>`]
///
/// [`<activity>`]: crate::Activity
#[derive(
    Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Default, Clone,
)]
pub struct Layout {
    /// Default width of the activity when launched in freeform mode.
    #[yaserde(attribute, prefix = "android", rename = "defaultWidth")]
    pub default_width: Option<String>,
    /// Default height of the activity when launched in freeform mode.
    #[yaserde(attribute, prefix = "android", rename = "defaultHeight")]
    pub default_height: Option<String>,
    /// Initial placement of the activity when launched in freeform mode. See the Gravity
    /// reference for suitable values.
    #[yaserde(attribute, prefix = "android")]
    pub gravity: Gravity,
    /// Minimum height and minimum width for the activity in both split-screen and
    /// freeform modes. If the user moves the divider in split-screen mode to make an
    /// activity smaller than the specified minimum, the system crops the activity to
    /// the size the user requests.
    ///
    /// For example, the following code shows how to specify an activity's default size
    /// and location, and its minimum size, when the activity is displayed in freeform
    /// mode:
    ///
    /// ## XML Example
    /// ```xml
    /// <activity android:name=".MyActivity">
    ///    <layout android:defaultHeight="500dp"
    ///            android:defaultWidth="600dp"
    ///            android:gravity="top|end"
    ///            android:minHeight="450dp"
    ///            android:minWidth="300dp" />
    /// </activity>
    /// ```
    #[yaserde(attribute, prefix = "android", rename = "minHeight")]
    pub min_height: Option<String>,
    /// Minimum height and minimum width for the activity in both split-screen and
    /// freeform modes. If the user moves the divider in split-screen mode to make an
    /// activity smaller than the specified minimum, the system crops the activity to
    /// the size the user requests.
    ///
    /// For example, the following code shows how to specify an activity's default size
    /// and location, and its minimum size, when the activity is displayed in freeform
    /// mode:
    ///
    /// ## XML Example
    /// ```xml
    /// <activity android:name=".MyActivity">
    ///    <layout android:defaultHeight="500dp"
    ///            android:defaultWidth="600dp"
    ///            android:gravity="top|end"
    ///            android:minHeight="450dp"
    ///            android:minWidth="300dp" />
    /// </activity>
    /// ```
    #[yaserde(attribute, prefix = "android", rename = "minWidth")]
    pub min_width: Option<String>,
}

/// Standard constants and tools for placing an object within a potentially
/// larger container.
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum Gravity {
    /// Raw bit controlling whether the right/bottom edge is clipped to its
    /// container, based on the gravity direction being applied.
    #[yaserde(rename = "axisClip")]
    #[default]
    AxisClip,
    /// Raw bit controlling how the right/bottom edge is placed.
    #[yaserde(rename = "axisPullAfter")]
    AxisPullAfter,
    /// Raw bit controlling how the left/top edge is placed.
    #[yaserde(rename = "axisPullBefore")]
    AxisPullBefore,
    /// Raw bit indicating the gravity for an axis has been specified.
    #[yaserde(rename = "axisSpecified")]
    AxisSpecified,
    /// Bits defining the horizontal axis.
    #[yaserde(rename = "axisXShift")]
    AxisXShift,
    /// Bits defining the vertical axis.
    #[yaserde(rename = "axisYShift")]
    AxisYShift,
    /// Push object to the bottom of its container, not changing its size.
    #[yaserde(rename = "bottom")]
    Bottom,
    /// Place the object in the center of its container in both the vertical and
    /// horizontal axis, not changing its size.
    #[yaserde(rename = "center")]
    Center,
    /// Place object in the horizontal center of its container, not changing its
    /// size.
    #[yaserde(rename = "centerHorizontal")]
    CenterHorizontal,
    /// Place object in the vertical center of its container, not changing its
    /// size.
    #[yaserde(rename = "centerVertical")]
    CenterVertical,
    /// Flag to clip the edges of the object to its container along the
    /// horizontal axis.
    #[yaserde(rename = "clipHorizontal")]
    ClipHorizontal,
    /// Flag to clip the edges of the object to its container along the vertical
    /// axis.
    #[yaserde(rename = "clipVertical")]
    ClipVertical,
    /// Special constant to enable clipping to an overall display along the
    /// horizontal dimension.
    #[yaserde(rename = "displayClipHorizontal")]
    DisplayClipHorizontal,
    /// Special constant to enable clipping to an overall display along the
    /// vertical dimension.
    #[yaserde(rename = "displayClipVertical")]
    DisplayClipVertical,
    /// Push object to x-axis position at the end of its container, not changing
    /// its size.
    #[yaserde(rename = "end")]
    End,
    /// Grow the horizontal and vertical size of the object if needed so it
    /// completely fills its container.
    #[yaserde(rename = "fill")]
    Fill,
    /// Grow the horizontal size of the object if needed so it completely fills
    /// its container.
    #[yaserde(rename = "fillHorizontal")]
    FillHorizontal,
    /// Grow the vertical size of the object if needed so it completely fills
    /// its container.
    #[yaserde(rename = "fillVertical")]
    FillVertical,
    /// Binary mask to get the absolute horizontal gravity of a gravity.
    #[yaserde(rename = "horizontalGravityMask")]
    HorizontalGravityMask,
    /// Push object to the left of its container, not changing its size.
    #[yaserde(rename = "left")]
    Left,
    /// Constant indicating that no gravity has been set *
    #[yaserde(rename = "noGravity")]
    NoGravity,
    /// Binary mask for the horizontal gravity and script specific direction
    /// bit.
    #[yaserde(rename = "relativeHorizontalGravityMask")]
    RelativeHorizontalGravityMask,
    /// Raw bit controlling whether the layout direction is relative or not
    /// (START/END instead of absolute LEFT/RIGHT).
    #[yaserde(rename = "relativeLayoutDirection")]
    RelativeLayoutDirection,
    /// Push object to the right of its container, not changing its size.
    #[yaserde(rename = "right")]
    Right,
    /// Push object to x-axis position at the start of its container, not
    /// changing its size.
    #[yaserde(rename = "start")]
    Start,
    /// Push object to the top of its container, not changing its size.
    #[yaserde(rename = "top")]
    Top,
    /// Binary mask to get the vertical gravity of a gravity.
    #[yaserde(rename = "verticalGravityMask")]
    VerticalGravityMask,
}


