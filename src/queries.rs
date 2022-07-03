use super::action::Action;
use super::attribute_list::{AttributeList, Semicolon};
use super::data::Data;
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
/// Learn more about how to use the <queries> element in the guide on [`package visibility
/// filtering`].
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
/// [`<manifest>`]: crate::AndroidManifest
/// [`package visibility filtering`]: https://developer.android.com/training/package-visibility
/// [`visible automatically`]: https://developer.android.com/training/package-visibility/automatic
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Clone)]
pub struct Queries {
    /// Specifies a single app that your app intends to access. This other app might
    /// integrate with your app, or your app might use services that the other app
    /// provides.
    pub package: Option<Package>,
    /// Specifies an [`intent filter signature`]. Your app can discover other apps that
    /// have matching [`<intent-filter>`] elements.
    pub intent: Option<Intent>,
    /// Specifies one or more [`content provider authorities`]. Your app can discover
    /// other apps whose content providers use the specified authorities.
    ///
    /// ## Note
    /// There are some restrictions on the options that you can include in this
    /// `<provider>` element, compared to a typical [`<provider>`] manifest element.
    /// Usually, you only specify the `android:authorities` attribute.
    ///
    /// [`<provider>`]: crate::Provider
    /// [`content provider authorities`]: https://developer.android.com/guide/topics/providers/content-provider-basics#ContentURIs
    pub provider: Vec<QueriesProvider>,
}

/// Specifies a single app that your app intends to access. This other app might integrate
/// with your app, or your app might use services that the other app provides.
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Clone)]
pub struct Package {
    /// `Required`. Specifies the package name of the other app.
    #[yaserde(attribute, prefix = "android")]
    pub name: String,
}

/// Specifies an [`intent filter signature`]. Your app can discover other apps that have
/// matching [`<intent-filter>`] elements.
///
/// ## Node
/// There are some restrictions on the options that you can include in this <intent>
/// element, compared to a typical intent filter signature. Learn more about these
/// restrictions in the "intent filter signature" section of the guide to [`declaring
/// package visibility needs`].
///
/// [`<intent-filter>`]: crate::IntentFilter
/// [`intent filter signature`]: https://developer.android.com/training/basics/intents/filters
/// [`declaring package visibility needs`]: https://developer.android.com/training/package-visibility/declaring#intent-filter-signature
#[derive(
    Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Default, Clone,
)]
pub struct Intent {
    pub action: Action,
    pub data: Vec<Data>,
}

/// Declares a content provider component used in [`<queries>`].
///
/// A content provider is a subclass of [`ContentProvider`] that supplies structured
/// access to data managed by the application. All content providers in your application
/// must be defined in a `<provider>` element in the manifest file; otherwise, the system
/// is unaware of them and doesn't run them.
///
/// You only declare content providers that are part of your application. Content
/// providers in other applications that you use in your application should not be
/// declared.
///
/// The Android system stores references to content providers according to an authority
/// string, part of the provider's content URI. For example, suppose you want to access a
/// content provider that stores information about health care professionals.
/// To do this, you call the method [`ContentResolver.query()`],
/// which among other arguments takes a URI that identifies the provider:
///
/// ## XML Example
/// ```xml
/// content://com.example.project.healthcareprovider/nurses/rn
/// ```
///
/// The `content:` `scheme` identifies the URI as a content URI pointing to an Android
/// content provider. The authority `com.example.project.healthcareprovider` identifies
/// the provider itself; the Android system looks up the authority in its list of known
/// providers and their authorities. The substring `nurses/rn` is a `path`, which the
/// content provider can use to identify subsets of the provider data.
///
/// In cases where you need to query a content provider but don't know the specific
/// package names, you can declare that provider authority in a <provider> element,
/// as shown in the following snippet:
///
/// ## XML Example
/// ```xml
/// <manifest package="com.example.suite.enterprise">
///     <queries>
///         <provider android:authorities="com.example.settings.files" />
///     </queries>
///    ...
/// </manifest>
/// ```
///
/// ## Node
/// If your <queries> element includes a <provider> element, you might see an editor
/// warning in Android Studio related to the <provider> element. As long as you're using
/// the latest "dot" release of the Android Gradle plugin, your build is unaffected, so
/// you can disregard the warning. Learn more in the blog post about [`Preparing your
/// Gradle build for package visibility in Android 11`].
///
/// ## Contained in
/// * [`<queries>`]
///
/// ## Introduced in
/// API Level 30
///
/// [`ContentProvider`]: https://developer.android.com/reference/android/content/ContentProvider
/// [`ContentResolver.query()`]: https://developer.android.com/reference/android/content/ContentResolver#query(android.net.Uri,%20java.lang.String[],%20android.os.Bundle,%20android.os.CancellationSignal)
/// [`Content Providers`]: https://developer.android.com/guide/topics/providers/content-providers
/// [`<queries>`]: crate::Queries
/// [`Preparing your Gradle build for package visibility in Android 11`]: https://android-developers.googleblog.com/2020/07/preparing-your-build-for-package-visibility-in-android-11.html
#[derive(
    Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Default, Clone,
)]
#[serde(rename = "provider")]
pub struct QueriesProvider {
    /// A list of one or more URI authorities that identify data offered by the content
    /// provider. Multiple authorities are listed by separating their names with a
    /// semicolon. To avoid conflicts, authority names should use a Java-style naming
    /// convention (such as com.example.provider.cartoonprovider). Typically, it's the
    /// name of the [`ContentProvider`] subclass that implements the provider
    ///
    /// There is no default. At least one authority must be specified.
    ///
    /// [`ContentProvider`]: https://developer.android.com/reference/android/content/ContentProvider
    #[yaserde(
        attribute,
        prefix = "android",
        skip_serializing_if = "check_authorities"
    )]
    #[serde(skip_serializing_if = "AttributeList::is_empty")]
    pub authorities: AttributeList<Semicolon, String>,
    /// The name of the class that implements the content provider, a subclass of
    /// [`ContentProvider`]. This should be a fully qualified class name (such
    /// as, `"com.example.project.TransportationProvider"`). However, as a
    /// shorthand, if the first character of the name is a period, it is
    /// appended to the package name specified in the [`<manifest>`] element.
    ///
    /// There is no default. The name must be specified.
    ///
    /// [`ContentProvider`]: https://developer.android.com/reference/android/content/ContentProvider
    /// [`<manifest>`]: crate::AndroidManifest
    #[yaserde(attribute, prefix = "android")]
    pub name: String,
}

impl QueriesProvider {
    pub fn check_authorities(&self, value: &AttributeList<Semicolon, String>) -> bool {
        value.is_empty()
    }
}
