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

use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_network::config::MultiaddrWithPeerId;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::str;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<Extensions>;

// The URL for the telemetry server.
const TELEMETRY_URL: &str = "wss://telemetry.zkverify.io/submit/";

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

fn chain_properties() -> Map<String, Value> {
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "tVFY".into());
    properties.insert("tokenDecimals".into(), 18.into());
    properties.insert("ss58Format".into(), 0.into());
    properties.insert("isEthereum".into(), true.into());
    // This is very important for us, it lets us track the usage of our templates, and have no downside for the node/runtime. Please do not remove :)
    properties.insert("basedOn".into(), "OpenZeppelin EVM Template".into());
    properties
}
pub fn development_config() -> Result<ChainSpec, String> {
    Ok(ChainSpec::builder(
        zkv_para_evm_runtime::WASM_BINARY
            .ok_or_else(|| "Development wasm not available".to_string())?,
        Extensions {
            relay_chain: "local".into(),
            para_id: 2000,
        },
    )
    .with_name("Development")
    .with_id("dev")
    .with_chain_type(ChainType::Development)
    .with_properties(chain_properties())
    .with_genesis_config_preset_name("development")
    .build())
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    Ok(ChainSpec::builder(
        zkv_para_evm_runtime::WASM_BINARY
            .ok_or_else(|| "Development wasm not available".to_string())?,
        Extensions {
            relay_chain: "local".into(),
            para_id: 2000,
        },
    )
    .with_name("Local Testnet")
    .with_id("local_testnet")
    .with_chain_type(ChainType::Local)
    .with_protocol_id("zkv_para_evm_local_testnet")
    .with_properties(chain_properties())
    .with_genesis_config_preset_name("local-test")
    .build())
}

fn boot_node_address(dns: &str, peer_id: &str) -> impl Iterator<Item = MultiaddrWithPeerId> {
    vec![
        format!("/dns/{dns}/tcp/30333/p2p/{peer_id}"),
        format!("/dns/{dns}/tcp/30334/ws/p2p/{peer_id}"),
        format!("/dns/{dns}/tcp/443/wss/p2p/{peer_id}"),
    ]
    .into_iter()
    .map(|s| s.parse().expect("Valid address. qed"))
}

pub fn testnet_config() -> Result<ChainSpec, String> {
    // The connection strings for bootnodes
    const BOOTNODE_1_DNS: &str = "boot-node-tn-vflow-1.zkverify.io";
    const BOOTNODE_1_PEER_ID: &str = "12D3KooWStRw5P6v8bydm3RjzsdSE75PNoFtCzZ5PnV1hkntWGim";
    const BOOTNODE_2_DNS: &str = "boot-node-tn-vflow-2.zkverify.io";
    const BOOTNODE_2_PEER_ID: &str = "12D3KooWFVarmg1RGuCnEsHVjYSxKd6idJ6cCEowkKkgaBPovt84";

    Ok(ChainSpec::builder(
        zkv_para_evm_runtime::WASM_BINARY
            .ok_or_else(|| "Development wasm not available".to_string())?,
        Extensions {
            relay_chain: "test".into(),
            para_id: 1,
        },
    )
    .with_name("VFlow Testnet")
    .with_id("vflow_testnet")
    .with_chain_type(ChainType::Live)
    .with_protocol_id("tvflow")
    .with_boot_nodes(
        boot_node_address(BOOTNODE_1_DNS, BOOTNODE_1_PEER_ID)
            .chain(boot_node_address(BOOTNODE_2_DNS, BOOTNODE_2_PEER_ID))
            .collect(),
    )
    .with_properties(chain_properties())
    .with_genesis_config_preset_name("testnet")
    .with_telemetry_endpoints(
        TelemetryEndpoints::new(vec![(
            TELEMETRY_URL.to_string(),
            sc_telemetry::CONSENSUS_INFO,
        )])
        .expect("Horizen Labs telemetry url is valid; qed"),
    )
    .build())
}
