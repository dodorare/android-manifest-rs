use super::intent_filter::IntentFilter;
use super::meta_data::MetaData;
use super::resources::{DrawableResource, Resource, StringResource, StringResourceOrString};
use serde::{Deserialize, Serialize};

/// Declares a service (a `Service` subclass) as one of the application's
/// components. Unlike activities, services lack a visual user interface.
/// They're used to implement long-running background operations or a rich
/// communications API that can be called by other applications. All services
/// must be represented by `<service>` elements in the manifest file. Any that
/// are not declared there will not be seen by the system and will never be run.
/// Note: On Android 8.0 (API level 26) and higher, the system places
/// limitations on what your app can do while it's running in the background.
/// For more information, see the guides that discuss background execution
/// limits and background location limits.
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "service")]
pub struct Service {
    /// A string that describes the service to users. The label should be set as
    /// a reference to a string resource, so that it can be localized like other
    /// strings in the user interface.
    #[serde(rename = "android:description")]
    pub description: Option<Resource<StringResource>>,
    /// Whether or not the service is direct-boot aware; that is, whether or not
    /// it can run before the user unlocks the device. Note: During Direct
    /// Boot, a service in your application can only access the data that is
    /// stored in device protected storage. The default value is `"false"`.
    #[serde(rename = "android:directBootAware")]
    pub direct_boot_aware: Option<bool>,
    /// Whether or not the service can be instantiated by the system — `"true"`
    /// if it can be, and `"false"` if not. The default value is `"true"`.
    /// The `<application>` element has its own enabled attribute that applies
    /// to all application components, including services.
    /// The `<application>` and <service> attributes must both be `"true"` (as
    /// they both are by default) for the service to be enabled. If either is
    /// `"false"`, the service is disabled; it cannot be instantiated.
    #[serde(rename = "android:enabled")]
    pub enabled: Option<bool>,
    /// Whether or not components of other applications can invoke the service
    /// or interact with it — `"true"` if they can, and `"false"` if not.
    /// When the value is "false", only components of the same application or
    /// applications with the same user ID can start the service or bind to it.
    /// The default value depends on whether the service contains intent
    /// filters. The absence of any filters means that it can be invoked only by
    /// specifying its exact class name. This implies that the service is
    /// intended only for application-internal use (since others would not know
    /// the class name).So in this case, the default value is "false".
    /// On the other hand, the presence of at least one filter implies that the
    /// service is intended for external use, so the default value is "true".
    /// This attribute is not the only way to limit the exposure of a service to
    /// other applications. You can also use a permission to limit the
    /// external entities that can interact with the service (see the
    /// `permission` attribute).
    #[serde(rename = "android:exported")]
    pub exported: Option<bool>,
    /// Specify that the service is a `foreground service` that satisfies a
    /// particular use case. For example, a foreground service type of
    /// "location" indicates that an app is getting the device's current
    /// location, usually to `continue a user-initiated action` related to
    /// device location. You can assign multiple foreground service types to
    /// a particular service.
    #[serde(rename = "android:foregroundServiceType")]
    pub foreground_service_type: Option<ForegroundServiceType>,
    /// An icon representing the service. This attribute must be set as a
    /// reference to a drawable resource containing the image definition. If
    /// it is not set, the icon specified for the application as a whole is used
    /// instead (see the <application> element's icon attribute).
    /// The service's icon — whether set here or by the `<application>` element
    /// — is also the default icon for all the service's intent filters (see the
    /// `<intent-filter>` element's icon attribute).
    #[serde(rename = "android:icon")]
    pub icon: Option<Resource<DrawableResource>>,
    /// If set to true, this service will run under a special process that is
    /// isolated from the rest of the system and has no permissions of its own.
    /// The only communication with it is through the Service API (binding and
    /// starting).
    #[serde(rename = "android:isolatedProcess")]
    pub isolated_process: Option<bool>,
    /// A name for the service that can be displayed to users. If this attribute
    /// is not set, the label set for the application as a whole is used instead
    /// (see the <application> element's label attribute). The service's
    /// label — whether set here or by the <application> element — is also the
    /// default label for all the service's intent filters (see the
    /// <intent-filter> element's label attribute). The label should be set
    /// as a reference to a string resource, so that it can be localized like
    /// other strings in the user interface. However, as a convenience while
    /// you're developing the application, it can also be set as a raw string.
    #[serde(rename = "android:label")]
    pub label: Option<StringResourceOrString>,
    /// The name of the `Service` subclass that implements the service. This
    /// should be a fully qualified class name (such as,
    /// `"com.example.project.RoomService"`). However, as a shorthand, if
    /// the first character of the name is a period (for example,
    /// `".RoomService"`), it is appended to the package name specified in the
    /// `<manifest>` element. Once you publish your application, you should
    /// not change this name (unless you've set `android:exported="false"`).
    /// There is no default. The name must be specified.
    #[serde(rename = "android:name")]
    pub name: String,
    /// The name of a permission that an entity must have in order to launch the
    /// service or bind to it. If a caller of `startService()`, `bindService()`,
    /// or `stopService()` has not been granted this permission, the method
    /// will not work and the Intent object will not be delivered to the
    /// service. If this attribute is not set, the permission set by the
    /// `<application>` element's `permission` attribute applies to the service.
    /// If neither attribute is set, the service is not protected by a
    /// permission. For more information on permissions, see the
    /// `Permissions` section in the introduction and a separate document,
    /// `Security and Permissions`.
    #[serde(rename = "android:permission")]
    pub permission: Option<String>,
    /// The name of the process where the service is to run. Normally, all
    /// components of an application run in the default process created for the
    /// application. It has the same name as the application package. The
    /// `<application>` element's `process` attribute can set a different
    /// default for all components. But component can override the default
    /// with its own process attribute, allowing you to spread your application
    /// across multiple processes. If the name assigned to this attribute
    /// begins with a colon (':'), a new process, private to the application, is
    /// created when it's needed and the service runs in that process.
    /// If the process name begins with a lowercase character, the service will
    /// run in a global process of that name, provided that it has permission to
    /// do so. This allows components in different applications to share a
    /// process, reducing resource usage.
    #[serde(rename = "android:process")]
    pub process: Option<String>,

    pub intent_filter: Option<IntentFilter>,

    pub meta_data: Option<MetaData>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ForegroundServiceType {
    Camera,
    ConnectedDevice,
    DataSync,
    Location,
    MediaPlayback,
    MediaProjection,
    Microphone,
    PhoneCall,
}
