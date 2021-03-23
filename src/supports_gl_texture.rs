use serde::{Deserialize, Serialize};

/// Declares a single GL texture compression format that the app supports.
///
/// An application "supports" a GL texture compression format if it is capable
/// of providing texture assets that are compressed in that format, once the
/// application is installed on a device. The application can provide the
/// compressed assets locally, from inside the .apk, or it can download them
/// from a server at runtime. Each <supports-gl-texture> element declares
/// exactly one supported texture compression format, specified as the value of
/// a android:name attribute. If your application supports multiple texture
/// compression formats, you can declare multiple <supports-gl-texture>
/// elements.
///
/// ## XML Example
/// ```xml
/// <supports-gl-texture
///     android:name="GL_OES_compressed_ETC1_RGB8_texture" />
/// <supports-gl-texture
///     android:name="GL_OES_compressed_paletted_texture" />
/// ```
///
/// Declared <supports-gl-texture> elements are informational, meaning that
/// the Android system itself does not examine the elements at install time
/// to ensure matching support on the device. However, other services (such as
/// Google Play) or applications can check your application's
/// <supports-gl-texture> declarations as part of handling or interacting with
/// your application.  For this reason, it's very important that you declare all
/// of the texture compression formats (from the list below) that your
/// application is capable of supporting. Applications and devices typically
/// declare their supported GL texture compression formats using the same set of
/// well-known strings, as listed below. The set of format strings may grow over
/// time, as needed, and since the values are strings, applications are free to
/// declare other formats as needed. Assuming that the application is built with
/// SDK Platform Tools r3 or higher, filtering based on the
/// <supports-gl-texture> element is activated for all API levels.
///
/// ## Contained in:
/// [`<manifest>`](crate::Manifest)
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename = "supports-gl-texture")]
pub struct SupportsGlTexture {
    /// Specifies a single GL texture compression format supported by the
    /// application, as a descriptor string. Common descriptor values are listed
    /// in the table below. `GL_OES_compressed_ETC1_RGB8_texture` Ericsson
    /// texture compression. Specified in OpenGL ES 2.0 and available in all
    /// Android-powered devices that support OpenGL ES 2.0.
    /// `GL_OES_compressed_paletted_texture` Generic paletted texture
    /// compression. `GL_AMD_compressed_3DC_texture` ATI 3Dc texture
    /// compression. `GL_AMD_compressed_ATC_texture` ATI texture
    /// compression. Available on devices running Adreno GPU, including HTC
    /// Nexus One, Droid Incredible, EVO, and others. For widest
    /// compatibility, devices may also declare a <supports-gl-texture> element
    /// with the descriptor GL_ATI_texture_compression_atitc.
    /// `GL_EXT_texture_compression_latc` Luminance alpha texture compression.
    /// `GL_EXT_texture_compression_dxt1` S3 DXT1 texture compression. Supported
    /// on devices running Nvidia Tegra2 platform, including Motorala Xoom,
    /// Motorola Atrix, Droid Bionic, and others.
    /// `GL_EXT_texture_compression_s3tc` S3 texture compression, nonspecific to
    /// DXT variant. Supported on devices running Nvidia Tegra2 platform,
    /// including Motorala Xoom, Motorola Atrix, Droid Bionic, and others.
    /// If your application requires a specific DXT variant, declare that
    /// descriptor instead of this one. `GL_IMG_texture_compression_pvrtc`
    /// PowerVR texture compression. Available in devices running PowerVR
    /// SGX530/540 GPU, such as Motorola DROID series; Samsung Galaxy S, Nexus
    /// S, and Galaxy Tab; and others.
    #[serde(rename = "android:name")]
    pub name: Option<SupportsGlTextureName>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub enum SupportsGlTextureName {
    /// Ericsson texture compression. Specified in OpenGL ES 2.0 and available
    /// in all Android-powered devices that support OpenGL ES 2.0.
    GL_OES_compressed_ETC1_RGB8_texture,
    /// Generic paletted texture compression.
    GL_OES_compressed_paletted_texture,
    /// ATI 3Dc texture compression
    GL_AMD_compressed_3DC_texture,
    /// ATI texture compression. Available on devices running Adreno GPU,
    /// including HTC Nexus One, Droid Incredible, EVO, and others.
    /// For widest compatibility, devices may also declare a
    /// <supports-gl-texture> element with the descriptor
    /// GL_ATI_texture_compression_atitc.
    GL_AMD_compressed_ATC_texture,
    /// Luminance alpha texture compression.
    GL_EXT_texture_compression_latc,
    /// S3 DXT1 texture compression. Supported on devices running Nvidia Tegra2
    /// platform, including Motorala Xoom, Motorola Atrix, Droid Bionic, and
    /// others.
    GL_EXT_texture_compression_dxt1,
    /// S3 texture compression, nonspecific to DXT variant. Supported on devices
    /// running Nvidia Tegra2 platform, including Motorala Xoom, Motorola Atrix,
    /// Droid Bionic, and others If your application requires a specific DXT
    /// variant, declare that descriptor instead of this one.
    GL_EXT_texture_compression_s3tc,
    /// PowerVR texture compression. Available in devices running PowerVR
    /// SGX530/540 GPU, such as Motorola DROID series; Samsung Galaxy S, Nexus
    /// S, and Galaxy Tab; and others.
    GL_IMG_texture_compression_pvrtc,
}
