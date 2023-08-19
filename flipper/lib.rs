#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default))]

/* Objetivo de etapa #3: 
    Modificar el storage para utilizar Mappings en lugar de Vectores
    Modificar lógica para que el poder de voto se corresponda con la reputación del contribuyente (mayor reputación -> mayor poder de voto)
    Emitir un evento por cada voto
    Agregar los siguientes controles:
    El único que puede agregar o eliminar contribuyentes es el Admin
    Los únicos que pueden votar son los contribuyentes registrados.
    La reputación es privada. Cada contribuyente puede consultar únicamente la propia. 
*/

#[ink::contract]
mod flipper {

    use ink::storage::Mapping;
    use scale::{Decode, Encode};

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

    #[ink(storage)]
    pub struct Flipper {
        admin: AccountId,
        votes: Mapping<AccountId, u32>,
        contributors: Mapping<AccountId, Contributor>
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

    impl Flipper {
        #[ink(constructor)]
        pub fn new(admin: AccountId) -> Self {
            Self { 
                admin,
                votes: Mapping::default(),
                contributors: Mapping::default()}
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
        self.env().emit_event(Vote { contributor_id:id });
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

}
