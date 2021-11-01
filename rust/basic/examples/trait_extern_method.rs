pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Int")]
    Int,
    #[error("call extern function error: {0}")]
    Return(String),
}

trait ResultExt {
    fn to_string(&self) -> String ;
}

impl<T> ResultExt for Result<T> {
    fn to_string(&self) -> String {
        String::from("Sang")
    }
}

trait ErrorExt {
    fn parse(&self) -> String ;
}

impl ErrorExt for Error {
    fn parse(&self) -> String {
        match self {
            Error::Int => "Hello World Int".to_string(),
            _ => "Other".to_string(),
        }
    }
}

fn main() {
    // println!("{}", add())
}


fn add() -> String {
    let i = Err(Error::Int);
    i?
}

