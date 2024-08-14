#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{attr, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult, to_json_binary};
use cw2::set_contract_version;

use crate::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{CONFIG, TOYS, Toy, Toys};

const CONTRACT_NAME: &str = "crates.io:roundup";
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
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint { toy_name, owner } => execute_mint(deps, info, toy_name, owner),
        ExecuteMsg::Transfer { toy_id, new_owner } => execute_transfer(deps, info, toy_id, new_owner),
    }
}

pub fn execute_mint(
    deps: DepsMut,
    _info: MessageInfo,
    toy_name: String,
    owner: String,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    let toy_id = config.next_id;
    config.next_id += 1;

    let toy = Toy {
        id: toy_id,
        name: toy_name,
        owner: deps.api.addr_validate(&owner)?,
    };

    CONFIG.save(deps.storage, &config)?;
    TOYS.save(deps.storage, toy_id, &toy)?;

    Ok(Response::new().add_attributes(vec![
        attr("method", "mint"),
        attr("toy_id", toy_id.to_string()),
    ]))
}

pub fn execute_transfer(
    deps: DepsMut,
    info: MessageInfo,
    toy_id: u64,
    new_owner: String,
) -> Result<Response, ContractError> {
    TOYS.update(deps.storage, toy_id, |toy| -> Result<Toy, ContractError> {
        match toy {
            Some(mut toy) => {
                if toy.owner != info.sender {
                    return Err(ContractError::Unauthorized {});
                }

                toy.owner = deps.api.addr_validate(&new_owner)?;
                Ok(toy)
            },
            None => Err(ContractError::NotFound {})
        }
    })?;

    Ok(Response::new().add_attribute("method", "transfer").add_attribute("toy_id", toy_id.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetToyInfo { toy_id } => to_json_binary(&query_toy_info(deps, toy_id)?),
        QueryMsg::ListToysByOwner { owner } => to_json_binary(&query_toys_by_owner(deps, owner)?),
    }
}

pub fn query_toy_info(
    deps: Deps,
    toy_id: u64,
) -> StdResult<Toy> {
    TOYS.load(deps.storage, toy_id)
}

pub fn query_toys_by_owner(
    deps: Deps,
    owner: String,
) -> StdResult<Toys> {
    let owner_addr = deps.api.addr_validate(&owner)?;

    Ok(TOYS
        .range(deps.storage, None, None, Order::Ascending)
        .filter_map(|item| match item {
            Ok((_, toy)) if toy.owner == owner_addr => Some(toy),
            _ => None,
        })
        .collect())
}
