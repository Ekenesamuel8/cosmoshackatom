// src/error.rs
use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("Unauthorized")]
    Unauthorized {},
    // Add other custom errors as needed
}
impl From<ContractError> for StdError {
    fn from(err: ContractError) -> Self {
        StdError::generic_err(err.to_string())
    }
}
