use serde::{Deserialize, Serialize};

/// Defines the path and required permissions for a specific subset of data within a content provider. This element can be specified multiple times to supply multiple paths.
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "path-permission")]
pub struct PathPermission {
    /// A complete URI path for a subset of content provider data. Permission can be granted only to the particular data identified by this path. 
    /// When used to provide search suggestion content, it must be appended with "/search_suggest_query".
    #[serde(rename = "android:path")]
    pub path: Option<String>,
    /// The initial part of a URI path for a subset of content provider data. Permission can be granted to all data subsets with paths that share this initial part.
    #[serde(rename = "android:pathPrefix")]
    pub path_prefix: Option<String>,
    /// A complete URI path for a subset of content provider data, but one that can use the following wildcards:
    /// An asterisk ('*'). This matches a sequence of 0 to many occurrences of the immediately preceding character.
    /// A period followed by an asterisk (".*"). This matches any sequence of 0 or more characters.
    /// Because '\' is used as an escape character when the string is read from XML (before it is parsed as a pattern), you will need to double-escape. 
    /// For example, a literal '*' would be written as "\\*" and a literal '\' would be written as "\\". This is basically the same as what you would need to write if constructing the string in Java code.
    /// For more information on these types of patterns, see the descriptions of PATTERN_LITERAL, PATTERN_PREFIX, and PATTERN_SIMPLE_GLOB in the PatternMatcher class.
    #[serde(rename = "android:pathPattern")]
    pub path_pattern: Option<String>,
    /// The name of a permission that clients must have in order to read or write the content provider's data. 
    /// This attribute is a convenient way of setting a single permission for both reading and writing.
    /// However, the readPermission and writePermission attributes take precedence over this one.
    #[serde(rename = "android:permission")]
    pub permission: Option<String>,
    /// A permission that clients must have in order to query the content provider.
    #[serde(rename = "android:readPermission")]
    pub read_permission: Option<String>,
    /// A permission that clients must have in order to make changes to the data controlled by the content provider.
    #[serde(rename = "android:writePermission")]
    pub write_permission: Option<String>,
}
