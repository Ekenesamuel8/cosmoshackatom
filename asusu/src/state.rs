use cosmwasm_std::Addr; // Addr is used to represent blockchain addresses.
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema; // Provides JSON schema generation for better compatibility.
use serde::{Deserialize, Serialize}; // For serialization and deserialization of data structures. // Import the Map and Item types for storage.

use cosmwasm_std::{StdResult, Storage};

// This function initializes storage by setting up any default values if necessary
pub fn init_storage(storage: &mut dyn Storage) -> StdResult<()> {
    // Example: Setting up an empty state
    let default_value: u128 = 0;
    Item::new("total_contributions").save(storage, &default_value)?;

    Ok(())
}

// Define the structure to store group details
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Group {
    pub id: String,                     // Unique identifier for the group
    pub creator: Addr,                  // Address of the group creator
    pub members: Vec<Addr>,             // List of member addresses
    pub contribution_amount: u128,      // Amount each member contributes
    pub payout_cycle: u64,              // Cycle duration for payouts
    pub security_deposit: Option<u128>, // Optional security deposit amount
}

// A bucket to store group information. Each group is identified by its ID (String).
pub const GROUPS: Map<&str, Group> = Map::new("groups");
pub const CONTRIBUTIONS: Map<&str, u128> = Map::new("contributions");
