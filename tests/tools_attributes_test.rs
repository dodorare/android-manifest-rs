use android_manifest::AndroidManifest;
use std::fs;

#[test]
fn test_parse_manifest_with_tools_attributes() {
    let xml_content = fs::read_to_string("tests/test_manifest_with_tools.xml")
        .expect("Failed to read test manifest file");
    
    let manifest: AndroidManifest = yaserde::de::from_str(&xml_content)
        .expect("Failed to parse manifest");
    
    // Test manifest level tools attributes
    assert_eq!(manifest.package, Some("com.example.testapp".to_string()));
    assert_eq!(manifest.ignore, Some("MissingVersion".to_string()));
    assert_eq!(manifest.locale, Some("en".to_string()));
    
    // Test application tools attributes
    let app = &manifest.application;
    assert_eq!(app.name, Some(".MyApplication".to_string()));
    assert_eq!(app.replace, Some("android:icon,android:theme".to_string()));
    assert_eq!(app.ignore, Some("GoogleAppIndexingWarning".to_string()));
    
    // Test activity tools attributes
    let main_activity = app.activity.iter()
        .find(|a| a.name == ".MainActivity")
        .expect("MainActivity not found");
    
    assert_eq!(main_activity.replace, Some("android:theme,android:exported".to_string()));
    assert_eq!(main_activity.node, Some("replace".to_string()));
    assert_eq!(main_activity.exported, Some(true.into()));
    
    let second_activity = app.activity.iter()
        .find(|a| a.name == ".SecondActivity")
        .expect("SecondActivity not found");
    
    assert_eq!(second_activity.remove, Some("android:windowSoftInputMode".to_string()));
    assert_eq!(second_activity.target_api, Some("21".to_string()));
    
    // Test service tools attributes
    let service = app.service.iter()
        .find(|s| s.name == ".MyService")
        .expect("MyService not found");
    
    assert_eq!(service.node, Some("merge".to_string()));
    assert_eq!(service.ignore, Some("ExportedService".to_string()));
    
    // Test receiver tools attributes
    let receiver = app.receiver.iter()
        .find(|r| r.name == ".MyReceiver")
        .expect("MyReceiver not found");
    
    assert_eq!(receiver.replace, Some("android:exported".to_string()));
    assert_eq!(receiver.selector, Some("com.example.lib1".to_string()));
    
    // Test provider tools attributes
    let provider = app.provider.iter()
        .find(|p| p.name == ".MyProvider")
        .expect("MyProvider not found");
    
    assert_eq!(provider.node, Some("strict".to_string()));
    assert_eq!(provider.strict, Some("android:authorities".to_string()));
    
    println!("✅ All tools attributes parsed successfully!");
}
