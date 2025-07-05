fn main() {
    // Print environment info during build
    println!("cargo:warning=Building for target: {}", std::env::var("TARGET").unwrap_or_default());
    println!("cargo:warning=Host: {}", std::env::var("HOST").unwrap_or_default());
    println!("cargo:warning=CC: {}", std::env::var("CC").unwrap_or_default());
    println!("cargo:warning=CXX: {}", std::env::var("CXX").unwrap_or_default());
}