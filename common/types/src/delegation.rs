use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use log::error;
use mixnet_contract_common::mixnode::DelegationEvent as ContractDelegationEvent;
use mixnet_contract_common::mixnode::PendingUndelegate as ContractPendingUndelegate;
use mixnet_contract_common::Delegation as MixnetContractDelegation;

use crate::currency::MajorCurrencyAmount;
use crate::error::TypesError;

#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/Delegation.ts")
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

#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/DelegationResult.ts")
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

impl TryFrom<MixnetContractDelegation> for DelegationResult {
    type Error = TypesError;

    fn try_from(delegation: MixnetContractDelegation) -> Result<Self, Self::Error> {
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
    ts(export_to = "ts-packages/types/src/types/rust/DelegationEvent.ts")
)]
#[derive(Deserialize, Serialize)]
pub enum DelegationEvent {
    Delegate(DelegationResult),
    Undelegate(PendingUndelegate),
}

impl TryFrom<ContractDelegationEvent> for DelegationEvent {
    type Error = TypesError;

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
    ts(export_to = "ts-packages/types/src/types/rust/PendingUndelegate.ts")
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
) -> Result<Vec<DelegationEvent>, TypesError> {
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
        .collect::<Vec<TypesError>>();

    error!("Failed to convert delegations: {:?}", errors);
    Err(TypesError::DelegationsInvalid)
}

#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/DelegationSummaryResponse.ts")
)]
#[derive(Deserialize, Serialize)]
pub struct DelegationsSummaryResponse {
    pub delegations: Vec<Delegation>,
    pub total_delegations: MajorCurrencyAmount,
    pub total_rewards: MajorCurrencyAmount,
}