
use ink::{primitives::AccountId, trait_definition};

#[trait_definition]
pub trait VoteContract {
    #[ink(message)]
    fn get_votes(&self, id: AccountId) -> u32;

    #[ink(message)]
    fn vote(&mut self, id: AccountId);
}