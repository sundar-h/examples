use std::{marker::PhantomPinned, pin::Pin};

fn main() {
    let mut test1 = PinTest::new("test1");
    let mut test1 = unsafe {
        Pin::new_unchecked(&mut test1)
    };
    PinTest::init(test1.as_mut());

    let mut test2 = PinTest::new("test2");
    let mut test2 = unsafe {
        Pin::new_unchecked(&mut test2)
    };
    PinTest::init(test2.as_mut());

    println!("a: {}, b: {}", PinTest::a(test1.as_ref()), PinTest::b(test1.as_ref()));
    // std::mem::swap(test1.get_mut(), test2.get_mut());
    println!("a: {}, b: {}", PinTest::a(test2.as_ref()), PinTest::b(test2.as_ref()));
}

#[derive(Debug)]
struct PinTest {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl PinTest {
    fn new(txt: &str) -> Self {
        Self {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        }
    }

    fn init<'a>(self: Pin<&'a mut Self>) {
        let self_ref = &self.a as *const String;
        let this = unsafe {
            self.get_unchecked_mut()
        };
        this.b = self_ref;
    }

    fn a<'a>(self: Pin<&'a Self>) -> &'a str {
        &self.get_ref().a
    }

    fn b<'a>(self: Pin<&'a Self>) -> &'a String{
        unsafe {
            &*(self.b)
        }
    }
}