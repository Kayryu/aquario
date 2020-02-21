#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure, StorageMap, StorageValue,
};
use frame_system::{self as system, ensure_signed};

pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as Oracle {
        pub PriceOf get(price_of): map Vec<u8> => u64;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        Feed(AccountId, u64), // (symbol,value)
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        fn feed(origin, symbol: Vec<u8>, value: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            Self::deposit_event(RawEvent::Feed(symbol, value));
            Ok(())
        }
    }
}