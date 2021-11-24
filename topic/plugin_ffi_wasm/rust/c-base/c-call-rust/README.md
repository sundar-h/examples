## Rust 导出共享库


## 内存泄漏检测工具 Valgrind
`brew install valgrind qcachegrind graphviz`
```bash
git clone https://github.com/LouisBrunner/valgrind-macos.git
sudo ./autogen.sh
sudo ./configure --prefix=/usr/local --enable-only64bit
sudo make
sudo make install
cargo install cargo-profiler
```

使用
```asm
brew install qcachegrind graphviz
cargo profiler callgrind --bin rust-call-c
```


## https://ports.macports.org/port/massif-visualizer/

* [pprof](https://github.com/tikv/pprof-rs)


[Rust导出共享库](https://cloud.tencent.com/developer/article/1669564)
[Rust FFI 编程](https://cloud.tencent.com/developer/search/article-Rust%20FFI%20%E7%BC%96%E7%A8%8B)
