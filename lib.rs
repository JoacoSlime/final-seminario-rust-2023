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
        #[cfg(not(test))]
        club_sem_rust: ClubSemRustRef, // Solo tiene un referencia en caso de no ser un test.
    }

    impl GestorDeCobros {
        #[ink(constructor)]
        #[cfg(not(test))]
        pub fn new(club_sem_rust: ClubSemRustRef) -> Self {
            Self {club_sem_rust}
        }
        
        #[cfg(not(test))]
        fn get_socios(&self) -> Vec<Socio> {
            self.club_sem_rust.get_socios()
        }
        
        #[cfg(test)]
        pub fn new() -> Self {
            Self{}
        }

        #[cfg(test)]
        /// Método mockeado para get_socios()
        fn get_socios(&self) -> Vec<Socio> {
            let deadline = 864_000_000;
            let hoy = 1_690_000_000_000;
            let precios = Vec::from([5000, 4000, 2000]);
            let descuento = Some(15);
            let mut a = Socio::new("Alice".to_string(), 44044044, 1, None, hoy+deadline, precios.clone());
            let mut b = Socio::new("Bob".to_string(), 45045045, 2, Some(6), hoy+deadline, precios.clone());
            let mut c = Socio::new("Carol".to_string(), 46046046, 2, Some(1), hoy+deadline, precios.clone());
            let mut d = Socio::new("Derek".to_string(), 47047047, 1, None, hoy+deadline, precios.clone());
            let e = Socio::new("Emily".to_string(), 48048048, 1, None, hoy+deadline, precios.clone());
            let mut f = Socio::new("Frank".to_string(), 49049049, 3, None, hoy+deadline, precios.clone());
            let g = Socio::new("Gary".to_string(), 50050050, 3, None, hoy+deadline, precios.clone());
            
            a.realizar_pago(None, 5000, hoy, precios.clone(), deadline);
            a.realizar_pago(descuento, 5000, hoy + 100_000, precios.clone(), deadline);
            a.realizar_pago(None, 4250, hoy + 300_000, precios.clone(), deadline);

            b.realizar_pago(None, 4000, hoy, precios.clone(), deadline);
            b.realizar_pago(None, 4000, hoy + deadline + 100_000, precios.clone(), deadline);

            c.realizar_pago(None, 4000, hoy, precios.clone(), deadline);
            c.realizar_pago(None, 4000, hoy + 100_000, precios.clone(), deadline);

            d.realizar_pago(None, 5000, hoy, precios.clone(), deadline);
            d.realizar_pago(descuento, 5000, hoy + 100_000, precios.clone(), deadline);
            d.realizar_pago(None, 4250, hoy + deadline + 200_000, precios.clone(), deadline);

            f.realizar_pago(None, 2000, hoy, precios.clone(), deadline);
            f.realizar_pago(descuento, 2000, hoy + 100_000, precios.clone(), deadline);
            f.realizar_pago(None, 1700, hoy + 200_000, precios.clone(), deadline);

            let r = Vec::from([a,b,c,d,e,f,g]);
            r
        }

        /// Devuelve un vector con un listado de todos los socios morosos.
        /// Se consideran morosos a aquellos socios que tienen pagos pendientes después de la fecha de vencimiento.
        /// 
        /// En caso de no haber ningún socio moroso, devuelve un vector vacío.
        #[ink(message)]
        pub fn socios_morosos(&self) -> Vec<Socio> {
            let hoy = self.env().block_timestamp();
            let socios = self.get_socios();
            let socios = socios.into_iter()
            .filter(|s| s.es_moroso(hoy)).collect();
            return socios;
        }

        /// Devuelve un vector con la lista de todos aquellos socios no morosos que tienen permitido asistir a una actividad deportiva específica del club.
        /// Se pasa por parámetro el id_deporte correspondiente a la actividad deportiva que se desea consultar.
        /// 
        /// En caso de consultar por una actividad que no es practicada por ningún Socio, devuelve un vector vacío.
        #[ink(message)]
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
        pub fn get_recaudacion(&self) -> Vec<Recaudacion> {
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
        use club_sem_rust::Socio;

        use super::GestorDeCobros;
        use super::Recaudacion;

        #[ink::test]
        #[should_panic(expected = "Categoría inválida.")]
        fn new_recaudacion_test_panic(){
            Recaudacion::new(2000, 1_000_000, 0);
        }

        #[ink::test]
        fn new_recaudacion_test_fecha(){
            let recaudacion: Recaudacion= Recaudacion::new(2000, 986007600, 1);
            let esperado = "31/03/2001".to_string();
            let resultado = recaudacion.fecha;

            assert_eq!(esperado, resultado, "se esperaba {:?} y se recibió {:?}", esperado, resultado)
        }

        #[ink::test]
        fn get_recaudacion_test(){
            let deadline = 864_000_000;
            let hoy: crate::gestor_de_cobros::Timestamp = 1_690_000_000_000;
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(hoy);
            let precios = Vec::from([5000, 4000, 2000]);
            let descuento = Some(15);
            let mut a = Socio::new("Alice".to_string(), 44044044, 1, None, hoy+deadline, precios.clone());
            let mut b = Socio::new("Bob".to_string(), 45045045, 2, Some(6), hoy+deadline, precios.clone());
            let mut c = Socio::new("Carol".to_string(), 46046046, 2, Some(1), hoy+deadline, precios.clone());
            let mut d = Socio::new("Derek".to_string(), 47047047, 1, None, hoy+deadline, precios.clone());
            let mut e = Socio::new("Emily".to_string(), 48048048, 1, None, hoy+deadline, precios.clone());
            let mut f = Socio::new("Frank".to_string(), 49049049, 3, None, hoy+deadline, precios.clone());
            let mut g = Socio::new("Gary".to_string(), 50050050, 3, None, hoy+deadline, precios.clone());
            
            a.realizar_pago(None, 5000, hoy, precios.clone(), deadline);
            a.realizar_pago(descuento, 5000, hoy + 100_000, precios.clone(), deadline);
            a.realizar_pago(None, 4250, hoy + 300_000, precios.clone(), deadline);

            b.realizar_pago(None, 4000, hoy, precios.clone(), deadline);
            b.realizar_pago(None, 4000, hoy + deadline + 100_000, precios.clone(), deadline);

            c.realizar_pago(None, 4000, hoy, precios.clone(), deadline);
            c.realizar_pago(None, 4000, hoy + 100_000, precios.clone(), deadline);

            d.realizar_pago(None, 5000, hoy, precios.clone(), deadline);
            d.realizar_pago(descuento, 5000, hoy + 100_000, precios.clone(), deadline);
            d.realizar_pago(None, 4250, hoy + deadline + 200_000, precios.clone(), deadline);

            e.realizar_pago(None, 5000, hoy + 100, precios.clone(), deadline);

            f.realizar_pago(None, 2000, hoy, precios.clone(), deadline);
            f.realizar_pago(descuento, 2000, hoy + 100_000, precios.clone(), deadline);
            f.realizar_pago(None, 1700, hoy + 200_000, precios.clone(), deadline);

            g.realizar_pago(None, 2000, hoy, precios.clone(), deadline);

            let gestor = GestorDeCobros::new();

            assert_eq!(gestor.get_recaudacion()[0].monto, 33_500);
            assert_eq!(gestor.get_recaudacion()[1].monto, 16_000);
            assert_eq!(gestor.get_recaudacion()[2].monto, 7_700);
        }

        #[ink::test]
        fn socios_no_morosos_test() { // Debe extenderse
            let deadline = 864_000_000;
            let hoy: crate::gestor_de_cobros::Timestamp = 1_690_000_000_000;
            let precios = Vec::from([5000,4000,2000]);
            let mut a = Socio::new("Alice".to_string(), 44044044, 1, None, hoy+deadline, precios.clone());
            let mut c = Socio::new("Carol".to_string(), 46046046, 2, Some(1), hoy+deadline, precios.clone());
            let mut d = Socio::new("Derek".to_string(), 47047047, 1, None, hoy+deadline, precios.clone());
            let e = Socio::new("Emily".to_string(), 48048048, 1, None, hoy+deadline, precios.clone());
            a.realizar_pago(None, 5000, hoy, precios.clone(), deadline);
            a.realizar_pago(Some(15), 5000, hoy + 100_000, precios.clone(), deadline);
            a.realizar_pago(None, 4250, hoy + 300_000, precios.clone(), deadline);

            c.realizar_pago(None, 4000, hoy, precios.clone(), deadline);
            c.realizar_pago(None, 4000, hoy + 100_000, precios.clone(), deadline);

            d.realizar_pago(None, 5000, hoy, precios.clone(), deadline);
            d.realizar_pago(Some(15), 5000, hoy + 100_000, precios.clone(), deadline);
            d.realizar_pago(None, 4250, hoy + deadline + 200_000, precios.clone(), deadline);

            let esperado: Vec<Socio> = Vec::from([a,c,d,e]);
            let gestor = GestorDeCobros::new();
            let resultado = gestor.socios_no_morosos(1);

            assert_eq!(esperado, resultado, "Se esperaba {:#?} y se obtuvo {:#?}", esperado, resultado);
        }

        #[ink::test]
        fn socios_morosos_test() {
            let hoy: crate::gestor_de_cobros::Timestamp = 1_690_000_000_000;
            let deadline = 864_000_000;

            let precios = Vec::from([5000, 4000, 2000]);
            let descuento = Some(15);
            let mut a = Socio::new("Alice".to_string(), 44044044, 1, None, hoy+deadline, precios.clone());
            let mut b = Socio::new("Bob".to_string(), 45045045, 2, Some(6), hoy+deadline, precios.clone());
            let mut c = Socio::new("Carol".to_string(), 46046046, 2, Some(1), hoy+deadline, precios.clone());
            let mut d = Socio::new("Derek".to_string(), 47047047, 1, None, hoy+deadline, precios.clone());
            let e = Socio::new("Emily".to_string(), 48048048, 1, None, hoy+deadline, precios.clone());
            let mut f = Socio::new("Frank".to_string(), 49049049, 3, None, hoy+deadline, precios.clone());
            let g = Socio::new("Gary".to_string(), 50050050, 3, None, hoy+deadline, precios.clone());
            
            a.realizar_pago(None, 5000, hoy, precios.clone(), deadline);
            a.realizar_pago(descuento, 5000, hoy + 100_000, precios.clone(), deadline);
            a.realizar_pago(None, 4250, hoy + 300_000, precios.clone(), deadline);

            b.realizar_pago(None, 4000, hoy, precios.clone(), deadline);
            b.realizar_pago(None, 4000, hoy + deadline + 100_000, precios.clone(), deadline);

            c.realizar_pago(None, 4000, hoy, precios.clone(), deadline);
            c.realizar_pago(None, 4000, hoy + 100_000, precios.clone(), deadline);

            d.realizar_pago(None, 5000, hoy, precios.clone(), deadline);
            d.realizar_pago(descuento, 5000, hoy + 100_000, precios.clone(), deadline);
            d.realizar_pago(None, 4250, hoy + deadline + 200_000, precios.clone(), deadline);

            f.realizar_pago(None, 2000, hoy, precios.clone(), deadline);
            f.realizar_pago(descuento, 2000, hoy + 100_000, precios.clone(), deadline);
            f.realizar_pago(None, 1700, hoy + 200_000, precios.clone(), deadline);

            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(hoy);
            let esperado: Vec<Socio> = Vec::new();
            let gestor = GestorDeCobros::new();
            let resultado = gestor.socios_morosos();

            assert_eq!(esperado, resultado, "Se esperaba {:#?} y se obtuvo {:#?}", esperado, resultado);

            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(hoy+deadline*2);
            let esperado: Vec<Socio> = Vec::from([e.clone(), g.clone()]);
            let gestor = GestorDeCobros::new();
            let resultado = gestor.socios_morosos();

            assert_eq!(esperado, resultado, "Se esperaba {:#?} y se obtuvo {:#?}", esperado, resultado);

            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(hoy+deadline*4);
            let esperado: Vec<Socio> = Vec::from([b.clone(), c.clone(), e.clone(), g.clone()]);
            let gestor = GestorDeCobros::new();
            let resultado = gestor.socios_morosos();

            assert_eq!(esperado, resultado, "Se esperaba {:#?} y se obtuvo {:#?}", esperado, resultado);

            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(hoy+deadline*5);
            let esperado: Vec<Socio> = Vec::from([a.clone(), b.clone(), c.clone(), d.clone(), e.clone(), f.clone(), g.clone()]);
            let gestor = GestorDeCobros::new();
            let resultado = gestor.socios_morosos();

            assert_eq!(esperado, resultado, "Se esperaba {:#?} y se obtuvo {:#?}", esperado, resultado);
        }
    }
}


