#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod gestor_de_cobros {
    use alloc::vec;
    use::ink::prelude::string;
    use::ink::prelude::vec;
    use club_sem_rust::ClubSemRustRef;
    use ink_e2e::env_logger::fmt::Timestamp;
    #[ink(storage)]
    pub struct GestorDeCobros {
        club_sem_rust: ClubSemRustRef,
        allowed_accounts: Vec<AccountId>,
    }

    impl GestorDeCobros {
        #[ink(constructor)]
        pub fn new(club_sem_rust: ClubSemRustRef) -> Self {
            Self { allowed_accounts: Vec::new(), club_sem_rust}
        }

        #[ink(message)]
         pub fn socios_morosos(&self ) -> Vec<Socio> {
            let hoy = self.env().block_timestamp();
            let socios = self.club_sem_rust.get_socios();
             let iter = socios.iter();
             iter.filter(|s| s.es_moroso(hoy)).collect()
        }
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
        pub fn recaudación(&self) -> Vec<u128> {
            let socios:Vec<Socio> = self.club_sem_rust.get_socios();
            let mut vec_recaudacion:Vec<u128> = Vec::new();

            let fecha_hoy:Timestamp = self.env().block_timestamp();
            // 864_000_000 es diez dias, 2_592_000_000 es un mes
            // if recibo.fecha > fecha_hoy-2_592_000_000 --> el recibo fue hecho dentro del mes

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

            return vec_recaudacion;
        }
    }


    #[derive(scale::Decode, scale::Encode, Debug, Clone, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Recaudacion{
        monto: u128,
        fecha: Timestamp,
        categoria: String,
    }
    impl Recaudacion {
        pub fn new(monto: u128, fecha: Timestamp, categoria: u8) -> Recaudacion{
            match categoria {
                1 => Recaudacion { monto, fecha, categoria: "Categoria A".to_string() },
                2 => Recaudacion { monto, fecha, categoria: "Categoria B".to_string() },
                3 => Recaudacion { monto, fecha, categoria: "Categoria C".to_string() },
            }
        }
    }
}
