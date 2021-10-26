#!/bin/bash

cbindgen --lang c --config cbindgen.toml --crate rust_2_c -o shared.h
