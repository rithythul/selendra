use std::{fmt::Debug, time::Instant};

use futures::channel::mpsc::{TrySendError, UnboundedSender};
use log::{debug, trace, warn};
use sc_consensus::{
	BlockCheckParams, BlockImport, BlockImportParams, ImportResult, JustificationImport,
};
use selendra_primitives::{BlockNumber, SELENDRA_ENGINE_ID};
use sp_consensus::{BlockOrigin, Error as ConsensusError};
use sp_runtime::{
	traits::{Block as BlockT, Header},
	Justification as SubstrateJustification,
};

use crate::{
	justification::{backwards_compatible_decode, DecodeError},
	metrics::{Checkpoint, Metrics},
	sync::substrate::{Justification, JustificationTranslator},
	BlockId,
};

/// A wrapper around a block import that also marks the start and end of the import of every block
/// in the metrics, if provided.
#[derive(Clone)]
pub struct TracingBlockImport<B, I>
where
	B: BlockT,
	I: BlockImport<B> + Send + Sync,
{
	inner: I,
	metrics: Metrics<<B::Header as Header>::Hash>,
}

impl<B, I> TracingBlockImport<B, I>
where
	B: BlockT,
	I: BlockImport<B> + Send + Sync,
{
	pub fn new(inner: I, metrics: Metrics<<B::Header as Header>::Hash>) -> Self {
		TracingBlockImport { inner, metrics }
	}
}
#[async_trait::async_trait]
impl<B, I> BlockImport<B> for TracingBlockImport<B, I>
where
	B: BlockT,
	I: BlockImport<B> + Send + Sync,
{
	type Error = I::Error;
	type Transaction = I::Transaction;

	async fn check_block(
		&mut self,
		block: BlockCheckParams<B>,
	) -> Result<ImportResult, Self::Error> {
		self.inner.check_block(block).await
	}

	async fn import_block(
		&mut self,
		block: BlockImportParams<B, Self::Transaction>,
	) -> Result<ImportResult, Self::Error> {
		let post_hash = block.post_hash();
		self.metrics.report_block(post_hash, Instant::now(), Checkpoint::Importing);

		let result = self.inner.import_block(block).await;

		if let Ok(ImportResult::Imported(_)) = &result {
			self.metrics.report_block(post_hash, Instant::now(), Checkpoint::Imported);
		}
		result
	}
}

/// A wrapper around a block import that also extracts any present jsutifications and send them to
/// our components which will process them further and possibly finalize the block. It also makes
/// blocks from major sync import as if they came from normal sync.
#[derive(Clone)]
pub struct SelendraBlockImport<B, I, JT>
where
	B: BlockT,
	B::Header: Header<Number = BlockNumber>,
	I: BlockImport<B> + Clone + Send,
	JT: JustificationTranslator<B::Header>,
{
	inner: I,
	justification_tx: UnboundedSender<Justification<<B as BlockT>::Header>>,
	translator: JT,
}

#[derive(Debug)]
enum SendJustificationError<H: Header<Number = BlockNumber>, TE: Debug> {
	Send(TrySendError<Justification<H>>),
	Consensus(Box<ConsensusError>),
	Decode(DecodeError),
	Translate(TE),
}

impl<H: Header<Number = BlockNumber>, TE: Debug> From<DecodeError>
	for SendJustificationError<H, TE>
{
	fn from(decode_error: DecodeError) -> Self {
		Self::Decode(decode_error)
	}
}

impl<B, I, JT> SelendraBlockImport<B, I, JT>
where
	B: BlockT,
	B::Header: Header<Number = BlockNumber>,
	I: BlockImport<B> + Clone + Send,
	JT: JustificationTranslator<B::Header>,
{
	pub fn new(
		inner: I,
		justification_tx: UnboundedSender<Justification<B::Header>>,
		translator: JT,
	) -> SelendraBlockImport<B, I, JT> {
		SelendraBlockImport { inner, justification_tx, translator }
	}

	fn send_justification(
		&mut self,
		block_id: BlockId<B::Header>,
		justification: SubstrateJustification,
	) -> Result<(), SendJustificationError<B::Header, JT::Error>> {
		debug!(target: "selendra-justification", "Importing justification for block {}.", block_id);
		if justification.0 != SELENDRA_ENGINE_ID {
			return Err(SendJustificationError::Consensus(Box::new(ConsensusError::ClientImport(
				"Selendra can import only Selendra justifications.".into(),
			))))
		}
		let justification_raw = justification.1;
		let selendra_justification = backwards_compatible_decode(justification_raw)?;
		let justification = self
			.translator
			.translate(selendra_justification, block_id)
			.map_err(SendJustificationError::Translate)?;

		self.justification_tx
			.unbounded_send(justification)
			.map_err(SendJustificationError::Send)
	}
}

#[async_trait::async_trait]
impl<B, I, JT> BlockImport<B> for SelendraBlockImport<B, I, JT>
where
	B: BlockT,
	B::Header: Header<Number = BlockNumber>,
	I: BlockImport<B> + Clone + Send,
	JT: JustificationTranslator<B::Header>,
{
	type Error = I::Error;
	type Transaction = I::Transaction;

	async fn check_block(
		&mut self,
		block: BlockCheckParams<B>,
	) -> Result<ImportResult, Self::Error> {
		self.inner.check_block(block).await
	}

	async fn import_block(
		&mut self,
		mut block: BlockImportParams<B, Self::Transaction>,
	) -> Result<ImportResult, Self::Error> {
		let number = *block.header.number();
		let post_hash = block.post_hash();

		let justifications = block.justifications.take();
		if matches!(block.origin, BlockOrigin::NetworkInitialSync) {
			trace!(target: "selendra-justification", "Treating block {:?} {:?} from major sync as from a normal sync.", number, block.header.hash());
			block.origin = BlockOrigin::NetworkBroadcast;
		}

		debug!(target: "selendra-justification", "Importing block {:?} {:?} {:?}", number, block.header.hash(), block.post_hash());
		let result = self.inner.import_block(block).await;

		if let Ok(ImportResult::Imported(_)) = result {
			if let Some(justification) =
				justifications.and_then(|just| just.into_justification(SELENDRA_ENGINE_ID))
			{
				debug!(target: "selendra-justification", "Got justification along imported block {:?}", number);

				if let Err(e) = self.send_justification(
					BlockId::new(post_hash, number),
					(SELENDRA_ENGINE_ID, justification),
				) {
					warn!(target: "selendra-justification", "Error while receiving justification for block {:?}: {:?}", post_hash, e);
				}
			}
		}

		result
	}
}

#[async_trait::async_trait]
impl<B, I, JT> JustificationImport<B> for SelendraBlockImport<B, I, JT>
where
	B: BlockT,
	B::Header: Header<Number = BlockNumber>,
	I: BlockImport<B> + Clone + Send,
	JT: JustificationTranslator<B::Header>,
{
	type Error = ConsensusError;

	async fn on_start(&mut self) -> Vec<(B::Hash, BlockNumber)> {
		debug!(target: "selendra-justification", "On start called");
		Vec::new()
	}

	async fn import_justification(
		&mut self,
		hash: B::Hash,
		number: BlockNumber,
		justification: SubstrateJustification,
	) -> Result<(), Self::Error> {
		use SendJustificationError::*;
		debug!(target: "selendra-justification", "import_justification called on {:?}", justification);
		self.send_justification(BlockId::new(hash, number), justification)
			.map_err(|error| match error {
				Send(_) => ConsensusError::ClientImport(String::from(
					"Could not send justification to ConsensusParty",
				)),
				Consensus(e) => *e,
				Decode(e) => ConsensusError::ClientImport(format!(
					"Justification for block {:?} decoded incorrectly: {}",
					number, e
				)),
				Translate(e) => ConsensusError::ClientImport(format!(
					"Could not translate justification: {}",
					e
				)),
			})
	}
}
