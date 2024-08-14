use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::{ToyCustomization, ToyCustomizations, RepairHistoryResponse, EnhancementResponse};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    AddAccessory {
        toy: String,
        accessory: String,
    },
    ChangeAppearance {
        toy: String,
        appearance: String,
    },
    UpgradeToy {
        toy: String,
        upgrade: String,
    },
    RepairToy {
        toy: String,
    },
    EnhanceToy {
        toy: String,
        enhancement: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ToyCustomization)]
    GetToyCustomization {
        toy: String,
    },
    #[returns(ToyCustomizations)]
    ListCustomizations {
        limit: Option<u32>,
    },
    #[returns(RepairHistoryResponse)]
    GetRepairHistory {
        toy: String,
    },
    #[returns(EnhancementResponse)]
    GetEnhancements {
        toy: String,
    },
}
