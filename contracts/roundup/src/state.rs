use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Toy {
    pub id: u64,
    pub name: String,
    pub owner: Addr,
}

pub type Toys = Vec<Toy>;

#[cw_serde]
pub struct Config {
    pub next_id: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const TOYS: Map<u64, Toy> = Map::new("toys");
