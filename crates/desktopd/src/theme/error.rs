use thiserror::Error;

#[derive(Error, Debug)]
pub enum ThemeError {
    #[error("key={key}")]
    GSettingsReadError { key: String },
    #[error("key={key} and value={value}")]
    GSettingsWriteError { key: String, value: String },
    #[error("value={0}")]
    UnknownStyleValue(String),
    #[error("value={0}")]
    UnknownAccentValue(String),
}
