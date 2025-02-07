#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    pallet_prelude::*,
    traits::{Currency, ReservableCurrency, ExistenceRequirement},
    PalletId,
};
use frame_system::pallet_prelude::*;
use sp_core::H160;
use sp_runtime::{
    traits::{AccountIdConversion, CheckedAdd, CheckedSub},
    ArithmeticError,
    Percent,
};
use sp_std::prelude::*;

// Constant for naming service configuration
const PALLET_ID: PalletId = PalletId(*b"sel/name");

// Pricing tiers for additional names
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum NameTier {
    Free,           // First two names
    Tier1,          // 3rd name
    Tier2,          // 4th name
    Tier3,          // 5th name
    Premium,        // Beyond 5 names
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: ReservableCurrency<Self::AccountId>;
        
        // Minimum and maximum name length
        #[pallet::constant]
        type MinNameLength: Get<u32>;
        #[pallet::constant]
        type MaxNameLength: Get<u32>;
        
        // Pricing configuration
        #[pallet::constant]
        type BasePricing: Get<BalanceOf<Self>>;
        
        // Maximum names per account
        #[pallet::constant]
        type MaxNamesPerAccount: Get<u32>;
    }

    // Pricing storage for dynamic pricing
    #[pallet::storage]
    #[pallet::getter(fn name_pricing)]
    pub type NamePricing<T: Config> = StorageMap<
        _, 
        Blake2_128Concat, 
        NameTier, 
        BalanceOf<T>
    >;

    // Name registration storage (enhanced)
    #[pallet::storage]
    #[pallet::getter(fn name_owner)]
    pub type NameOwnership<T: Config> = StorageMap<
        _, 
        Blake2_128Concat, 
        BoundedVec<u8, T::MaxNameLength>, 
        (T::AccountId, NameTier)
    >;

    // Reverse mapping (account to names with tiers)
    #[pallet::storage]
    #[pallet::getter(fn account_names)]
    pub type AccountNames<T: Config> = StorageMap<
        _, 
        Blake2_128Concat, 
        T::AccountId, 
        BoundedVec<(BoundedVec<u8, T::MaxNameLength>, NameTier), ConstU32<10>>
    >;

    // Ethereum address mapping
    #[pallet::storage]
    #[pallet::getter(fn name_to_address)]
    pub type NameToAddress<T: Config> = StorageMap<
        _, 
        Blake2_128Concat, 
        BoundedVec<u8, T::MaxNameLength>, 
        H160
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A name was successfully registered
        NameRegistered {
            name: BoundedVec<u8, T::MaxNameLength>,
            owner: T::AccountId,
            tier: NameTier,
            price_paid: BalanceOf<T>,
        },
        /// Name pricing updated
        NamePricingUpdated {
            tier: NameTier,
            new_price: BalanceOf<T>,
        },
        /// A name was transferred to a new owner
        NameTransferred {
            name: BoundedVec<u8, T::MaxNameLength>,
            previous_owner: T::AccountId,
            new_owner: T::AccountId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Name is too short
        NameTooShort,
        /// Name is too long
        NameTooLong,
        /// Name already taken
        NameAlreadyTaken,
        /// Not the owner of the name
        NotNameOwner,
        /// Invalid name characters
        InvalidNameCharacters,
        /// Insufficient balance to register name
        InsufficientBalance,
        /// Maximum names per account reached
        MaxNamesExceeded,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register a new name with dynamic pricing
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn register_name(
            origin: OriginFor<T>, 
            name: BoundedVec<u8, T::MaxNameLength>,
            address: H160,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Validate name
            Self::validate_name(&name)?;

            // Check name availability
            ensure!(!NameOwnership::<T>::contains_key(&name), Error::<T>::NameAlreadyTaken);

            // Determine name tier and price
            let (tier, price) = Self::calculate_name_tier_and_price(&sender)?;

            // Charge for name registration if not free
            if price > Zero::zero() {
                T::Currency::transfer(
                    &sender, 
                    &Self::account_id(), 
                    price, 
                    ExistenceRequirement::KeepAlive
                )?;
            }

            // Store name ownership
            NameOwnership::<T>::insert(&name, (&sender, tier.clone()));

            // Update account names
            AccountNames::<T>::mutate(&sender, |names| {
                if let Some(names) = names {
                    if names.len() < T::MaxNamesPerAccount::get() as usize {
                        names.push((name.clone(), tier.clone())).unwrap_or_default();
                    } else {
                        return Err(Error::<T>::MaxNamesExceeded);
                    }
                } else {
                    *names = Some(vec![(name.clone(), tier.clone())].try_into().unwrap_or_default());
                }
                Ok(())
            })?;

            // Store name to address mapping
            NameToAddress::<T>::insert(&name, address);

            // Emit event
            Self::deposit_event(Event::NameRegistered { 
                name, 
                owner: sender, 
                tier,
                price_paid: price
            });

            Ok(())
        }

        /// Update name pricing (governance action)
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn update_name_pricing(
            origin: OriginFor<T>, 
            tier: NameTier,
            new_price: BalanceOf<T>,
        ) -> DispatchResult {
            // Ensure only governance can update pricing
            T::EnsureOrigin::ensure_origin(origin)?;

            // Update pricing
            NamePricing::<T>::insert(tier.clone(), new_price);

            // Emit pricing update event
            Self::deposit_event(Event::NamePricingUpdated { 
                tier, 
                new_price 
            });

            Ok(())
        }

        /// Transfer name ownership
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn transfer_name(
            origin: OriginFor<T>, 
            name: BoundedVec<u8, T::MaxNameLength>,
            new_owner: T::AccountId,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Ensure sender is current owner
            let current_owner = NameOwnership::<T>::get(&name)
                .ok_or(Error::<T>::NotNameOwner)?;
            ensure!(current_owner.0 == sender, Error::<T>::NotNameOwner);

            // Update ownership
            NameOwnership::<T>::insert(&name, (&new_owner, current_owner.1.clone()));

            // Update account names
            AccountNames::<T>::mutate(&current_owner.0, |names| {
                if let Some(names) = names {
                    names.retain(|n| n.0 != &name);
                }
            });

            AccountNames::<T>::mutate(&new_owner, |names| {
                if let Some(names) = names {
                    if names.len() < T::MaxNamesPerAccount::get() as usize {
                        names.push((name.clone(), current_owner.1.clone())).unwrap_or_default();
                    } else {
                        return Err(Error::<T>::MaxNamesExceeded);
                    }
                } else {
                    *names = Some(vec![(name.clone(), current_owner.1.clone())].try_into().unwrap_or_default());
                }
                Ok(())
            })?;

            // Emit event
            Self::deposit_event(Event::NameTransferred { 
                name, 
                previous_owner: sender, 
                new_owner 
            });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        // Calculate account ID for the pallet
        fn account_id() -> T::AccountId {
            PALLET_ID.into_account_truncating()
        }

        // Name validation helper
        fn validate_name(name: &BoundedVec<u8, T::MaxNameLength>) -> DispatchResult {
            // Check length
            ensure!(
                name.len() >= T::MinNameLength::get() as usize, 
                Error::<T>::NameTooShort
            );
            ensure!(
                name.len() <= T::MaxNameLength::get() as usize, 
                Error::<T>::NameTooLong
            );

            // Validate characters (only lowercase letters and numbers)
            ensure!(
                name.iter().all(|&c| 
                    (c >= b'a' && c <= b'z') || (c >= b'0' && c <= b'9')
                ),
                Error::<T>::InvalidNameCharacters
            );

            Ok(())
        }

        // Calculate name tier and pricing
        fn calculate_name_tier_and_price(
            account: &T::AccountId
        ) -> Result<(NameTier, BalanceOf<T>), DispatchError> {
            // Get current names for the account
            let current_names = AccountNames::<T>::get(account)
                .unwrap_or_default();
            
            // Determine tier based on existing names
            let tier = match current_names.len() {
                0 | 1 => NameTier::Free,
                2 => NameTier::Tier1,
                3 => NameTier::Tier2,
                4 => NameTier::Tier3,
                _ => NameTier::Premium,
            };

            // Get base pricing and apply tier-based multiplier
            let base_price = T::BasePricing::get();
            let price = match tier {
                NameTier::Free => Zero::zero(),
                NameTier::Tier1 => 3_000_000_000_000_000_000u128.into(), // 3 SEL
                NameTier::Tier2 => 6_000_000_000_000_000_000u128.into(), // 6 SEL
                NameTier::Tier3 => 12_000_000_000_000_000_000u128.into(), // 12 SEL
                NameTier::Premium => 24_000_000_000_000_000_000u128.into(), // 24 SEL
            };

            // Optional: Check if custom pricing exists
            let final_price = NamePricing::<T>::get(&tier)
                .unwrap_or(price);

            Ok((tier, final_price))
        }

        // Resolve name to address
        pub fn resolve_name(name: &BoundedVec<u8, T::MaxNameLength>) -> Option<H160> {
            NameToAddress::<T>::get(name)
        }
    }

    // Default genesis configuration
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub initial_pricing: Vec<(NameTier, BalanceOf<T>)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                initial_pricing: vec![
                    (NameTier::Free, Zero::zero()),
                    (NameTier::Tier1, 3_000_000_000_000_000_000), // 3 SEL
                    (NameTier::Tier2, 6_000_000_000_000_000_000), // 6 SEL
                    (NameTier::Tier3, 12_000_000_000_000_000_000), // 12 SEL
                    (NameTier::Premium, 24_000_000_000_000_000_000), // 24 SEL
                ],
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            for (tier, price) in &self.initial_pricing {
                NamePricing::<T>::insert(tier, price);
            }
        }
    }
}
