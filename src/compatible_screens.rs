use serde::{Deserialize, Serialize};

/// # contained in:
/// `<manifest>`
///
/// # description:
/// Specifies each screen configuration with which the application is compatible. Only one instance of the
/// `<compatible-screens>` element is allowed in the manifest, but it can contain multiple `<screen>` elements. Each <screen> element specifies a specific screen size-density combination with which the application is compatible.
/// The Android system does not read the `<compatible-screens>` manifest element (neither at install-time nor at runtime).
/// This element is informational only and may be used by external services (such as Google Play) to better understand the application's compatibility with specific
/// screen configurations and enable filtering for users. Any screen configuration that is not declared in this element is a screen with which the application is not compatible.
///
/// `Caution:` Normally, you should not use this manifest element. Using this element can dramatically reduce the potential user base for your application, by not allowing users to install your application if they have a device with a
/// screen configuration that you have not listed. You should use it only as a last resort, when the application absolutely does not work with specific screen configurations. Instead of using this element, you should follow the guide to
/// Supporting Multiple Screens to provide scalable support for multiple screens using alternative layouts and bitmaps for different screen sizes and densities.
///
/// If you want to set only a minimum screen size for your your application, then you should use the `<supports-screens>` element. For example, if you want your application to be available only for large and xlarge screen
/// devices, the `<supports-screens>` element allows you to declare that your application does not support small and normal screen sizes. External services (such as Google Play) will filter your application accordingly. 
/// You can also use the `<supports-screens>` element to declare whether the system should resize your application for different screen sizes.
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "compatible-screens")]
pub struct CompatibleScreensy {
    screen: Screen,
}
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "screen")]
pub struct Screen {
    /// `Required`. Specifies the screen size for this screen configuration.
    /// For information about the different screen sizes, see
    /// https://developer.android.com/guide/practices/screens_support#range
    #[serde(rename = "android:screenSize")]
    pub screen_size: Option<ScreenSize>,
    /// Required. Specifies the screen density for this screen configuration.
    #[serde(rename = "android:screenDensity")]
    pub screen_density: Option<ScreenDensity>,
}
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ScreenSize {
    Small,
    Normal,
    Large,
    Xlarge,
}

// todo: need to be finalized
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ScreenDensity {
    /// "ldpi" (approximately 120 dpi)
    Ldpi,
    /// "mdpi" (approximately 160 dpi)
    Mdpi,
    /// "hdpi" (approximately 240 dpi)
    Hdpi,
    /// "xhdpi" (approximately 320 dpi)
    Xhdpi,
    /// "xxhdpi" (approximately 480 dpi)
    Xxhdpi,
    /// "xxxhdpi" (approximately 560-640 dpi)
    Xxxhdpi,
}
