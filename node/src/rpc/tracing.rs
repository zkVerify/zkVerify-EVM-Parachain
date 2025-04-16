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

use sp_core::H256;
use tokio::sync::Semaphore;
use zkv_para_evm_rpc_debug::{DebugHandler, DebugRequester};

use super::*;
use crate::eth::{EthApi, EthConfiguration};

#[derive(Clone)]
pub struct RpcRequesters {
    pub debug: Option<DebugRequester>,
}

use fc_rpc_core::types::{FeeHistoryCache, FilterPool};
use fc_storage::StorageOverride;
use fp_rpc::EthereumRuntimeRPCApi;
use sc_client_api::{
    backend::{Backend, StateBackend, StorageProvider},
    client::BlockchainEvents,
    BlockOf,
};
use sc_service::TaskManager;

#[allow(dead_code)]
pub struct SpawnTasksParams<'a, B: BlockT, C, BE> {
    pub task_manager: &'a TaskManager,
    pub client: Arc<C>,
    pub substrate_backend: Arc<BE>,
    pub frontier_backend: Arc<fc_db::Backend<B, C>>,
    pub filter_pool: Option<FilterPool>,
    pub overrides: Arc<dyn StorageOverride<B>>,
    pub fee_history_limit: u64,
    pub fee_history_cache: FeeHistoryCache,
}
use sp_runtime::traits::{BlakeTwo256, Header as HeaderT};

pub struct TracingConfig {
    pub tracing_requesters: RpcRequesters,
}

// Spawn the tasks that are required to run a tracing node.
pub fn spawn_tracing_tasks<B, C, BE>(
    eth_config: &EthConfiguration,
    params: SpawnTasksParams<B, C, BE>,
) -> RpcRequesters
where
    C: ProvideRuntimeApi<B> + BlockOf,
    C: StorageProvider<B, BE>,
    C: HeaderBackend<B> + HeaderMetadata<B, Error = BlockChainError> + 'static,
    C: BlockchainEvents<B>,
    C: Send + Sync + 'static,
    C::Api: EthereumRuntimeRPCApi<B> + zkv_para_evm_rpc_primitives_debug::DebugRuntimeApi<B>,
    C::Api: BlockBuilder<B>,
    B: BlockT<Hash = H256> + Send + Sync + 'static,
    B::Header: HeaderT<Number = u32>,
    BE: Backend<B> + 'static,
    BE::State: StateBackend<BlakeTwo256>,
{
    let permit_pool = Arc::new(Semaphore::new(eth_config.ethapi_max_permits as usize));

    let (debug_task, debug_requester) = if eth_config.ethapi.contains(&EthApi::Debug) {
        let (debug_task, debug_requester) = DebugHandler::task(
            Arc::clone(&params.client),
            Arc::clone(&params.substrate_backend),
            match *params.frontier_backend {
                fc_db::Backend::KeyValue(ref b) => b.clone(),
                fc_db::Backend::Sql(ref b) => b.clone(),
            },
            Arc::clone(&permit_pool),
            Arc::clone(&params.overrides),
            eth_config.tracing_raw_max_memory_usage,
        );
        (Some(debug_task), Some(debug_requester))
    } else {
        (None, None)
    };

    // `debug` task if enabled. Essential.
    // Proxies rpc requests to it's handler.
    if let Some(debug_task) = debug_task {
        params.task_manager.spawn_essential_handle().spawn(
            "ethapi-debug",
            Some("eth-tracing"),
            debug_task,
        );
    }

    RpcRequesters {
        debug: debug_requester,
    }
}
