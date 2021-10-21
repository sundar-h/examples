#[derive(thiserror::Error, Debug)]
pub enum PluginError {
    #[error("load plugin")]
    Load(#[from] libloading::Error),
    #[error("convent ffi type")]
    StringToC(#[from] std::ffi::NulError),
    #[error("received string from C")]
    StringFromC(#[from] std::str::Utf8Error),
    #[error("call extern function error: {0}")]
    Return(String),
}
