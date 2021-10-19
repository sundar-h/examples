struct Object {
    // 这个类型，不可改变了
    _unused: [u8; 0],
}

fn main() {
    let _unused: [u8; 0] = [];
    let mut o = Object{_unused};

    o._unused.fill(3u8);
    o._unused.fill(3u8);
    o._unused.fill(3u8);
    o._unused.fill(3u8);
    o._unused.fill(3u8);

    println!("{:?}", o._unused);
}