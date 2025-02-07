//! Fee management pallet for Selendra Network
//! 
//! This pallet implements dynamic fee adjustment based on network usage
//! and provides mechanisms for fee distribution to validators.
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement, WithdrawReasons},
        weights::Weight,
    };
    use frame_system::pallet_prelude::*;
    use selendra_primitives::{Balance, Fee};
    use sp_runtime::traits::{Zero, Saturating};

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The currency type
        type Currency: Currency<Self::AccountId>;

        /// Base fee
        #[pallet::constant]
        type BaseFee: Get<Balance>;

        /// Maximum fee multiplier
        #[pallet::constant]
        type MaxMultiplier: Get<u32>;

        /// Target block fullness (0-100)
        #[pallet::constant]
        type TargetBlockFullness: Get<u32>;

        /// Fee adjustment period in blocks
        #[pallet::constant]
        type AdjustmentPeriod: Get<BlockNumberFor<Self>>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type CurrentFee<T: Config> = StorageValue<_, Fee, ValueQuery>;

    #[pallet::storage]
    pub type BlockFees<T: Config> = StorageValue<_, Balance, ValueQuery>;

    #[pallet::storage]
    pub type NetworkUsage<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BlockNumberFor<T>,
        u32, // Block fullness percentage
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Fee parameters updated
        FeeUpdated {
            base: Balance,
            multiplier: u32,
            complexity: u32,
        },
        /// Fees collected for block
        FeesCollected {
            block_number: BlockNumberFor<T>,
            amount: Balance,
        },
        /// Fees distributed to validators
        FeesDistributed {
            block_number: BlockNumberFor<T>,
            amount: Balance,
            validator_count: u32,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Fee calculation overflow
        FeeOverflow,
        /// Invalid fee parameters
        InvalidFeeParameters,
        /// Distribution failed
        DistributionFailed,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            // Reset block fees
            BlockFees::<T>::put(Zero::zero());
            Weight::zero()
        }

        fn on_finalize(n: BlockNumberFor<T>) {
            // Record network usage
            let block_weight = frame_system::Pallet::<T>::block_weight();
            let max_weight = frame_system::Pallet::<T>::block_weights().max_block;
            let fullness = (block_weight.ref_time() * 100 / max_weight.ref_time()) as u32;
            NetworkUsage::<T>::insert(n, fullness);

            // Adjust fees if needed
            if Self::should_adjust_fees(n) {
                Self::adjust_fees();
            }

            // Distribute collected fees
            let fees = BlockFees::<T>::get();
            if !fees.is_zero() {
                Self::distribute_fees(n, fees);
            }
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Update fee parameters manually (governance)
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn update_fee_params(
            origin: OriginFor<T>,
            base: Balance,
            multiplier: u32,
            complexity: u32,
        ) -> DispatchResult {
            ensure_root(origin)?;

            ensure!(
                multiplier <= T::MaxMultiplier::get(),
                Error::<T>::InvalidFeeParameters
            );

            let fee = Fee {
                base,
                multiplier,
                complexity,
            };

            CurrentFee::<T>::put(fee);

            Self::deposit_event(Event::FeeUpdated {
                base,
                multiplier,
                complexity,
            });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Calculate fee for an operation
        pub fn calculate_fee(base_weight: Weight, complexity: u32) -> Result<Balance, Error<T>> {
            let fee = CurrentFee::<T>::get();
            let base = T::BaseFee::get()
                .saturating_mul(base_weight.ref_time() as u128)
                .saturating_div(1_000_000);

            let adjusted = base
                .saturating_mul(fee.multiplier as u128)
                .saturating_div(100);

            let complex = if complexity > 0 {
                adjusted
                    .saturating_mul(fee.complexity as u128)
                    .saturating_mul(complexity as u128)
                    .saturating_div(100)
            } else {
                Zero::zero()
            };

            Ok(adjusted.saturating_add(complex))
        }

        /// Record fee payment
        pub fn record_fee_payment(who: &T::AccountId, fee: Balance) -> DispatchResult {
            T::Currency::withdraw(
                who,
                fee,
                WithdrawReasons::FEE,
                ExistenceRequirement::KeepAlive,
            )?;

            BlockFees::<T>::mutate(|fees| {
                *fees = fees.saturating_add(fee);
            });

            Ok(())
        }

        /// Check if fees should be adjusted
        fn should_adjust_fees(n: BlockNumberFor<T>) -> bool {
            n % T::AdjustmentPeriod::get() == Zero::zero()
        }

        /// Adjust fees based on network usage
        fn adjust_fees() {
            let fee = CurrentFee::<T>::get();
            let usage: Vec<_> = NetworkUsage::<T>::iter().map(|(_, u)| u).collect();
            
            if usage.is_empty() {
                return;
            }

            let avg_usage: u32 = usage.iter().sum::<u32>() / usage.len() as u32;
            let target = T::TargetBlockFullness::get();

            let new_multiplier = if avg_usage > target {
                (fee.multiplier as u64)
                    .saturating_mul(110)
                    .saturating_div(100)
                    .min(T::MaxMultiplier::get() as u64) as u32
            } else {
                (fee.multiplier as u64)
                    .saturating_mul(90)
                    .saturating_div(100)
                    .max(1) as u32
            };

            if new_multiplier != fee.multiplier {
                let new_fee = Fee {
                    multiplier: new_multiplier,
                    ..fee
                };
                CurrentFee::<T>::put(new_fee);

                Self::deposit_event(Event::FeeUpdated {
                    base: new_fee.base,
                    multiplier: new_fee.multiplier,
                    complexity: new_fee.complexity,
                });
            }
        }

        /// Distribute collected fees to validators
        fn distribute_fees(block_number: BlockNumberFor<T>, amount: Balance) {
            use pallet_selendra_consensus::ValidatorSet;
            let validators = ValidatorSet::<T>::get();
            
            if validators.is_empty() {
                return;
            }

            let share = amount
                .checked_div(validators.len() as u128)
                .unwrap_or_default();

            if share.is_zero() {
                return;
            }

            let mut distributed = Balance::zero();
            for validator in validators.iter() {
                if let Ok(()) = T::Currency::deposit_creating(validator, share) {
                    distributed = distributed.saturating_add(share);
                }
            }

            Self::deposit_event(Event::FeesDistributed {
                block_number,
                amount: distributed,
                validator_count: validators.len() as u32,
            });
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
                SelendraFees: crate::pallet::{Pallet, Call, Storage, Event<T>},
            }
        );

        parameter_types! {
            pub const BlockHashCount: u64 = 250;
            pub const BaseFee: Balance = 100;
            pub const MaxMultiplier: u32 = 1000;
            pub const TargetBlockFullness: u32 = 50;
            pub const AdjustmentPeriod: BlockNumber = 10;
        }

        impl frame_system::Config for Test {
            // ... same as consensus mock
        }

        impl pallet_balances::Config for Test {
            type Balance = Balance;
            type RuntimeEvent = RuntimeEvent;
            type DustRemoval = ();
            type ExistentialDeposit = ExistentialDeposit;
            type AccountStore = System;
            type WeightInfo = ();
            type MaxLocks = ();
            type MaxReserves = ();
            type ReserveIdentifier = [u8; 8];
        }

        impl Config for Test {
            type RuntimeEvent = RuntimeEvent;
            type Currency = Balances;
            type BaseFee = BaseFee;
            type MaxMultiplier = MaxMultiplier;
            type TargetBlockFullness = TargetBlockFullness;
            type AdjustmentPeriod = AdjustmentPeriod;
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
    fn calculate_fee_works() {
        new_test_ext().execute_with(|| {
            let weight = Weight::from_parts(1_000_000, 0);
            let fee = SelendraFees::calculate_fee(weight, 0).unwrap();
            assert_eq!(fee, 100); // BaseFee
        });
    }

    #[test]
    fn fee_adjustment_works() {
        new_test_ext().execute_with(|| {
            // Simulate high usage
            for i in 0..10u64 {
                NetworkUsage::<Test>::insert(i, 80);
            }
            
            SelendraFees::adjust_fees();
            let fee = CurrentFee::<Test>::get();
            assert!(fee.multiplier > 1);
        });
    }
}
