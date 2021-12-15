const fn bigger(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }
    b
}

const BIGGEST: i32 = bigger(3, 5);

fn main() {
    println!("{} = {}", stringify!(biggest), BIGGEST);
}
