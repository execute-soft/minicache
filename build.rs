use std::env;

fn main() {
    // Tell cargo to rerun build script if any source files change
    println!("cargo:rerun-if-changed=src/");

    // Set up conditional compilation for N-API features
    if cfg!(feature = "napi") {
        println!("cargo:rustc-cfg=napi_feature");
    }

    // Add specific flags for the target platform when building N-API
    let _target = env::var("TARGET").unwrap_or_default();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    if cfg!(feature = "napi") {
        match target_os.as_str() {
            "windows" => {
                // Windows-specific flags for better performance
                println!("cargo:rustc-link-arg=/SUBSYSTEM:CONSOLE");
            }
            "macos" => {
                // macOS-specific optimizations
                println!("cargo:rustc-link-arg=-Wl,-dead_strip");
            }
            "linux" => {
                // Linux-specific optimizations
                println!("cargo:rustc-link-arg=-Wl,--gc-sections");
            }
            _ => {}
        }

        // Architecture-specific optimizations
        match target_arch.as_str() {
            "aarch64" => {
                // ARM64 optimizations
                println!("cargo:rustc-env=TARGET_FEATURE=+neon");
            }
            "x86_64" => {
                // x86_64 optimizations
                println!("cargo:rustc-env=TARGET_FEATURE=+sse4.2");
            }
            _ => {}
        }
    }

    // Print build information
    if env::var("CARGO_FEATURE_NAPI").is_ok() {
        println!("cargo:warning=Building with N-API bindings enabled");
        println!("cargo:rustc-env=BUILD_MODE=napi");
    } else {
        println!("cargo:rustc-env=BUILD_MODE=library");
    }
}
