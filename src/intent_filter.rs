use super::action::Action;
use super::category::Category;
use super::data::Data;
use super::resources::{DrawableResource, Resource, StyleResource};
use serde::{Deserialize, Serialize};

/// Specifies the types of intents that an activity, service, or broadcast receiver can
/// respond to.
///
/// An intent filter declares the capabilities of its parent component — what an activity
/// or service can do and what types of broadcasts a receiver can handle. It opens the
/// component to receiving intents of the advertised type, while filtering out those that
/// are not meaningful for the component. Most of the contents of the filter are
/// described by its [`<action>`], [`<category>`], and [`<data>`] subelements.
///
/// For a more detailed discussion of filters, see the separate [`Intents and Intent
/// Filters`] document, as well as the [`Intents Filters`]
/// section in the introduction.
///
/// ## XML Syntax
/// ```xml
/// <intent-filter android:icon="drawable resource"
///                android:label="string resource"
///                android:priority="integer" >
///     ...
/// </intent-filter>
/// ```
///
/// ## Contained in
/// * [`<activity>`]
/// * [`<activity-alias>`]
/// * [`<service>`]
/// * [`<receiver>`]
/// * [`<provider>`]
///
/// ## Must contain
/// * [`<action>`]
///
/// ## Can contain
/// * [`<category>`]
/// * [`<data>`]
///
/// ## Introduced in
/// API Level 1
///
/// [`Intents and Intent Filters`]: https://developer.android.com/guide/components/intents-filters
/// [`Intents Filters`]: https://developer.android.com/guide/topics/manifest/manifest-intro#ifs
/// [`<activity>`]: crate::Activity
/// [`<activity-alias>`]: crate::ActivityAlias
/// [`<service>`]: crate::Service
/// [`<receiver>`]: crate::Receiver
/// [`<provider>`]: crate::Provider
/// [`<action>`]: crate::Action
/// [`<category>`]: crate::Category
/// [`<data>`]: crate::Data
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Default)]
pub struct IntentFilter {
    /// An icon that represents the parent activity, service, or broadcast receiver when
    /// that component is presented to the user as having the capability described by
    /// the filter.
    ///
    /// This attribute must be set as a reference to a drawable resource containing the
    /// image definition. The default value is the icon set by the parent component's
    /// icon attribute. If the parent does not specify an icon, the default is the
    /// icon set by the [`<application>`] element.
    ///
    /// For more on intent filter icons, see [`Icons and Labels`] in the introduction.
    ///
    /// [`<application>`]: crate::Application
    /// [`Icons and Labels`]: https://developer.android.com/guide/topics/manifest/manifest-intro#iconlabel
    #[yaserde(attribute, prefix = "android")]
    pub icon: Option<Resource<DrawableResource>>,
    /// A user-readable label for the parent component. This label, rather than the one
    /// set by the parent component, is used when the component is presented to the
    /// user as having the capability described by the filter. The label should be set
    /// as a reference to a string resource, so that it can be localized like other
    /// strings in the user interface. However, as a convenience while you're
    /// developing the application, it can also be set as a raw string.
    ///
    /// The default value is the label set by the parent component. If the parent does not
    /// specify a label, the default is the label set by the [`<application>`]
    /// element's [`label`] attribute.s
    ///
    /// For more on intent filter labels, see [`Icons and Labels`] in the introduction.
    ///
    /// [`<application>`]: crate::Application
    /// [`label`]: crate::Application#structfield.label
    /// [`Icons and Labels`]: https://developer.android.com/guide/topics/manifest/manifest-intro#iconlabel
    #[yaserde(attribute, prefix = "android")]
    pub label: Option<Resource<StyleResource>>,
    /// The priority that should be given to the parent component with regard to handling
    /// intents of the type described by the filter. This attribute has meaning for
    /// both activities and broadcast receivers:
    ///
    /// * It provides information about how able an activity is to respond to an intent
    ///   that matches the filter, relative
    /// to other activities that could also respond to the intent. When an intent
    /// could be handled by multiple activities with different priorities, Android
    /// will consider only those with higher priority values as potential targets for
    /// the intent.
    /// * It controls the order in which broadcast receivers are executed to
    /// receive broadcast messages. Those with higher priority values are called
    /// before those with lower values. (The order applies only to
    /// synchronous messages; it's ignored for asynchronous messages.)
    ///
    /// Use this attribute only if you really need to impose a specific order in
    /// which the broadcasts are received, or want to force Android to prefer
    /// one activity over others. The value must be an integer, such as
    /// "100". Higher numbers have a higher priority. The default value is 0
    /// In certain circumstances the requested priority is ignored and the value
    /// is capped to 0. This occurs when:
    ///
    /// * A non-privileged application requests any priority > 0
    /// * A privileged application requests a priority > 0 for [`ACTION_VIEW`],
    ///   [`ACTION_SEND`], [`ACTION_SENDTO`] or
    /// [`ACTION_SEND_MULTIPLE`]
    ///
    /// Also see [`setPriority()`].
    ///
    /// [`ACTION_VIEW`]: https://developer.android.com/reference/android/content/Intent#ACTION_VIEW
    /// [`ACTION_SEND`]: https://developer.android.com/reference/android/content/Intent#ACTION_SEND
    /// [`ACTION_SENDTO`]: https://developer.android.com/reference/android/content/Intent#ACTION_SENDTO
    /// [`ACTION_SEND_MULTIPLE`]: https://developer.android.com/reference/android/content/Intent#ACTION_SEND_MULTIPLE
    /// [`setPriority()`]: https://developer.android.com/reference/android/content/IntentFilter#setPriority(int)
    #[yaserde(attribute, prefix = "android")]
    pub priority: Option<i32>,
    /// The order in which the filter should be processed when multiple filters match.
    /// order differs from priority in that priority applies across apps, while order
    /// disambiguates multiple matching filters in a single app.
    ///
    /// When multiple filters could match, use a directed intent instead.
    ///
    /// The value must be an integer, such as "100". Higher numbers are matched first. The
    /// default value is 0.
    ///
    /// This attribute was introduced in API Level 28.
    #[yaserde(attribute, prefix = "android")]
    pub ordery: Option<i32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub action: Vec<Action>,
    // #[serde(skip_serializing_if = "Vec::is_empty", default)]
    // pub category: Vec<Category>,
    // #[serde(skip_serializing_if = "Vec::is_empty", default)]
    // pub data: Vec<Data>,
}
