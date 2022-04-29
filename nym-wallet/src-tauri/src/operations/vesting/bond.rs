use crate::error::BackendError;
use crate::nymd_client;
use crate::state::State;
use crate::{Gateway, MixNode};

use nym_types::currency::MajorCurrencyAmount;
use std::sync::Arc;
use tokio::sync::RwLock;
use validator_client::nymd::VestingSigningClient;

#[tauri::command]
pub async fn vesting_bond_gateway(
  gateway: Gateway,
  pledge: MajorCurrencyAmount,
  owner_signature: String,
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<(), BackendError> {
  let pledge = pledge.into_cosmwasm_coin()?;
  nymd_client!(state)
    .vesting_bond_gateway(gateway, &owner_signature, pledge)
    .await?;
  Ok(())
}

#[tauri::command]
pub async fn vesting_unbond_gateway(
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<(), BackendError> {
  nymd_client!(state).vesting_unbond_gateway().await?;
  Ok(())
}

#[tauri::command]
pub async fn vesting_unbond_mixnode(
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<(), BackendError> {
  nymd_client!(state).vesting_unbond_mixnode().await?;
  Ok(())
}

#[tauri::command]
pub async fn vesting_bond_mixnode(
  mixnode: MixNode,
  owner_signature: String,
  pledge: MajorCurrencyAmount,
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<(), BackendError> {
  let pledge = pledge.into_cosmwasm_coin()?;
  nymd_client!(state)
    .vesting_bond_mixnode(mixnode, &owner_signature, pledge)
    .await?;
  Ok(())
}

#[tauri::command]
pub async fn withdraw_vested_coins(
  amount: MajorCurrencyAmount,
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<(), BackendError> {
  let amount = amount.into_cosmwasm_coin()?;
  nymd_client!(state).withdraw_vested_coins(amount).await?;
  Ok(())
}

#[tauri::command]
pub async fn vesting_update_mixnode(
  profit_margin_percent: u8,
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<(), BackendError> {
  nymd_client!(state)
    .vesting_update_mixnode_config(profit_margin_percent)
    .await?;
  Ok(())
}
