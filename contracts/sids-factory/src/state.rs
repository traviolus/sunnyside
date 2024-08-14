use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Map;

#[cw_serde]
pub struct ToyCustomization {
    pub toy: Addr,
    pub accessories: Vec<String>,
    pub appearance: Option<String>,
    pub upgrades: Vec<String>,
    pub enhancements: Vec<String>,
}

pub type ToyCustomizations = Vec<ToyCustomization>;

#[cw_serde]
pub struct RepairHistoryResponse {
    pub repairs: Vec<String>,
}

pub type EnhancementResponse = Vec<String>;

pub const CUSTOMIZED_TOYS: Map<&Addr, ToyCustomization> = Map::new("customized_toys");
pub const REPAIR_HISTORY: Map<&Addr, Vec<String>> = Map::new("repair_history");
