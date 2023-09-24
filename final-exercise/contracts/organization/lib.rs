#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default))]

/*
Trabajo Práctico - Enunciado final
https://github.com/NeoPower-Digital/ink-examples/blob/main/courses/Polkadot%20Hub%20-%20ink!%20en%20Espa%C3%B1ol/Trabajo%20Pr%C3%A1ctico.md
*/

pub mod vote_contract;

#[ink::contract]
mod organization {
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use scale::{Decode, Encode};
    use crate::vote_contract::VoteContract;
    use mint::ContractRef;
    use sp_arithmetic::FixedU128;
    use scale::CompactAs;
   
    #[ink(event)]
    pub struct NewContributor {
        #[ink(topic)]
        contributor_id: AccountId
    }

    #[ink(event)]
    pub struct RemoveContributor {
        #[ink(topic)]
        contributor_id: AccountId
    }

    #[ink(event)]
    pub struct Vote {
        #[ink(topic)]
        contributor_id: AccountId
    }

    #[ink(event)]
    pub struct VotesContributor {
        #[ink(topic)]
        contributor_id: AccountId,
        #[ink(topic)]
        votes: i32

    }

    #[ink(event)]
    pub struct ReputationContributor {
        #[ink(topic)]
        contributor_id: AccountId
    }

    #[ink(event)]
    pub struct Fund {
        #[ink(topic)]
        balance: Balance
    }


    #[ink(storage)]
    pub struct Organization {
        admin: AccountId,
        votes: Mapping<AccountId, i32>,
        balances: Mapping<AccountId, Balance>,
        contributors: Mapping<AccountId, u32>,
        vouting_round: VoutingRound,
        contract: ContractRef
    }

    #[derive(PartialEq, Eq, Debug, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum ContractError {
        YouAreNotFound,
        AccountWithoutBalance,
        InsufficientFunds,
        ExpectedWithdrawalAmountExceedsAccountBalance,
        WithdrawTransferFailed,
    }
   
    #[derive(Encode, Decode, Debug, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct VoutingRound {
        votes: u32,
        open: bool,
        balance: Balance
    }

    impl Organization {
        #[ink(constructor, payable)]
        pub fn new(admin: AccountId, contract_code_hash: Hash) -> Self {
            Self { 
                admin,
                votes: Mapping::default(),
                contributors: Mapping::default(),
                balances: Mapping::default(),
                vouting_round: VoutingRound{votes:0, open:false, balance:0},
                contract: ContractRef::new()
                .code_hash(contract_code_hash)
                .endowment(0)
                .salt_bytes(Vec::new()) // Sequence of bytes
                .instantiate()

            }
        }
        
        #[ink(message)]
        pub fn add_contributor(&mut self, id:AccountId) {
            assert!(self.env().caller() == self.admin);
            self.contributors.insert(id, &1);
            self.env().emit_event(NewContributor { contributor_id:id });
        }

        pub fn remove_contributor(&mut self, id:AccountId){
            assert!(self.env().caller() == self.admin);
            assert!(self.contributors.contains(id));
            self.contributors.remove(id);
            self.env().emit_event(RemoveContributor { contributor_id:id });
        }

        #[ink(message)]
        pub fn vote(&mut self, id: AccountId, vote:i32) {
        assert!(self.contributors.contains(id));
        assert!(self.vouting_round.open);
        assert!(self.vouting_round.votes>0);
        self.vouting_round.votes -= 1;
        let  votes = self.votes.get(id).unwrap_or(0);
        let reputation = self.contributors.get(id).unwrap();
        let new_votes = votes + vote;
        let new_reputation = self.rule_reptation_vote(reputation, votes, vote);

        self.votes.insert(id, &new_votes);
        self.contributors.insert(id, &new_reputation);
        self.env().emit_event(Vote { contributor_id:id});
        }

        #[ink(message)]
        pub fn get_reputation(&self) -> u32 {
            let id:AccountId = self.env().caller();
            assert!(self.contributors.contains(id));
           
            let reputation= self.contributors.get(id).unwrap();
            self.env().emit_event(ReputationContributor{ contributor_id:id });
            reputation
        }

        #[ink(message)]
        pub fn get_balance(&self) -> Result<Balance, ContractError> {
            let id:AccountId = self.env().caller();
            assert!(self.contributors.contains(id));
            match self.balances.get(id) {
                Some(acount_balance) => {
                    self.env().emit_event(Fund{ balance:acount_balance });
                    Ok(acount_balance)},
                None => Err(ContractError::YouAreNotFound),
        }
    }

        #[ink(message)]
        pub fn get_balance_admin(&self) ->  Result<Balance, ContractError> {
            assert!(self.env().caller() == self.admin);
            let id:AccountId = self.env().caller();
            match self.balances.get(id) {
                Some(acount_balance) => {
                    self.env().emit_event(Fund{ balance:acount_balance });
                    Ok(acount_balance)},
                None => Err(ContractError::YouAreNotFound),


            }
        }

        #[ink(message)]
        pub fn get_votes(&self, id: AccountId) ->  Option<i32>{
            assert!(self.contributors.contains(id));
            let  v = self.votes.get(id).unwrap_or(0);
            self.env().emit_event(VotesContributor{ contributor_id:id, votes:v });
            Some(v)
        }

        
        #[ink(message)] 
        pub fn get_squareroot(&self, num: u32) -> u32 {  
            let d1 = FixedU128::from_u32(num);  
            let d2 = FixedU128::sqrt(d1); 
            let d3 = *d2.encode_as();
            d3 as u32
        }


        fn rule_reptation_vote(&self, member_pts:u32, target_pts:i32, value:i32) -> u32 {
            if (target_pts  + value) < 1 { return 1 }
            (target_pts as u32 + value as u32) * self.get_squareroot(member_pts)
        }


        #[ink(message)]
        pub fn get_addresss(&self) -> AccountId {
            self.env().account_id()
        }

        pub fn open_vouting_round(&mut self, votes:u32, founds:Balance ) -> bool {
            assert!(self.env().caller() == self.admin);
            let acount_balance_admin: Balance = self.get_balance_admin().unwrap_or(0);
            assert!(acount_balance_admin >= founds);
            self.vouting_round.votes = votes;
            assert!(votes > 0);
            self.vouting_round.balance = founds;
            self.vouting_round.open = true;
            self.vouting_round.open
        }
    }

    impl VoteContract for Organization {
        #[ink(message)]
        fn get_votes(&self, id: AccountId) -> i32 {
            self.get_votes(id).unwrap()
        }
    
        #[ink(message)]
        fn vote(&mut self, id: AccountId, vote: i32){
            self.vote(id, vote)
        }
    }
 
}

   


