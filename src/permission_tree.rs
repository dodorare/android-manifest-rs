use super::resources::{MipmapOrDrawableResource, StringResourceOrString};
use serde::{Deserialize, Serialize};

/// Declares the base name for a tree of permissions.
///
/// The application takes ownership of all names within the tree. It can dynamically add
/// new permissions to the tree by calling [`PackageManager.addPermission()`] Names within
/// the tree are separated by periods `('.')`. For example, if the base name is
/// com.example.project.taxes.
///
/// Permissions like the following might be added:
///
/// * `com.example.project.taxes.CALCULATE`
/// * `com.example.project.taxes.deductions.MAKE_SOME_UP`
/// * `com.example.project.taxes.deductions.EXAGGERATE`
///
/// Note that this element does not declare a permission itself, only a
/// namespace in which further permissions can be placed. See the [`<permission>`]
/// element for information on declaring permissions.
///
/// ## XML Syntax
/// ```xml
/// <permission-tree android:icon="drawable resource"
///                  android:label="string resource" ]
///                  android:name="string" />
/// ```
///
/// ## Contained in
/// * [`<manifest>`]
///
/// ## Introduced in
/// API Level 1
///
/// [`PackageManager.addPermission()`]: https://developer.android.com/reference/android/content/pm/PackageManager#addPermission(android.content.pm.PermissionInfo)
/// [`<permission>`]: crate::Permission
/// [`<manifest>`]: crate::AndroidManifest
#[derive(
    Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Default, Clone,
)]
pub struct PermissionTree {
    /// An icon representing all the permissions in the tree. This attribute must be set
    /// as a reference to a drawable resource containing the image definition.
    #[yaserde(attribute, prefix = "android")]
    pub icon: Option<MipmapOrDrawableResource>,
    /// A user-readable name for the group. As a convenience, the label can be directly
    /// set as a raw string for quick and dirty programming. However, when the
    /// application is ready to be published, it should be set as a reference to a
    /// string resource, so that it can be localized like other strings in the user
    /// interface.
    #[yaserde(attribute, prefix = "android")]
    pub label: Option<StringResourceOrString>,
    /// The name that's at the base of the permission tree.  It serves as a prefix to all
    /// permission names in the tree. Java-style scoping should be used to ensure that
    /// the name is unique. The name must have more than two period-separated segments
    /// in its path — for example, `com.example.base` is OK, but `com.example` is not.
    #[yaserde(attribute, prefix = "android")]
    pub name: Option<String>,
}
