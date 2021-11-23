// build.rs

// 第一种方式: 命令行的方式
// use std::env;
// use std::path::Path;
// use std::process::Command;

// fn main() {
//     let out_dir = env::var("OUT_DIR").unwrap();
//
//     // Note that there are a number of downsides to this approach, the comments
//     // below detail how to improve the portability of these commands.
//     Command::new("gcc")
//         .args(&["src/echo.c", "-c", "-fPIC", "-o"])
//         .arg(&format!("{}/echo.o", out_dir))
//         .status()
//         .unwrap();
//     // 动态库
//     Command::new("gcc")
//         .args(&["-shared", "-o libecho.so", "echo.o"])
//         .current_dir(&Path::new(&out_dir))
//         .status()
//         .unwrap();
//     // 静态库
//     // Command::new("ar").args(&["crus", "libhello.a", "hello.o"])
//     //                   .current_dir(&Path::new(&out_dir))
//     //                   .status().unwrap();
//
//     println!("cargo:rustc-link-search=native={}", out_dir);
//     // println!("cargo:rustc-link-lib=static=hello");
//     println!("cargo:rerun-if-changed=src/hello.c");
// }

// 第二种方式: 使用crate简化这个过程
fn main() {
    cc::Build::new().file("src/echo.c").compile("echo");
    println!("cargo:rerun-if-changed=src/echo.c");
}
