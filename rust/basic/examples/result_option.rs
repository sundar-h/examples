
fn main() {
    let error: String = only_bad_news().into_err();
    println!("{}", error);
}

// #![feature(never_type)]

fn only_bad_news() -> Result<!, String> {
    Err("Oops, it failed".into())
}

