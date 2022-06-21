fn main() {
    let uri = "http://127.0.0.1:8102/helloworld.Greeter/SayHello";
    let v: Vec<&str> = uri.split('/').collect();
    println!("{:?}", v);
    println!("{:?}", v.last());
}
