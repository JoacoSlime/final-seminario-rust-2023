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

        /// Genera un reporte de socios morosos.
        /// 
        /// Devuelve aquellos socios con pagos pendientes después de la fecha de vencimiento.
        /// 
        /// # Ejemplos
        /// 
        /// ```
        /// let morosos = self.socios_morosos()
        /// ```
        #[ink(message)]
        pub fn socios_morosos(&self, id_deporte: u32) -> bool {
            todo!()
        }

        #[ink(message)]
        pub fn socios_no_morosos(&self, id_deporte: u32) -> bool {
            todo!()
        }

        #[ink(message)]
        pub fn recaudación(&self) -> bool {
            todo!()
        }
    }
}
