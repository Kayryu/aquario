#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure, StorageMap, StorageValue,
};
use frame_system::{self as system, ensure_signed};
use token;

type Amount = u64;
pub trait Trait: system::Trait + token::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

pub enum CdpAction {
    IncPawn,
    DecPawn,
    IncDebit,
    DecDebit
}

pub struct CollateralProperty {
    pub liquidation_ratio: Amount,
    pub liquidation_penalty: Amount,
    pub debit_ceiling: Amount,
    pub stability_fee: Amount,
}

pub struct CDP<AccountId> {
    pub id: u64,
    pub pawn: Vec<u8>,
    pub owner: AccountId,
    pub pawn_amount: Amount,
    pub debit_amount: Amount,
}

impl<AccountId> CDP<AccountId> {
    pub fn low(&self, property: &CollateralProperty) -> Amount {
        // debit * Liquidation ratio / pawn
        1
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as CdpManager {

        pub PropertyOf get(property_of): map Vec<u8> => CollateralProperty;
        pub StableToken get(stable_token): Vec<u8>;
        pub CdpOf get(cdp_of): map u64 => CDP<T::AccountId>;
        pub CdpTotalOf get(cdp_total_of): u64;

        pub PawnTotalOf get(pawn_total_of): map Vec<u8> => Amount;
        pub DebitTotalOf get(debit_total_of): map Vec<u8> => Amount;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        CdpOpened(AccountId, Vec<u8>, u64), //(actor, pawn, id)
        CdpUpdated(Vec<u8>, u64, value, CdpAction),
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// open a cdp
        fn open(origin, pawn: Vec<u8>, pawn_amount: Amount, debit_amount: Amount) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // check 
            check_open(sender.clone(), pawn, pawn_amount, debit_amount)?;
            
            // overflow check
            let cdp_total_of = CdpTotalOfOf::get() + 1;

            let mut cdp = CDP::default();
            cdp.id = cdp_total_of;
            cdp.pawn = pawn.clone();
            cdp.owner = sender.clone();
            cdp.pawn_amount = pawn_amount;
            cdp.debit_amount = debit_amount;

            // update token;
            <token::Module<T>>::burn(sender.clone(), pawn.clone(), pawn_amount)?;
            <token::Module<T>>::mint(sender.clone(), Self::stable_token::get(), debit_amount)?;

            // update pawn and debit;
            PawnTotalOf::mutate(pawn.clone(), |t| *t += pawn_amount);
            DebitTotalOf::mutate(pawn.clone(), |t| *t += debit_amount);

            // insert
            <CdpOf<T>>::insert(cdp_total_of, cdp);
            CdpTotalOfOf::put(cdp_total_of);
            Ok(())
        }

        // operator cdp
        fn update(origin, id: u64, value: Amount, action: CdpAction) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            let cdp = CdpOf::get(id);
            ensure!(sender == cdp.owner, "Not owner");

            let ausd = Self::stable_token::get();
            // update cdp;
            let new_cdp = update_cdp(sender.clone(), pawn, pawn_amount, debit_amount)?;
            match action {
                CdpAction::IncPawn => {
                    <token::Module<T>>::burn(sender.clone(), cdp.pawn.clone(), value)?;
                    PawnTotalOf::mutate(cdp.pawn.clone(), |t| *t += pawn_amount);
                },
                CdpAction::DecPawn => {
                    <token::Module<T>>::mint(sender.clone(), cdp.pawn.clone(), value)?;
                    PawnTotalOf::mutate(cdp.pawn.clone(), |t| *t -= pawn_amount);
                },
                CdpAction::IncDebit => {
                    <token::Module<T>>::mint(sender.clone(), ausd.clone(), value)?;
                    DebitTotalOf::mutate(cdp.pawn.clone(), |t| *t += pawn_amount);
                },
                CdpAction::DecDebit => {
                    // if debit == 0 should close cdp.
                    <token::Module<T>>::burn(sender.clone(), ausd.clone(), value)?;
                    DebitTotalOf::mutate(cdp.pawn.clone(), |t| *t -= pawn_amount);
                }
            }
            <CdpOf<T>>::insert(new_cdp.id, new_cdp);
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    pub fn update_cdp(to: T::AccountId, pawn: Vec<u8>, pawn_amount: u64, debit_amount: u64) -> DispatchResult {
        
        Ok(())
    }

    pub fn check_open(to: T::AccountId, pawn: Vec<u8>, pawn_amount: u64, debit_amount: u64) -> DispatchResult {
        Ok(())
    }

    pub fn update_fee(id: u64) -> DispatchResult {
        
    }
}