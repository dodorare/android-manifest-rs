use android_manifest::{AndroidManifest, Application};

#[test]
fn test_package_optional_serialization() {
    // Test with package = None - should not appear in XML
    let manifest = AndroidManifest {
        package: None,
        application: Application::default(),
        ..Default::default()
    };
    
    let xml = yaserde::ser::to_string(&manifest).expect("Failed to serialize");
    println!("XML without package:\n{}", xml);
    
    assert!(!xml.contains("package="), 
            "package attribute should not appear when None");
    
    // Test with package = Some(...) - should appear in XML
    let manifest2 = AndroidManifest {
        package: Some("com.example.app".to_string()),
        application: Application::default(),
        ..Default::default()
    };
    
    let xml2 = yaserde::ser::to_string(&manifest2).expect("Failed to serialize");
    println!("\nXML with package:\n{}", xml2);
    
    assert!(xml2.contains("package=\"com.example.app\""), 
            "package attribute should appear when Some");
}

#[test]
fn test_package_optional_deserialization() {
    // Test parsing XML without package attribute
    let xml_without = r#"<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android">
    <application />
</manifest>"#;
    
    let manifest: AndroidManifest = yaserde::de::from_str(xml_without)
        .expect("Failed to parse manifest without package");
    
    assert_eq!(manifest.package, None, "package should be None when not present in XML");
    
    // Test parsing XML with package attribute
    let xml_with = r#"<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
          package="com.example.test">
    <application />
</manifest>"#;
    
    let manifest2: AndroidManifest = yaserde::de::from_str(xml_with)
        .expect("Failed to parse manifest with package");
    
    assert_eq!(manifest2.package, Some("com.example.test".to_string()), 
               "package should be Some when present in XML");
}
