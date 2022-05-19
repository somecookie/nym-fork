use crate::error::BackendError;
use crate::nymd_client;
use crate::state::State;
use crate::utils::DelegationResult;
use crate::utils::{from_contract_delegation_events, DelegationEvent};
use cosmwasm_std::{Coin as CosmWasmCoin, Uint128};
use mixnet_contract_common::IdentityKey;
use nym_types::currency::{CurrencyDenom, MajorCurrencyAmount};
use nym_types::delegation::Delegation;
use nym_types::error::TypesError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(
  test,
  ts(
    export,
    export_to = "../../ts-packages/types/src/types/rust/DelegationSummaryResponse.ts"
  )
)]
#[derive(Deserialize, Serialize)]
pub struct DelegationsSummaryResponse {
  pub delegations: Vec<Delegation>,
  pub total_delegations: MajorCurrencyAmount,
  pub total_rewards: MajorCurrencyAmount,
}

#[tauri::command]
pub async fn get_pending_delegation_events(
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<Vec<DelegationEvent>, BackendError> {
  let events = nymd_client!(state)
    .get_pending_delegation_events(nymd_client!(state).address().to_string(), None)
    .await?;

  from_contract_delegation_events(events)
}

#[tauri::command]
pub async fn delegate_to_mixnode(
  identity: &str,
  amount: MajorCurrencyAmount,
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<DelegationResult, BackendError> {
  let delegation: CosmWasmCoin = amount.clone().into_minor_cosmwasm_coin()?;
  nymd_client!(state)
    .delegate_to_mixnode(identity, &delegation)
    .await?;
  Ok(DelegationResult::new(
    nymd_client!(state).address().as_ref(),
    identity,
    Some(amount),
  ))
}

#[tauri::command]
pub async fn undelegate_from_mixnode(
  identity: &str,
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<DelegationResult, BackendError> {
  nymd_client!(state)
    .remove_mixnode_delegation(identity)
    .await?;
  Ok(DelegationResult::new(
    nymd_client!(state).address().as_ref(),
    identity,
    None,
  ))
}

#[tauri::command]
pub async fn get_all_mix_delegations(
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<Vec<Delegation>, BackendError> {
  let delegations = nymd_client!(state)
    .get_delegator_delegations_paged(nymd_client!(state).address().to_string(), None, None) // get all delegations, ignoring paging
    .await?
    .delegations;

  match delegations
    .into_iter()
    .map(|d| d.try_into())
    .collect::<Result<Vec<Delegation>, TypesError>>()
  {
    Ok(res) => Ok(res),
    Err(e) => Err(e.into()),
  }
}

#[tauri::command]
pub async fn get_delegator_rewards(
  address: String,
  mix_identity: IdentityKey,
  proxy: Option<String>,
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<Uint128, BackendError> {
  Ok(
    nymd_client!(state)
      .get_delegator_rewards(address, mix_identity, proxy)
      .await?,
  )
}

#[tauri::command]
pub async fn get_delegation_summary(
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<DelegationsSummaryResponse, BackendError> {
  let address = nymd_client!(state).address().to_string();
  let vesting_contract_address: Option<String> = nymd_client!(state)
    .vesting_contract_address()
    .ok()
    .map(|v| v.to_string());

  let denom_minor = state.read().await.current_network().denom();
  let denom: CurrencyDenom = denom_minor.clone().try_into()?;

  let delegations = get_all_mix_delegations(state.clone()).await?;
  let mut total_delegations = MajorCurrencyAmount::zero(&denom);
  let mut total_rewards = MajorCurrencyAmount::zero(&denom);

  for d in delegations.clone() {
    total_delegations = total_delegations + d.amount;
    let reward = get_delegator_rewards(
      address.to_string(),
      d.node_identity,
      vesting_contract_address.clone(),
      state.clone(),
    )
    .await?;
    total_rewards = total_rewards
      + MajorCurrencyAmount::from_minor_uint128_and_denom(reward, denom_minor.as_ref())?;
  }

  Ok(DelegationsSummaryResponse {
    delegations,
    total_delegations,
    total_rewards,
  })
}
