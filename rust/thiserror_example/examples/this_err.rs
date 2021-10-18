fn main() {
}

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the daat for key `{0}` is not avalible"))]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?}")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}
