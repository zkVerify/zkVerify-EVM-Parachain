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

use chain_spec::ChainSpec as GenericChainSpec;
use cumulus_primitives_core::ParaId;
use frame_benchmarking_cli::{BenchmarkCmd, SUBSTRATE_REFERENCE_HARDWARE};
use log::info;
use sc_cli::{
    ChainSpec, CliConfiguration, DefaultConfigurationValues, ImportParams, KeystoreParams,
    NetworkParams, Result, RpcEndpoint, SharedParams, SubstrateCli,
};
use sc_service::config::{BasePath, PrometheusConfig};
use sp_runtime::traits::AccountIdConversion;
use zkv_para_evm_runtime::Block;

use crate::{
    chain_spec,
    cli::{Cli, RelayChainCli, Subcommand},
    service::new_partial,
};

fn load_spec(id: &str) -> std::result::Result<Box<dyn ChainSpec>, String> {
    Ok(match id {
        "" | "test" | "testnet" => Box::new(GenericChainSpec::from_json_bytes(
            &include_bytes!("../chain-specs/zkverify_evm_vflow.json")[..],
        )?),
        "testnet_build" => Box::new(chain_spec::testnet_config()?),
        "dev" => Box::new(chain_spec::development_config()?),
        "local" => Box::new(chain_spec::local_testnet_config()?),
        path => Box::new(GenericChainSpec::from_json_file(std::path::PathBuf::from(
            path,
        ))?),
    })
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "zkVerify EVM Parachain".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        format!(
            "zkVerify EVM Parachain\n\nThe command-line arguments provided first will be passed to the \
			 parachain node, while the arguments provided after -- will be passed to the relay \
			 chain node.\n\n{} <parachain-args> -- <relay-chain-args>",
            Self::executable_name()
        )
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/zkVerify/zkVerify-EVM-Parachain/issues/new".into()
    }

    fn copyright_start_year() -> i32 {
        2024
    }

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        load_spec(id)
    }
}

impl SubstrateCli for RelayChainCli {
    fn impl_name() -> String {
        "Parachain Collator Template".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        format!(
            "zkVerify EVM Collator Node\n\nThe command-line arguments provided first will be passed \
			 to the parachain node, while the arguments provided after -- will be passed to the \
			 relay chain node.\n\n{} <parachain-args> -- <relay-chain-args>",
            Self::executable_name()
        )
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/zkVerify/zkVerify-EVM-Parachain/issues/new".into()
    }

    fn copyright_start_year() -> i32 {
        2024
    }

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        polkadot_cli::Cli::from_iter([RelayChainCli::executable_name()].iter()).load_spec(id)
    }
}

macro_rules! construct_async_run {
	(|$components:ident, $cli:ident, $cmd:ident, $config:ident| $( $code:tt )* ) => {{
		let runner = $cli.create_runner($cmd)?;
        let eth = $cli.eth.clone();
		let sealing = $cli.sealing;
		runner.async_run(|$config| {
			let $components = new_partial(&$config, &eth, sealing)?;
			let task_manager = $components.task_manager;
			{ $( $code )* }.map(|v| (v, task_manager))
		})
	}}
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
    let cli = Cli::from_args();

    match &cli.subcommand {
        Some(Subcommand::Key(cmd)) => cmd.run(&cli),
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, components.import_queue))
            })
        }
        Some(Subcommand::ExportBlocks(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, config.database))
            })
        }
        Some(Subcommand::ExportState(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, config.chain_spec))
            })
        }
        Some(Subcommand::ImportBlocks(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, components.import_queue))
            })
        }
        Some(Subcommand::Revert(cmd)) => {
            construct_async_run!(|components, cli, cmd, config| {
                Ok(cmd.run(components.client, components.backend, None))
            })
        }
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            runner.sync_run(|config| {
                let polkadot_cli = RelayChainCli::new(
                    &config,
                    [RelayChainCli::executable_name()]
                        .iter()
                        .chain(cli.relay_chain_args.iter()),
                );

                let polkadot_config = SubstrateCli::create_configuration(
                    &polkadot_cli,
                    &polkadot_cli,
                    config.tokio_handle.clone(),
                )
                .map_err(|err| format!("Relay chain argument error: {err}"))?;

                cmd.run(config, polkadot_config)
            })
        }
        Some(Subcommand::ExportGenesisHead(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| {
                let partials = new_partial(&config, &cli.eth, cli.sealing)?;

                cmd.run(partials.client)
            })
        }
        Some(Subcommand::ExportGenesisWasm(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|_config| {
                let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
                cmd.run(&*spec)
            })
        }
        Some(Subcommand::Benchmark(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            // Switch on the concrete benchmark sub-command-
            match cmd {
                BenchmarkCmd::Pallet(cmd) => {
                    if cfg!(feature = "runtime-benchmarks") {
                        runner.sync_run(|config| {
                            cmd.run_with_spec::<sp_runtime::traits::HashingFor<Block>, crate::service::HostFunctions>(
                                Some(config.chain_spec),
                            )
                        })
                    } else {
                        Err(
                            "Benchmarking wasn't enabled when building the node. You can enable \
						     it with `--features runtime-benchmarks`."
                                .into(),
                        )
                    }
                }
                BenchmarkCmd::Block(cmd) => runner.sync_run(|config| {
                    let partials = new_partial(&config, &cli.eth, cli.sealing)?;
                    cmd.run(partials.client)
                }),
                #[cfg(not(feature = "runtime-benchmarks"))]
                BenchmarkCmd::Storage(_) => Err(sc_cli::Error::Input(
                    "Compile with --features=runtime-benchmarks to enable storage benchmarks."
                        .into(),
                )),
                #[cfg(feature = "runtime-benchmarks")]
                BenchmarkCmd::Storage(cmd) => runner.sync_run(|config| {
                    let partials = new_partial(&config, &cli.eth, cli.sealing)?;
                    let db = partials.backend.expose_db();
                    let storage = partials.backend.expose_storage();
                    cmd.run(config, partials.client.clone(), db, storage)
                }),
                #[cfg(feature = "runtime-benchmarks")]
                BenchmarkCmd::Overhead(cmd) => runner.sync_run(|config| {
                    use crate::benchmarking::{create_inherent_data, RemarkBuilder};

                    let partials = new_partial(&config, &cli.eth, cli.sealing)?;
                    let ext_builder = RemarkBuilder::new(partials.client.clone());
                    let para_id = ParaId::from(
                        chain_spec::Extensions::try_get(&*config.chain_spec)
                            .map(|e| e.para_id)
                            .ok_or("Could not find parachain ID in chain-spec.")?,
                    );
                    cmd.run(
                        config.chain_spec.name().into(),
                        partials.client.clone(),
                        create_inherent_data(partials.client.clone(), para_id),
                        Vec::new(),
                        &ext_builder,
                        true, // It's a parachain -> should record the rpoof
                    )
                }),
                #[cfg(not(feature = "runtime-benchmarks"))]
                BenchmarkCmd::Overhead(_) => Err(sc_cli::Error::Input(
                    "Compile with --features=runtime-benchmarks to enable overhead benchmarks."
                        .into(),
                )),
                BenchmarkCmd::Machine(cmd) => {
                    runner.sync_run(|config| cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone()))
                }
                // NOTE: this allows the Client to leniently implement
                // new benchmark commands without requiring a companion MR.
                #[allow(unreachable_patterns)]
                _ => Err("Benchmarking sub-command unsupported".into()),
            }
        }
        None => {
            let runner = cli.create_runner(&cli.run.normalize())?;
            let collator_options = cli.run.collator_options();

            runner.run_node_until_exit(|config| async move {
				let para_id = ParaId::from(
					chain_spec::Extensions::try_get(&*config.chain_spec)
						.map(|e| e.para_id)
						.ok_or("Could not find parachain ID in chain-spec.")?,
				);

				if cli.sealing.is_some() {
					info!("Starting in manual sealing mode without relay chain");
					return crate::service::start_manual_seal_node::<sc_network::NetworkWorker<_, _>>(
						config,
						para_id,
						&cli.eth,
						cli.sealing
					)
					.map_err(Into::into);
				}

				let hwbench = (!cli.no_hardware_benchmarks)
					.then_some(config.database.path().map(|database_path| {
						let _ = std::fs::create_dir_all(database_path);
						sc_sysinfo::gather_hwbench(
							Some(database_path),
							&SUBSTRATE_REFERENCE_HARDWARE,
						)
					}))
					.flatten();

				let polkadot_cli = RelayChainCli::new(
					&config,
					[RelayChainCli::executable_name()].iter().chain(cli.relay_chain_args.iter()),
				);

				let parachain_account =
					AccountIdConversion::<polkadot_primitives::AccountId>::into_account_truncating(
						&para_id,
					);

				let tokio_handle = config.tokio_handle.clone();
				let polkadot_config =
					SubstrateCli::create_configuration(&polkadot_cli, &polkadot_cli, tokio_handle)
						.map_err(|err| format!("Relay chain argument error: {err}"))?;

				info!("Parachain Id: {para_id}");
				info!("Parachain Account: {parachain_account}");
				info!("Is collating: {}", if config.role.is_authority() { "yes" } else { "no" });

				crate::service::start_parachain_node(
					config,
					polkadot_config,
					collator_options,
					&cli.eth,
					para_id,
					hwbench,
				)
				.await
				.map(|r| r.0)
				.map_err(Into::into)
			})
        }
    }
}

impl DefaultConfigurationValues for RelayChainCli {
    fn p2p_listen_port() -> u16 {
        30334
    }

    fn rpc_listen_port() -> u16 {
        9945
    }

    fn prometheus_listen_port() -> u16 {
        9616
    }
}

impl CliConfiguration<Self> for RelayChainCli {
    fn shared_params(&self) -> &SharedParams {
        self.base.base.shared_params()
    }

    fn import_params(&self) -> Option<&ImportParams> {
        self.base.base.import_params()
    }

    fn network_params(&self) -> Option<&NetworkParams> {
        self.base.base.network_params()
    }

    fn keystore_params(&self) -> Option<&KeystoreParams> {
        self.base.base.keystore_params()
    }

    fn base_path(&self) -> Result<Option<BasePath>> {
        Ok(self
            .shared_params()
            .base_path()?
            .or_else(|| self.base_path.clone().map(Into::into)))
    }

    fn rpc_addr(&self, default_listen_port: u16) -> Result<Option<Vec<RpcEndpoint>>> {
        self.base.base.rpc_addr(default_listen_port)
    }

    fn prometheus_config(
        &self,
        default_listen_port: u16,
        chain_spec: &Box<dyn ChainSpec>,
    ) -> Result<Option<PrometheusConfig>> {
        self.base
            .base
            .prometheus_config(default_listen_port, chain_spec)
    }

    fn init<F>(&self, _support_url: &String, _impl_version: &String, _logger_hook: F) -> Result<()>
    where
        F: FnOnce(&mut sc_cli::LoggerBuilder),
    {
        unreachable!("PolkadotCli is never initialized; qed");
    }

    fn chain_id(&self, is_dev: bool) -> Result<String> {
        let chain_id = self.base.base.chain_id(is_dev)?;

        Ok(if chain_id.is_empty() {
            self.chain_id.clone().unwrap_or_default()
        } else {
            chain_id
        })
    }

    fn role(&self, is_dev: bool) -> Result<sc_service::Role> {
        self.base.base.role(is_dev)
    }

    fn transaction_pool(&self, is_dev: bool) -> Result<sc_service::config::TransactionPoolOptions> {
        self.base.base.transaction_pool(is_dev)
    }

    fn trie_cache_maximum_size(&self) -> Result<Option<usize>> {
        self.base.base.trie_cache_maximum_size()
    }

    fn rpc_methods(&self) -> Result<sc_service::config::RpcMethods> {
        self.base.base.rpc_methods()
    }

    fn rpc_max_connections(&self) -> Result<u32> {
        self.base.base.rpc_max_connections()
    }

    fn rpc_cors(&self, is_dev: bool) -> Result<Option<Vec<String>>> {
        self.base.base.rpc_cors(is_dev)
    }

    fn default_heap_pages(&self) -> Result<Option<u64>> {
        self.base.base.default_heap_pages()
    }

    fn force_authoring(&self) -> Result<bool> {
        self.base.base.force_authoring()
    }

    fn disable_grandpa(&self) -> Result<bool> {
        self.base.base.disable_grandpa()
    }

    fn max_runtime_instances(&self) -> Result<Option<usize>> {
        self.base.base.max_runtime_instances()
    }

    fn announce_block(&self) -> Result<bool> {
        self.base.base.announce_block()
    }

    fn telemetry_endpoints(
        &self,
        chain_spec: &Box<dyn ChainSpec>,
    ) -> Result<Option<sc_telemetry::TelemetryEndpoints>> {
        self.base.base.telemetry_endpoints(chain_spec)
    }

    fn node_name(&self) -> Result<String> {
        self.base.base.node_name()
    }
}
