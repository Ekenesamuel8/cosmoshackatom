use schemars::JsonSchema; // For JSON schema generation.
use serde::{Deserialize, Serialize}; // For serialization and deserialization.

// InstantiateMsg is used when initializing the contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: String, // Address of the contract admin
}

// ExecuteMsg defines actions users can take on the contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    JoinGroup { group_id: String },  // Join a group by ID
    Contribute { group_id: String }, // Contribute to a group
    Payout { group_id: String },     // Request payout for a group
    LeaveGroup { group_id: String }, // Leave a group
}

// QueryMsg defines queries users can make to retrieve data.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    GetGroup { group_id: String },         // Fetch group details by ID
    GetContributions { group_id: String }, // Fetch contributions for a group
}
