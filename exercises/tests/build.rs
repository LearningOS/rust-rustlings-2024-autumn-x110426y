//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let env_var_command = format!("cargo:rustc-env=TEST_FOO={}", timestamp);
    println!("{}", env_var_command);

    let feature_command = "cargo:rustc-cfg=feature=\"pass\"";
    println!("{}", feature_command);
}





