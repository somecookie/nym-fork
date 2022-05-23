use crate::error::BackendError;
use crate::nymd_client;
use crate::state::State;
use nym_types::currency::MajorCurrencyAmount;
use nym_types::transaction::{TauriTxResult, TransactionDetails};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
use validator_client::nymd::{AccountId, CosmosCoin};

#[tauri::command]
pub async fn send(
  address: &str,
  amount: MajorCurrencyAmount,
  memo: String,
  state: tauri::State<'_, Arc<RwLock<State>>>,
) -> Result<TauriTxResult, BackendError> {
  let address = AccountId::from_str(address)?;
  let cosmos_amount: CosmosCoin = amount.clone().into_minor_cosmos_coin()?;
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
