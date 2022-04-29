use crate::error::BackendError;
use crate::nymd_client;
use crate::state::State;
use nym_types::currency::MajorCurrencyAmount;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
use validator_client::nymd::{AccountId, CosmosCoin, TxResponse};

#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(
  test,
  ts(
    export,
    export_to = "../../ts-packages/types/src/types/rust/TauriTxResult.ts"
  )
)]
#[derive(Deserialize, Serialize)]
pub struct TauriTxResult {
  block_height: u64,
  code: u32,
  details: TransactionDetails,
  gas_used: u64,
  gas_wanted: u64,
  tx_hash: String,
}

#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(
  test,
  ts(
    export,
    export_to = "../../ts-packages/types/src/types/rust/TransactionDetails.ts"
  )
)]
#[derive(Deserialize, Serialize)]
pub struct TransactionDetails {
  amount: MajorCurrencyAmount,
  from_address: String,
  to_address: String,
}

impl TauriTxResult {
  fn new(t: TxResponse, details: TransactionDetails) -> TauriTxResult {
    TauriTxResult {
      block_height: t.height.value(),
      code: t.tx_result.code.value(),
      details,
      gas_used: t.tx_result.gas_used.value(),
      gas_wanted: t.tx_result.gas_wanted.value(),
      tx_hash: t.hash.to_string(),
    }
  }
}

#[tauri::command]
pub async fn send(
  address: &str,
  amount: MajorCurrencyAmount,
  memo: String,
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<TauriTxResult, BackendError> {
  let address = AccountId::from_str(address)?;
  let cosmos_amount: CosmosCoin = amount.clone().into_cosmos_coin()?;
  let result = nymd_client!(state)
    .send(&address, vec![cosmos_amount], memo)
    .await?;
  Ok(TauriTxResult::new(
    result,
    TransactionDetails {
      from_address: nymd_client!(state).address().to_string(),
      to_address: address.to_string(),
      amount,
    },
  ))
}
