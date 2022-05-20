use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use mixnet_contract_common::Delegation as MixnetContractDelegation;

use crate::currency::MajorCurrencyAmount;
use crate::error::TypesError;

#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(
        export,
        export,
        export_to = "../../ts-packages/types/src/types/rust/Delegation.ts"
    )
)]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, JsonSchema)]
pub struct Delegation {
    pub owner: String,
    pub node_identity: String,
    pub amount: MajorCurrencyAmount,
    pub block_height: u64,
    pub proxy: Option<String>, // proxy address used to delegate the funds on behalf of anouther address
}

impl TryFrom<MixnetContractDelegation> for Delegation {
    type Error = TypesError;

    fn try_from(value: MixnetContractDelegation) -> Result<Self, Self::Error> {
        let MixnetContractDelegation {
            owner,
            node_identity,
            amount,
            block_height,
            proxy,
        } = value;

        let amount: MajorCurrencyAmount = amount.try_into()?;

        Ok(Delegation {
            owner: owner.into_string(),
            node_identity,
            amount,
            block_height,
            proxy: proxy.map(|p| p.into_string()),
        })
    }
}
