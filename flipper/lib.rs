#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default))]

#[ink::contract]
mod flipper {

    use ink::prelude::vec::Vec;
    use scale::{Decode, Encode};

//El objetivo de esta etapa #2 es modificar el smart contract que tienen en su repositorio para empezar a darle forma a nuestra a esta organización:
// Storage: 
// Incluir a los contribuyentes con su reputación asociada (usar vectores).
// Incluir una cuenta administradora, que podrá agregar/eliminar contribuyentes.

// Mensajes:
// Agregar/Eliminar contribuyente
// Votar (sólamente un contribuyente puede votar a otro)
// Consultar reputación de contribuyente

    #[ink(event)]
    pub struct NewContributor {
        #[ink(topic)]
        contributor: Contributor
    }

    #[ink(event)]
    pub struct Vote {
        #[ink(topic)]
        contributor: Contributor
    }

    #[ink(event)]
    pub struct Reputation {
        #[ink(topic)]
        reputation: u32
    }


    #[ink(storage)]
    pub struct Flipper {
        admin: AccountId,
        contributors: Vec<Contributor>,

    }

    
    #[derive(Encode, Decode, Debug, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Contributor {
        contributor_id: AccountId,
        reputation: u32
    }

    impl Flipper {
        #[ink(constructor)]
        pub fn new(admin: AccountId) -> Self {
            let contributors_vector:Vec<Contributor> = Vec::new();
            Self { 
                admin,
                contributors:contributors_vector}
        }

        #[ink(message)]
        pub fn add_contributor(&mut self, contributor:Contributor) {
            assert!(self.env().caller() == self.admin);
            self.contributors.push(contributor.clone());
            self.env().emit_event(NewContributor { contributor });
        }

        #[ink(message)]
        pub fn add_contributors(&mut self, contributors: Vec<Contributor>) {
            assert!(self.env().caller() == self.admin);
            for item in contributors {
                let contributor = Contributor {contributor_id: item.contributor_id, reputation: item.reputation };
                self.contributors.push(contributor.clone());
                self.env().emit_event(NewContributor { contributor });
            }

        }

        #[ink(message)]
        pub fn vote(&mut self, id: AccountId) {

           assert!(self.is_contributor(id));
           let contributor = self.vote_contributor(id);
         
            self.env().emit_event(Vote { contributor });
        }

        #[ink(message)]
        pub fn reputation(&mut self, id: AccountId) {
            assert!(self.is_contributor(id));
           let contributor:Contributor = self.get_contributor(id);
           let reputation = contributor.reputation;
            self.env().emit_event(Reputation { reputation });
        }

        #[ink(message)]
        pub fn get_addresss(&self) -> AccountId {
            self.env().account_id()
        }

      
        fn is_contributor(&mut self, id: AccountId)-> bool {
            let mut result:bool = false;
            for item in &self.contributors {
                if item.contributor_id == id {
                    result = true;
                    break;
                }
            }
            result
        } 
       
        fn vote_contributor (&mut self, id: AccountId) -> Contributor {
            let mut contributor:Contributor = Contributor {contributor_id: id, reputation: 0};
            for item in &mut self.contributors {
                if item.contributor_id == id {
                    item.reputation += 1;
                    contributor = item.clone();
                    break
                }
            }
            assert!(id == contributor.contributor_id);
            assert!(contributor.reputation > 0);
            contributor
        }

        fn get_contributor (&mut self, id: AccountId) -> Contributor {
            let mut contributor:Contributor = Contributor {contributor_id: id, reputation: 0};
            for item in &self.contributors {
                if item.contributor_id == id {
                    contributor = item.clone();
                    break
                }
            }
            assert!(id == contributor.contributor_id);
            assert!(contributor.reputation > 0);
            contributor
        }

    }

}
