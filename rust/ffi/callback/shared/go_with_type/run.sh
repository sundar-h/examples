#!/bin/bash

rm ../c_protocols.h ../c_protocols.so
go build -o ../c_protocols.so -buildmode=c-shared main.go
bindgen ../c_protocols.h -o ../../src/c_protocols.rs
