#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_env, mock_info, MockStorage};
    use cosmwasm_std::{coins, from_binary};
    use cosmwasm_std::testing::mock_dependencies_with_balances;

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies_with_balances(&[("creator", &coins(1000, "atom"))]);
        let msg = InstantiateMsg { admin: "admin".into() };
        let info = mock_info("creator", &coins(1000, "atom"));

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(res.messages.len(), 0);
    }

    #[test]
    fn test_join_group() {
        let mut deps = mock_dependencies_with_balances(&[("creator", &coins(1000, "atom"))]);
        let msg = InstantiateMsg { admin: "admin".into() };
        let info = mock_info("creator", &coins(1000, "atom"));

        let _ = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let join_msg = ExecuteMsg::JoinGroup { group_id: "default".into() };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), join_msg).unwrap();
        assert_eq!(res.messages.len(), 0);
    }

    // Add more tests for contribute, payout, leave_group, and query functions
}
