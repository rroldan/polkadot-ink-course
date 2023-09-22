#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default))]
/*
Objetivo de etapa #4: 
Crear un contrato PSP34 (Utilizar Templates de OpenBrush) que sirva de certificado de votación
Transferir al contribuyente un NFT que certifique su voto
Definir un trait que represente el comportamiento de votación e implementarlo en el contrato
Votar
Obtener reputación/votos de un contribuyente
*/

pub mod vote_contract;

#[ink::contract]
mod organization {
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use scale::{Decode, Encode};
    use crate::vote_contract::VoteContract;
    use mint::ContractRef;
   
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
        votes: u32

    }

    #[ink(event)]
    pub struct ReputationContributor {
        #[ink(topic)]
        contributor_id: AccountId
    }

    #[ink(event)]
    pub struct BalanceContributor {
        #[ink(topic)]
        balance_contributor: Balance
    }

    #[ink(event)]
    pub struct BalnceAdmin {
        #[ink(topic)]
        balance_admin: Balance
    }

    #[ink(storage)]
    pub struct Organization {
        admin: AccountId,
        votes: Mapping<AccountId, u32>,
        balances: Mapping<AccountId, Balance>,
        contributors: Mapping<AccountId, Contributor>,
        contract: ContractRef,
    }

    #[derive(Encode, Decode, Debug, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Reputation {
    Easy ,
    Medium,  
    Hard   
    }
    
    #[derive(Encode, Decode, Debug, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Contributor {
        contributor_id: AccountId,
        reputation: Reputation
    }

    impl Organization {
        #[ink(constructor, payable)]
        pub fn new(admin: AccountId, contract_code_hash: Hash) -> Self {
            Self { 
                admin,
                votes: Mapping::default(),
                contributors: Mapping::default(),
                balances: Mapping::default(),
                contract: ContractRef::new()
                .code_hash(contract_code_hash)
                .endowment(0)
                .salt_bytes(Vec::new()) // Sequence of bytes
                .instantiate()
            }
        }
        
        #[ink(message)]
        pub fn add_contributor(&mut self, id:AccountId, r:Reputation) {
            assert!(self.env().caller() == self.admin);
            let contributor: Contributor = Contributor{contributor_id:id, reputation:r};
            self.contributors.insert(id, &contributor);
            self.env().emit_event(NewContributor { contributor_id:id });
        }

        pub fn remove_contributor(&mut self, id:AccountId){
            assert!(self.env().caller() == self.admin);
            assert!(self.contributors.contains(id));
            self.contributors.remove(id);
            self.env().emit_event(RemoveContributor { contributor_id:id });
        }

        #[ink(message)]
        pub fn vote(&mut self, id: AccountId) {
        assert!(self.contributors.contains(id));
        let  votes = self.votes.get(id).unwrap_or(0);
        let contributor: Contributor = self.contributors.get(id).unwrap();

        self.votes.insert(id, &(self.rule_reptation_vote(votes, contributor.reputation)));
        let result = self.contract.mint_token();
        assert!(result.is_err());
        self.env().emit_event(Vote { contributor_id:id});
        }

        #[ink(message)]
        pub fn get_reputation(&self) -> Option<Reputation>  {
            let id:AccountId = self.env().caller();
            assert!(self.contributors.contains(id));
           
            let contributor: Contributor = self.contributors.get(id).unwrap();
            self.env().emit_event(ReputationContributor{ contributor_id:id });
            Some(contributor.reputation)
        }

        #[ink(message)]
        pub fn get_balance(&self) ->  Option<Balance> {
            let id:AccountId = self.env().caller();
            assert!(self.contributors.contains(id));
            let balance = self.balances.get(id).unwrap_or(0);
            self.env().emit_event(BalanceContributor{ balance_contributor:balance });
            Some(balance)

        }

        #[ink(message)]
        pub fn get_balance_admin(&self) ->  Option<Balance> {
            assert!(self.env().caller() == self.admin);
            let id:AccountId = self.env().caller();
            let balance = self.balances.get(id).unwrap_or(0);
            self.env().emit_event(BalanceContributor{ balance_contributor:balance });
            Some(balance)
        }

        #[ink(message)]
        pub fn get_votes(&self, id: AccountId) ->  Option<u32>{
            assert!(self.contributors.contains(id));
            let  v = self.votes.get(id).unwrap_or(0);
            self.env().emit_event(VotesContributor{ contributor_id:id, votes:v });
            Some(v)
        }


        fn rule_reptation_vote(&self, votes:u32, reputation:Reputation) -> u32 {
            votes + reputation as u32
        }

        #[ink(message)]
        pub fn get_addresss(&self) -> AccountId {
            self.env().account_id()
        }
    }

    impl VoteContract for Organization {
        #[ink(message)]
        fn get_votes(&self, id: AccountId) -> u32 {
            self.get_votes(id).unwrap()
        }
    
        #[ink(message)]
        fn vote(&mut self, id: AccountId){
            self.vote(id)
        }
    }
 
}

   


