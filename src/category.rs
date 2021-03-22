use serde::{Deserialize, Serialize};

/// Adds a category name to an intent filter.
///
/// See [`Intents and Intent Filters`](https://developer.android.com/guide/components/intents-filters) for details on intent filters and
/// the role of category specifications within a filter.
///
/// ## Contained in:
/// [`<intent-filter>`](crate::IntentFilter)
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "category")]
pub struct Category {
    /// The name of the category. Standard categories are defined in the Intent
    /// class as CATEGORY_name constants. The name assigned here can be derived
    /// from those constants by prefixing `"android.intent.category."` to
    /// the name that follows CATEGORY_. For example, the string value for
    /// CATEGORY_LAUNCHER is`"android.intent.category.LAUNCHER"`.
    ///
    /// ## Note
    /// In order to receive implicit intents, you must include the
    /// `CATEGORY_DEFAULT` category in the intent filter. The methods
    /// `startActivity()` and `startActivityForResult()` treat all intents
    /// as if they declared the `CATEGORY_DEFAULT` category. If you do not
    /// declare it in your intent filter, no implicit intents will resolve to
    /// your activity. Custom categories should use the package name as a
    /// prefix, to ensure that they are unique.
    #[serde(rename = "android:name")]
    pub name: Option<bool>,
}
