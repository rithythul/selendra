//! Governance pallet for Selendra Network
//! 
//! This pallet implements on-chain governance including proposals,
//! voting, and automated execution of approved proposals.
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, Get, ReservableCurrency},
        weights::Weight,
    };
    use frame_system::pallet_prelude::*;
    use selendra_primitives::{Balance, Proposal, ProposalAction, EmergencyAction};
    use sp_runtime::traits::{Zero, Saturating, CheckedAdd};

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The currency type
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        /// Minimum deposit for proposals
        #[pallet::constant]
        type MinimumDeposit: Get<Balance>;

        /// Maximum proposal duration
        #[pallet::constant]
        type MaxProposalDuration: Get<BlockNumberFor<Self>>;

        /// Voting period duration
        #[pallet::constant]
        type VotingPeriod: Get<BlockNumberFor<Self>>;

        /// Maximum number of active proposals
        #[pallet::constant]
        type MaxActiveProposals: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type Proposals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash, // proposal_hash
        (Proposal, ProposalStatus<T::AccountId, BlockNumberFor<T>>),
        ValueQuery,
    >;

    #[pallet::storage]
    pub type Votes<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::Hash, // proposal_hash
        Blake2_128Concat,
        T::AccountId, // voter
        Vote,
        ValueQuery,
    >;

    #[pallet::storage]
    pub type ProposalCount<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo)]
    pub enum ProposalStatus<AccountId, BlockNumber> {
        Active {
            proposer: AccountId,
            deposit: Balance,
            end_block: BlockNumber,
        },
        Approved {
            approved_block: BlockNumber,
        },
        Rejected {
            rejected_block: BlockNumber,
        },
        Executed {
            executed_block: BlockNumber,
        },
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo)]
    pub enum Vote {
        Aye,
        Nay,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Proposal created
        ProposalCreated {
            proposal_hash: T::Hash,
            proposer: T::AccountId,
            proposal: Proposal,
        },
        /// Vote cast
        VoteCast {
            proposal_hash: T::Hash,
            voter: T::AccountId,
            vote: Vote,
        },
        /// Proposal approved
        ProposalApproved {
            proposal_hash: T::Hash,
        },
        /// Proposal rejected
        ProposalRejected {
            proposal_hash: T::Hash,
        },
        /// Proposal executed
        ProposalExecuted {
            proposal_hash: T::Hash,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Too many active proposals
        TooManyProposals,
        /// Proposal already exists
        ProposalExists,
        /// Invalid proposal
        InvalidProposal,
        /// Insufficient deposit
        InsufficientDeposit,
        /// Invalid voting period
        InvalidVotingPeriod,
        /// Already voted
        AlreadyVoted,
        /// Not active proposal
        NotActive,
        /// Cannot execute yet
        CannotExecute,
        /// Execution failed
        ExecutionFailed,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            // Process proposals that have ended voting
            Self::process_proposals(n);
            Weight::zero()
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new proposal
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn propose(
            origin: OriginFor<T>,
            proposal: Proposal,
            deposit: Balance,
        ) -> DispatchResult {
            let proposer = ensure_signed(origin)?;

            ensure!(
                deposit >= T::MinimumDeposit::get(),
                Error::<T>::InsufficientDeposit
            );

            ensure!(
                proposal.end_block <= T::MaxProposalDuration::get(),
                Error::<T>::InvalidVotingPeriod
            );

            let proposal_hash = T::Hashing::hash_of(&proposal);
            ensure!(
                !Proposals::<T>::contains_key(proposal_hash),
                Error::<T>::ProposalExists
            );

            let count = ProposalCount::<T>::get();
            ensure!(
                count < T::MaxActiveProposals::get(),
                Error::<T>::TooManyProposals
            );

            // Reserve deposit
            T::Currency::reserve(&proposer, deposit)?;

            let end_block = frame_system::Pallet::<T>::block_number()
                .saturating_add(T::VotingPeriod::get());

            let status = ProposalStatus::Active {
                proposer: proposer.clone(),
                deposit,
                end_block,
            };

            Proposals::<T>::insert(proposal_hash, (proposal.clone(), status));
            ProposalCount::<T>::put(count.saturating_add(1));

            Self::deposit_event(Event::ProposalCreated {
                proposal_hash,
                proposer,
                proposal,
            });

            Ok(())
        }

        /// Vote on a proposal
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn vote(
            origin: OriginFor<T>,
            proposal_hash: T::Hash,
            vote: Vote,
        ) -> DispatchResult {
            let voter = ensure_signed(origin)?;

            Proposals::<T>::try_mutate(proposal_hash, |proposal_entry| -> DispatchResult {
                let (proposal, status) = proposal_entry.as_mut()
                    .ok_or(Error::<T>::InvalidProposal)?;

                if let ProposalStatus::Active { end_block, .. } = status {
                    ensure!(
                        frame_system::Pallet::<T>::block_number() < *end_block,
                        Error::<T>::NotActive
                    );
                } else {
                    return Err(Error::<T>::NotActive.into());
                }

                ensure!(
                    !Votes::<T>::contains_key(proposal_hash, &voter),
                    Error::<T>::AlreadyVoted
                );

                Votes::<T>::insert(proposal_hash, &voter, vote.clone());

                Self::deposit_event(Event::VoteCast {
                    proposal_hash,
                    voter,
                    vote,
                });

                Ok(())
            })
        }

        /// Execute an approved proposal
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn execute(
            origin: OriginFor<T>,
            proposal_hash: T::Hash,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            Proposals::<T>::try_mutate(proposal_hash, |proposal_entry| -> DispatchResult {
                let (proposal, status) = proposal_entry.as_mut()
                    .ok_or(Error::<T>::InvalidProposal)?;

                if let ProposalStatus::Approved { .. } = status {
                    Self::do_execute_proposal(proposal_hash, proposal.clone())?;

                    *status = ProposalStatus::Executed {
                        executed_block: frame_system::Pallet::<T>::block_number(),
                    };

                    Self::deposit_event(Event::ProposalExecuted {
                        proposal_hash,
                    });

                    Ok(())
                } else {
                    Err(Error::<T>::CannotExecute.into())
                }
            })
        }
    }

    impl<T: Config> Pallet<T> {
        /// Process proposals that have ended voting
        fn process_proposals(now: BlockNumberFor<T>) {
            for (proposal_hash, (proposal, status)) in Proposals::<T>::iter() {
                if let ProposalStatus::Active { end_block, proposer, deposit } = status {
                    if now >= end_block {
                        let (approved, total_votes) = Self::tally_votes(proposal_hash);
                        
                        if approved && total_votes >= proposal.participation_requirement as usize {
                            // Proposal approved
                            Proposals::<T>::insert(proposal_hash, (proposal, ProposalStatus::Approved {
                                approved_block: now,
                            }));

                            // Return deposit
                            let _ = T::Currency::unreserve(&proposer, deposit);

                            Self::deposit_event(Event::ProposalApproved {
                                proposal_hash,
                            });
                        } else {
                            // Proposal rejected
                            Proposals::<T>::insert(proposal_hash, (proposal, ProposalStatus::Rejected {
                                rejected_block: now,
                            }));

                            // Slash deposit
                            T::Currency::slash_reserved(&proposer, deposit);

                            Self::deposit_event(Event::ProposalRejected {
                                proposal_hash,
                            });
                        }

                        ProposalCount::<T>::mutate(|count| {
                            *count = count.saturating_sub(1);
                        });
                    }
                }
            }
        }

        /// Count votes for a proposal
        fn tally_votes(proposal_hash: T::Hash) -> (bool, usize) {
            let mut aye_votes = 0;
            let mut total_votes = 0;

            for (_, vote) in Votes::<T>::iter_prefix(proposal_hash) {
                match vote {
                    Vote::Aye => aye_votes += 1,
                    Vote::Nay => {},
                }
                total_votes += 1;
            }

            (aye_votes * 100 > total_votes * 50, total_votes)
        }

        /// Execute an approved proposal
        fn do_execute_proposal(
            proposal_hash: T::Hash,
            proposal: Proposal,
        ) -> DispatchResult {
            match proposal.action {
                ProposalAction::Parameter { name, value } => {
                    // Parameter changes would be implemented here
                    // This is a placeholder for future implementation
                }
                ProposalAction::Upgrade { code } => {
                    // Runtime upgrades would be implemented here
                    // This is a placeholder for future implementation
                }
                ProposalAction::Emergency { action } => {
                    match action {
                        EmergencyAction::Pause(function) => {
                            // Pause functionality would be implemented here
                        }
                        EmergencyAction::Resume(function) => {
                            // Resume functionality would be implemented here
                        }
                        EmergencyAction::SetEmergencyContact { contact } => {
                            // Set emergency contact would be implemented here
                        }
                    }
                }
            }

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{assert_ok, assert_noop};
    use mock::*;

    mod mock {
        use super::*;
        use frame_support::parameter_types;
        use sp_runtime::BuildStorage;

        type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
        type Block = frame_system::mocking::MockBlock<Test>;

        frame_support::construct_runtime!(
            pub enum Test where
                Block = Block,
                NodeBlock = Block,
                UncheckedExtrinsic = UncheckedExtrinsic,
            {
                System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
                Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
                SelendraGovernance: crate::pallet::{Pallet, Call, Storage, Event<T>},
            }
        );

        parameter_types! {
            pub const BlockHashCount: u64 = 250;
            pub const MinimumDeposit: Balance = 100;
            pub const MaxProposalDuration: BlockNumber = 1000;
            pub const VotingPeriod: BlockNumber = 100;
            pub const MaxActiveProposals: u32 = 10;
        }

        impl frame_system::Config for Test {
            // ... same as previous mocks
        }

        impl pallet_balances::Config for Test {
            // ... same as previous mocks
        }

        impl Config for Test {
            type RuntimeEvent = RuntimeEvent;
            type Currency = Balances;
            type MinimumDeposit = MinimumDeposit;
            type MaxProposalDuration = MaxProposalDuration;
            type VotingPeriod = VotingPeriod;
            type MaxActiveProposals = MaxActiveProposals;
        }

        pub fn new_test_ext() -> sp_io::TestExternalities {
            let mut t = frame_system::GenesisConfig::default()
                .build_storage::<Test>()
                .unwrap();

            pallet_balances::GenesisConfig::<Test> {
                balances: vec![(1, 1000), (2, 1000)],
            }
            .assimilate_storage(&mut t)
            .unwrap();

            t.into()
        }
    }

    #[test]
    fn propose_works() {
        new_test_ext().execute_with(|| {
            let proposal = Proposal {
                title: b"Test".to_vec(),
                description: b"Test Proposal".to_vec(),
                action: ProposalAction::Parameter {
                    name: b"param".to_vec(),
                    value: b"value".to_vec(),
                },
                end_block: 100,
                participation_requirement: 50,
                approval_requirement: 50,
            };

            assert_ok!(SelendraGovernance::propose(
                RuntimeOrigin::signed(1),
                proposal,
                100
            ));
        });
    }

    #[test]
    fn vote_works() {
        new_test_ext().execute_with(|| {
            // First create a proposal
            let proposal = Proposal {
                title: b"Test".to_vec(),
                description: b"Test Proposal".to_vec(),
                action: ProposalAction::Parameter {
                    name: b"param".to_vec(),
                    value: b"value".to_vec(),
                },
                end_block: 100,
                participation_requirement: 50,
                approval_requirement: 50,
            };

            assert_ok!(SelendraGovernance::propose(
                RuntimeOrigin::signed(1),
                proposal.clone(),
                100
            ));

            let proposal_hash = <Test as frame_system::Config>::Hashing::hash_of(&proposal);

            assert_ok!(SelendraGovernance::vote(
                RuntimeOrigin::signed(2),
                proposal_hash,
                Vote::Aye
            ));
        });
    }
}
