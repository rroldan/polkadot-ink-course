#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default))]

#[ink::contract]
mod flipper {

    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
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

    #[ink(storage)]
    pub struct Flipper {
        admin: AccountId,
        contributors_mapping: Mapping<AccountId, Contributor>,

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
           
            Self { 
                admin,
                contributors_mapping:Mapping::default() }
        }

        #[ink(message)]
        pub fn add_contributor(&mut self, contributor:Contributor) {
            assert!(self.env().caller() == self.admin);
            self.contributors_mapping.insert(contributor.contributor_id, &contributor);
            self.env().emit_event(NewContributor { contributor });
        }

        pub fn add_contributors(&mut self, contributors: Vec<Contributor>) {
            assert!(self.env().caller() == self.admin);
            for item in contributors {
                let contributor = Contributor {contributor_id: item.contributor_id, reputation: item.reputation };
                self.contributors_mapping.insert(contributor.contributor_id, &contributor);
                self.env().emit_event(NewContributor { contributor });
            }

        }

        #[ink(message)]
        pub fn vote(&mut self, id: AccountId) {
            assert!(self.contributors_mapping.contains(id));

            let mut contributor:Contributor = self.contributors_mapping.get(id).expect("Oh no, Contributor not found");
            contributor.reputation += 1;
        
            self.contributors_mapping.insert(id, &contributor);
            self.env().emit_event(Vote { contributor });
        }

        #[ink(message)]
        pub fn get_addresss(&self) -> AccountId {
            self.env().account_id()
        }
    }

   

    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = FlipperRef::default();

            // When
            let contract_account_id = client
                .instantiate("flipper", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<FlipperRef>(contract_account_id.clone())
                .call(|flipper| flipper.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = FlipperRef::new(false);
            let contract_account_id = client
                .instantiate("flipper", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<FlipperRef>(contract_account_id.clone())
                .call(|flipper| flipper.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<FlipperRef>(contract_account_id.clone())
                .call(|flipper| flipper.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<FlipperRef>(contract_account_id.clone())
                .call(|flipper| flipper.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
