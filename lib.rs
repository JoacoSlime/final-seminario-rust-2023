#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[ink::contract]
mod gestor_de_cobros {
    use club_sem_rust::Recibo;
    use club_sem_rust::Socio;
    use club_sem_rust::ClubSemRustRef;


    use ink::prelude::vec::Vec;


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
            let canon = 1_000_000_000_000;
            let precios = Vec::from([5*canon, 4*canon, 2*canon]);
            let descuento = Some(15);
            let mut a = Socio::new("Alice".to_string(), 44044044, [0;32].into(), 1, None, hoy+deadline, precios.clone());
            let mut b = Socio::new("Bob".to_string(), 45045045, [1;32].into(), 2, Some(6), hoy+deadline, precios.clone());
            let mut c = Socio::new("Carol".to_string(), 46046046, [2;32].into(), 2, Some(1), hoy+deadline, precios.clone());
            let mut d = Socio::new("Derek".to_string(), 47047047, [3;32].into(), 1, None, hoy+deadline, precios.clone());
            let e = Socio::new("Emily".to_string(), 48048048, [4;32].into(), 1, None, hoy+deadline, precios.clone());
            let mut f = Socio::new("Frank".to_string(), 49049049, [5;32].into(), 3, None, hoy+deadline, precios.clone());
            let g = Socio::new("Gary".to_string(), 50050050, [6;32].into(), 3, None, hoy+deadline, precios.clone());
            
            a.realizar_pago(descuento, 3, 5*canon, hoy, precios.clone(), deadline);
            a.realizar_pago(descuento, 3, 5*canon, hoy + 100_000, precios.clone(), deadline);
            a.realizar_pago(descuento, 3, 5*canon, hoy + 200_000, precios.clone(), deadline);
            a.realizar_pago(descuento, 3, 425*(canon/100), hoy + 400_000, precios.clone(), deadline);

            b.realizar_pago(descuento, 3, 4*canon, hoy, precios.clone(), deadline);
            b.realizar_pago(descuento, 3, 4*canon, hoy + deadline + 100_000, precios.clone(), deadline);

            c.realizar_pago(descuento, 3, 4*canon, hoy, precios.clone(), deadline);
            c.realizar_pago(descuento, 3, 4*canon, hoy + 100_000, precios.clone(), deadline);

            d.realizar_pago(descuento, 3, 5*canon, hoy, precios.clone(), deadline);
            d.realizar_pago(descuento, 3, 5*canon, hoy + 100_000, precios.clone(), deadline);
            d.realizar_pago(descuento, 3, 5*canon, hoy + 200_000, precios.clone(), deadline);
            d.realizar_pago(descuento, 3, 425*(canon/100), hoy + deadline + 200_000, precios.clone(), deadline);

            f.realizar_pago(descuento, 3, 2*canon, hoy, precios.clone(), deadline);
            f.realizar_pago(descuento, 3, 2*canon, hoy + 100_000, precios.clone(), deadline);
            f.realizar_pago(descuento, 3, 2*canon, hoy + 200_000, precios.clone(), deadline);
            f.realizar_pago(descuento, 3, 17*(canon/10), hoy + 300_000, precios.clone(), deadline);

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
            let socios_morosos = socios.into_iter()
            .filter(|s| s.es_moroso(hoy)).collect();
            return socios_morosos;
        }

        /// Devuelve un vector con la lista de todos aquellos socios no morosos que tienen permitido asistir a una actividad deportiva específica del club.
        /// Se pasa por parámetro el id_deporte correspondiente a la actividad deportiva que se desea consultar.
        /// 
        /// En caso de consultar por una actividad que no es practicada por ningún Socio, devuelve un vector vacío.
        #[ink(message)]
        pub fn socios_no_morosos(&self, id_deporte: u32) -> Vec<Socio> {
            if id_deporte == 2 {
                let hoy :u64 = self.env().block_timestamp();
                let socios :Vec<Socio> = self.get_socios();
                let socios_no_morosos :Vec<Socio> = socios.into_iter()
                .filter(|s| !s.es_moroso(hoy)).collect();
                return socios_no_morosos;
            }else{
                if id_deporte < 1 || id_deporte > 8 {
                    panic!("El ID de deporte ingresado es inválido.")
                }else{
                    let hoy :u64 = self.env().block_timestamp();
                    let socios :Vec<Socio> = self.get_socios();
                    let socios_no_morosos :Vec<Socio> = socios.into_iter()
                    .filter(|s| s.puede_hacer_deporte(id_deporte) && !s.es_moroso(hoy))
                    .collect();
                    return socios_no_morosos;
                }
            }
        }

        /// Recibe mes y año y devuelve la recaudación total de ese mes
        /// La recaudación cuenta todos los pagos realizados por todos los socios de todas las categorías.
        /// 
        /// Se considera el paso de 30 días como el paso de un mes.
        /// 2_592_000_000 representa 30 dias para el tipo Timestamp.
        /// 
        #[ink(message)]
        pub fn get_recaudacion(&self, mes:u8, año:u16) -> u128 {

            let socios:Vec<Socio> = self.get_socios();

            let fecha_from: Timestamp = self.date_to_timestamp(mes as u64, año as u64);
            let fecha_to: Timestamp = self.date_to_timestamp(mes as u64, año as u64) + 2_592_000_000;

            let recibos: Vec<Recibo> = socios.iter()
            .map(|s| s.generar_recibos()).flatten().collect();
            let recaudacion:u128 = recibos.iter()
            .filter(|r| r.fecha_entre(fecha_from, fecha_to))
            .map(|r| r.get_monto())
            .sum();
            
            return recaudacion;

        }
        /// Método auxiliar para conversión de Fecha a Timestamp.
        /// 
        /// 1 día = 86_400_000 mili segundos
        /// 
        /// # Panic
        /// 
        ///  - No funciona para fechas más antiguas que el Epoch de Unix (1ro de Enero, 1970).
        ///  - Se debe ingresar un mes del 1 al 12, devuelve panic si se ingresa un número por fuera de este rango.
        fn date_to_timestamp(&self, mes:u64, año:u64) -> Timestamp {
            if mes < 1 || mes > 12{
                panic!("El número de mes enviado no es válido");
            }
            if año < 1970 {
                panic!("La fecha ingresada es menor que la Unix epoch (1ro de Enero, 1970)");
            }else{
                let mut segs_años: u64 = 0;
                for i in 1970..año{
                    if self.es_bisiesto(i as u16){
                        segs_años += 86_400_000 * 366;
                    }else{
                        segs_años += 86_400_000 * 365;
                    }
                }
                let mut segs_meses: u64 = 0;
                if mes == 1 { return  segs_años as Timestamp;
                }else{
                    for j in 1..mes{
                        match j {
                            1 => segs_meses += 86_400_000*31,
                            2 => { if self.es_bisiesto(año as u16){
                                segs_meses += 86_400_000*29;
                                }else{
                                    segs_meses += 86_400_000*28;
                                }
                            },
                            3 => segs_meses += 86_400_000*31,
                            4 => segs_meses += 86_400_000*30,
                            5 => segs_meses += 86_400_000*31,
                            6 => segs_meses += 86_400_000*30,
                            7 => segs_meses += 86_400_000*31,
                            8 => segs_meses += 86_400_000*31,
                            9 => segs_meses += 86_400_000*30,
                            10 => segs_meses += 86_400_000*31,
                            11 => segs_meses += 86_400_000*30,
                            _ => panic!("El número de mes enviado no es válido"),
                        }
                    }
                    return  segs_años + segs_meses as Timestamp;    
                }
            }
        }

        fn es_bisiesto(&self, año:u16) -> bool {
            if año %4 !=0 {
                return false;
            }else{
                if año %4 == 0 && año % 100 != 0{
                    return true;
                }else{
                    if año % 4 == 0 && año % 100 == 0 && año % 400 != 0{
                        return false;
                    }else{
                        if año % 4 == 0 && año % 100 == 0 && año % 400 == 0 {
                            return true;
                        }else{
                            return false;
                        } 
                    }
                }
            }
        }

    }

    #[cfg(test)]
    mod gestor_de_cobros_tests {
        use club_sem_rust::Socio;

        use super::GestorDeCobros;

        #[test]
        #[should_panic(expected = "La fecha ingresada es menor que la Unix epoch (1ro de Enero, 1970)")]
        pub fn test_panic_date_to_timestamp(){
            let gestor = GestorDeCobros::new();

            gestor.date_to_timestamp(1, 1969);
        }

        #[test]
        #[should_panic(expected = "El número de mes enviado no es válido")]
        pub fn test_panic_mes_date_to_timestamp(){
            let gestor = GestorDeCobros::new();

            gestor.date_to_timestamp(13, 2000);
        }

        #[test]
        pub fn test_date_to_timestamp(){
            let gestor = GestorDeCobros::new();

            let result_1 = gestor.date_to_timestamp(1, 2023);
            let result_2 = gestor.date_to_timestamp(2, 2023);
            let result_2_bisiesto = gestor.date_to_timestamp(2, 2004);
            let result_3_bisiesto = gestor.date_to_timestamp(3, 2004);
            let result_3 = gestor.date_to_timestamp(3, 2023);
            let result_4 = gestor.date_to_timestamp(4, 2023);
            let result_5 = gestor.date_to_timestamp(5, 2023);
            let result_6 = gestor.date_to_timestamp(6, 2023);
            let result_7 = gestor.date_to_timestamp(7, 2023);
            let result_8 = gestor.date_to_timestamp(8, 2023);
            let result_9 = gestor.date_to_timestamp(9, 2023);
            let result_10 = gestor.date_to_timestamp(10, 2023);
            let result_11 = gestor.date_to_timestamp(11, 2023);
            let result_12 = gestor.date_to_timestamp(12, 2023);
            let result_2100 = gestor.date_to_timestamp(10, 2100);

            assert_eq!(result_1, 1_672_531_200_000);
            assert_eq!(result_2, 1_675_209_600_000);
            assert_eq!(result_2_bisiesto, 1_075_593_600_000);
            assert_eq!(result_3_bisiesto, 1_078_099_200_000);
            assert_eq!(result_3, 1_677_628_800_000);
            assert_eq!(result_4, 1_680_307_200_000);
            assert_eq!(result_5, 1_682_899_200_000);
            assert_eq!(result_6, 1_685_577_600_000);
            assert_eq!(result_7, 1_688_169_600_000);
            assert_eq!(result_8, 1_690_848_000_000);
            assert_eq!(result_9, 1_693_526_400_000);
            assert_eq!(result_10, 1_696_118_400_000);
            assert_eq!(result_11, 1_698_796_800_000);
            assert_eq!(result_12, 1_701_388_800_000);
            assert_eq!(result_2100, 4_126_032_000_000);

        }

        #[ink::test]
        fn get_recaudacion_test(){
            let canon = 1_000_000_000_000;
            let hoy: crate::gestor_de_cobros::Timestamp = 1_690_000_000_000; // Saturday, July 22, 2023 4:26:40 AM
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(hoy);

            let gestor = GestorDeCobros::new();

            assert_eq!(gestor.get_recaudacion(7, 2023), 53_950*canon/1000);
        }

        #[ink::test]
        fn socios_no_morosos_test() { // Debe extenderse
            let canon = 1_000_000_000_000;
            let deadline = 864_000_000;
            let hoy: crate::gestor_de_cobros::Timestamp = 1_690_000_000_000;
            let precios = Vec::from([5*canon,4*canon,2*canon]);
            let descuento = Some(15);
            let mut a = Socio::new("Alice".to_string(), 44044044, [0;32].into(), 1, None, hoy+deadline, precios.clone());
            let mut c = Socio::new("Carol".to_string(), 46046046, [2;32].into(), 2, Some(1), hoy+deadline, precios.clone());
            let mut d = Socio::new("Derek".to_string(), 47047047, [3;32].into(), 1, None, hoy+deadline, precios.clone());
            let e = Socio::new("Emily".to_string(), 48048048, [4;32].into(), 1, None, hoy+deadline, precios.clone());
            
            a.realizar_pago(descuento, 3, 5*canon, hoy, precios.clone(), deadline);
            a.realizar_pago(descuento, 3, 5*canon, hoy + 100_000, precios.clone(), deadline);
            a.realizar_pago(descuento, 3, 5*canon, hoy + 200_000, precios.clone(), deadline);
            a.realizar_pago(descuento, 3, 425*canon/100, hoy + 400_000, precios.clone(), deadline);

            c.realizar_pago(descuento, 3, 4*canon, hoy, precios.clone(), deadline);
            c.realizar_pago(descuento, 3, 4*canon, hoy + 100_000, precios.clone(), deadline);

            d.realizar_pago(descuento, 3, 5*canon, hoy, precios.clone(), deadline);
            d.realizar_pago(descuento, 3, 5*canon, hoy + 100_000, precios.clone(), deadline);
            d.realizar_pago(descuento, 3, 5*canon, hoy + 200_000, precios.clone(), deadline);
            d.realizar_pago(descuento, 3, 425*canon/100, hoy + deadline + 200_000, precios.clone(), deadline);

            let esperado: Vec<Socio> = Vec::from([a,c,d,e]);
            let gestor = GestorDeCobros::new();
            let resultado = gestor.socios_no_morosos(1);

            assert_eq!(esperado, resultado, "Se esperaba {:#?} y se obtuvo {:#?}", esperado, resultado);
        }

        #[ink::test]
        fn socios_morosos_test() {
            let canon = 1_000_000_000_000;
            let hoy: crate::gestor_de_cobros::Timestamp = 1_690_000_000_000;
            let deadline = 864_000_000;

            let precios = Vec::from([5*canon, 4*canon, 2*canon]);
            let descuento = Some(15);
            let mut a = Socio::new("Alice".to_string(), 44044044, [0;32].into(), 1, None, hoy+deadline, precios.clone());
            let mut b = Socio::new("Bob".to_string(), 45045045, [1;32].into(), 2, Some(6), hoy+deadline, precios.clone());
            let mut c = Socio::new("Carol".to_string(), 46046046, [2;32].into(), 2, Some(1), hoy+deadline, precios.clone());
            let mut d = Socio::new("Derek".to_string(), 47047047, [3;32].into(), 1, None, hoy+deadline, precios.clone());
            let e = Socio::new("Emily".to_string(), 48048048, [4;32].into(), 1, None, hoy+deadline, precios.clone());
            let mut f = Socio::new("Frank".to_string(), 49049049, [5;32].into(), 3, None, hoy+deadline, precios.clone());
            let g = Socio::new("Gary".to_string(), 50050050, [6;32].into(), 3, None, hoy+deadline, precios.clone());
            
            a.realizar_pago(descuento, 3, 5*canon, hoy, precios.clone(), deadline);
            a.realizar_pago(descuento, 3, 5*canon, hoy + 100_000, precios.clone(), deadline);
            a.realizar_pago(descuento, 3, 5*canon, hoy + 200_000, precios.clone(), deadline);
            a.realizar_pago(descuento, 3, 425*canon/100, hoy + 400_000, precios.clone(), deadline);

            b.realizar_pago(descuento, 3, 4*canon, hoy, precios.clone(), deadline);
            b.realizar_pago(descuento, 3, 4*canon, hoy + deadline + 100_000, precios.clone(), deadline);

            c.realizar_pago(descuento, 3, 4*canon, hoy, precios.clone(), deadline);
            c.realizar_pago(descuento, 3, 4*canon, hoy + 100_000, precios.clone(), deadline);

            d.realizar_pago(descuento, 3, 5*canon, hoy, precios.clone(), deadline);
            d.realizar_pago(descuento, 3, 5*canon, hoy + 100_000, precios.clone(), deadline);
            d.realizar_pago(descuento, 3, 5*canon, hoy + 200_000, precios.clone(), deadline);
            d.realizar_pago(descuento, 3, 425*canon/100, hoy + deadline + 200_000, precios.clone(), deadline);

            f.realizar_pago(descuento, 3, 2*canon, hoy, precios.clone(), deadline);
            f.realizar_pago(descuento, 3, 2*canon, hoy + 100_000, precios.clone(), deadline);
            f.realizar_pago(descuento, 3, 2*canon, hoy + 200_000, precios.clone(), deadline);
            f.realizar_pago(descuento, 3, 17*canon/10, hoy + 300_000, precios.clone(), deadline);

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

            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(hoy+deadline*6);
            let esperado: Vec<Socio> = Vec::from([a.clone(), b.clone(), c.clone(), d.clone(), e.clone(), f.clone(), g.clone()]);
            let gestor = GestorDeCobros::new();
            let resultado = gestor.socios_morosos();

            assert_eq!(esperado, resultado, "Se esperaba {:#?} y se obtuvo {:#?}", esperado, resultado);
        }
    
    }

}


