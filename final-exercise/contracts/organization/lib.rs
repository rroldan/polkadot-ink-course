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
        contributors: Mapping<AccountId, u32>,
        reputation: Vec<(AccountId, u32)>,
        balances: Mapping<AccountId, Balance>,
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
            let reputation_vec:Vec<(AccountId, u32)>=Vec::new();

            Self { 
                admin,
                votes: Mapping::default(),
                contributors:  Mapping::default(),
                reputation: reputation_vec,
                balances: Mapping::default(),
                vouting_round: VoutingRound{votes:0, open:false, balance:0},
                contract: ContractRef::new()
                .code_hash(contract_code_hash)
                .endowment(0)
                .salt_bytes(Vec::new()) 
                .instantiate()

            }
        }
        
        #[ink(message)]
        pub fn add_contributor(&mut self, id:AccountId) {
            assert!(self.env().caller() == self.admin);
            self.reputation.push((id,1));
            self.env().emit_event(NewContributor { contributor_id:id });
        }

        pub fn remove_contributor(&mut self, id:AccountId){
            assert!(self.env().caller() == self.admin);
            assert!(self.contributors.contains(id));
            self.contributors.remove(id);
            let index = self.reputation.iter().position(|x| x.0 == id).unwrap();
            self.reputation.remove(index);
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
        self.update_reputation(id, new_reputation);

        let result = self.contract.mint_token();
        assert!(result.is_err());

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

        
        pub fn sum_reputation_all(&self) -> u128 {
            let mut sum:u32 = 0;
            for (_ , reputation) in &self.reputation {
                sum += reputation;      
            }
            sum as u128
        }

        #[ink(message)]
        pub fn update_reputation(&mut self, id:AccountId, new_reputation:u32) {
            
            for mut item in &self.reputation {
                let mut aux = item.clone();
                if aux.0 == id { 
                    aux.1 = new_reputation;
                    item = &aux;
                }
            }
        }


        pub fn clear_reputation(&mut self) {
                self.votes = Mapping::default();
                self.contributors =  Mapping::default();
                self.reputation.clear();
                self.balances = Mapping::default();
        }


        #[ink(message)]
        pub fn open_vouting_round(&mut self, votes:u32, founds:Balance ) -> bool {
            assert!(self.env().caller() == self.admin);
            let acount_balance_admin: Balance = self.get_balance_admin().unwrap_or(0);
            assert!(acount_balance_admin >= founds);
            self.vouting_round.votes = votes;
            assert!(votes > 0);
            self.vouting_round.balance = self.env().transferred_value();
            self.vouting_round.open = true;
            self.vouting_round.open
        }

        pub fn close_vouting_round(&mut self) {
            assert!(self.env().caller() == self.admin);
            assert!(self.vouting_round.open);
            let weights:u128 = self.vouting_round.balance/self.sum_reputation_all();
            //self.transfer(&mut self);
           self.clear_reputation();
           self.vouting_round = VoutingRound{votes:0, open:false, balance:0};




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

   


