use std::{marker::{PhantomPinned}, pin::Pin};

fn main() {
    // run_test()
    // run_pin_test();
    run_pin_boxed_test();
}

// 自引用结构
#[derive(Debug)]
struct Test {
    // a和b指向的都是同一个对上的数据
    // 但是b存储的是栈上变量a的地址
    a: String,
    b: *const String,
}

impl Test {
    fn new(txt: &str) -> Self {
        Self {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        self.b = &self.a as *const String;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        unsafe { &*(self.b) }
    }
}

// impl Test1<'a> {
//     a: String,
//     b: &'a String,
// }

// impl Test1<'a> {
//     fn new(txt: &str) -> Self {
//         Self {
//             a: txt.to_string(),
//         }
//     }
// }

fn run_test() {
    let mut test1 = Test::new("test1");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();

    println!("a: {}, b: {}", test1.a(), test1.b());
    println!("a: {}, b: {}", test2.a(), test2.b());

    // 交换存储位置
    std::mem::swap(&mut test1, &mut test2);
    // 或者 直接变更a的值
    test1.a = "changed a".to_string();

    // 堆上的数据交换了，所以a交换;
    // 栈上的数据交换了, 栈上存储的对地址 交换， 由于对内容已经交换，但是对地址不变，所以 负负得正 不变
    // a: test1, b: test1
    // a: test2, b: test2
    // a: changed a, b: test1
    // a: test1, b: changed a
    // 这是 结构体内部的 a 和 b 就没有关系了; b不在指向同一个结构体内a的地址
    println!("a: {}, b: {}", test1.a(), test1.b());
    println!("a: {}, b: {}", test2.a(), test2.b());
}

#[derive(Debug)]
struct PinTest {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl PinTest {
    fn new(txt: &str) -> Self {
        PinTest {
            a: txt.to_string(),
            b: std::ptr::null(),
            _marker: PhantomPinned, // // This makes our type `!Unpin`
        }
    }

    fn init<'a>(self: Pin<&'a mut Self>) {
        let self_ptr = &self.a as *const String;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ptr;
    }

    fn a<'a>(self: Pin<&'a Self>) -> &'a str {
        &self.get_ref().a
    }

    fn b<'a>(self: Pin<&'a Self>) -> &'a String {
        unsafe { &*(self.b) }
    }
}

fn run_pin_test() {
    let mut test1 = PinTest::new("test1");
    // Notice how we shadow `test1` to prevent it from being accessed again
    let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
    PinTest::init(test1.as_mut());

    let mut test2 = PinTest::new("test2");
    let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
    PinTest::init(test2.as_mut());

    println!(
        "a: {}, b: {}",
        PinTest::a(test1.as_ref()),
        PinTest::b(test1.as_ref())
    );
    // std::mem::swap(test1.get_mut(), test2.get_mut());
    println!(
        "a: {}, b: {}",
        PinTest::a(test2.as_ref()),
        PinTest::b(test2.as_ref())
    );
}

#[derive(Debug)]
struct PinBoxTest {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl PinBoxTest {
    fn new(txt: &str)-> Pin<Box<Self>>{
        let t = PinBoxTest{
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        };
    let mut boxed = Box::pin(t);
    let self_ptr = &boxed.as_ref().a as *const String;
    unsafe {boxed.as_mut().get_unchecked_mut().b = self_ptr};

    boxed
    }

    fn a<'a>(self: Pin<&'a Self>) -> &'a str {
        &self.get_ref().a
    }

    fn b<'a>(self: Pin<&'a Self>) -> &'a String {
        unsafe {&*(self.b)}
    }

}

fn run_pin_boxed_test() {
    let test1 = PinBoxTest::new("test1");
    let test2 = PinBoxTest::new("test2");

    println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
    println!("a: {}, b: {}", test2.as_ref().a(), test2.as_ref().b());
}