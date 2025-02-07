//! Validator management pallet for Selendra Network
//! 
//! This pallet implements validator selection, staking, and rewards
//! distribution mechanisms.
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency, LockIdentifier, LockableCurrency},
        weights::Weight,
    };
    use frame_system::pallet_prelude::*;
    use selendra_primitives::{Balance, ValidatorStatus, JailReason};
    use sp_runtime::traits::{Zero, Saturating, CheckedAdd};

    const STAKING_ID: LockIdentifier = *b"staking ";

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The currency type
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId> + LockableCurrency<Self::AccountId>;

        /// Minimum stake required
        #[pallet::constant]
        type MinimumStake: Get<Balance>;

        /// Maximum validators
        #[pallet::constant]
        type MaxValidators: Get<u32>;

        /// Reward period in blocks
        #[pallet::constant]
        type RewardPeriod: Get<BlockNumberFor<Self>>;

        /// Slash percentage for offline
        #[pallet::constant]
        type OfflineSlash: Get<u32>;

        /// Slash percentage for equivocation
        #[pallet::constant]
        type EquivocationSlash: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type ValidatorStake<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Balance,
        ValueQuery,
    >;

    #[pallet::storage]
    pub type ValidatorRewards<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Balance,
        ValueQuery,
    >;

    #[pallet::storage]
    pub type ValidatorPerformance<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        ValidatorMetrics,
        ValueQuery,
    >;

    #[pallet::storage]
    pub type LastReward<T: Config> = StorageValue<_, BlockNumberFor<T>, ValueQuery>;

    #[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, Default)]
    pub struct ValidatorMetrics {
        /// Blocks produced
        pub blocks_produced: u32,
        /// Blocks missed
        pub blocks_missed: u32,
        /// Total uptime percentage
        pub uptime: u32,
        /// Performance score (0-100)
        pub performance_score: u32,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Validator staked
        ValidatorStaked {
            validator: T::AccountId,
            amount: Balance,
        },
        /// Validator unstaked
        ValidatorUnstaked {
            validator: T::AccountId,
            amount: Balance,
        },
        /// Validator slashed
        ValidatorSlashed {
            validator: T::AccountId,
            amount: Balance,
            reason: JailReason,
        },
        /// Rewards distributed
        RewardsDistributed {
            total_reward: Balance,
            validator_count: u32,
        },
        /// Performance updated
        PerformanceUpdated {
            validator: T::AccountId,
            metrics: ValidatorMetrics,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Insufficient stake
        InsufficientStake,
        /// Already staked
        AlreadyStaked,
        /// Not a validator
        NotValidator,
        /// Cannot unstake yet
        CannotUnstake,
        /// Too many validators
        TooManyValidators,
        /// Invalid amount
        InvalidAmount,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            // Distribute rewards if it's time
            if Self::should_reward(n) {
                Self::distribute_rewards();
                LastReward::<T>::put(n);
            }
            Weight::zero()
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Stake to become a validator
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn stake(
            origin: OriginFor<T>,
            amount: Balance,
        ) -> DispatchResult {
            let validator = ensure_signed(origin)?;

            ensure!(
                amount >= T::MinimumStake::get(),
                Error::<T>::InsufficientStake
            );

            ensure!(
                !ValidatorStake::<T>::contains_key(&validator),
                Error::<T>::AlreadyStaked
            );

            let validators_count = ValidatorStake::<T>::iter().count() as u32;
            ensure!(
                validators_count < T::MaxValidators::get(),
                Error::<T>::TooManyValidators
            );

            T::Currency::set_lock(
                STAKING_ID,
                &validator,
                amount,
                WithdrawReasons::all(),
            );

            ValidatorStake::<T>::insert(&validator, amount);
            ValidatorPerformance::<T>::insert(&validator, Default::default());

            Self::deposit_event(Event::ValidatorStaked {
                validator: validator.clone(),
                amount,
            });

            Ok(())
        }

        /// Unstake and leave validator set
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn unstake(
            origin: OriginFor<T>,
        ) -> DispatchResult {
            let validator = ensure_signed(origin)?;

            let stake = ValidatorStake::<T>::get(&validator);
            ensure!(stake > Zero::zero(), Error::<T>::NotValidator);

            // Check if validator can unstake (not jailed, etc)
            ensure!(
                Self::can_unstake(&validator),
                Error::<T>::CannotUnstake
            );

            T::Currency::remove_lock(STAKING_ID, &validator);
            
            ValidatorStake::<T>::remove(&validator);
            ValidatorPerformance::<T>::remove(&validator);

            Self::deposit_event(Event::ValidatorUnstaked {
                validator: validator.clone(),
                amount: stake,
            });

            Ok(())
        }

        /// Update validator performance metrics
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn update_performance(
            origin: OriginFor<T>,
            validator: T::AccountId,
            blocks_produced: u32,
            blocks_missed: u32,
        ) -> DispatchResult {
            ensure_root(origin)?;

            ensure!(
                ValidatorStake::<T>::contains_key(&validator),
                Error::<T>::NotValidator
            );

            let total_blocks = blocks_produced.saturating_add(blocks_missed);
            let uptime = if total_blocks > 0 {
                (blocks_produced * 100) / total_blocks
            } else {
                0
            };

            // Simple performance score calculation
            let performance_score = uptime;

            let metrics = ValidatorMetrics {
                blocks_produced,
                blocks_missed,
                uptime,
                performance_score,
            };

            ValidatorPerformance::<T>::insert(&validator, metrics.clone());

            Self::deposit_event(Event::PerformanceUpdated {
                validator,
                metrics,
            });

            Ok(())
        }

        /// Slash a validator
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn slash(
            origin: OriginFor<T>,
            validator: T::AccountId,
            reason: JailReason,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let stake = ValidatorStake::<T>::get(&validator);
            ensure!(stake > Zero::zero(), Error::<T>::NotValidator);

            let slash_percentage = match reason {
                JailReason::Offline => T::OfflineSlash::get(),
                JailReason::Equivocation => T::EquivocationSlash::get(),
                _ => 10, // Default 10% slash for other violations
            };

            let slash_amount = (stake * slash_percentage as u128) / 100;
            if slash_amount > Zero::zero() {
                T::Currency::slash(&validator, slash_amount);
                
                ValidatorStake::<T>::mutate(&validator, |s| {
                    *s = s.saturating_sub(slash_amount);
                });

                Self::deposit_event(Event::ValidatorSlashed {
                    validator: validator.clone(),
                    amount: slash_amount,
                    reason,
                });
            }

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Check if rewards should be distributed
        fn should_reward(now: BlockNumberFor<T>) -> bool {
            now >= LastReward::<T>::get() + T::RewardPeriod::get()
        }

        /// Distribute rewards to validators
        fn distribute_rewards() {
            let mut total_reward = Balance::zero();
            let mut validator_count = 0;

            // Calculate rewards based on performance
            for (validator, metrics) in ValidatorPerformance::<T>::iter() {
                let stake = ValidatorStake::<T>::get(&validator);
                if stake.is_zero() {
                    continue;
                }

                // Basic reward calculation
                let base_reward = stake / 100; // 1% of stake
                let performance_multiplier = metrics.performance_score;
                let reward = (base_reward * performance_multiplier as u128) / 100;

                if reward > Zero::zero() {
                    ValidatorRewards::<T>::mutate(&validator, |r| {
                        *r = r.saturating_add(reward);
                    });
                    total_reward = total_reward.saturating_add(reward);
                    validator_count += 1;
                }
            }

            if total_reward > Zero::zero() {
                Self::deposit_event(Event::RewardsDistributed {
                    total_reward,
                    validator_count,
                });
            }
        }

        /// Check if validator can unstake
        fn can_unstake(validator: &T::AccountId) -> bool {
            // Add additional checks here (e.g., lock period, ongoing slashes)
            true
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
                SelendraValidator: crate::pallet::{Pallet, Call, Storage, Event<T>},
            }
        );

        parameter_types! {
            pub const BlockHashCount: u64 = 250;
            pub const MinimumStake: Balance = 1000;
            pub const MaxValidators: u32 = 100;
            pub const RewardPeriod: BlockNumber = 100;
            pub const OfflineSlash: u32 = 10;
            pub const EquivocationSlash: u32 = 30;
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
            type MinimumStake = MinimumStake;
            type MaxValidators = MaxValidators;
            type RewardPeriod = RewardPeriod;
            type OfflineSlash = OfflineSlash;
            type EquivocationSlash = EquivocationSlash;
        }

        pub fn new_test_ext() -> sp_io::TestExternalities {
            let mut t = frame_system::GenesisConfig::default()
                .build_storage::<Test>()
                .unwrap();

            pallet_balances::GenesisConfig::<Test> {
                balances: vec![(1, 10000), (2, 10000)],
            }
            .assimilate_storage(&mut t)
            .unwrap();

            t.into()
        }
    }

    #[test]
    fn stake_works() {
        new_test_ext().execute_with(|| {
            assert_ok!(SelendraValidator::stake(
                RuntimeOrigin::signed(1),
                1000
            ));
            assert_eq!(ValidatorStake::<Test>::get(1), 1000);
        });
    }

    #[test]
    fn unstake_works() {
        new_test_ext().execute_with(|| {
            assert_ok!(SelendraValidator::stake(
                RuntimeOrigin::signed(1),
                1000
            ));
            assert_ok!(SelendraValidator::unstake(
                RuntimeOrigin::signed(1)
            ));
            assert_eq!(ValidatorStake::<Test>::get(1), 0);
        });
    }

    #[test]
    fn slash_works() {
        new_test_ext().execute_with(|| {
            assert_ok!(SelendraValidator::stake(
                RuntimeOrigin::signed(1),
                1000
            ));
            assert_ok!(SelendraValidator::slash(
                RuntimeOrigin::root(),
                1,
                JailReason::Offline
            ));
            assert_eq!(ValidatorStake::<Test>::get(1), 900); // 10% slash
        });
    }
}
