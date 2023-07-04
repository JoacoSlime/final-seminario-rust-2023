#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod gestor_de_cobros {
    use alloc::vec;
    use club_sem_rust::ClubSemRustRef;
    #[ink(storage)]
    pub struct GestorDeCobros {
        club_sem_rust: ClubSemRustRef,
    }

    impl GestorDeCobros {
        #[ink(constructor)]
        pub fn new(club_sem_rust: ClubSemRustRef) -> Self {
            Self { allowed_accounts: Vec::new(), club_sem_rust}
        }

        #[ink(message)]
<<<<<<< HEAD
         pub fn socios_morosos(&self ) -> Vec<Socio> {
            let hoy = self.env().block_timestamp();
            let socios = self.club_sem_rust.get_socios();
             let iter = socios.iter();
             iter.filter(|s| s.es_moroso(hoy)).collect()
        }
=======
        pub fn socios_morosos(&self, id_deporte: u32) -> bool {
            todo!()
        }

        /// 
        /// Retorna un vector con los socios no morosos que pueden concurrir a un deporte específico.
        /// 
>>>>>>> 4f7bd41 (Añadido documentación a socios_no_morosos(). Revertidas primeras lineas del nuevo contrato .)
        #[ink(message)]
        pub fn socios_no_morosos(&self, id_deporte: u32) -> Vec<Socio> {
            let socios = self.club_sem_rust.get_socios();
            let iter = socios.iter();
            iter.filter( |s|
                s.puede_hacer_deporte(id_deporte) &&
                !s.es_moroso()
            ).collect()
        }

        #[ink(message)]
<<<<<<< HEAD
        pub fn recaudación(&self) -> Vec<u128> {
            let socios:Vec<Socio> = self.club_sem_rust.get_socios();
            let mut vec_recaudacion:Vec<u128> = Vec::new();

            let recaudacion_categoria_a:u128 = socios.iter().filter(|s| s.mi_categoria(1))
            .map(|s| s.generar_recibos().get_monto())
            .count();
            
            let recaudacion_categoria_b:u128 = socios.iter().filter(|s| s.mi_categoria(2))
            .map(|s| s.generar_recibos().get_monto())
            .count();

            let recaudacion_categoria_c:u128 = socios.iter().filter(|s| s.mi_categoria(3))
            .map(|s| s.generar_recibos().get_monto())
            .count();

            vec_recaudacion.push(recaudacion_categoria_a);
            vec_recaudacion.push(recaudacion_categoria_b);
            vec_recaudacion.push(recaudacion_categoria_c);

            //let recibos = self.club_sem_rust.get_recibos();



            return vec_recaudacion;
=======
        pub fn recaudación(&self) -> bool {
            todo!()
>>>>>>> 4f7bd41 (Añadido documentación a socios_no_morosos(). Revertidas primeras lineas del nuevo contrato .)
        }
    }
}
