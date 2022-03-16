#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:{{project-name}}";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate() -> Result<Response, ContractError> {
    {};
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute() -> Result<Response, ContractError> {
    {};
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query() -> StdResult<Binary> {
    {};
}

