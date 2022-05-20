use crate::error::BackendError;
use crate::nymd_client;
use crate::state::State;
use crate::Operation;
use log::error;
use mixnet_contract_common::mixnode::DelegationEvent as ContractDelegationEvent;
use mixnet_contract_common::mixnode::PendingUndelegate as ContractPendingUndelegate;
use mixnet_contract_common::Delegation;
use nym_types::currency::CurrencyDenom;
use nym_types::currency::MajorCurrencyAmount;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[allow(non_snake_case)]
#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
  feature = "generate-ts",
  ts(export, export_to = "../../ts-packages/types/src/types/rust/AppEnv.ts")
)]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AppEnv {
  pub ADMIN_ADDRESS: Option<String>,
  pub SHOW_TERMINAL: Option<String>,
}

fn get_env_as_option(key: &str) -> Option<String> {
  match ::std::env::var(key) {
    Ok(res) => Some(res),
    Err(_e) => None,
  }
}

#[tauri::command]
pub fn get_env() -> AppEnv {
  AppEnv {
    ADMIN_ADDRESS: get_env_as_option("ADMIN_ADDRESS"),
    SHOW_TERMINAL: get_env_as_option("SHOW_TERMINAL"),
  }
}

#[tauri::command]
pub async fn owns_mixnode(
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<bool, BackendError> {
  Ok(
    nymd_client!(state)
      .owns_mixnode(nymd_client!(state).address())
      .await?
      .is_some(),
  )
}

#[tauri::command]
pub async fn owns_gateway(
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<bool, BackendError> {
  Ok(
    nymd_client!(state)
      .owns_gateway(nymd_client!(state).address())
      .await?
      .is_some(),
  )
}

// NOTE: this uses OUTDATED defaults that might have no resemblance with the reality
// as for the actual transaction, the gas cost is being simulated beforehand
#[tauri::command]
pub async fn outdated_get_approximate_fee(
  operation: Operation,
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<MajorCurrencyAmount, BackendError> {
  let approximate_fee = operation.default_fee(nymd_client!(state).gas_price());
  let denom: CurrencyDenom = state.read().await.current_network().denom().try_into()?;
  let mut total_fee = MajorCurrencyAmount::zero(&denom);
  for fee in approximate_fee.amount {
    total_fee = total_fee + MajorCurrencyAmount::from_cosmrs_coin(&fee)?;
  }
  Ok(total_fee)
}

#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
  feature = "generate-ts",
  ts(
    export,
    export_to = "../../ts-packages/types/src/types/rust/DelegationResult.ts"
  )
)]
#[derive(Serialize, Deserialize)]
pub struct DelegationResult {
  source_address: String,
  target_address: String,
  amount: Option<MajorCurrencyAmount>,
}

impl DelegationResult {
  pub fn new(
    source_address: &str,
    target_address: &str,
    amount: Option<MajorCurrencyAmount>,
  ) -> DelegationResult {
    DelegationResult {
      source_address: source_address.to_string(),
      target_address: target_address.to_string(),
      amount,
    }
  }
}

impl TryFrom<Delegation> for DelegationResult {
  type Error = BackendError;

  fn try_from(delegation: Delegation) -> Result<Self, Self::Error> {
    let amount: MajorCurrencyAmount = delegation.amount.clone().try_into()?;
    Ok(DelegationResult {
      source_address: delegation.owner().to_string(),
      target_address: delegation.node_identity(),
      amount: Some(amount),
    })
  }
}

#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
  feature = "generate-ts",
  ts(
    export,
    export_to = "../../ts-packages/types/src/types/rust/DelegationEvent.ts"
  )
)]
#[derive(Deserialize, Serialize)]
pub enum DelegationEvent {
  Delegate(DelegationResult),
  Undelegate(PendingUndelegate),
}

impl TryFrom<ContractDelegationEvent> for DelegationEvent {
  type Error = BackendError;

  fn try_from(event: ContractDelegationEvent) -> Result<Self, Self::Error> {
    match event {
      ContractDelegationEvent::Delegate(delegation) => {
        let result = DelegationEvent::Delegate(delegation.try_into()?);
        Ok(result)
      }
      ContractDelegationEvent::Undelegate(pending_undelegate) => {
        let result = DelegationEvent::Undelegate(pending_undelegate.into());
        Ok(result)
      }
    }
  }
}

#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
  feature = "generate-ts",
  ts(
    export,
    export_to = "../../ts-packages/types/src/types/rust/PendingUndelegate.ts"
  )
)]
#[derive(Deserialize, Serialize)]
pub struct PendingUndelegate {
  mix_identity: String,
  delegate: String,
  proxy: Option<String>,
  block_height: u64,
}

impl From<ContractPendingUndelegate> for PendingUndelegate {
  fn from(pending_undelegate: ContractPendingUndelegate) -> Self {
    PendingUndelegate {
      mix_identity: pending_undelegate.mix_identity(),
      delegate: pending_undelegate.delegate().to_string(),
      proxy: pending_undelegate.proxy().map(|p| p.to_string()),
      block_height: pending_undelegate.block_height(),
    }
  }
}

pub fn from_contract_delegation_events(
  events: Vec<ContractDelegationEvent>,
) -> Result<Vec<DelegationEvent>, BackendError> {
  let (events, errors): (Vec<_>, Vec<_>) = events
    .into_iter()
    .map(|delegation_event| delegation_event.try_into())
    .partition(Result::is_ok);

  if errors.is_empty() {
    let events = events
      .into_iter()
      .filter_map(|e| e.ok())
      .collect::<Vec<DelegationEvent>>();
    return Ok(events);
  }
  let errors = errors
    .into_iter()
    .filter_map(|e| e.err())
    .collect::<Vec<BackendError>>();

  error!("Failed to convert delegations: {:?}", errors);
  Err(BackendError::DelegationsInvalid)
}
