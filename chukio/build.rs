use std::env;
use std::path::PathBuf;

pub fn main() {
    // Add rustup to the rpath in order to properly link with the correct rustc version.
    let rustup_home = env::var("RUSTUP_HOME").unwrap();
    let toolchain = env::var("RUSTUP_TOOLCHAIN").unwrap();
    let rustc_lib: PathBuf = [&rustup_home, "toolchains", &toolchain, "lib"]
        .iter()
        .collect();

    // Apply the rpath to every target that invokes the linker (tests, examples, binaries, etc.).
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", rustc_lib.display());
}
