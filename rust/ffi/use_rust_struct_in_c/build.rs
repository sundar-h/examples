extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/shared.c")
        .compile("shared.so");
}
