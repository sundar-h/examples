#[derive(thiserror::Error, Debug)]
pub enum PluginError {
    #[error("load plugin")]
    Load(#[from] std::io::Error),
}