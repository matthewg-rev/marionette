use rustc_version::{version};

fn main() {
    let version = version().unwrap();
    let marionette_version = env!("CARGO_PKG_VERSION");
    println!("cargo:rustc-env=RUSTC_VERSION={}", version);
    println!("cargo:rustc-env=MARIONETTE_VERSION={}", marionette_version);
}