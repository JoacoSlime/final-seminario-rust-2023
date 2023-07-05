#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod gestor_de_cobros {
    use alloc::vec;
    use club_sem_rust::Recibo;
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

        ///
        /// Genera un vector con la recaudacion del mes actual para cada Categoría,
        /// esto significa, la suma de los montos pagados en un mes de todos los Recibos
        /// de los Socios de cada Categoría.
        /// 
        /// Se considera el paso de 30 días como el paso de un mes.
        /// Y 2_592_000_000 representa 30 dias para el tipo Timestamp.
        /// 
        /// Se contará la Recaudación para cada una de las tres Categorías
        /// desde el momento en el que se invoca a este método hasta 30 días en el pasado.
        /// 
        #[ink(message)]
        pub fn recaudación(&self) -> Vec<u128> {
            let socios:Vec<Socio> = self.club_sem_rust.get_socios();
            let mut vec_recaudacion:Vec<u128> = Vec::new();

            let fecha_hoy:Timestamp = self.env().block_timestamp();
            

            let recibos_categoria_a:Vec<Recibo> = socios.iter()
            .filter(|s| s.mi_categoria(1))
            .map(|s| s.generar_recibos())
            .collect();
            let recaudacion_categoria_a:u128 = recibos_categoria_a.iter()
            .filter(|r| r.fecha_entre(fecha_hoy-2_592_000_000, fecha_hoy))
            .map(|r| r.get_monto())
            .count();

            let recaudacion_a: Recaudacion = Recaudacion::new(recaudacion_categoria_a, fecha_hoy, 1);
            
            let recibos_categoria_b:Vec<Recibo> = socios.iter()
            .filter(|s| s.mi_categoria(2))
            .map(|s| s.generar_recibos())
            .collect();
            let recaudacion_categoria_b:u128 = recibos_categoria_b.iter()
            .filter(|r| r.fecha_entre(fecha_hoy-2_592_000_000, fecha_hoy))
            .map(|r| r.get_monto())
            .count();

            let recaudacion_b: Recaudacion = Recaudacion::new(recaudacion_categoria_b, fecha_hoy, 2);

            let recibos_categoria_c:Vec<Recibo> = socios.iter()
            .filter(|s| s.mi_categoria(3))
            .map(|s| s.generar_recibos())
            .collect();
            let recaudacion_categoria_c:u128 = recibos_categoria_c.iter()
            .filter(|r| r.fecha_entre(fecha_hoy-2_592_000_000, fecha_hoy))
            .map(|r| r.get_monto())
            .count();

            let recaudacion_c: Recaudacion = Recaudacion::new(recaudacion_categoria_c, fecha_hoy, 3);

            vec_recaudacion.push(recaudacion_a);
            vec_recaudacion.push(recaudacion_b);
            vec_recaudacion.push(recaudacion_c);

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
