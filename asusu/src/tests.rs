#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, StdError};

    #[test]
    fn test_join_group() {
        let mut deps = mock_dependencies();

        // Mock user and group ID
        let info = mock_info("user1", &[]);
        let group_id = "group123".to_string();

        // Store a dummy group before testing
        let group = Group { members: vec![] };
        GROUPS.save(deps.as_mut().storage, &group_id, &group).unwrap();

        // Call join_group
        let res = join_group(deps.as_mut(), mock_env(), info.clone(), group_id.clone());

        // Ensure success
        assert!(res.is_ok());
        let group = GROUPS.load(deps.as_ref().storage, &group_id).unwrap();
        assert_eq!(group.members, vec![info.sender]);
    }
}
