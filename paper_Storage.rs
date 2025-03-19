use frame_support::{
    decl_event, decl_module, decl_storage, dispatch::Result, StorageMap, StorageValue,
};
use frame_system::ensure_signed;

pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as PaperStorage {
        Papers: map T::AccountId => Vec<u8>;
        TokenSupply: u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event<T>() = default;

        pub fn add_paper(origin, title: Vec<u8>, author: Vec<u8>, content: Vec<u8>) -> Result {
            let sender = ensure_signed(origin)?;
            let paper = (title, author, content);
            <Papers<T>>::insert(&sender, paper);
            <TokenSupply<T>>::put(1000);
            Ok(())
        }

        pub fn get_paper(origin, account: T::AccountId) -> Result {
            let _ = ensure_signed(origin)?;
            let paper = <Papers<T>>::get(&account);
            Ok(paper)
        }
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
    {
        PaperAdded(AccountId),
    }
);
