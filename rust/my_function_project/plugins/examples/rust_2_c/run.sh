#!/bin/bash

# cargo install --force cbindgen
# https://github.com/eqrion/cbindgen

# 加入到workspace 中 
cbindgen --lang c --config cbindgen.toml --crate rust_2_c
