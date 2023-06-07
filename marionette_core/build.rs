use rustc_version::{version};

fn main() {
    let version = version().unwrap();
    println!("cargo:rustc-env=RUSTC_VERSION={}", version);
}