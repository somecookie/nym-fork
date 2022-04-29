use crate::error::BackendError;
use crate::nymd_client;
use crate::state::State;
use crate::{Gateway, MixNode};
use nym_types::currency::MajorCurrencyAmount;
use nym_types::gateway::GatewayBond;
use nym_types::mixnode::MixNodeBond;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tauri::command]
pub async fn bond_gateway(
  gateway: Gateway,
  pledge: MajorCurrencyAmount,
  owner_signature: String,
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<(), BackendError> {
  let pledge = pledge.into_cosmwasm_coin()?;
  nymd_client!(state)
    .bond_gateway(gateway, owner_signature, pledge)
    .await?;
  Ok(())
}

#[tauri::command]
pub async fn unbond_gateway(
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<(), BackendError> {
  nymd_client!(state).unbond_gateway().await?;
  Ok(())
}

#[tauri::command]
pub async fn unbond_mixnode(
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<(), BackendError> {
  nymd_client!(state).unbond_mixnode().await?;
  Ok(())
}

#[tauri::command]
pub async fn bond_mixnode(
  mixnode: MixNode,
  owner_signature: String,
  pledge: MajorCurrencyAmount,
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<(), BackendError> {
  let pledge = pledge.into_cosmwasm_coin()?;
  nymd_client!(state)
    .bond_mixnode(mixnode, owner_signature, pledge)
    .await?;
  Ok(())
}

#[tauri::command]
pub async fn update_mixnode(
  profit_margin_percent: u8,
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<(), BackendError> {
  nymd_client!(state)
    .update_mixnode_config(profit_margin_percent)
    .await?;
  Ok(())
}

#[tauri::command]
pub async fn mixnode_bond_details(
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<Option<MixNodeBond>, BackendError> {
  let guard = state.read().await;
  let client = guard.current_client()?;
  let bond = client.nymd.owns_mixnode(client.nymd.address()).await?;
  let res = MixNodeBond::from_mixnet_contract_mixnode_bond(bond)?;
  Ok(res)
}

#[tauri::command]
pub async fn gateway_bond_details(
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<Option<GatewayBond>, BackendError> {
  let guard = state.read().await;
  let client = guard.current_client()?;
  let bond = client.nymd.owns_gateway(client.nymd.address()).await?;
  let res = GatewayBond::from_mixnet_contract_gateway_bond(bond)?;
  Ok(res)
}

#[tauri::command]
pub async fn get_operator_rewards(
  address: String,
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<MajorCurrencyAmount, BackendError> {
  let denom = state.read().await.current_network().denom();
  let rewards_as_minor = nymd_client!(state).get_operator_rewards(address).await?;
  let amount: MajorCurrencyAmount =
    MajorCurrencyAmount::from_minor_uint128_and_denom(rewards_as_minor, &denom.to_string())?;
  Ok(amount)
}
