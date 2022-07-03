use super::resources::{
    MipmapOrDrawableResource, Resource, StringResource, StringResourceOrString,
};
use serde::{Deserialize, Serialize};

/// Declares a name for a logical grouping of related permissions.
///
/// Individual permission join the group through the permissionGroup attribute of the
/// [`<permission>`] element. Members of a group are presented together in
/// the user interface.
///
/// Note that this element does not declare a permission itself, only a category in which
/// permissions can be placed. See the [`<permission>`] element for element for
/// information on declaring permissions and assigning them to groups.
///
/// ## XML Syntax
/// ```xml
/// <permission-group android:description="string resource"
///                   android:icon="drawable resource"
///                   android:label="string resource"
///                   android:name="string" />
/// ```
///
/// ## Contained in
/// * [`<manifest>`]
///
/// ## Introduced in
/// API Level 1
///
/// [`<manifest>`]: crate::AndroidManifest
/// [`<permission>`]: crate::Permission
#[derive(
    Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Default, Clone,
)]
pub struct PermissionGroup {
    /// User-readable text that describes the group. The text should be longer and more
    /// explanatory than the label. This attribute must be set as a reference to a
    /// string resource. Unlike the label attribute, it cannot be a raw string.
    #[yaserde(attribute, prefix = "android")]
    pub description: Option<Resource<StringResource>>,
    /// An icon representing the permission. This attribute must be set as a reference to
    /// a drawable resource containing the image definition.
    #[yaserde(attribute, prefix = "android")]
    pub icon: Option<MipmapOrDrawableResource>,
    /// A user-readable name for the group. As a convenience, the label can be directly
    /// set as a raw string while you're developing the application. However, when the
    /// application is ready to be published, it should be set as a reference to a
    /// string resource, so that it can be localized like other strings in the user
    /// interface.
    #[yaserde(attribute, prefix = "android")]
    pub label: Option<StringResourceOrString>,
    /// The name of the group. This is the name that can be assigned to a
    /// [`<permission>`] element's [`<permissionGroup>`] attribute.
    ///
    /// [`<permission>`]: crate::Permission
    /// [`<permissionGroup>`]: crate::Permission#structfield.permission_group
    #[yaserde(attribute, prefix = "android")]
    pub name: Option<String>,
}
