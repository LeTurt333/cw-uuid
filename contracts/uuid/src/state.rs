use cosmwasm_std::{
    Addr,
};
use cw_storage_plus::{
    Item,
};
use crate::ContractError;

pub const STATE_RANDOMNESS: Item<[u8; 32]> = Item::new("state_randomness");
pub const INCREMENT: Item<u32> = Item::new("increment");
pub const NOIS_PROXY: Item<Addr> = Item::new("nois_proxy");


pub trait Increment {
    /// Increment (saturating) an `Item<u32>` by 1
    /// ```rust ignore
    /// pub const COUNTER: Item<u32> = Item::new("counter");
    /// COUNTER.save(deps.storage, &1)?;
    /// COUNTER.increment(deps.storage)?;
    /// let count = COUNTER.load(deps.storage)?;
    /// assert_eq!(count, 2);
    /// ```
    fn increment(&self, store: &mut dyn cosmwasm_std::Storage) -> Result<(), ContractError>;
}

impl Increment for Item<'_, u32> {

    fn increment(&self, store: &mut dyn cosmwasm_std::Storage) -> Result<(), ContractError> {

        self
            .update(store, |old| -> Result<u32, cosmwasm_std::StdError> {
                Ok(old.saturating_add(1))
            })
            .map_err(|_| ContractError::GenericError("Error Incrementing".to_string()))?;

        Ok(())
    }
}