use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use mixnet_contract_common::{Addr, Delegation as MixnetContractDelegation, IdentityKey};

use crate::currency::MajorCurrencyAmount;
use crate::error::TypesError;

#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(
    test,
    ts(
        export,
        export,
        export_to = "../../ts-packages/types/src/types/rust/Delegation.ts"
    )
)]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, JsonSchema)]
pub struct Delegation {
    pub owner: Addr,
    pub node_identity: IdentityKey,
    pub amount: MajorCurrencyAmount,
    pub block_height: u64,
    pub proxy: Option<Addr>, // proxy address used to delegate the funds on behalf of anouther address
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
            owner,
            node_identity,
            amount,
            block_height,
            proxy,
        })
    }
}
