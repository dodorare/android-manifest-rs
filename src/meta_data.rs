use super::resources::{DrawableResource, Resource, StringResource};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "meta-data")]
pub struct MetaData {
    /// A unique name for the item. To ensure that the name is unique, use a Java-style naming convention — for example, `"com.example.project.activity.fred"`.
    #[serde(rename = "android:name")]
    pub name: Option<String>,
    /// A reference to a resource. The ID of the resource is the value assigned to the item. The ID can be retrieved from the meta-data Bundle by the `Bundle.getInt()` method.
    #[serde(rename = "android:resource")]
    pub resource: Option<String>,
}
