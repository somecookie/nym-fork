use serde::{Deserialize, Serialize};

use validator_client::nymd::TxResponse;

use crate::currency::MajorCurrencyAmount;

#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/TauriTxResult.ts")
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

impl TauriTxResult {
    pub fn new(t: TxResponse, details: TransactionDetails) -> TauriTxResult {
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

#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/TransactionDetails.ts")
)]
#[derive(Deserialize, Serialize)]
pub struct TransactionDetails {
    pub amount: MajorCurrencyAmount,
    pub from_address: String,
    pub to_address: String,
}
