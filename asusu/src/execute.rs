use crate::state::{CONTRIBUTIONS, GROUPS};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};

pub fn join_group(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    group_id: String,
) -> StdResult<Response> {
    let mut group = GROUPS.load(deps.storage, &group_id)?;
    group.members.push(info.sender.clone());

    GROUPS.save(deps.storage, &group_id, &group)?;

    Ok(Response::new()
        .add_attribute("method", "join_group")
        .add_attribute("group_id", group_id))
}

pub fn contribute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    group_id: String,
) -> StdResult<Response> {
    let funds = info.funds.first().map(|c| c.amount.u128()).unwrap_or(0);
    let key = format!("{}_{}", group_id, info.sender);

    let mut contribution = CONTRIBUTIONS.may_load(deps.storage, &key)?.unwrap_or(0);
    contribution += funds;

    CONTRIBUTIONS.save(deps.storage, &key, &contribution)?;

    Ok(Response::new()
        .add_attribute("method", "contribute")
        .add_attribute("group_id", group_id)
        .add_attribute("amount", funds.to_string()))
}

// Implement payout and leave_group functions similarly

pub fn payout(
    _deps: DepsMut,     // Mutable dependencies
    _env: Env,          // Blockchain environment
    _info: MessageInfo, // Transaction metadata
    _group_id: String,  // ID of the group to request payout
) -> StdResult<Response> {
    // Logic for requesting payout
    Ok(Response::new().add_attribute("action", "payout"))
}

pub fn leave_group(
    _deps: DepsMut,     // Mutable dependencies
    _env: Env,          // Blockchain environment
    _info: MessageInfo, // Transaction metadata
    _group_id: String,  // ID of the group to leave
) -> StdResult<Response> {
    // Logic for leaving a group
    Ok(Response::new().add_attribute("action", "leave_group"))
}
