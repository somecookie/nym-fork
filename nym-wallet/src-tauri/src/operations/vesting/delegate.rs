use crate::error::BackendError;
use crate::nymd_client;
use crate::state::State;
use nym_types::currency::MajorCurrencyAmount;
use nym_types::delegation::{from_contract_delegation_events, DelegationEvent, DelegationResult};
use std::sync::Arc;
use tokio::sync::RwLock;
use validator_client::nymd::VestingSigningClient;

#[tauri::command]
pub async fn get_pending_vesting_delegation_events(
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<Vec<DelegationEvent>, BackendError> {
  let guard = state.read().await;
  let client = &guard.current_client()?.nymd;
  let vesting_contract = client.vesting_contract_address()?;

  let events = client
    .get_pending_delegation_events(
      client.address().to_string(),
      Some(vesting_contract.to_string()),
    )
    .await?;
  match from_contract_delegation_events(events) {
    Ok(res) => Ok(res),
    Err(e) => Err(e.into()),
  }
}

#[tauri::command]
pub async fn vesting_delegate_to_mixnode(
  identity: &str,
  amount: MajorCurrencyAmount,
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<DelegationResult, BackendError> {
  let delegation = amount.clone().into_minor_cosmwasm_coin()?;
  nymd_client!(state)
    .vesting_delegate_to_mixnode(identity, &delegation)
    .await?;
  Ok(DelegationResult::new(
    nymd_client!(state).address().as_ref(),
    identity,
    Some(amount),
  ))
}

#[tauri::command]
pub async fn vesting_undelegate_from_mixnode(
  identity: &str,
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<DelegationResult, BackendError> {
  nymd_client!(state)
    .vesting_undelegate_from_mixnode(identity)
    .await?;
  Ok(DelegationResult::new(
    nymd_client!(state).address().as_ref(),
    identity,
    None,
  ))
}
