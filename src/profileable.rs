use serde::{Deserialize, Serialize};

/// Specifies how the following tools can access profiling information:
///
/// * [`android.os.Trace`] tracing APIs
/// * [`simpleperf`]
/// * [`am profile`] commands
///
/// Unless this element is declared, and the [`<shell>`] tag set to true,
/// the above tools and APIs will work only when an app is [`debuggable`].
/// Debuggable apps incur significant and varied performance degradation, 
/// and are not useful for measuring timing accurately.This element is strongly 
/// recommended for local performance measurements, in order to capture accurate results.
///
/// ## XML Syntax
/// ```xml
/// <profileable android:shell=["true" | "false"] />
/// ``` 
///
/// ## Contained in
/// * [`<application>`]
///
/// ## Introduced in
/// API Level 29
///
/// [`android.os.Trace`]: https://developer.android.com/reference/kotlin/android/os/Trace
/// [`simpleperf`]: https://developer.android.com/ndk/guides/simpleperf
/// [`am profile`]: https://developer.android.com/studio/command-line/adb#am
/// [`<application>`]: crate::Application
/// [`<shell>`]: https://developer.android.com/guide/topics/manifest/profileable-element#shell
/// [`debuggable`]: https://developer.android.com/guide/topics/manifest/application-element#debug
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Default)]
pub struct Profileable {
    /// Indicates whether this application may be profiled by the shell process.
    #[yaserde(attribute, prefix = "android")]
    pub shell: bool,
}