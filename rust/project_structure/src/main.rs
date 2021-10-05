extern crate project_structure;

fn main() {
    // 将执行放到lib.rs中 是代码更容易测试
    project_structure::greetings::hello();
}
