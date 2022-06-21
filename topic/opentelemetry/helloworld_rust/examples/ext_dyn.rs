use std::rc::Rc;

fn main() {
    let bp = BasePlus {
        base: Rc::new(Base {
            name: "Cat".to_string(),
        }),
    };
    bp.echo();
}

trait Echo {
    fn echo(&self) -> String;
}

struct Base {
    name: String,
}

impl Echo for Base {
    fn echo(&self) -> String {
        println!("{:?}", self.name);
        self.name.clone()
    }
}

struct BasePlus {
    pub base: Rc<dyn Echo>,
}

impl Echo for BasePlus {
    fn echo(&self) -> String {
        println!("BasePlus");
        self.base.echo()
    }
}
