use sc_service::ChainType;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

use crate::chain_spec;
use crate::primitives::{AccountId, Balance};
use crate::runtime::{RuntimeGenesisConfig, WASM_BINARY};

// Create genesis configuration for the Selendra network
pub fn selendra_genesis() -> RuntimeGenesisConfig {
    // Predefined accounts with initial balance and authority
    let initial_authorities: Vec<(AccountId, AccountId, sr25519::Public)> = vec![
        // Add initial validators/authorities here
    ];

    // Initial endowed accounts
    let endowed_accounts: Vec<AccountId> = vec![
        // Add accounts with initial SEL token balance
    ];

    RuntimeGenesisConfig {
        system: Default::default(),
        balances: Default::default(),
        // Configure naming service genesis
        naming: crate::pallets::naming::GenesisConfig {
            initial_pricing: vec![
                (crate::pallets::naming::NameTier::Free, 0),
                (crate::pallets::naming::NameTier::Tier1, 3_000_000_000_000_000_000),
                (crate::pallets::naming::NameTier::Tier2, 6_000_000_000_000_000_000),
                (crate::pallets::naming::NameTier::Tier3, 12_000_000_000_000_000_000),
                (crate::pallets::naming::NameTier::Premium, 24_000_000_000_000_000_000),
            ],
        },
        // Add other pallet configurations as needed
        ..Default::default()
    }
}

pub fn load_test_net() -> Result<Box<dyn sc_service::ChainSpec>, String> {
    Ok(Box::new(sc_chain_spec::ChainSpec::from_genesis(
        "Selendra TestNet",
        "selendra_testnet",
        ChainType::Development,
        selendra_genesis,
        vec![],
        None,
        None,
        None,
        None,
    )))
}
