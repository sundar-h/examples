mod split;

fn main() {
    let bp = BasePlus {
        base: Base {
            name: "Cat".to_string(),
        },
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

struct BasePlus<T> {
    pub base: T,
}

impl<T: Echo> Echo for BasePlus<T> {
    fn echo(&self) -> String {
        println!("BasePlus");
        self.base.echo()
    }
}
