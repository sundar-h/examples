trait Animal {
    fn walk(&self) {
        println!("Animal::walk");
    }
}

// 原因:
// [trait-object](https://doc.rust-lang.org/reference/types/trait-object.html)
// * 由于trait对象可以包含引用，这些引用的寿命需要作为trait对象的一部分来表达。
// 这个寿命被写成Trait + 'a. 有一些默认值，允许这个寿命通常以合理的选择来推断

// 直接这么实现的话 传参 (&dyn Animal)使用的时候会报错:
// 另一种传参方式 Box<dyn Animal>则可以
// t.eat();
// |       ^^^ lifetime `'static` required
// impl dyn Animal {
//     fn eat(&self) {
//         println!("dyn Animal::eat");
//     }
// }


// 正确的写法: 考虑到dyn trait可能包含引用
impl<'a> dyn Animal  + 'a{
    fn eat(&self) {
        println!("dyn Animal::eat");
    }
}

struct SAnimal;

impl Animal for SAnimal {
    fn walk(&self) {
        println!("SAnimal::walk");
    }
}

trait People: Animal {
    fn think(&self) {
        println!("People::think");
    }
}

struct SPeople;

impl People for SPeople {
    fn think(&self) {
        println!("SPeople::think");
    }
}

impl Animal for SPeople {
    fn walk(&self) {
        println!("SPeople::walk");
    }
}

// 这里Super trait和 Sub trait 都可以
fn call_reference(t: &dyn Animal) {
    t.walk();
    t.eat();
}

// 这里Super trait和 Sub trait 都可以
fn call_box(t: Box<dyn Animal>) {
    t.walk();
    t.eat();
}

pub fn run() {
    println!("*****************************8");
    let animal = SAnimal;
    call_reference(&animal);
    println!("************Box*****************8");
    call_box(Box::new(animal));

    println!("*****************************8");
    let people = SPeople;
    // 可以降级 当子类型使用
    call_reference(&people);
    println!("************Box*****************8");
    // impl trait 也可以降级 当子类型使用
    call_box(Box::new(people));
}