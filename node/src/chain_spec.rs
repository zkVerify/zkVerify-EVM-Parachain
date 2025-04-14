// Copyright 2025, Horizen Labs, Inc.
// Copyright (C) Parity Technologies (UK) Ltd.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use std::{collections::BTreeMap, str::FromStr};

use cumulus_primitives_core::ParaId;
use fp_evm::GenesisAccount;
use hex_literal::hex;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sp_core::{ecdsa, Pair, Public, H160, U256};
use sp_runtime::traits::{IdentifyAccount, Verify};
use zkv_para_evm_runtime::{
    pallet_network_type::NetworkTypeEnum, AccountId, AuraId,
    OpenZeppelinPrecompiles as Precompiles, Runtime, Signature,
};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;
const WEI_TO_VFY: u128 = 1_000_000_000_000_000_000;

//the default account IDs used in PolkadotJS for ethereum dev accounts
//they are added in the PolkadotJs consolle only if chain is dev or local
//They can also be generated with a wallet created using the below SUBSTRATE_DEFAULT_SEED_PHRASE with Metamask or Ganache
pub const ALITH: [u8; 20] = hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac");
pub const BALTATHAR: [u8; 20] = hex!("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0");
pub const CHARLETH: [u8; 20] = hex!("798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc");
pub const DOROTY: [u8; 20] = hex!("773539d4Ac0e786233D90A233654ccEE26a613D9");
pub const ETHAN: [u8; 20] = hex!("Ff64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB");
pub const FAITH: [u8; 20] = hex!("C0F0f4ab324C46e55D02D0033343B4Be8A55532d");

//the default seed phrase used by substrate for dev accounts like  Alice, Alith, Bob etc..
pub const SUBSTRATE_DEFAULT_SEED_PHRASE: &str =
    "bottom drive obey lake curtain smoke basket hold race lonely fit walk";

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(seed, None)
        .expect("static values are valid; qed")
        .public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain
/// in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
    get_from_seed::<AuraId>(seed)
}

pub fn get_ethereum_keys_from_seed(seed: &str) -> ecdsa::Public {
    get_from_seed::<ecdsa::Public>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we
/// have just one key).
pub fn template_session_keys(keys: AuraId) -> zkv_para_evm_runtime::SessionKeys {
    zkv_para_evm_runtime::SessionKeys { aura: keys }
}

pub fn development_config() -> ChainSpec {
    // Give your base currency a unit name and decimal places

    ChainSpec::builder(
        zkv_para_evm_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: "local".into(),
            para_id: 2000,
        },
    )
    .with_name("Development")
    .with_id("dev")
    .with_chain_type(ChainType::Development)
    .with_properties(chain_properties())
    .with_genesis_config_patch(initial_genesis(
        2000.into(),
        AccountId::from(ALITH),
        // initial collators.
        vec![
            (
                AccountId::from(ALITH),
                get_collator_keys_from_seed(&format!(
                    "{}//{}",
                    SUBSTRATE_DEFAULT_SEED_PHRASE, "Alith"
                )),
            ),
            (
                AccountId::from(BALTATHAR),
                get_collator_keys_from_seed(&format!(
                    "{}//{}",
                    SUBSTRATE_DEFAULT_SEED_PHRASE, "Baltathar"
                )),
            ),
        ],
        true,
        vec![
            AccountId::from(ALITH),     // Alith
            AccountId::from(BALTATHAR), // Baltathar
            AccountId::from(CHARLETH),  // Charleth
            AccountId::from(DOROTY),    // Dorothy
            AccountId::from(ETHAN),     // Ethan
            AccountId::from(FAITH),     // Faith
        ],
    ))
    .build()
}

pub fn local_testnet_config(
    chain_type: ChainType,
    name: &str,
    id: &str,
    add_dev_test_data: bool,
) -> ChainSpec {
    #[allow(deprecated)]
    ChainSpec::builder(
        zkv_para_evm_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: "local".into(),
            para_id: 2000,
        },
    )
    .with_name(name)
    .with_id(id)
    .with_chain_type(chain_type)
    .with_genesis_config_patch(initial_genesis(
        2000.into(),
        AccountId::from(ALITH),
        // initial collators.
        vec![
            (
                AccountId::from(ALITH),
                get_collator_keys_from_seed(&format!(
                    "{}//{}",
                    SUBSTRATE_DEFAULT_SEED_PHRASE, "Alith"
                )),
            ),
            (
                AccountId::from(BALTATHAR),
                get_collator_keys_from_seed(&format!(
                    "{}//{}",
                    SUBSTRATE_DEFAULT_SEED_PHRASE, "Baltathar"
                )),
            ),
        ],
        add_dev_test_data,
        vec![
            AccountId::from(ALITH),     // Alith
            AccountId::from(BALTATHAR), // Baltathar
        ],
    ))
    .with_protocol_id("zkv_para_evm_local_testnet")
    .with_properties(chain_properties())
    .build()
}

fn chain_properties() -> Map<String, Value> {
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "ZEN".into());
    properties.insert("tokenDecimals".into(), 18.into());
    properties.insert("ss58Format".into(), 42.into());
    // This is very important for us, it lets us track the usage of our templates, and have no downside for the node/runtime. Please do not remove :)
    properties.insert("basedOn".into(), "OpenZeppelin EVM Template".into());
    properties
}

fn initial_genesis(
    id: ParaId,
    root: AccountId,
    initial_collators: Vec<(AccountId, AuraId)>,
    add_dev_test_data: bool,
    #[cfg(not(feature = "runtime-benchmarks"))] endowed_accounts: Vec<AccountId>,
    #[cfg(feature = "runtime-benchmarks")] mut endowed_accounts: Vec<AccountId>,
) -> serde_json::Value {
    let precompiles = Precompiles::<Runtime>::used_addresses()
        .map(|addr| {
            (
                addr,
                GenesisAccount {
                    nonce: Default::default(),
                    balance: Default::default(),
                    storage: Default::default(),
                    // bytecode to revert without returning data
                    // (PUSH1 0x00 PUSH1 0x00 REVERT)
                    code: vec![0x60, 0x00, 0x60, 0x00, 0xFD],
                },
            )
        })
        .into_iter();
    let mut accounts: BTreeMap<H160, GenesisAccount> = precompiles.collect();

    if add_dev_test_data {
        // e2e useful account
        accounts.insert(
            H160::from_str("A0CCf49aDBbdfF7A814C07D1FcBC2b719d674959")
                .expect("internal H160 is valid"),
            fp_evm::GenesisAccount {
                balance: U256::from(2 * WEI_TO_VFY),
                code: Default::default(),
                nonce: Default::default(),
                storage: Default::default(),
            },
        );
    }

    #[cfg(feature = "runtime-benchmarks")]
    {
        endowed_accounts.push(AccountId::from(hex!("1000000000000000000000000000000000000001")));
        let acc = sp_core::ecdsa::Pair::from_string("//Bob", None).expect("static values are valid; qed");
        endowed_accounts.push(AccountId::from(acc.public()));
    }

    let mut ret_json = serde_json::json!({
        "balances": {
            "balances": endowed_accounts.iter().cloned().map(|k| (k, 1u64 << 62)).collect::<Vec<_>>(),
        },
        "parachainInfo": {
            "parachainId": id,
        },
        "session": {
            "keys": initial_collators
                .iter()
                .map(|(acc, aura)| {
                    (
                        acc,                        // account id
                        acc,                         // validator id
                        template_session_keys(aura.clone()), // session keys
                    )
                })
            .collect::<Vec<_>>(),
        },
        "collatorSelection": {
            "invulnerables": initial_collators.into_iter().map(|(acc, _)| acc).collect::<Vec<_>>(),
            "candidacyBond": 100,
        },
        "evmChainId": {
            "chainId": 9999
        },
        "evm": {
            "accounts": accounts
        },
        "polkadotXcm": {
            "safeXcmVersion": Some(SAFE_XCM_VERSION),
        },
        "sudo": { "key": Some(root) },
        "networkType": {"value": NetworkTypeEnum::TestNet},
    });

    if add_dev_test_data {
        //ZK Verify Wrapper test data
        match ret_json.as_object_mut() {
            Some(obj) => {
                obj.insert(
					"assets".to_string(),
					serde_json::json!({
						"assets": vec![(1, root, false, 1)],
						"metadata": vec![(1, "ZK Verify Wrapper".as_bytes().to_vec(), "xcZKV".as_bytes().to_vec(), 18)],
						"accounts": vec![(1, root, 1_000_000)]
					}),
				);
            }
            None => {}
        }
    }

    ret_json
}
