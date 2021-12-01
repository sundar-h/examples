#!/bin/sh

prefix=/mnt/hgfs/sanghaifa/data/examples/topic/rust-memory-leak/target/debug/build/jemalloc-sys-315785adf64e61f1/out
exec_prefix=/mnt/hgfs/sanghaifa/data/examples/topic/rust-memory-leak/target/debug/build/jemalloc-sys-315785adf64e61f1/out
libdir=${exec_prefix}/lib

LD_PRELOAD=${libdir}/libjemalloc.so.2
export LD_PRELOAD
exec "$@"
