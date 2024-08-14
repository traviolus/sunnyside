use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::state::{PlayScenario, PlayScenarios};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    AddToyToPlayScenario {
        scenario_name: String,
        toy_id: u64,
    },
    RemoveToyFromScenario {
        scenario_name: String,
        toy_id: u64,
    },
    CreatePlayScenario {
        name: String,
    },
    DeletePlayScenario {
        name: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(PlayScenario)]
    GetPlayScenario {
        name: String,
    },
    #[returns(PlayScenarios)]
    ListPlayScenarios {},
}
