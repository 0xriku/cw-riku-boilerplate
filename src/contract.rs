#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{State, STATE};

const CONTRACT_NAME: &str = "crates.io:cw-riku-boilerplate";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  msg: InstantiateMsg,
) -> Result<Response, ContractError> {
  let state = State {
    data: msg.data.clone(),
    owner: info.sender.clone(),
  };
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
  STATE.save(deps.storage, &state)?;
  Ok(
    Response::new()
      .add_attribute("method", "instantiate")
      .add_attribute("owner", info.sender)
      .add_attribute("data", msg.data),
  )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  msg: ExecuteMsg,
) -> Result<Response, ContractError> {
  match msg {
    ExecuteMsg::Write { data } => set_data(deps, info, data),
  }
}

pub fn set_data(
  deps: DepsMut,
  info: MessageInfo,
  data: String,
) -> Result<Response, ContractError> {
  // Acquire mutable reference to storage to update Item located at key "state", made accessible through STATE convenience function defined in state.rs
  STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
    // Check that message sender is permitted to invoke this function
    if info.sender != state.owner {
      return Err(ContractError::NotAuthorized {});
    }
    // Update value of data to value passed in ExecuteMsg::Write
    state.data = data.clone();
    Ok(state)
  })?;
  Ok(Response::new().add_attribute("method", "set_data"))
}
