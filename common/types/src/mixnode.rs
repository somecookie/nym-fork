use crate::currency::MajorCurrencyAmount;
use crate::error::TypesError;
use mixnet_contract_common::{Addr, Layer, MixNode, MixNodeBond as MixnetContractMixNodeBond};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(
    test,
    ts(
        export,
        export,
        export_to = "../../ts-packages/types/src/types/rust/MixNodeBond.ts"
    )
)]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct MixNodeBond {
    pub pledge_amount: MajorCurrencyAmount,
    pub total_delegation: MajorCurrencyAmount,
    pub owner: Addr,
    pub layer: Layer,
    pub block_height: u64,
    pub mix_node: MixNode,
    pub proxy: Option<Addr>,
    pub accumulated_rewards: Option<MajorCurrencyAmount>,
}

impl MixNodeBond {
    pub fn from_mixnet_contract_mixnode_bond(
        bond: Option<MixnetContractMixNodeBond>,
    ) -> Result<Option<MixNodeBond>, TypesError> {
        match bond {
            Some(bond) => {
                let bond: MixNodeBond = bond.try_into()?;
                Ok(Some(bond))
            }
            None => Ok(None),
        }
    }
}

impl TryFrom<MixnetContractMixNodeBond> for MixNodeBond {
    type Error = TypesError;

    fn try_from(value: MixnetContractMixNodeBond) -> Result<Self, Self::Error> {
        let MixnetContractMixNodeBond {
            pledge_amount,
            total_delegation,
            owner,
            layer,
            block_height,
            mix_node,
            proxy,
            accumulated_rewards,
        } = value;

        if pledge_amount.denom != total_delegation.denom {
            return Err(TypesError::InvalidDenom(
                "The pledge and delegation denominations do not match".to_string(),
            ));
        }

        let pledge_amount: MajorCurrencyAmount = pledge_amount.try_into()?;
        let total_delegation: MajorCurrencyAmount = total_delegation.try_into()?;
        let accumulated_rewards: Option<MajorCurrencyAmount> = accumulated_rewards.and_then(|r| {
            MajorCurrencyAmount::from_minor_uint128_and_denom(r, &pledge_amount.denom.to_string())
                .ok()
        });

        Ok(MixNodeBond {
            pledge_amount,
            total_delegation,
            owner,
            layer,
            block_height,
            mix_node,
            proxy,
            accumulated_rewards,
        })
    }
}
