#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdError, StdResult, to_json_binary};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{PLAY_SCENARIOS, PlayScenario, PlayScenarios};

const CONTRACT_NAME: &str = "crates.io:andys-room";
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
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddToyToPlayScenario { scenario_name, toy_id } => {
            execute_add_toy_to_play_scenario(deps, scenario_name, toy_id)
        }
        ExecuteMsg::RemoveToyFromScenario { scenario_name, toy_id } => {
            execute_remove_toy_from_scenario(deps, scenario_name, toy_id)
        }
        ExecuteMsg::CreatePlayScenario { name } => {
            execute_create_play_scenario(deps, name)
        }
        ExecuteMsg::DeletePlayScenario { name } => {
            execute_delete_play_scenario(deps, name)
        }
    }
}

fn execute_add_toy_to_play_scenario(
    deps: DepsMut,
    scenario_name: String,
    toy_id: u64,
) -> Result<Response, ContractError> {
    PLAY_SCENARIOS.update(deps.storage, scenario_name.clone(), |scenario| {
        match scenario {
            Some(mut scenario) => {
                if !scenario.toys.contains(&toy_id) {
                    scenario.toys.push(toy_id);
                }
                Ok(scenario)
            }
            None => Err(ContractError::Std(StdError::generic_err("Scenario not found"))),
        }
    })?;

    Ok(Response::new().add_attribute("method", "add_toy_to_play_scenario").add_attribute("scenario", scenario_name))
}

fn execute_remove_toy_from_scenario(
    deps: DepsMut,
    scenario_name: String,
    toy_id: u64,
) -> Result<Response, ContractError> {
    PLAY_SCENARIOS.update(deps.storage, scenario_name.clone(), |scenario| {
        match scenario {
            Some(mut scenario) => {
                scenario.toys.retain(|&id| id != toy_id);
                Ok(scenario)
            }
            None => Err(ContractError::Std(StdError::generic_err("Scenario not found"))),
        }
    })?;

    Ok(Response::new().add_attribute("method", "remove_toy_from_scenario").add_attribute("scenario", scenario_name))
}

fn execute_create_play_scenario(
    deps: DepsMut,
    name: String,
) -> Result<Response, ContractError> {
    let scenario = PlayScenario {
        name: name.clone(),
        toys: Vec::new(),
    };

    PLAY_SCENARIOS.save(deps.storage, name.clone(), &scenario)?;

    Ok(Response::new().add_attribute("method", "create_play_scenario").add_attribute("scenario", name))
}

fn execute_delete_play_scenario(
    deps: DepsMut,
    name: String,
) -> Result<Response, ContractError> {
    PLAY_SCENARIOS.remove(deps.storage, name.clone());

    Ok(Response::new().add_attribute("method", "delete_play_scenario").add_attribute("scenario", name))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPlayScenario { name } => to_json_binary(&query_get_play_scenario(deps, name)?),
        QueryMsg::ListPlayScenarios {} => to_json_binary(&query_list_play_scenarios(deps)?),
    }
}

fn query_get_play_scenario(
    deps: Deps,
    name: String,
) -> StdResult<PlayScenario> {
    PLAY_SCENARIOS.load(deps.storage, name)
}

fn query_list_play_scenarios(
    deps: Deps,
) -> StdResult<PlayScenarios> {
    PLAY_SCENARIOS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| item.map(|(_, scenario)| scenario))
        .collect::<StdResult<Vec<_>>>()
}
