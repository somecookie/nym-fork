use crate::currency::MajorCurrencyAmount;
use crate::error::TypesError;
use mixnet_contract_common::{Addr, Gateway, GatewayBond as MixnetContractGatewayBond};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(
    test,
    ts(
        export,
        export,
        export_to = "../../ts-packages/types/src/types/rust/GatewayBond.ts"
    )
)]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct GatewayBond {
    pub pledge_amount: MajorCurrencyAmount,
    pub owner: Addr,
    pub block_height: u64,
    pub gateway: Gateway,
    pub proxy: Option<Addr>,
}

impl GatewayBond {
    pub fn from_mixnet_contract_gateway_bond(
        bond: Option<MixnetContractGatewayBond>,
    ) -> Result<Option<GatewayBond>, TypesError> {
        match bond {
            Some(bond) => {
                let bond: GatewayBond = bond.try_into()?;
                Ok(Some(bond))
            }
            None => Ok(None),
        }
    }
}

impl TryFrom<MixnetContractGatewayBond> for GatewayBond {
    type Error = TypesError;

    fn try_from(value: MixnetContractGatewayBond) -> Result<Self, Self::Error> {
        let MixnetContractGatewayBond {
            pledge_amount,
            owner,
            block_height,
            gateway,
            proxy,
        } = value;

        let pledge_amount: MajorCurrencyAmount = pledge_amount.try_into()?;

        Ok(GatewayBond {
            pledge_amount,
            owner,
            block_height,
            gateway,
            proxy,
        })
    }
}
