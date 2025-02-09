use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// Import CosmWasm standard library for key functionalities like contract entry points,
// dependencie environmen messag informatio and standard results.
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::init_storage;
// Import message definitions (InstantiateMsg, ExecuteMsg, QueryMsg) from msg.rs.

use crate::execute; // Import execution logic from execute.rs.
use crate::query; // Import query logic from query.rs.
                  // Import storage initialization logic from state.rs.

// Instantiate handler: called once to initialize the contract's state
#[entry_point] // Marks this as the entry point for contract initialization
pub fn instantiate(
    deps: DepsMut, // DepsMut provides mutable access to the contract's dependencies (e.g., storage).
    _env: Env,     // Env contains blockchain-specific information (e.g., block height, chain ID).
    _info: MessageInfo, // MessageInfo contains metadata about the transaction (e.g., sender address).
    _msg: InstantiateMsg, // InstantiateMsg contains the initialization parameters.
) -> StdResult<Response> {
    init_storage(deps.storage)?; // Call storage initialization logic
    Ok(Response::new().add_attribute("action", "instantiate"))
}

// Execute handler: processes various execute messages (e.g., join group, contribute, payout)
#[entry_point]
pub fn execute(
    deps: DepsMut,     // Mutable access to dependencies
    env: Env,          // Blockchain-specific information
    info: MessageInfo, // Metadata about the transaction
    msg: ExecuteMsg,   // ExecuteMsg specifies the action requested (e.g., contribute).
) -> StdResult<Response> {
    match msg {
        // Match the specific execute messag and call the corresponding function
        ExecuteMsg::JoinGroup { group_id } => execute::join_group(deps, env, info, group_id),
        ExecuteMsg::Contribute { group_id } => execute::contribute(deps, env, info, group_id),
        ExecuteMsg::Payout { group_id } => execute::payout(deps, env, info, group_id),
        ExecuteMsg::LeaveGroup { group_id } => execute::leave_group(deps, env, info, group_id),
    }
}

// Query handler: processes query messages to fetch contract state
#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    query::handle_query(deps, env, msg) // Delegate to query logic in query.rs
}
