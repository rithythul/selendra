//! Consensus pallet for Selendra Network
//! 
//! This pallet implements a hybrid consensus mechanism combining:
//! - BABE for block production
//! - GRANDPA for finality
//! - Proof of Stake for validator selection
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, Get, OnTimestampSet},
    };
    use frame_system::pallet_prelude::*;
    use selendra_primitives::{BlockNumber, ValidatorStatus};
    use sp_runtime::traits::{Zero, CheckedAdd};

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Minimum number of validators
        #[pallet::constant]
        type MinValidators: Get<u32>;

        /// Maximum number of validators
        #[pallet::constant]
        type MaxValidators: Get<u32>;

        /// Blocks per epoch
        #[pallet::constant]
        type EpochLength: Get<BlockNumber>;

        /// Time until finality
        #[pallet::constant]
        type FinalityLag: Get<BlockNumber>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type CurrentEpoch<T> = StorageValue<_, BlockNumber, ValueQuery>;

    #[pallet::storage]
    pub type Validators<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        ValidatorStatus,
        ValueQuery,
    >;

    #[pallet::storage]
    pub type ValidatorSet<T: Config> = StorageValue<
        _,
        BoundedVec<T::AccountId, T::MaxValidators>,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// New epoch started
        NewEpoch {
            epoch_index: BlockNumber,
            start_block: BlockNumber,
        },
        /// Validator added to set
        ValidatorAdded {
            validator: T::AccountId,
        },
        /// Validator removed from set
        ValidatorRemoved {
            validator: T::AccountId,
        },
        /// Validator status updated
        ValidatorStatusUpdated {
            validator: T::AccountId,
            status: ValidatorStatus,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Too many validators
        TooManyValidators,
        /// Too few validators
        TooFewValidators,
        /// Validator already exists
        ValidatorExists,
        /// Validator not found
        ValidatorNotFound,
        /// Invalid status transition
        InvalidStatusTransition,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            // Check if we need to start new epoch
            if Self::should_start_new_epoch(n) {
                Self::rotate_validator_set();
                Self::start_new_epoch(n);
            }
            Weight::zero()
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Add a new validator
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn add_validator(
            origin: OriginFor<T>,
            validator: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;

            ensure!(
                !Validators::<T>::contains_key(&validator),
                Error::<T>::ValidatorExists
            );

            let current_count = ValidatorSet::<T>::get().len() as u32;
            ensure!(
                current_count < T::MaxValidators::get(),
                Error::<T>::TooManyValidators
            );

            Validators::<T>::insert(&validator, ValidatorStatus::Active);

            Self::deposit_event(Event::ValidatorAdded {
                validator: validator.clone(),
            });

            Ok(())
        }

        /// Remove a validator
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn remove_validator(
            origin: OriginFor<T>,
            validator: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;

            ensure!(
                Validators::<T>::contains_key(&validator),
                Error::<T>::ValidatorNotFound
            );

            let current_count = ValidatorSet::<T>::get().len() as u32;
            ensure!(
                current_count > T::MinValidators::get(),
                Error::<T>::TooFewValidators
            );

            Validators::<T>::remove(&validator);

            Self::deposit_event(Event::ValidatorRemoved {
                validator: validator.clone(),
            });

            Ok(())
        }

        /// Update validator status
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn update_validator_status(
            origin: OriginFor<T>,
            validator: T::AccountId,
            status: ValidatorStatus,
        ) -> DispatchResult {
            ensure_root(origin)?;

            Validators::<T>::try_mutate(&validator, |current_status| {
                if Self::is_valid_transition(current_status, &status) {
                    *current_status = status.clone();
                    Self::deposit_event(Event::ValidatorStatusUpdated {
                        validator: validator.clone(),
                        status,
                    });
                    Ok(())
                } else {
                    Err(Error::<T>::InvalidStatusTransition.into())
                }
            })
        }
    }

    impl<T: Config> Pallet<T> {
        /// Check if we should start a new epoch
        fn should_start_new_epoch(block_number: BlockNumberFor<T>) -> bool {
            let epoch_length: BlockNumberFor<T> = T::EpochLength::get().into();
            block_number % epoch_length == Zero::zero()
        }

        /// Start a new epoch
        fn start_new_epoch(block_number: BlockNumberFor<T>) {
            let new_epoch = CurrentEpoch::<T>::get().checked_add(&One::one())
                .expect("Epoch number overflow");
            
            CurrentEpoch::<T>::put(new_epoch);

            Self::deposit_event(Event::NewEpoch {
                epoch_index: new_epoch,
                start_block: block_number,
            });
        }

        /// Rotate the validator set
        fn rotate_validator_set() {
            // Get active validators
            let active_validators: Vec<_> = Validators::<T>::iter()
                .filter(|(_, status)| matches!(status, ValidatorStatus::Active))
                .map(|(v, _)| v)
                .collect();

            // Create new validator set
            let new_set: BoundedVec<_, T::MaxValidators> = active_validators
                .try_into()
                .expect("Validator count checked in add/remove; qed");

            ValidatorSet::<T>::put(new_set);
        }

        /// Check if status transition is valid
        fn is_valid_transition(current: &ValidatorStatus, new: &ValidatorStatus) -> bool {
            match (current, new) {
                // Can always transition to jailed
                (_, ValidatorStatus::Jailed { .. }) => true,
                // Can't transition from jailed except to inactive
                (ValidatorStatus::Jailed { .. }, status) => matches!(status, ValidatorStatus::Inactive),
                // All other transitions allowed
                _ => true,
            }
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
                SelendraConsensus: crate::pallet::{Pallet, Call, Storage, Event<T>},
            }
        );

        parameter_types! {
            pub const BlockHashCount: u64 = 250;
            pub const MinValidators: u32 = 4;
            pub const MaxValidators: u32 = 100;
            pub const EpochLength: BlockNumber = 10;
            pub const FinalityLag: BlockNumber = 2;
        }

        impl frame_system::Config for Test {
            type BaseCallFilter = frame_support::traits::Everything;
            type BlockWeights = ();
            type BlockLength = ();
            type DbWeight = ();
            type RuntimeOrigin = RuntimeOrigin;
            type RuntimeCall = RuntimeCall;
            type Index = u64;
            type BlockNumber = u64;
            type Hash = H256;
            type Hashing = BlakeTwo256;
            type AccountId = u64;
            type Lookup = IdentityLookup<Self::AccountId>;
            type Header = Header;
            type RuntimeEvent = RuntimeEvent;
            type BlockHashCount = BlockHashCount;
            type Version = ();
            type PalletInfo = PalletInfo;
            type AccountData = ();
            type OnNewAccount = ();
            type OnKilledAccount = ();
            type SystemWeightInfo = ();
            type SS58Prefix = ();
            type OnSetCode = ();
            type MaxConsumers = frame_support::traits::ConstU32<16>;
        }

        impl Config for Test {
            type RuntimeEvent = RuntimeEvent;
            type MinValidators = MinValidators;
            type MaxValidators = MaxValidators;
            type EpochLength = EpochLength;
            type FinalityLag = FinalityLag;
        }

        pub fn new_test_ext() -> sp_io::TestExternalities {
            let t = GenesisConfig {
                system: Default::default(),
            }
            .build_storage()
            .unwrap();
            t.into()
        }
    }

    #[test]
    fn add_validator_works() {
        new_test_ext().execute_with(|| {
            assert_ok!(SelendraConsensus::add_validator(RuntimeOrigin::root(), 1));
            assert!(Validators::<Test>::contains_key(1));
            assert_eq!(
                Validators::<Test>::get(1),
                ValidatorStatus::Active
            );
        });
    }

    #[test]
    fn remove_validator_works() {
        new_test_ext().execute_with(|| {
            assert_ok!(SelendraConsensus::add_validator(RuntimeOrigin::root(), 1));
            assert_ok!(SelendraConsensus::remove_validator(RuntimeOrigin::root(), 1));
            assert!(!Validators::<Test>::contains_key(1));
        });
    }

    #[test]
    fn update_status_works() {
        new_test_ext().execute_with(|| {
            assert_ok!(SelendraConsensus::add_validator(RuntimeOrigin::root(), 1));
            assert_ok!(SelendraConsensus::update_validator_status(
                RuntimeOrigin::root(),
                1,
                ValidatorStatus::Jailed {
                    reason: JailReason::Offline,
                    until: 100,
                }
            ));
            assert_eq!(
                Validators::<Test>::get(1),
                ValidatorStatus::Jailed {
                    reason: JailReason::Offline,
                    until: 100,
                }
            );
        });
    }
}
