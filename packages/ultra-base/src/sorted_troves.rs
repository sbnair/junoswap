use cosmwasm_std::{Addr, Uint256};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {
    pub name: String,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Insert {
        id: String,
        nicr: Uint256,
        prev_id: String,
        next_id: String,
    },
    ReInsert {
        id: String,
        new_nicr: Uint256,
        prev_id: String,
        next_id: String,
    },
    Remove {
        id: String,
    },
    SetParams {
        size: Uint256,
        borrower_operation_address: String,
        trove_manager_address: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    GetParams {},
    GetData {},
    GetSize {},
    GetMaxSize {},
    GetFirst {},
    GetLast {},
    GetNext {
        id: String,
    },
    GetPrev {
        id: String,
    },
    FindInsertPosition {
        nicr: Uint256,
        prev_id: String,
        next_id: String,
    },
    ValidInsertPosition {
        nicr: Uint256,
        prev_id: String,
        next_id: String,
    },
    IsEmpty {},
    IsFull {},
    GetBorrowerOperationAddress {},
    GetTroveManagerAddress {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SudoMsg {
    /// Update the contract parameters
    /// Can only be called by governance
    UpdateParams {
        name: Option<String>,
        owner: Option<Addr>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ParamsResponse {
    pub name: String,
    pub owner: Addr,
}
