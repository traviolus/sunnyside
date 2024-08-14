use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::state::{Toy, Toys};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Mint {
        toy_name: String,
        owner: String,
    },
    Transfer {
        toy_id: u64,
        new_owner: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Toy)]
    GetToyInfo {
        toy_id: u64,
    },
    #[returns(Toys)]
    ListToysByOwner {
        owner: String,
    },
}
