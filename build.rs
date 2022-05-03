extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(false)
        .file("cmdns/src/mdns.c")
        .compile("libcmdns.a");
}