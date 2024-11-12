pub mod business;
use anyhow::bail;
use thiserror::Error;

//Set my own result type is base on std::result::Result
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Something went wrong :{0}")]
    AllMyWrong(#[from] anyhow::Error),
}

pub fn function_a() -> anyhow::Result<String> {
    bail!("function a error")
}
