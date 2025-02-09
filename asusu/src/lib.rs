// Declare the submodules for the contract logic
pub mod contract; // Handles the main contract logic (instantiate, execute, query)
pub mod error; // Defines custom error handling
pub mod execute; // Handles execution logic (e.g., join group, contribute)
pub mod msg; // Defines input and output messages
pub mod query;
pub mod state; // Handles data structures and storage // Handles query logic (e.g., fetch group details)

// Import the custom error handler to make it accessible from other files
pub use crate::error::ContractError;
