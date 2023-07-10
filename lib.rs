#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod gestor_de_cobros {
    use club_sem_rust::Recibo;
    use club_sem_rust::Socio;
    use club_sem_rust::ClubSemRustRef;

    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::prelude::string::ToString;
    
    extern crate chrono;
    use chrono::prelude::*;
    use chrono::{DateTime, Utc};

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

        fn get_socios(&self) -> Vec<Socio> {
            self.club_sem_rust.get_socios()
        }

        #[ink(message)]
         pub fn socios_morosos(&self) -> Vec<Socio> {
            let hoy = self.env().block_timestamp();
            let socios = self.get_socios();
            let socios = socios.into_iter()
            .filter(|s| s.es_moroso(hoy)).collect();
            return socios;
        }

        #[ink(message)]
        /// Devuelve un vector con la lista de aquellos socios no morosos que tengan el deporte correspondiente a la id_deporte.
        pub fn socios_no_morosos(&self, id_deporte: u32) -> Vec<Socio> {
            let socios = self.get_socios();
            let iter = socios.into_iter();
            iter.filter( |s|
                s.puede_hacer_deporte(id_deporte) &&
                !s.es_moroso(self.env().block_timestamp())
            ).collect()
        }

        /// Genera un vector con la recaudacion de cada Categoría durante el transcurso de un mes,
        /// esto significa, la suma de todos los montos pagados a lo largo de 30 días
        /// de todos los Recibos clasificados por Categorías. 
        /// 
        /// Se considera el paso de 30 días como el paso de un mes.
        /// 2_592_000_000 representa 30 dias para el tipo Timestamp.
        /// 
        /// Se contará la Recaudación para cada una de las tres Categorías
        /// desde el momento en el que se invoca a este método hasta 30 días en el pasado.
        /// 
        #[ink(message)]
        pub fn recaudacion(&self) -> Vec<Recaudacion> {
            let socios:Vec<Socio> = self.get_socios();
            let mut vec_recaudacion:Vec<Recaudacion> = Vec::new();

            let fecha_hoy:Timestamp = self.env().block_timestamp();
            
            let closure = |s: &Socio, n: u32| {
                if s.mi_categoria(n) {
                    Some(s.generar_recibos())
                } else {
                    None
                }
            };

            let recibos_categoria_a: Vec<Recibo> = socios.iter()
            .filter_map(|s| closure(s,1)).flatten().collect();
            let recaudacion_categoria_a:u128 = recibos_categoria_a.iter()
            .filter(|r| r.fecha_entre(fecha_hoy-2_592_000_000, fecha_hoy))
            .map(|r| r.get_monto())
            .count() as u128;

            let recaudacion_a: Recaudacion = Recaudacion::new(recaudacion_categoria_a, fecha_hoy, 1);
            
            let recibos_categoria_b:Vec<Recibo> = socios.iter()
            .filter_map(|s| closure(s,2)).flatten().collect();
            let recaudacion_categoria_b:u128 = recibos_categoria_b.iter()
            .filter(|r| r.fecha_entre(fecha_hoy-2_592_000_000, fecha_hoy))
            .map(|r| r.get_monto())
            .count() as u128;

            let recaudacion_b: Recaudacion = Recaudacion::new(recaudacion_categoria_b, fecha_hoy, 2);

            let recibos_categoria_c:Vec<Recibo> = socios.iter()
            .filter_map(|s| closure(s,3)).flatten().collect();
            let recaudacion_categoria_c:u128 = recibos_categoria_c.iter()
            .filter(|r| r.fecha_entre(fecha_hoy-2_592_000_000, fecha_hoy))
            .map(|r| r.get_monto())
            .count() as u128;

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
        fecha: String,
        categoria: String,
    }
    /// Construye una nueva Recaudacion.
    /// Representa la recaudación total de una determinada Categoría a lo largo de un mes.
    /// 
    /// # Panics
    /// 
    /// Puede llegar a devolver panic si se ingresa un número de id_categoria inválido.
    impl Recaudacion {
        pub fn new(monto: u128, fecha: Timestamp, categoria: u8) -> Recaudacion{
            let naive = NaiveDateTime::from_timestamp_opt(fecha as i64, 0);
            if naive.is_some(){
                let datetime: DateTime<Utc> = DateTime::from_utc(naive.unwrap(), Utc);
                let timestamp_string = datetime.format("%d/%m/%Y").to_string();
                match categoria {
                    1 => Recaudacion { monto, fecha: timestamp_string, categoria: "Categoria A".to_string() },
                    2 => Recaudacion { monto, fecha: timestamp_string, categoria: "Categoria B".to_string() },
                    3 => Recaudacion { monto, fecha: timestamp_string, categoria: "Categoria C".to_string() },
                    _ => panic!("Categoría inválida."),
                }
            }else{
                panic!("La Fecha recibida no es válida.")
            }
        }
    }

    #[cfg(test)]
    mod gestor_de_cobros_tests {
        use super::Recaudacion;

        #[ink::test]
        #[should_panic(expected = "Categoría inválida.")]
        fn recaudacion_test_panic(){
            Recaudacion::new(2000, 1_000_000, 0);
        }

        #[ink::test]
        fn recaudacion_test_fecha(){
            let recaudacion: Recaudacion= Recaudacion::new(2000, 986007600, 1);
            let esperado = "31/03/2001".to_string();
            let resultado = recaudacion.fecha;

            assert_eq!(esperado, resultado, "se esperaba {:?} y se recibió {:?}", esperado, resultado)
        }

    }
    }


