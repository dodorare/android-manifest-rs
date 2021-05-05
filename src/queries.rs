use serde::{Deserialize, Serialize};

/// Specifies the set of other apps that an app intends to interact with. 
///
/// These other apps can be specified by package name, by intent signature, or by
/// provider authority, as described in later sections on this page.
///
/// ## Node
/// Some packages are [`visible automatically`]. Your app can always see these packages in 
/// its queries for other installed apps. To view other packages, declare your app's need 
/// for increased package visibility using the <queries> element.
///
/// Learn more about how to use the <queries> element in the guide on [`package visibility filtering`].
///
/// ## XML Syntax
/// ```xml
/// <queries>
///      <package android:name="string" />
///      <intent>
///         ...
///      </intent>
///      <provider android:authorities="list" />
///  </queries>
/// ```
///
/// ## Contained in
/// * [`<manifest>`]
///
/// ## Introduced in
/// API Level 30
///
/// [`<manifest>`]: crate::Manifest
/// [`package visibility filtering`]: https://developer.android.com/training/package-visibility
/// [`visible automatically`]: https://developer.android.com/training/package-visibility/automatic
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq)]
pub struct Queries {
   pub package: Option<Package>,
}   

/// Specifies a single app that your app intends to access. This other app might integrate with your app,
/// or your app might use services that the other app provides.
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq)]
pub struct Package {
    /// `Required`. Specifies the package name of the other app.
    #[yaserde(attribute, prefix = "android")]
    pub name: String,
}

/// Specifies an [`intent filter signature`]. Your app can discover other apps that have matching 
/// [`<intent-filter>`] elements.
/// 
/// ## Node
/// There are some restrictions on the options that you can include in this <intent> element,
/// compared to a typical intent filter signature. Learn more about these restrictions in the
/// "intent filter signature" section of the guide to [`declaring package visibility needs`].
///
/// [`<intent-filter>`]: crate::IntentFilter
/// [`intent filter signature`]: https://developer.android.com/training/basics/intents/filters
/// [`declaring package visibility needs`]: https://developer.android.com/training/package-visibility/declaring#intent-filter-signature
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq)]
pub struct Intent{

}

/// Specifies one or more [`content provider authorities`]. Your app can discover other apps 
/// whose content providers use the specified authorities.
///
/// ## Note
/// There are some restrictions on the options that you can include in this `<provider>` element,
/// compared to a typical [`<provider>`] manifest element. Usually, you only specify the
/// `android:authorities` attribute.
///
/// [`<provider>`]: crate::Provider
/// [`content provider authorities`]: https://developer.android.com/guide/topics/providers/content-provider-basics#ContentURIs
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq)]
pub struct Provider{

}