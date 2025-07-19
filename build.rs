fn main() {
    // Print environment info during build
    println!("cargo:warning=Building for target: {}", std::env::var("TARGET").unwrap_or_default());
    println!("cargo:warning=Host: {}", std::env::var("HOST").unwrap_or_default());
    println!("cargo:warning=CC: {}", std::env::var("CC").unwrap_or_default());
    println!("cargo:warning=CXX: {}", std::env::var("CXX").unwrap_or_default());
    println!("cargo:warning=AR: {}", std::env::var("AR").unwrap_or_default());
    println!("cargo:warning=TARGET_CC: {}", std::env::var("TARGET_CC").unwrap_or_default());
    println!("cargo:warning=CC_aarch64_unknown_linux_gnu: {}", std::env::var("CC_aarch64_unknown_linux_gnu").unwrap_or_default());
    println!("cargo:warning=CXX_aarch64_unknown_linux_gnu: {}", std::env::var("CXX_aarch64_unknown_linux_gnu").unwrap_or_default());
    println!("cargo:warning=AR_aarch64_unknown_linux_gnu: {}", std::env::var("AR_aarch64_unknown_linux_gnu").unwrap_or_default());
    println!("cargo:warning=CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER: {}", std::env::var("CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER").unwrap_or_default());
    println!("cargo:warning=CXXFLAGS: {}", std::env::var("CXXFLAGS").unwrap_or_default());
    println!("cargo:warning=CFLAGS: {}", std::env::var("CFLAGS").unwrap_or_default());
    println!("cargo:warning=HARDWARE_DESC: {}", std::env::var("HARDWARE_DESC").unwrap_or_default());
}