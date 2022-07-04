pub mod helloworld {
    tonic::include_proto!("a.b.c.d.f");
}

use helloworld::HelloRequest;

fn main() {
    let r = HelloRequest::default();
    println!("Hello, world!");
}
