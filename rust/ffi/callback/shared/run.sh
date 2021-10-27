#!/bin/bash

#如果是独立的外部.h .c 文件的话, build不要加main.go 使用。目录的方式
#因为这个浪费了大量时间，晕
#rm ../src/ffi.rs
#go build -o ./libgo.so -buildmode=c-shared .
#mv libgo.so ../
#bindgen libgo.h -o ../src/ffi.rs
#

rm ../src/c_protocols.rs
bindgen ./protocols/c_protocols.c -o ../src/c_protocols.rs