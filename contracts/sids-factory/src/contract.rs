#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult, to_json_binary};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{CUSTOMIZED_TOYS, EnhancementResponse, REPAIR_HISTORY, RepairHistoryResponse, ToyCustomization, ToyCustomizations};

const CONTRACT_NAME: &str = "crates.io:sids-factory";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddAccessory { toy, accessory } => try_add_accessory(deps, toy, accessory),
        ExecuteMsg::ChangeAppearance { toy, appearance } => try_change_appearance(deps, toy, appearance),
        ExecuteMsg::UpgradeToy { toy, upgrade } => try_upgrade_toy(deps, toy, upgrade),
        ExecuteMsg::RepairToy { toy } => try_repair_toy(deps, env, toy),
        ExecuteMsg::EnhanceToy { toy, enhancement } => try_enhance_toy(deps, toy, enhancement),
    }
}

fn try_add_accessory(
    deps: DepsMut,
    toy: String,
    accessory: String,
) -> Result<Response, ContractError> {
    let toy_addr = deps.api.addr_validate(&toy)?;
    let mut toy_customization = CUSTOMIZED_TOYS.load(deps.storage, &toy_addr)?;

    toy_customization.accessories.push(accessory.clone());
    CUSTOMIZED_TOYS.save(deps.storage, &toy_addr, &toy_customization)?;

    Ok(Response::new().add_attribute("method", "add_accessory").add_attribute("accessory", accessory))
}

fn try_change_appearance(
    deps: DepsMut,
    toy: String,
    appearance: String,
) -> Result<Response, ContractError> {
    let toy_addr = deps.api.addr_validate(&toy)?;
    let mut toy_customization = CUSTOMIZED_TOYS.load(deps.storage, &toy_addr)?;

    toy_customization.appearance = Some(appearance.clone());
    CUSTOMIZED_TOYS.save(deps.storage, &toy_addr, &toy_customization)?;

    Ok(Response::new().add_attribute("method", "change_appearance").add_attribute("appearance", appearance))
}

fn try_upgrade_toy(
    deps: DepsMut,
    toy: String,
    upgrade: String,
) -> Result<Response, ContractError> {
    let toy_addr = deps.api.addr_validate(&toy)?;
    let mut toy_customization = CUSTOMIZED_TOYS.load(deps.storage, &toy_addr)?;

    toy_customization.upgrades.push(upgrade.clone());
    CUSTOMIZED_TOYS.save(deps.storage, &toy_addr, &toy_customization)?;

    Ok(Response::new().add_attribute("method", "upgrade_toy").add_attribute("upgrade", upgrade))
}

fn try_repair_toy(
    deps: DepsMut,
    env: Env,
    toy: String,
) -> Result<Response, ContractError> {
    let toy_addr = deps.api.addr_validate(&toy)?;
    let mut repair_history = REPAIR_HISTORY.load(deps.storage, &toy_addr)?;

    repair_history.push(env.block.height.to_string());
    REPAIR_HISTORY.save(deps.storage, &toy_addr, &repair_history)?;

    Ok(Response::new().add_attribute("method", "repair_toy"))
}

fn try_enhance_toy(
    deps: DepsMut,
    toy: String,
    enhancement: String,
) -> Result<Response, ContractError> {
    let toy_addr = deps.api.addr_validate(&toy)?;
    let mut toy_customization = CUSTOMIZED_TOYS.load(deps.storage, &toy_addr)?;

    toy_customization.enhancements.push(enhancement.clone());
    CUSTOMIZED_TOYS.save(deps.storage, &toy_addr, &toy_customization)?;

    Ok(Response::new().add_attribute("method", "enhance_toy").add_attribute("enhancement", enhancement))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetToyCustomization { toy } => to_json_binary(&query_get_toy_customization(deps, toy)?),
        QueryMsg::ListCustomizations { limit } => to_json_binary(&query_list_customizations(deps, limit)?),
        QueryMsg::GetRepairHistory { toy } => to_json_binary(&query_get_repair_history(deps, toy)?),
        QueryMsg::GetEnhancements { toy } => to_json_binary(&query_get_enhancements(deps, toy)?),
    }
}

fn query_get_toy_customization(
    deps: Deps,
    toy: String,
) -> StdResult<ToyCustomization> {
    CUSTOMIZED_TOYS.load(deps.storage, &deps.api.addr_validate(&toy)?)
}

fn query_list_customizations(
    deps: Deps,
    limit: Option<u32>,
) -> StdResult<ToyCustomizations> {
    let limit = limit.unwrap_or(10) as usize;
    CUSTOMIZED_TOYS
        .range(deps.storage, None, None, Order::Ascending)
        .take(limit)
        .map(|item| {
            let (addr, customization) = item?;
            Ok(ToyCustomization {
                toy: addr,
                accessories: customization.accessories,
                appearance: customization.appearance,
                upgrades: customization.upgrades,
                enhancements: customization.enhancements,
            })
        })
        .collect::<StdResult<Vec<_>>>()
}

fn query_get_repair_history(
    deps: Deps,
    toy: String,
) -> StdResult<RepairHistoryResponse> {
    let toy_addr = deps.api.addr_validate(&toy)?;
    let repair_history = REPAIR_HISTORY.load(deps.storage, &toy_addr)?;

    let response = RepairHistoryResponse { repairs: repair_history };

    Ok(response)
}

fn query_get_enhancements(
    deps: Deps,
    toy: String,
) -> StdResult<EnhancementResponse> {
    let toy_addr = deps.api.addr_validate(&toy)?;
    let toy_customization = CUSTOMIZED_TOYS.load(deps.storage, &toy_addr)?;

    Ok(toy_customization.enhancements)
}
