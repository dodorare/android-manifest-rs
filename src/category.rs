use serde::{Deserialize, Serialize};

/// Adds a category name to an intent filter.
///
/// See [`Intents and Intent Filters`] for details on intent filters and
/// the role of category specifications within a filter.
///
/// ## XML Syntax
/// ```xml
///  <category android:name="string" />
/// ```
///
/// ## Contained in:
/// * [`<intent-filter>`]
///
/// ## Introduced in
/// API Level 1
///
/// [`Intents and Intent Filters`]: https://developer.android.com/guide/components/intents-filters
/// [`<intent-filter>`]: crate::IntentFilter
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Default)]
pub struct Category {
    /// The name of the category. Standard categories are defined in the [`Intent`]
    /// class as CATEGORY_name constants. The name assigned here can be derived
    /// from those constants by prefixing `"android.intent.category."` to
    /// the name that follows CATEGORY_. For example, the string value for
    /// CATEGORY_LAUNCHER is`"android.intent.category.LAUNCHER"`.
    ///
    /// ## Note
    /// In order to receive implicit intents, you must include the
    /// [`CATEGORY_DEFAULT`] category in the intent filter. The methods
    /// [`startActivity()`] and [`startActivityForResult()`] treat all intents
    /// as if they declared the [`CATEGORY_DEFAULT`] category. If you do not
    /// declare it in your intent filter, no implicit intents will resolve to
    /// your activity.
    ///
    /// Custom categories should use the package name as a prefix, to ensure that
    /// they are unique.
    ///
    /// [`Intent`]: https://developer.android.com/reference/android/content/Intent
    /// [`CATEGORY_DEFAULT`]: https://developer.android.com/reference/android/content/Intent#CATEGORY_DEFAULT
    /// [`startActivity()`]: https://developer.android.com/reference/android/app/Activity#startActivity(android.content.Intent)
    /// [`startActivityForResult()`]: https://developer.android.com/reference/android/app/Activity#startActivityForResult(android.content.Intent,%20int)
    #[yaserde(attribute, prefix = "android")]
    pub name: Option<String>,
}
