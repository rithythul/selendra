#![allow(missing_docs)]

pub mod aleph_cli;
use aleph_cli::AlephCli;

use crate::eth::EthConfiguration;

#[derive(Debug, clap::Parser)]
pub struct Cli {
	#[command(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[allow(missing_docs)]
	#[command(flatten)]
	pub run: sc_cli::RunCmd,

	#[command(flatten)]
	pub eth: EthConfiguration,

	#[command(flatten)]
	pub aleph: AlephCli,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
	/// Key management cli utilities
	#[command(subcommand)]
	Key(sc_cli::KeySubcommand),

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

	/// Sub-commands concerned with benchmarking.
	#[cfg(feature = "runtime-benchmarks")]
	#[command(subcommand)]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),

	/// Sub-commands concerned with benchmarking.
	#[cfg(not(feature = "runtime-benchmarks"))]
	Benchmark,

	/// Db meta columns information.
	FrontierDb(fc_cli::FrontierDbCmd),
}