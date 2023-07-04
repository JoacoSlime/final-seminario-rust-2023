#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod gestor_de_cobros {
    use club_sem_rust::ClubSemRustRef;
    #[ink(storage)]
    pub struct GestorDeCobros {
        allowed_accounts: Vec<AccountId>,
        club_sem_rust: ClubSemRustRef,
    }

    impl GestorDeCobros {
        #[ink(constructor)]
        pub fn new(club_sem_rust: ClubSemRustRef) -> Self {
            Self { allowed_accounts: Vec::new(), club_sem_rust}
        }

        #[ink(message)]
        pub fn socios_morosos(&self, id_deporte: u32) -> bool {
            let socios = self.club_sem_rust.get_socios();
            todo!()
        }

        #[ink(message)]
        pub fn socios_no_morosos(&self, id_deporte: u32) -> bool {
            let socios = self.club_sem_rust.get_socios();
            todo!()
        }

        #[ink(message)]
        pub fn recaudación(&self) -> bool {
            let recibos = self.club_sem_rust.get_recibos();
            todo!()
        }
    }
}
