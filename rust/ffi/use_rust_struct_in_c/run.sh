#!/bin/bash

#Rust 库 工具 ffi_convert ???

#cargo install bindgen
bindgen src/shared.h


#cbindgen --lang c --config cbindgen.toml --crate rust_2_c -o shared.h
