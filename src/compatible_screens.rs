use serde::{Deserialize, Serialize};

/// Specifies each screen configuration with which the application is
/// compatible.
///
/// Only one instance of the `<compatible-screens>` element is
/// allowed in the manifest, but it can contain multiple `<screen>` elements.
/// Each <screen> element specifies a specific screen size-density combination
/// with which the application is compatible.
///
/// The Android system does not read the `<compatible-screens>` manifest element (neither
/// at install-time nor at runtime). This element is informational only and may be used by
/// external services (such as Google Play) to better understand the application's
/// compatibility with specific screen configurations and enable filtering for
/// users. Any screen configuration that is not declared in this element is a
/// screen with which the application is not compatible. Thus, external services (such as
/// Google Play) should not provide the application to devices with such screens.
///
/// ## Caution
/// Normally, you should not use this manifest element. Using this
/// element can dramatically reduce the potential user base for your
/// application, by not allowing users to install your application if they have
/// a device with a screen configuration that you have not listed. You should
/// use it only as a last resort, when the application absolutely does not work
/// with specific screen configurations. Instead of using this element, you
/// should follow the guide to [`Supporting Multiple Screens`] to provide scalable
/// support for multiple screens using alternative layouts and bitmaps for
/// different screen sizes and densities.
///
/// If you want to set only a minimum screen size for your your application,
/// then you should use the [`<supports-screens>`] element. For example, if you
/// want your application to be available only for large and xlarge screen
/// devices, the [`<supports-screens>`] element allows you to declare that your
/// application does not support small and normal screen sizes. External
/// services (such as Google Play) will filter your application accordingly. You
/// can also use the [`<supports-screens>`] element to declare whether the system
/// should resize your application for different screen sizes.
///
/// Also see the [`Filters on Google Play`] document for more information about how Google
/// Play filters applications using this and other manifest elements.
///
/// ## XML Syntax
/// ```xml
/// <compatible-screens>
///    <screen android:screenSize=["small" | "normal" | "large" | "xlarge"]
///            android:screenDensity=["ldpi" | "mdpi" | "hdpi" | "xhdpi"
///                                   | "280" | "360" | "420" | "480" | "560" ] />
///    ...
/// </compatible-screens>
/// ```
///
/// ## Contained in
/// * [`<manifest>`]
///
/// ## Introduced in
/// API Level 9
///
/// [`Supporting Multiple Screens`]: https://developer.android.com/guide/practices/screens_support
/// [`<supports-screens>`]: crate::SupportsScreens
/// [`Filters on Google Play`]: https://developer.android.com/google/play/filters
/// [`<manifest>`]: crate::AndroidManifest
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Clone)]
pub struct CompatibleScreens {
    pub screen: Vec<Screen>,
}

/// Specifies a single screen configuration with which the application is
/// compatible.
///
/// At least one instance of this element must be placed inside the
/// `<compatible-screens>` element. This element must include both the
/// `android:screenSize` and `android:screenDensity` attributes (if you do not
/// declare both attributes, then the element is ignored).
///
/// ## XML Example
/// If your application is compatible with only small and normal screens, regardless of
/// screen density, then you must specify twelve different <screen> elements, because each
/// screen size has six different density configurations. You must declare each one of
/// these; any combination of size and density that you do not specify is considered a
/// screen configuration with which your application is not compatible. Here's what the
/// manifest entry looks like if your application is compatible with only small and normal
/// screens:
/// ```xml
/// <manifest ... >
///     ...
///     <compatible-screens>
///        <!-- all small size screens -->
///        <screen android:screenSize="small" android:screenDensity="ldpi" />
///        <screen android:screenSize="small" android:screenDensity="mdpi" />
///        <screen android:screenSize="small" android:screenDensity="hdpi" />
///        <screen android:screenSize="small" android:screenDensity="xhdpi" />
///        <screen android:screenSize="small" android:screenDensity="xxhdpi" />
///        <screen android:screenSize="small" android:screenDensity="xxxhdpi" />
///        <!-- all normal size screens -->
///        <screen android:screenSize="normal" android:screenDensity="ldpi" />
///        <screen android:screenSize="normal" android:screenDensity="mdpi" />
///        <screen android:screenSize="normal" android:screenDensity="hdpi" />
///        <screen android:screenSize="normal" android:screenDensity="xhdpi" />
///        <screen android:screenSize="normal" android:screenDensity="xxhdpi" />
///        <screen android:screenSize="normal" android:screenDensity="xxxhdpi" />
///     </compatible-screens>
///     <application ... >
///        ...
///     <application>
/// </manifest>
/// ```
///
/// Contained in:
/// [`<compatible-screens>`]
///
/// [`<compatible-screens>`]: crate::CompatibleScreens
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Clone)]
pub struct Screen {
    /// `Required`. Specifies the screen size for this screen configuration.
    ///
    /// For information about the different screen sizes, see [`Supporting Multiple
    /// Screens`].
    ///
    /// [`Supporting Multiple Screens`]: https://developer.android.com/guide/practices/screens_support#range
    #[yaserde(attribute, prefix = "android", rename = "screenSize")]
    pub screen_size: ScreenSize,
    /// `Required.` Specifies the screen density for this screen configuration.
    ///
    /// For information about the different screen densities, see [`Supporting Multiple
    /// Screens`].
    ///
    /// Accepted values:
    /// * "ldpi" (approximately 120 dpi)
    /// * "mdpi" (approximately 160 dpi)
    /// * "hdpi" (approximately 240 dpi)
    /// * "xhdpi" (approximately 320 dpi)
    /// * "280"
    /// * "360"
    /// * "420"
    /// * "480"
    /// * "560"
    ///
    /// [`Supporting Multiple Screens`]: https://developer.android.com/guide/practices/screens_support#range
    #[yaserde(attribute, prefix = "android", rename = "screenDensity")]
    pub screen_density: String,
}

/// Android runs on a variety of devices that have different screen sizes and pixel
/// densities. The system performs basic scaling and resizing to adapt your user interface
/// to different screens.
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ScreenSize {
    /// Screens that are of similar size to a low-density QVGA screen. The minimum layout
    /// size for a small screen is approximately 320x426 dp units. Examples are QVGA
    /// low-density and VGA high density.
    #[yaserde(rename = "small")]
    Small,
    /// Screens that are of similar size to a medium-density HVGA screen. The minimum
    /// layout size for a normal screen is approximately 320x470 dp units. Examples of
    /// such screens a WQVGA low-density, HVGA medium-density, WVGA high-density.
    #[yaserde(rename = "normal")]
    Normal,
    /// Screens that are of similar size to a medium-density VGA screen. The minimum
    /// layout size for a large screen is approximately 480x640 dp units. Examples are VGA
    /// and WVGA medium-density screens.
    #[yaserde(rename = "large")]
    Large,
    /// Screens that are considerably larger than the traditional medium-density HVGA
    /// screen. The minimum layout size for an xlarge screen is approximately 720x960 dp
    /// units. In most cases, devices with extra-large screens would be too large to carry
    /// in a pocket and would most likely be tablet-style devices. Added in API level 9.
    #[yaserde(rename = "xlarge")]
    Xlarge,
}

impl Default for ScreenSize {
    fn default() -> Self {
        ScreenSize::Normal
    }
}
