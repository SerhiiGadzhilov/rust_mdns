extern crate cc;

#[cfg(windows)]
fn main() {
    println!("cargo:rustc-link-lib=iphlpapi");
    cc::Build::new()
        .cpp(false)
        .file("cmdns/src/mdns.c")
        .compile("libcmdns.a");
}

#[cfg(not(windows))]
fn main() {
    cc::Build::new()
        .cpp(false)
        .file("cmdns/src/mdns.c")
        .compile("libcmdns.a");
}