//! Selendra Node
//!
//! Main entry point for the Selendra blockchain node.

use sc_cli::{RunCmd, SubstrateCli};
use sc_service::Configuration;
use selendra_node::{
    chain_spec,
    cli::{Cli, Subcommand},
    service,
};

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Selendra Node".into()
    }

    fn impl_version() -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn description() -> String {
        env!("CARGO_PKG_DESCRIPTION").into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/selendra/selendra/issues".into()
    }

    fn copyright_start_year() -> i32 {
        2021
    }

    fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
        Ok(match id {
            "dev" => Box::new(chain_spec::development_config()?),
            "" | "local" => Box::new(chain_spec::local_testnet_config()?),
            path => Box::new(chain_spec::ChainSpec::from_json_file(
                std::path::PathBuf::from(path),
            )?),
        })
    }
}

fn main() -> sc_cli::Result<()> {
    let cli = Cli::from_args();

    match &cli.subcommand {
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let (client, _, import_queue, task_manager) = service::new_chain_ops(&config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(Subcommand::ExportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let (client, _, _, task_manager) = service::new_chain_ops(&config)?;
                Ok((cmd.run(client, config.database), task_manager))
            })
        }
        Some(Subcommand::ExportState(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let (client, _, _, task_manager) = service::new_chain_ops(&config)?;
                Ok((cmd.run(client, config.chain_spec), task_manager))
            })
        }
        Some(Subcommand::ImportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let (client, _, import_queue, task_manager) = service::new_chain_ops(&config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.database))
        }
        Some(Subcommand::Revert(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let (client, backend, _, task_manager) = service::new_chain_ops(&config)?;
                Ok((cmd.run(client, backend, None), task_manager))
            })
        }
        None => {
            let runner = cli.create_runner(&cli.run.normalize())?;
            runner.run_node_until_exit(|config| async move {
                service::new_full(config).map_err(sc_cli::Error::Service)
            })
        }
    }
}

mod cli {
    use clap::Parser;
    use sc_cli::{KeySubcommand, SignCmd, VanityCmd, VerifyCmd};

    #[derive(Debug, Parser)]
    pub struct Cli {
        #[clap(subcommand)]
        pub subcommand: Option<Subcommand>,

        #[clap(flatten)]
        pub run: RunCmd,
    }

    #[derive(Debug, clap::Subcommand)]
    pub enum Subcommand {
        /// Build a chain specification.
        BuildSpec(sc_cli::BuildSpecCmd),

        /// Validate blocks.
        CheckBlock(sc_cli::CheckBlockCmd),

        /// Export blocks.
        ExportBlocks(sc_cli::ExportBlocksCmd),

        /// Export the state of a given block into a chain spec.
        ExportState(sc_cli::ExportStateCmd),

        /// Import blocks.
        ImportBlocks(sc_cli::ImportBlocksCmd),

        /// Remove the whole chain.
        PurgeChain(sc_cli::PurgeChainCmd),

        /// Revert the chain to a previous state.
        Revert(sc_cli::RevertCmd),

        /// Key management cli utilities
        Key(KeySubcommand),

        /// Verify a signature for a message, provided on STDIN, with a given (public or secret) key.
        Verify(VerifyCmd),

        /// Generate a seed that provides a vanity address.
        Vanity(VanityCmd),

        /// Sign a message, with a given (secret) key.
        Sign(SignCmd),
    }
}

mod chain_spec {
    use selendra_runtime::{AccountId, BalancesConfig, GenesisConfig, SystemConfig, WASM_BINARY};
    use sc_service::ChainType;
    use sp_core::{sr25519, Pair, Public};
    use sp_runtime::traits::{IdentifyAccount, Verify};

    /// Helper function to generate a crypto pair from seed
    pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
        TPublic::Pair::from_string(&format!("//{}", seed), None)
            .expect("static values are valid; qed")
            .public()
    }

    type AccountPublic = <Signature as Verify>::Signer;

    /// Helper function to generate an account ID from seed
    pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
    where
        AccountPublic: From<<TPublic::Pair as Pair>::Public>,
    {
        AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
    }

    /// Development configuration
    pub fn development_config() -> Result<ChainSpec, String> {
        let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

        Ok(ChainSpec::from_genesis(
            "Development",
            "dev",
            ChainType::Development,
            move || {
                testnet_genesis(
                    wasm_binary,
                    vec![
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                        get_account_id_from_seed::<sr25519::Public>("Charlie"),
                        get_account_id_from_seed::<sr25519::Public>("Dave"),
                    ],
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                )
            },
            vec![],
            None,
            None,
            None,
            None,
        ))
    }

    /// Local testnet configuration
    pub fn local_testnet_config() -> Result<ChainSpec, String> {
        let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

        Ok(ChainSpec::from_genesis(
            "Local Testnet",
            "local_testnet",
            ChainType::Local,
            move || {
                testnet_genesis(
                    wasm_binary,
                    vec![
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                    ],
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                )
            },
            vec![],
            None,
            None,
            None,
            None,
        ))
    }

    /// Configure initial storage state for FRAME modules.
    fn testnet_genesis(
        wasm_binary: &[u8],
        initial_authorities: Vec<AccountId>,
        root_key: AccountId,
    ) -> GenesisConfig {
        GenesisConfig {
            system: SystemConfig {
                code: wasm_binary.to_vec(),
                ..Default::default()
            },
            balances: BalancesConfig {
                balances: initial_authorities
                    .iter()
                    .cloned()
                    .map(|k| (k, 1 << 60))
                    .collect(),
            },
        }
    }
}

mod service {
    use selendra_runtime::{self, RuntimeApi};
    use sc_client_api::ExecutorProvider;
    use sc_consensus::LongestChain;
    use sc_executor::NativeElseWasmExecutor;
    use sc_service::{error::Error as ServiceError, Configuration, TaskManager};
    use sc_telemetry::{Telemetry, TelemetryWorker};
    use std::sync::Arc;

    type FullClient = sc_service::TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<ExecutorDispatch>>;
    type FullBackend = sc_service::TFullBackend<Block>;
    type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;

    pub fn new_partial(
        config: &Configuration,
    ) -> Result<
        sc_service::PartialComponents<
            FullClient,
            FullBackend,
            FullSelectChain,
            sc_consensus::DefaultImportQueue<Block, FullClient>,
            sc_transaction_pool::FullPool<Block, FullClient>,
            Option<Telemetry>,
        >,
        ServiceError,
    > {
        let telemetry = config
            .telemetry_endpoints
            .clone()
            .filter(|x| !x.is_empty())
            .map(|endpoints| -> Result<_, sc_telemetry::Error> {
                let worker = TelemetryWorker::new(16)?;
                let telemetry = worker.handle().new_telemetry(endpoints);
                Ok((worker, telemetry))
            })
            .transpose()?;

        let executor = NativeElseWasmExecutor::<ExecutorDispatch>::new(
            config.wasm_method,
            config.default_heap_pages,
            config.max_runtime_instances,
            config.runtime_cache_size,
        );

        let (client, backend, keystore_container, task_manager) =
            sc_service::new_full_parts::<Block, RuntimeApi, _>(
                config,
                telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
                executor,
            )?;
        let client = Arc::new(client);

        let telemetry = telemetry.map(|(worker, telemetry)| {
            task_manager.spawn_handle().spawn("telemetry", None, worker.run());
            telemetry
        });

        let select_chain = sc_consensus::LongestChain::new(backend.clone());

        let transaction_pool = sc_transaction_pool::BasicPool::new_full(
            config.transaction_pool.clone(),
            config.role.is_authority().into(),
            config.prometheus_registry(),
            task_manager.spawn_essential_handle(),
            client.clone(),
        );

        let import_queue = sc_consensus::import_queue(
            client.clone(),
            client.clone(),
            select_chain.clone(),
            None,
            None,
            &task_manager.spawn_essential_handle(),
            config.prometheus_registry(),
            telemetry.as_ref().map(|x| x.handle()),
        )?;

        Ok(sc_service::PartialComponents {
            client,
            backend,
            task_manager,
            import_queue,
            keystore_container,
            select_chain,
            transaction_pool,
            other: (telemetry),
        })
    }

    pub fn new_full(config: Configuration) -> Result<TaskManager, ServiceError> {
        let sc_service::PartialComponents {
            client,
            backend,
            mut task_manager,
            import_queue,
            keystore_container,
            select_chain,
            transaction_pool,
            other: (mut telemetry),
        } = new_partial(&config)?;

        let (network, system_rpc_tx, tx_handler_controller, network_starter, sync_service) =
            sc_service::build_network(sc_service::BuildNetworkParams {
                config: &config,
                client: client.clone(),
                transaction_pool: transaction_pool.clone(),
                spawn_handle: task_manager.spawn_handle(),
                import_queue,
                block_announce_validator_builder: None,
                warp_sync_params: None,
            })?;

        if config.offchain_worker.enabled {
            sc_service::build_offchain_workers(
                &config,
                task_manager.spawn_handle(),
                client.clone(),
                network.clone(),
            );
        }

        let rpc_extensions_builder = {
            let client = client.clone();
            let pool = transaction_pool.clone();

            Box::new(move |deny_unsafe, _| {
                let deps = crate::rpc::FullDeps {
                    client: client.clone(),
                    pool: pool.clone(),
                    deny_unsafe,
                };
                crate::rpc::create_full(deps).map_err(Into::into)
            })
        };

        let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
            network: network.clone(),
            client: client.clone(),
            keystore: keystore_container.sync_keystore(),
            task_manager: &mut task_manager,
            transaction_pool: transaction_pool.clone(),
            rpc_builder: rpc_extensions_builder,
            backend,
            system_rpc_tx,
            tx_handler_controller,
            sync_service,
            config,
            telemetry: telemetry.as_mut(),
        })?;

        network_starter.start_network();

        Ok(task_manager)
    }

    pub fn new_chain_ops(
        config: &Configuration,
    ) -> Result<
        (
            Arc<FullClient>,
            Arc<FullBackend>,
            sc_consensus::BasicQueue<Block, PrefixedMemoryDB<BlakeTwo256>>,
            TaskManager,
        ),
        ServiceError,
    > {
        let sc_service::PartialComponents {
            client,
            backend,
            mut task_manager,
            import_queue,
            ..
        } = new_partial(config)?;

        Ok((client, backend, import_queue, task_manager))
    }
}

mod rpc {
    use std::sync::Arc;
    use selendra_runtime::{opaque::Block, AccountId, Balance, BlockNumber, Hash, Index};
    use sp_api::ProvideRuntimeApi;
    use sp_blockchain::HeaderBackend;
    use sp_block_builder::BlockBuilder;
    use sc_rpc_api::DenyUnsafe;
    use sc_transaction_pool_api::TransactionPool;

    /// Full client dependencies.
    pub struct FullDeps<C, P> {
        /// The client instance to use.
        pub client: Arc<C>,
        /// Transaction pool instance.
        pub pool: Arc<P>,
        /// Whether to deny unsafe calls
        pub deny_unsafe: DenyUnsafe,
    }

    /// Instantiate all full RPC extensions.
    pub fn create_full<C, P>(
        deps: FullDeps<C, P>,
    ) -> jsonrpc_core::IoHandler<sc_rpc::Metadata>
    where
        C: ProvideRuntimeApi<Block>,
        C: HeaderBackend<Block> + 'static,
        C: Send + Sync + 'static,
        C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
        C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
        C::Api: BlockBuilder<Block>,
        P: TransactionPool + 'static,
    {
        use substrate_frame_rpc_system::{FullSystem, SystemApi};
        use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};

        let mut io = jsonrpc_core::IoHandler::default();
        let FullDeps {
            client,
            pool,
            deny_unsafe,
        } = deps;

        io.extend_with(SystemApi::to_delegate(FullSystem::new(
            client.clone(),
            pool,
            deny_unsafe,
        )));

        io.extend_with(TransactionPaymentApi::to_delegate(TransactionPayment::new(
            client.clone(),
        )));

        io
    }
}
