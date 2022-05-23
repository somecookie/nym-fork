use std::sync::Arc;

use tokio::sync::RwLock;

use mixnet_contract_common::ContractStateParams;
use nym_wallet_types::admin::TauriContractStateParams;

use crate::error::BackendError;
use crate::nymd_client;
use crate::state::State;

#[tauri::command]
pub async fn get_contract_settings(
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<TauriContractStateParams, BackendError> {
  Ok(nymd_client!(state).get_contract_settings().await?.into())
}

#[tauri::command]
pub async fn update_contract_settings(
  params: TauriContractStateParams,
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<TauriContractStateParams, BackendError> {
  let mixnet_contract_settings_params: ContractStateParams = params.try_into()?;
  nymd_client!(state)
    .update_contract_settings(mixnet_contract_settings_params.clone())
    .await?;
  Ok(mixnet_contract_settings_params.into())
}
