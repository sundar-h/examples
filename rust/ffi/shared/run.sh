#!/bin/bash

go build -o ../libgo.so -buildmode=c-shared main.go
bindgen libgomqtt.h -o ../src/ffi.rs
