#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure, StorageMap, StorageValue,
};
use frame_system::{self as system, ensure_signed};

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as Token {
        pub Symbols get(symbols): Vec<Vec<u8>>;
        pub TotalSupplyOf get(total_supply_of): map Vec<u8> => u64;
        pub BalanceOf get(balance_of): map (Vec<u8>, T::AccountId) => u64;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        // notifies upon token transfers
        Transfer(Vec<u8>, AccountId, AccountId, u64), // (symbol, from, to, value)
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        // initialize the token
        // transfers the total_supply amout to the caller
        fn init(origin, symbol: Vec<u8>, total: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(!Symbols::exists(symbol), "Repeat symbol"); 

            <TotalSupplyOf<T>>::insert(symbol.clone(), total);
            <BalanceOf<T>>::insert((symbol, sender), total);

            Ok(())
        }

        // transfer tokens from one account to another
        fn transfer(_origin, symbol: Vec<u8>, to: T::AccountId, value: u64) -> DispatchResult {
            let sender = ensure_signed(_origin)?;
            let sender_balance = Self::balance_of((symbol.clone(), sender.clone()));
            ensure!(sender_balance >= value, "Not enough balance.");

            let updated_from_balance = sender_balance.checked_sub(value).ok_or("overflow in calculating balance")?;
            let receiver_balance = Self::balance_of((symbol.clone(), to.clone()));
            let updated_to_balance = receiver_balance.checked_add(value).ok_or("overflow in calculating balance")?;

            // reduce sender's balance
            <BalanceOf<T>>::insert((symbol.clone(),sender.clone()), updated_from_balance);

            // increase receiver's balance
            <BalanceOf<T>>::insert((symbol.clone(), to.clone()), updated_to_balance);

            Self::deposit_event(RawEvent::Transfer(symbol, sender, to, value));

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    pub fn burn(to: T::AccountId, symbol: Vec<u8>, value: u64) -> DispatchResult {
        let to_balance = Self::balance_of((symbol.clone(), to.clone()));
        let updated_to_balance = to_balance.checked_sub(value).ok_or("overflow in calculating balance")?;
        <BalanceOf<T>>::insert((symbol.clone(),to), updated_from_balance);
        Ok(())
    }

    pub fn mint(to: T::AccountId, symbol: Vec<u8>, value: u64) -> DispatchResult {
        let to_balance = Self::balance_of((symbol.clone(), to.clone()));
        let updated_to_balance = to_balance.checked_add(value).ok_or("overflow in calculating balance")?;
        <BalanceOf<T>>::insert((symbol.clone(),to), updated_from_balance);
        Ok(())
    }
}