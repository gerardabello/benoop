use reqwest::StatusCode;

use crate::config::RequestConfig;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error ({op:}): {source:}")]
    Io {
        source: std::io::Error,
        op: &'static str,
    },
    #[error("unexpected status code: {status_code:}")]
    UnexpectedStatusCode {
        status_code: StatusCode,
        expected: Vec<StatusCode>,
        request: RequestConfig,
    },

    #[error("request error: {source:}")]
    RequestError {
        request: RequestConfig,
        source: reqwest::Error,
    },

    #[error("No config file found. Run 'init' to create one.")]
    MissingDefaultConfigFile,

    #[error("Configuration file already exists")]
    ConfigFileAlreadyExists,

    #[error("No config file found at \"{0}\"")]
    CannotFindConfigFile(String),

    #[error("Error parsing config file (\"{0}\"): {1:}")]
    CannotParseConfig(String, serde_yaml::Error),

    #[error("Error parsing baseline file (\"{0}\"): {1:}")]
    CannotParseBaseline(String, bincode::Error),
}
