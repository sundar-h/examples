// build.rs, in the project root folder
// fn main() {
//     println!("cargo:rustc-link-search=all=src");      // works like "rustc -L src ..."
//     println!("cargo:rustc-link-lib=dylib=doubler.o"); // works like "rustc -l doubler.o"
// }

// update build.rs file as:
// extern crate cc;
//
// fn main() {
//     cc::Build::new()
//         .file("shared/common/shared.c")
//         .compile("libshared.so");
// }


fn main() {
}