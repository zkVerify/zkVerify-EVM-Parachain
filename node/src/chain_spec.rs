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
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<Extensions>;

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
    .with_genesis_config_preset_name("local")
    .build())
}

pub fn testnet_config() -> Result<ChainSpec, String> {
    Ok(ChainSpec::builder(
        zkv_para_evm_runtime::WASM_BINARY
            .ok_or_else(|| "Testnet wasm not available".to_string())?,
        Extensions {
            relay_chain: "zkVerify Volta".into(),
            para_id: 1,
        },
    )
    .with_name("VFlow Testnet")
    .with_id("vflow_testnet")
    .with_chain_type(ChainType::Live)
    .with_protocol_id("tvflow")
    .with_properties(chain_properties())
    .with_genesis_config_preset_name("testnet")
    .with_boot_nodes(vec![])
    .with_telemetry_endpoints(
        TelemetryEndpoints::new(vec![]).expect("Horizen Labs telemetry url is valid; qed"),
    )
    .build())
}
