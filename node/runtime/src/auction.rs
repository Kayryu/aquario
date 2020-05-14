#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure, StorageMap, StorageValue,
};
use frame_system::{self as system, ensure_signed};
use token;
use cdp::{self, CdpOf}
use oracle::PriceOf;

pub trait Trait: system::Trait + cdp::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

pub struct AuctionBid<AccountId> {
    pub id: u64,
    pub debit: u64,
    pub user: AccountId,
    pub start_time: u64,
    pub end_time: u64,
}

decl_storage! {
    trait Store for Module<T: Trait> as Auction {
        pub AuctionDuration get(total_auction): u64;
        pub TotalAuction get(total_auction): u64;
        pub AuctionBidOf get(auction_bit_of): map u64 => AuctionBid<T::AccountId>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        // notifies upon token transfers
        Transfer(Vec<u8>, AccountId, AccountId, u64), // (from, to, value)
        AuctionBidOpened(Vec<u8>, u64, u64), //(pawn, cdp_id, bid_id)
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        pub fn liquidate(origin, pawn: Vec<u8>, id: u64) -> DispatchResult {
            <cdp::Module<T>>::update_fee(id);
            let cdp = CdpOf::get(id);
            // get price
            let price = PriceOf::get(pawn);
            ensure!(cdp.low() > price, "Cpd is save");
            
            // check overflow
            let total = TotalAuction::get() + 1;
            let mut bid = AuctionBid::default();

            bid.id = total;
            // TODO others

            <AuctionBidOf<T>>::insert(total, bid);
            TotalAuction::put(total);
            CdpOf::remove(id);
        }

    }
}