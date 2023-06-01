use std::fmt::{Debug, Display};

use codec::{Decode, Encode};
use selendra_primitives::BlockNumber;
use sp_runtime::traits::{CheckedSub, Header as SubstrateHeader, One};

use crate::{
	sync::{Header, Justification as JustificationT},
	BlockId, SelendraJustification,
};

mod chain_status;
mod finalizer;
mod status_notifier;
mod translator;
mod verification;

pub use chain_status::SubstrateChainStatus;
pub use status_notifier::SubstrateChainStatusNotifier;
pub use translator::Error as TranslateError;
pub use verification::{SessionVerifier, SubstrateFinalizationInfo, VerifierCache};

impl<H: SubstrateHeader<Number = BlockNumber>> Header for H {
	type Identifier = BlockId<H>;

	fn id(&self) -> Self::Identifier {
		BlockId { hash: self.hash(), number: *self.number() }
	}

	fn parent_id(&self) -> Option<Self::Identifier> {
		let number = self.number().checked_sub(&One::one())?;
		Some(BlockId { hash: *self.parent_hash(), number })
	}
}

/// Proper `SelendraJustification` or a variant indicating virtual justification
/// for the genesis block, which is the only block that can be the top finalized
/// block with no proper justification.
#[derive(Clone, Debug, Encode, Decode)]
pub enum InnerJustification {
	SelendraJustification(SelendraJustification),
	Genesis,
}

/// A justification, including the related header.
#[derive(Clone, Debug, Encode, Decode)]
pub struct Justification<H: SubstrateHeader<Number = BlockNumber>> {
	header: H,
	inner_justification: InnerJustification,
}

impl<H: SubstrateHeader<Number = BlockNumber>> Justification<H> {
	pub fn selendra_justification(
		header: H,
		selendra_justification: SelendraJustification,
	) -> Self {
		Justification {
			header,
			inner_justification: InnerJustification::SelendraJustification(selendra_justification),
		}
	}

	pub fn genesis_justification(header: H) -> Self {
		Justification { header, inner_justification: InnerJustification::Genesis }
	}
}

impl<H: SubstrateHeader<Number = BlockNumber>> Header for Justification<H> {
	type Identifier = BlockId<H>;

	fn id(&self) -> Self::Identifier {
		self.header().id()
	}

	fn parent_id(&self) -> Option<Self::Identifier> {
		self.header().parent_id()
	}
}

impl<H: SubstrateHeader<Number = BlockNumber>> JustificationT for Justification<H> {
	type Header = H;
	type Unverified = Self;

	fn header(&self) -> &Self::Header {
		&self.header
	}

	fn into_unverified(self) -> Self::Unverified {
		self
	}
}

/// Translates raw selendra justifications into ones acceptable to sync.
pub trait JustificationTranslator<H: SubstrateHeader<Number = BlockNumber>>: Send + Sync {
	type Error: Display + Debug;

	fn translate(
		&self,
		raw_justification: SelendraJustification,
		block_id: BlockId<H>,
	) -> Result<Justification<H>, Self::Error>;
}
