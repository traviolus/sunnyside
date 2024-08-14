use cosmwasm_schema::cw_serde;
use cw_storage_plus::Map;

#[cw_serde]
pub struct PlayScenario {
    pub name: String,
    pub toys: Vec<u64>,
}

pub type PlayScenarios = Vec<PlayScenario>;

pub const PLAY_SCENARIOS: Map<String, PlayScenario> = Map::new("play_scenarios");
