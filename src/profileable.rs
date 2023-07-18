use serde::{Deserialize, Serialize};

use crate::VarOrBool;

/// Specifies how profilers can access this application.
///
/// ## XML Syntax
/// ```xml
/// <profileable android:shell=["true" | "false"]
///              android:enable=["true" | "false"]
///  />
/// ```
///
/// ## Contained in
/// * [`<application>`]
///
/// ## Introduced in
/// API Level 29
///
/// [`<application>`]: crate::Application
#[derive(
    Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Default, Clone,
)]
pub struct Profileable {
    /// Specifies whether the user of the device can profile this application
    /// through local debugging tools. These include
    ///
    /// * [`android.os.Trace`] tracing APIs (Android 11 and lower)
    /// * [`simpleperf`]
    /// * [`am profile commands`]
    /// * [`perfetto profilers`] (native memory, Java memory, CPU).
    ///
    /// If this isn't set or is set to false, these tools and APIs will work
    /// only when an app is debuggable. Debuggable apps incur significant and
    /// varied performance degradation, and are not useful for measuring timing
    /// accurately. This element is strongly recommended for local performance
    /// measurements, in order to capture accurate results.
    ///
    /// [`android.os.Trace`]: https://developer.android.com/reference/kotlin/android/os/Trace
    /// [`simpleperf`]: https://developer.android.com/ndk/guides/simpleperf
    /// [`am profile commands`]: https://developer.android.com/studio/command-line/perfetto
    #[yaserde(attribute, prefix = "android")]
    pub shell: VarOrBool,
    /// Specifies whether the application can be profiled by system services or
    /// shell tools (for the latter, you must also set [`android:shell`]). If
    /// false, the application cannot be profiled at all. Defaults to true. This
    /// attribute was added in API level 30.
    ///
    /// [`android:shell`]: https://developer.android.com/guide/topics/manifest/profileable-element#shell
    #[yaserde(attribute, prefix = "android")]
    pub enable: VarOrBool,
}
