fn main() {
    println!("cargo:rerun-if-env-changed=TEST_FOO");
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:TEST_FOO={}", timestamp);
    
    // Set pass feature flag only when TEST_PASS is set
    if std::env::var("TEST_PASS").is_ok() {
        println!("cargo:rustc-cfg=feature=\"pass\"");
    }
}
