fn main() {
    let i = 666;
    let c = || {
        println!("hello {}", i)
    };
    c();
}