//! Bridge pallet for Selendra Network
//! 
//! This pallet implements cross-chain communication and asset transfer
//! between Selendra and other blockchain networks.
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement, WithdrawReasons},
    };
    use frame_system::pallet_prelude::*;
    use selendra_primitives::{Balance, BridgeMessage, AssetId};
    use sp_runtime::traits::{Zero, CheckedAdd};

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The currency type
        type Currency: Currency<Self::AccountId>;

        /// Maximum message size
        #[pallet::constant]
        type MaxMessageSize: Get<u32>;

        /// Required confirmations for inbound messages
        #[pallet::constant]
        type RequiredConfirmations: Get<u32>;

        /// Timeout for outbound messages (in blocks)
        #[pallet::constant]
        type MessageTimeout: Get<BlockNumberFor<Self>>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type OutboundMessages<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        (BlockNumberFor<T>, T::Hash), // (block_number, message_hash)
        BridgeMessage,
        ValueQuery,
    >;

    #[pallet::storage]
    pub type InboundMessages<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash, // message_hash
        (BridgeMessage, u32), // (message, confirmations)
        ValueQuery,
    >;

    #[pallet::storage]
    pub type RelayerSet<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        bool,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Message sent to other chain
        MessageSent {
            message_hash: T::Hash,
            target_chain: u32,
        },
        /// Message received from other chain
        MessageReceived {
            message_hash: T::Hash,
            source_chain: u32,
        },
        /// Message confirmed by relayer
        MessageConfirmed {
            message_hash: T::Hash,
            confirmations: u32,
        },
        /// Message executed
        MessageExecuted {
            message_hash: T::Hash,
        },
        /// Relayer added
        RelayerAdded {
            relayer: T::AccountId,
        },
        /// Relayer removed
        RelayerRemoved {
            relayer: T::AccountId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Message too large
        MessageTooLarge,
        /// Invalid message
        InvalidMessage,
        /// Message already confirmed
        AlreadyConfirmed,
        /// Not enough confirmations
        NotEnoughConfirmations,
        /// Message expired
        MessageExpired,
        /// Not a relayer
        NotRelayer,
        /// Already a relayer
        AlreadyRelayer,
        /// Asset transfer failed
        TransferFailed,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            // Clean up expired messages
            Self::clean_expired_messages();
            Weight::zero()
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Send a message to another chain
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn send_message(
            origin: OriginFor<T>,
            message: BridgeMessage,
            target_chain: u32,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(
                scale_encode::Encode::encode(&message).len() <= T::MaxMessageSize::get() as usize,
                Error::<T>::MessageTooLarge
            );

            let message_hash = T::Hashing::hash_of(&message);
            let block_number = frame_system::Pallet::<T>::block_number();

            // Handle specific message types
            match &message {
                BridgeMessage::Transfer { asset, amount, .. } => {
                    // Lock tokens
                    Self::lock_tokens(&sender, *asset, *amount)?;
                }
                BridgeMessage::ContractCall { .. } => {
                    // Additional validation could be added here
                }
            }

            OutboundMessages::<T>::insert((block_number, message_hash), message);

            Self::deposit_event(Event::MessageSent {
                message_hash,
                target_chain,
            });

            Ok(())
        }

        /// Confirm an inbound message (relayer only)
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn confirm_message(
            origin: OriginFor<T>,
            message: BridgeMessage,
            source_chain: u32,
        ) -> DispatchResult {
            let relayer = ensure_signed(origin)?;
            ensure!(RelayerSet::<T>::get(&relayer), Error::<T>::NotRelayer);

            let message_hash = T::Hashing::hash_of(&message);

            InboundMessages::<T>::try_mutate(
                message_hash,
                |(stored_message, confirmations)| -> DispatchResult {
                    if *confirmations == 0 {
                        *stored_message = message.clone();
                    }
                    ensure!(*stored_message == message, Error::<T>::InvalidMessage);
                    
                    *confirmations = confirmations
                        .checked_add(1)
                        .ok_or(Error::<T>::AlreadyConfirmed)?;

                    Self::deposit_event(Event::MessageConfirmed {
                        message_hash,
                        confirmations: *confirmations,
                    });

                    if *confirmations >= T::RequiredConfirmations::get() {
                        Self::execute_message(message_hash, message.clone())?;
                    }

                    Ok(())
                },
            )
        }

        /// Add a relayer
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn add_relayer(
            origin: OriginFor<T>,
            relayer: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(!RelayerSet::<T>::get(&relayer), Error::<T>::AlreadyRelayer);

            RelayerSet::<T>::insert(&relayer, true);

            Self::deposit_event(Event::RelayerAdded {
                relayer: relayer.clone(),
            });

            Ok(())
        }

        /// Remove a relayer
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn remove_relayer(
            origin: OriginFor<T>,
            relayer: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(RelayerSet::<T>::get(&relayer), Error::<T>::NotRelayer);

            RelayerSet::<T>::remove(&relayer);

            Self::deposit_event(Event::RelayerRemoved {
                relayer: relayer.clone(),
            });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Clean up expired messages
        fn clean_expired_messages() {
            let current_block = frame_system::Pallet::<T>::block_number();
            let timeout = T::MessageTimeout::get();

            OutboundMessages::<T>::retain(|(block, _), _| {
                current_block.saturating_sub(*block) < timeout
            });
        }

        /// Execute a confirmed message
        fn execute_message(
            message_hash: T::Hash,
            message: BridgeMessage,
        ) -> DispatchResult {
            match message {
                BridgeMessage::Transfer { asset, amount, recipient, .. } => {
                    Self::unlock_tokens(&recipient.try_into().map_err(|_| Error::<T>::InvalidMessage)?, asset, amount)?;
                }
                BridgeMessage::ContractCall { contract, data, gas_limit, .. } => {
                    // Contract call execution would be implemented here
                    // This is a placeholder for future implementation
                }
            }

            Self::deposit_event(Event::MessageExecuted {
                message_hash,
            });

            Ok(())
        }

        /// Lock tokens for cross-chain transfer
        fn lock_tokens(
            sender: &T::AccountId,
            asset: AssetId,
            amount: Balance,
        ) -> DispatchResult {
            // For now, we only support the native token
            if asset == 0 {
                T::Currency::withdraw(
                    sender,
                    amount,
                    WithdrawReasons::TRANSFER,
                    ExistenceRequirement::KeepAlive,
                )?;
            }
            Ok(())
        }

        /// Unlock tokens received from other chain
        fn unlock_tokens(
            recipient: &T::AccountId,
            asset: AssetId,
            amount: Balance,
        ) -> DispatchResult {
            // For now, we only support the native token
            if asset == 0 {
                T::Currency::deposit_creating(recipient, amount);
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
                SelendraBridge: crate::pallet::{Pallet, Call, Storage, Event<T>},
            }
        );

        parameter_types! {
            pub const BlockHashCount: u64 = 250;
            pub const MaxMessageSize: u32 = 1024;
            pub const RequiredConfirmations: u32 = 2;
            pub const MessageTimeout: BlockNumber = 100;
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
            type MaxMessageSize = MaxMessageSize;
            type RequiredConfirmations = RequiredConfirmations;
            type MessageTimeout = MessageTimeout;
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
    fn send_message_works() {
        new_test_ext().execute_with(|| {
            let message = BridgeMessage::Transfer {
                asset: 0,
                amount: 100,
                recipient: vec![1, 2, 3],
                target_chain: 1,
            };

            assert_ok!(SelendraBridge::send_message(
                RuntimeOrigin::signed(1),
                message,
                1
            ));
        });
    }

    #[test]
    fn confirm_message_works() {
        new_test_ext().execute_with(|| {
            // Add relayer
            assert_ok!(SelendraBridge::add_relayer(
                RuntimeOrigin::root(),
                1
            ));

            let message = BridgeMessage::Transfer {
                asset: 0,
                amount: 100,
                recipient: vec![1, 2, 3],
                target_chain: 1,
            };

            assert_ok!(SelendraBridge::confirm_message(
                RuntimeOrigin::signed(1),
                message,
                1
            ));
        });
    }
}
