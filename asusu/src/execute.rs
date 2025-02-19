use cosmwasm_std::{StdError, StdResult, Response, MessageInfo, Env, DepsMut};
use crate::state::{GROUPS, CONTRIBUTIONS};
use crate::error::ContractError;

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
    let funds: u128 = info.funds.first().map(|c| c.amount.u128()).unwrap_or(0);
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
    deps: DepsMut,     // Mutable dependencies
    _env: Env,          // Blockchain environment
    info: MessageInfo, // Transaction metadata
    group_id: String,  // ID of the group to request payout
) -> StdResult<Response> {
    // Load the group from storage.
    let group = GROUPS.load(deps.storage, &group_id)
        .map_err(|_| StdError::not_found("Group"))?;
    
    // Ensure that only the group creator (admin) can trigger a payout.
    if info.sender != group.creator {
        return Err(ContractError::Unauthorized {}.into());
    }
    
    // Calculate total contributions for the group.
    let mut total_contributions: u128 = 0;
    for member in &group.members {
        // Our key is constructed as "groupid_memberaddress"
        let key = format!("{}_{}", group_id, member);
        total_contributions += CONTRIBUTIONS.may_load(deps.storage, &key)?.unwrap_or(0);
    }
    
    // Check if there are members to distribute funds to.
    let member_count = group.members.len();
    if member_count == 0 {
        return Err(StdError::generic_err("No members in the group"));
    }
    
    // Calculate equal payout per member.
    let payout_amount = total_contributions / (member_count as u128);
    
    // In a real contract, you would create a BankMsg to transfer coins.
    // Here, we simulate payout by returning a response with attributes.
    Ok(Response::new()
        .add_attribute("action", "payout")
        .add_attribute("group_id", group_id)
        .add_attribute("total_contributions", total_contributions.to_string())
        .add_attribute("payout_per_member", payout_amount.to_string()))

}

pub fn leave_group(
    deps: DepsMut,     // Mutable dependencies
    _env: Env,          // Blockchain environment
    info: MessageInfo, // Transaction metadata
    group_id: String,  // ID of the group to leave
) -> StdResult<Response> {
    // Load the group from storage.
    let mut group = GROUPS.load(deps.storage, &group_id)
        .map_err(|_| StdError::not_found("Group"))?;
    
    // Check if the sender is actually a member of the group.
    if !group.members.contains(&info.sender) {
        return Err(StdError::generic_err("You are not a member of this group")).into();
    }
    
    // Remove the sender from the group members.
    group.members.retain(|member| member != &info.sender);
    
    // Save the updated group back to storage.
    GROUPS.save(deps.storage, &group_id, &group)?;
    
    // Remove the member's contributions using the key format "groupid_sender"
    let key = format!("{}_{}", group_id, info.sender);
    CONTRIBUTIONS.remove(deps.storage, &key);
    
    Ok(Response::new()
        .add_attribute("action", "leave_group")
        .add_attribute("group_id", group_id)
        .add_attribute("member", info.sender))

}
