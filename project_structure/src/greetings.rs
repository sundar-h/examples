pub fn hello() -> String {
    // println!("hello world!");
    ("Hello, world!").to_string()
}

#[test]
fn test_hello() {
    assert_eq!(hello(), "Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::hello;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, world!");
    }
}
