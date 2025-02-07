//! Core primitives for the Selendra Network.
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{
    generic,
    traits::{BlakeTwo256, IdentifyAccount, Verify},
    MultiSignature, OpaqueExtrinsic,
};

/// Type for block number.
pub type BlockNumber = u32;

/// Type for header hash.
pub type Hash = sp_core::H256;

/// Type for account ID.
pub type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Type for balance values.
pub type Balance = u128;

/// Type for index of a transaction.
pub type Nonce = u32;

/// Type for hashing blocks.
pub type Hashing = BlakeTwo256;

/// The block header type.
pub type Header = generic::Header<BlockNumber, Hashing>;

/// The block type.
pub type Block = generic::Block<Header, OpaqueExtrinsic>;

/// Signature type.
pub type Signature = MultiSignature;

/// Asset ID type
pub type AssetId = u32;

/// Timestamp type
pub type Moment = u64;

/// Fee calculation primitive
#[derive(Encode, Decode, Clone, TypeInfo)]
pub struct Fee {
    /// Base fee in SEL
    pub base: Balance,
    /// Fee multiplier based on network congestion
    pub multiplier: u32,
    /// Additional fee for complex operations
    pub complexity: u32,
}

/// Validator status
#[derive(Encode, Decode, Clone, TypeInfo)]
pub enum ValidatorStatus {
    /// Validator is active
    Active,
    /// Validator is inactive
    Inactive,
    /// Validator is jailed
    Jailed {
        /// Reason for being jailed
        reason: JailReason,
        /// Block number when jailing expires
        until: BlockNumber,
    },
}

/// Reasons for jailing a validator
#[derive(Encode, Decode, Clone, TypeInfo)]
pub enum JailReason {
    /// Validator was offline
    Offline,
    /// Validator equivocated
    Equivocation,
    /// Validator had poor performance
    PoorPerformance,
    /// Validator violated protocol rules
    ProtocolViolation,
}

/// Bridge message types
#[derive(Encode, Decode, Clone, TypeInfo)]
pub enum BridgeMessage {
    /// Transfer assets between chains
    Transfer {
        /// Asset being transferred
        asset: AssetId,
        /// Amount being transferred
        amount: Balance,
        /// Recipient on target chain
        recipient: Vec<u8>,
        /// Target chain identifier
        target_chain: u32,
    },
    /// Execute contract call on target chain
    ContractCall {
        /// Target contract address
        contract: Vec<u8>,
        /// Call data
        data: Vec<u8>,
        /// Gas limit
        gas_limit: u64,
        /// Target chain identifier
        target_chain: u32,
    },
}

/// Governance proposal
#[derive(Encode, Decode, Clone, TypeInfo)]
pub struct Proposal {
    /// Proposal title
    pub title: Vec<u8>,
    /// Proposal description
    pub description: Vec<u8>,
    /// Proposed action
    pub action: ProposalAction,
    /// Block when voting ends
    pub end_block: BlockNumber,
    /// Required participation rate (0-100)
    pub participation_requirement: u32,
    /// Required approval rate (0-100)
    pub approval_requirement: u32,
}

/// Types of governance actions
#[derive(Encode, Decode, Clone, TypeInfo)]
pub enum ProposalAction {
    /// Change runtime parameter
    Parameter {
        /// Name of parameter
        name: Vec<u8>,
        /// New value
        value: Vec<u8>,
    },
    /// Upgrade runtime
    Upgrade {
        /// New runtime code
        code: Vec<u8>,
    },
    /// Emergency action
    Emergency {
        /// Action to take
        action: EmergencyAction,
    },
}

/// Emergency actions
#[derive(Encode, Decode, Clone, TypeInfo)]
pub enum EmergencyAction {
    /// Pause specific functionality
    Pause(Vec<u8>),
    /// Resume specific functionality
    Resume(Vec<u8>),
    /// Set emergency contact
    SetEmergencyContact {
        /// Contact information
        contact: Vec<u8>,
    },
}
