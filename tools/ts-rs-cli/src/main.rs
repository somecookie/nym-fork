use ts_rs::TS;
use walkdir::WalkDir;

use mixnet_contract_common::mixnode::RewardedSetNodeStatus;
use nym_types::account::{Account, AccountWithMnemonic, Balance};
use nym_types::currency::{CurrencyDenom, MajorAmountString, MajorCurrencyAmount};
use nym_types::delegation::Delegation;
use nym_types::gateway::{Gateway, GatewayBond};
use nym_types::mixnode::{MixNode, MixNodeBond};
use nym_types::transaction::{TauriTxResult, TransactionDetails};
use nym_types::vesting::{OriginalVestingResponse, PledgeData, VestingAccountInfo, VestingPeriod};
use nym_wallet_types::admin::TauriContractStateParams;
use nym_wallet_types::app::AppEnv;
use nym_wallet_types::epoch::Epoch;
use nym_wallet_types::network::Network;
use nym_wallet_types::network_config::{Validator, ValidatorUrl, ValidatorUrls};
use validator_api_requests::models::{
    CoreNodeStatusResponse, InclusionProbabilityResponse, MixnodeStatus, MixnodeStatusResponse,
    SelectionChance, StakeSaturationResponse,
};
use validator_client::nymd::fee::helpers::Operation;
use vesting_contract_common::Period;

macro_rules! do_export {
    ($a:ty) => {{
        match <$a>::export() {
            Ok(()) => {
                println!("✅ {}", <$a>::name());
            }
            Err(e) => {
                println!("❌ {} failed: {}", <$a>::name(), e);
            }
        }
    }};
}

fn main() {
    println!("Starting export of types using ts-rs...");
    println!();

    //
    // macro expands into `println!("Exporting {}...", Type::name()); Type::export();` with some error handling
    //

    // common/client-libs/validator-client/src/nymd/fee
    do_export!(Operation);
    // common/cosmwasm-smart-contracts/mixnet-contract/src
    do_export!(RewardedSetNodeStatus);
    // common/cosmwasm-smart-contracts/vesting-contract/src
    do_export!(Period);

    // common/types/src
    do_export!(Account);
    do_export!(AccountWithMnemonic);
    do_export!(Balance);
    do_export!(CurrencyDenom);
    do_export!(MajorAmountString);
    do_export!(MajorCurrencyAmount);
    do_export!(Delegation);
    do_export!(Gateway);
    do_export!(GatewayBond);
    do_export!(MixNode);
    do_export!(MixNodeBond);
    do_export!(VestingAccountInfo);
    do_export!(PledgeData);
    do_export!(OriginalVestingResponse);
    do_export!(VestingPeriod);
    do_export!(TransactionDetails);
    do_export!(TauriTxResult);

    // validator-api-requests
    do_export!(MixnodeStatus);
    do_export!(InclusionProbabilityResponse);
    do_export!(SelectionChance);
    do_export!(StakeSaturationResponse);
    do_export!(MixnodeStatusResponse);
    do_export!(CoreNodeStatusResponse);

    // nym-wallet
    do_export!(Validator);
    do_export!(ValidatorUrl);
    do_export!(ValidatorUrls);
    do_export!(Epoch);
    do_export!(TauriContractStateParams);
    do_export!(AppEnv);
    do_export!(Network);

    for file in WalkDir::new("./")
        .into_iter()
        .filter_map(|file| file.ok())
        .filter(|f| {
            let path = format!("{}", f.path().display());
            path != "./"
                && !path.starts_with("./src")
                && !path.starts_with("./target")
                && !path.starts_with("./Cargo.toml")
                && !path.starts_with("./.gitignore")
                && f.file_type().is_file()
        })
    {
        println!("{}", file.path().display());
    }

    println!();
    println!("Done");
}

// common/client-libs/validator-client/src/nymd/fee/helpers.rs:    feature = "generate-ts",
// common/cosmwasm-smart-contracts/mixnet-contract/src/mixnode.rs:    feature = "generate-ts",
// common/cosmwasm-smart-contracts/vesting-contract/src/lib.rs:    feature = "generate-ts",
// common/types/src/account.rs:    feature = "generate-ts",
// common/types/src/account.rs:    feature = "generate-ts",
// common/types/src/account.rs:    feature = "generate-ts",
// common/types/src/currency.rs:    feature = "generate-ts",
// common/types/src/currency.rs:    feature = "generate-ts",
// common/types/src/currency.rs:    feature = "generate-ts",
// common/types/src/delegation.rs:    feature = "generate-ts",
// common/types/src/gateway.rs:    feature = "generate-ts",
// common/types/src/mixnode.rs:    feature = "generate-ts",
// nym-wallet/src-tauri/src/network.rs:  feature = "generate-ts",
// nym-wallet/src-tauri/src/network_config.rs:  feature = "generate-ts",
// nym-wallet/src-tauri/src/network_config.rs:  feature = "generate-ts",
// nym-wallet/src-tauri/src/network_config.rs:  feature = "generate-ts",
// nym-wallet/src-tauri/src/operations/mixnet/admin.rs:  feature = "generate-ts",
// nym-wallet/src-tauri/src/operations/mixnet/delegate.rs:  feature = "generate-ts",
// nym-wallet/src-tauri/src/operations/mixnet/epoch.rs:  feature = "generate-ts",
// nym-wallet/src-tauri/src/operations/mixnet/send.rs:  feature = "generate-ts",
// nym-wallet/src-tauri/src/operations/mixnet/send.rs:  feature = "generate-ts",
// nym-wallet/src-tauri/src/operations/vesting/mod.rs:  feature = "generate-ts",
// nym-wallet/src-tauri/src/operations/vesting/mod.rs:  feature = "generate-ts",
// nym-wallet/src-tauri/src/operations/vesting/mod.rs:  feature = "generate-ts",
// nym-wallet/src-tauri/src/operations/vesting/mod.rs:  feature = "generate-ts",
// nym-wallet/src-tauri/src/utils.rs:  feature = "generate-ts",
// nym-wallet/src-tauri/src/utils.rs:  feature = "generate-ts",
// nym-wallet/src-tauri/src/utils.rs:  feature = "generate-ts",
// nym-wallet/src-tauri/src/utils.rs:  feature = "generate-ts",
// validator-api/validator-api-requests/src/models.rs:    feature = "generate-ts",
// validator-api/validator-api-requests/src/models.rs:    feature = "generate-ts",
// validator-api/validator-api-requests/src/models.rs:    feature = "generate-ts",
// validator-api/validator-api-requests/src/models.rs:    feature = "generate-ts",
// validator-api/validator-api-requests/src/models.rs:    feature = "generate-ts",
// validator-api/validator-api-requests/src/models.rs:    feature = "generate-ts",
