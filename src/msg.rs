use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, CustomMsg, Uint256};

#[cw_serde]
pub struct InstantiateMsg {
    pub job_id: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    PutVote { claims: Vec<ClaimInfo>, votes: Vec<Vote> },
    SetPaloma {},
    UpdateCompass { new_compass: String },
    UpdateBlueprint { new_blueprint: String },
}

#[cw_serde]
pub struct ClaimInfo {
    pub bot: String,
    pub min_amount: Uint256,
    pub max_amount: Uint256,
}

#[cw_serde]
pub struct Vote {
    pub gauge_address: String,
    pub user_weight: Uint256,
}

/// Message struct for cross-chain calls.
#[cw_serde]
pub struct PalomaMsg {
    /// The ID of the paloma scheduled job to run.
    pub job_id: String,
    /// The payload, ABI encoded for the target chain.
    pub payload: Binary,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetJobIdResponse)]
    GetJobId {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetJobIdResponse {
    pub job_id: String,
}

impl CustomMsg for PalomaMsg {}
