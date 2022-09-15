use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError{
    #[error("invalid webpage")]
    InvalidPage,

    #[error("cannot reach URL")]
    NetworkError,

    #[error("request failed with code {0}")]
    NotOk(String)
}