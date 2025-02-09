use crate::msg::QueryMsg;
use crate::state::{CONTRIBUTIONS, GROUPS};
use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdResult};

pub fn handle_query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetGroup { group_id } => {
            let group = GROUPS.load(deps.storage, &group_id)?;
            to_json_binary(&group)
        }
        QueryMsg::GetContributions { group_id: _ } => {
            let contributions = CONTRIBUTIONS
                .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
                .filter_map(|res| res.ok())
                .collect::<Vec<_>>();
            to_json_binary(&contributions)
        }
    }
}
