#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use self::club_sem_rust::ClubSemRustRef;
#[ink::contract]
mod club_sem_rust {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;

    #[derive(scale::Decode, scale::Encode, Debug, Clone, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Socio{
        id_deporte: Option<u32>,
        id_categoria: u32,
        dni: u32,
        nombre: String,
        pagos: Vec<Pago>,
    }
    impl Socio{
        ///
        /// Construye un nuevo Socio con sus variables de a cuerdo a lo que le enviemos por parametro.
        ///
        /// Empieza con un Pago pendiente
        ///
        pub fn new(nombre: String, dni:u32, id_categoria: u32, id_deporte: Option<u32>, vencimiento:Timestamp) -> Socio {
            Socio {
                id_deporte,
                id_categoria,
                dni,
                nombre,
                pagos: Vec::new(),
            }
            // TODO: AVISO, esta función puede ser modificada en un futuro en caso de no querer llamar a Deportes::get_deportes(id_deporte) en otras funciones. - Joaco
        }

        pub fn puede_hacer_deporte(&self, id_deporte: u32) -> bool {
            todo!()
        }

        pub fn generar_recibo(&self) -> Vec<Recibo> {
            todo!()
        }

        pub fn cumple_bonificacion(&self, pagos_consecutivos: u32) -> bool {
            if self.pagos.len() < pagos_consecutivos as usize {
                return false
            }else{
                let m = self.pagos.len() - pagos_consecutivos as usize;
                let j = self.pagos.len();
                for i in m..j{
                    if self.pagos[i].aplico_descuento || !self.pagos[i].a_tiempo{
                        return false
                    }
                }
                return true
            }
        }

        pub fn cambiar_categoria(&mut self, id_categoria: u32, id_deporte: Option<u32>) {
            self.id_categoria = id_categoria;
            self.id_deporte = id_deporte;
        }

        pub fn get_mi_deporte(&self) -> Option<Vec<Deporte>>{
            // Arreglado para derivar lógica (Probablemente haya que modificarlo, pero al menos ahora compila (?) - Joaco
            if let Some(id) = self.id_deporte {
                Categoria::match_categoria(self.id_categoria).get_deporte(id)
            } else {
                None
            }
        }
    }

    

    #[derive(scale::Decode, scale::Encode, Debug, Clone, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Recibo {
        nombre: String,
        dni: u32,
        monto: u128,
        categoria: Categoria,
        fecha: Timestamp,
    }
    impl Recibo {
        pub fn new(nombre: String, dni:u32, monto:u128, categoria:Categoria, fecha:Timestamp) -> Recibo {
            Recibo { nombre, dni, monto, categoria, fecha, }
        }
        //o, recibe id_categoria y lo matchea con un tipo categoria 
    }



    #[derive(scale::Decode, scale::Encode, Debug, Clone, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Pago {
        vencimiento: Timestamp, // if(current_time >= vencimiento) then vencido
        categoria: Categoria,   // vencimiento = now + deadline_duration
        pendiente: bool,
        a_tiempo: bool,
        aplico_descuento: bool,
        fecha_pago: Option<Timestamp>,
        monto_pagado: Option<u128>,
    }
    impl Pago {
        pub fn new(vencimiento:Timestamp, id_categoria: u32) -> Pago {
            Pago { vencimiento, categoria: Categoria::new(id_categoria), pendiente: true, a_tiempo: false, aplico_descuento: false, fecha_pago: None , monto_pagado: None}
        }
    
        pub fn verificar_pago(&self, monto: u128) -> bool {
            self.categoria.mensual() == monto
        }
    
        pub fn realizar_pago(&mut self, monto: u128, fecha: Timestamp) {
            todo!()
            /*
            > verifica el pago
            > si es correcto el monto ingresado
            > aplico_descuento? verificar eso
            > poner el monto_pagado, fecha_pago, pendiente -> false
            > a_tiempo -> true SI fecha_hoy < fecha_vencimiento
            */
        }
    }


    #[derive(scale::Decode, scale::Encode, Debug, Clone, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Categoria {
        A,
        B,
        C,
    }
    impl Categoria {
        pub fn new(id_categoria:u32) -> Categoria {
            match id_categoria {
                1 => Self::A,
                2 => Self::B,
                3 => Self::C,
                _ => panic!("ID de categoría inválido, por favor revise el socio."),
            }
        }

        pub fn match_categoria(id_categoria: u32) -> Self {
            match id_categoria {
                1 => Self::A,
                2 => Self::B,
                3 => Self::C,
                _ => panic!("ID de categoría inválido, por favor revise el socio."),
            }
        }
    
        pub fn get_deporte(&self, id_deporte: u32) -> Option<Vec<Deporte>> {
            match self {
                Self::A => Some(Deporte::get_deportes()),
                Self::B => Some(vec![Deporte::match_deporte(id_deporte)]),
                Self::C => None,
            }
        }
    
        pub fn mensual(&self) -> u128 {
            match self {
                Categoria::A => 5000,
                Categoria::B => 3000,
                Categoria::C => 2000,
            }
        }
    }



    #[derive(scale::Decode, scale::Encode, Debug, Clone, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Deporte {
        Futbol,
        Gimnasio,
        Basquet,
        Rugby,
        Hockey,
        Natacion,
        Tenis,
        Paddle
    }
    impl Deporte {
        ///
        /// Devuelve el vector de todos los deportes existentes.
        ///
        /// TODO: Explicación a rellenear por el implementador el método
        ///
        pub fn get_deportes() -> Vec<Deporte> {
            vec![
                Self::Futbol,
                Self::Gimnasio,
                Self::Basquet,
                Self::Rugby,
                Self::Hockey,
                Self::Natacion,
                Self::Tenis,
                Self::Paddle
            ]
        }
    
        pub fn match_deporte(id_deporte: u32) -> Self {
            match id_deporte {
                1 => Self::Futbol,
                2 => Self::Gimnasio,
                3 => Self::Basquet,
                4 => Self::Rugby,
                5 => Self::Hockey,
                6 => Self::Natacion,
                7 => Self::Tenis,
                8 => Self::Paddle,
                _ => panic!("Id del deporte inválido, revise el ID del socio."),
            }
        }
    }









    /// Storage del contrato
    /// 
    /// Contiene:
    /// - La lista de los socios registrados.
    /// - El porcentaje de descuentos.
    /// - Los precios de cada categoría.
    /// - El tiempo máximo para verificar exitosamente un pago.
    /// - La cantidad de pagos consecutivos necesarios para dar un descuento.
    /// - El ID de las cuentas habilitadas a usar métodos que hacen escrituras.
    /// - Un boolean que indica si el archivo está bloqueado
    #[ink(storage)]
    #[derive(PartialEq)]
    pub struct ClubSemRust {
        socios: Vec<Socio>,
        descuento: u128,
        precio_categorias: (u128, u128, u128),
        duracion_deadline: Timestamp,
        pagos_consecutivos_bono: u32,
        cuentas_habilitadas: Vec<AccountId>,
        esta_bloqueado: bool,
    }

    impl ClubSemRust {
        #[ink(constructor)]
        pub fn new(descuento: u128, duracion_deadline: Timestamp, precio_categoria_a: u128, precio_categoria_b: u128, precio_categoria_c: u128, pagos_consecutivos_bono: u32) -> Self {
            Self {
                socios: Vec::new(),
                descuento,
                duracion_deadline,
                precio_categorias:(precio_categoria_a, precio_categoria_b ,precio_categoria_c),
                pagos_consecutivos_bono,
                cuentas_habilitadas: Vec::new(),
                esta_bloqueado: false,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            // 864000000 es 10 días 
            Self::new(15, 864000000, 5000, 3000, 2000, 3)
        }

        #[ink(message)]
        pub fn set_duracion_deadline(&mut self, d_deadline: Timestamp) {
            self.duracion_deadline = d_deadline;
        }
        
        #[ink(message)]
        pub fn get_duracion_deadline(&self) -> Timestamp {
            self.duracion_deadline
        }

        #[ink(message)]
        pub fn set_descuento(&mut self, descuento: u128) {
            self.descuento = descuento;
        }

        
        #[ink(message)]
        pub fn get_descuento(&self) -> u128 {
            self.descuento
        }
        
        #[ink(message)]
        pub fn registrar_nuevo_socio(&mut self, nombre: String, dni:u32, id_categoria: u32, id_deporte: Option<u32>) {
            let hoy = self.env().block_timestamp() + self.duracion_deadline;
            let socio = Socio::new(nombre, dni, id_categoria, id_deporte, hoy);
            self.socios.push(socio);
        }
        
        #[ink(message)]
        pub fn registrar_pago(&mut self, dni: u32, monto: u128) {
            let hoy = self.env().block_timestamp();
            todo!()
        }
        
        #[ink(message)]
        pub fn get_socios(&self) -> Vec<Socio> {
            self.socios.clone()
        }
        
        #[ink(message)]
        pub fn get_recibos(&self, dni: u32) -> Option<Vec<Recibo>> {
            if let Some(socio) = self.socios.iter().find(|s| s.dni == dni){
                Some(socio.generar_recibo())
            } else {
                None
            }
        }
        
        #[ink(message)]
        pub fn agregar_cuenta(&mut self, id: AccountId) {
            todo!()
        }

        #[ink(message)]
        pub fn flip_bloqueo(&mut self) {
            self.esta_bloqueado = !self.esta_bloqueado
        }
        
        ///
        /// Retorna true si una cuenta está habilitada.
        ///
        /// Itera sobre el vector de AccountId de la estructura y devuelve true si encuentra 
        /// una cuenta que concuerde con el id pasado por parámetro
        ///
        fn esta_habilitada(&self, id: AccountId) -> bool {
            self.cuentas_habilitadas.iter().any(|account_id| *account_id == id)
        }
    }




    

    #[cfg(test)]
    mod deporte_tests {
        use crate::club_sem_rust::Deporte;

        #[test]
        fn get_deportes_test(){
            let esperado: Vec<Deporte> = vec![
                Deporte::Futbol,
                Deporte::Gimnasio,
                Deporte::Basquet,
                Deporte::Rugby,
                Deporte::Hockey,
                Deporte::Natacion,
                Deporte::Tenis,
                Deporte::Paddle
            ];
            let recibido: Vec<Deporte> = Deporte::get_deportes();

            assert_eq!(esperado, recibido, "Error en Deporte::get_deportes(), se esperaba {:?}, y se recibió {:?}", esperado, recibido)
        }

        fn match_deporte_test() {
            let ids = [
                (1, Deporte::Futbol),
                (2, Deporte::Gimnasio),
                (3, Deporte::Basquet),
                (4, Deporte::Rugby),
                (5, Deporte::Hockey),
                (6, Deporte::Natacion),
                (7, Deporte::Tenis),
                (8, Deporte::Paddle),
            ];
            for (id, dep) in ids {
                let esperado = dep;
                let resultado = Deporte::match_deporte(id);
                assert_eq!(esperado, resultado, "Error, para id {} se esperaba {:?}, y se recibió {:?}", id, esperado, resultado);
            };
            let resultado = std::panic::catch_unwind(|| Deporte::match_deporte(0));
            assert!(resultado.is_err());
            let resultado = std::panic::catch_unwind(|| Deporte::match_deporte(9));
            assert!(resultado.is_err());
        }
    }

    #[cfg(test)]
    mod club_sem_rust_tests {
        use crate::club_sem_rust::ClubSemRust;
        use crate::club_sem_rust::Socio;

        #[test]
        fn new_test(){
            let esperado = ClubSemRust{
                socios: Vec::new(),
                descuento: 25,
                precio_categorias: (400, 300, 200),
                duracion_deadline: 999,
                pagos_consecutivos_bono: 10,
                cuentas_habilitadas: Vec::new(),
                esta_bloqueado: false
            };
            let resultado = ClubSemRust::new(25, 999, 400, 300, 200, 10);

            assert_eq!(esperado, resultado, "Error en ClubSemRust::new(), se esperaba {:?} y se obtuvo {:?}", esperado, resultado)
        }

        #[test]
        fn default_test() {
            let esperado = ClubSemRust{
                socios: Vec::new(),
                descuento: 15,
                precio_categorias: (5000, 3000, 2000),
                duracion_deadline: 864000000,
                pagos_consecutivos_bono: 3,
                cuentas_habilitadas: Vec::new(),
                esta_bloqueado: false
            };
            let resultado = ClubSemRust::default();

            assert_eq!(esperado, resultado, "Error en ClubSemRust::default(), se esperaba {:?} y se recibió {:?}", esperado, resultado)
        }

        #[test]
        fn get_duracion_deadline_test() {
            let esperado = 864000000;
            let club = ClubSemRust::default();
            let resultado = club.get_duracion_deadline();

            assert_eq!(esperado, resultado, "Error en ClubSemRust::get_duracion_deadline(), se esperaba {:?} y se recibió {:?}, esperado, resultado");

            let esperado = 999; 
            let club = ClubSemRust::new(25, 999, 400, 300, 200, 10);
            let resultado = club.get_duracion_deadline();
            
            assert_eq!(esperado, resultado, "Error en ClubSemRust::get_duracion_deadline(), se esperaba {:?} y se recibió {:?}, esperado, resultado");
        }

        #[test]
        fn set_duracion_deadline_test() {
            let esperado = ClubSemRust{
                socios: Vec::new(),
                descuento: 15,
                precio_categorias: (5000, 3000, 2000),
                duracion_deadline: 999,
                pagos_consecutivos_bono: 3,
                cuentas_habilitadas: Vec::new(),
                esta_bloqueado: false
            };
            let mut club = ClubSemRust::default();
            club.set_duracion_deadline(999);
            
            assert_eq!(esperado, resultado, "Error en ClubSemRust::set_duracion_deadline(), se esperaba {:?} y se recibió {:?}, esperado, resultado");
        }

        #[test]
        fn get_descuento_test() {
            let esperado = 15;
            let club = ClubSemRust::default();
            let resultado = club.get_descuento();

            assert_eq!(esperado, resultado, "Error en ClubSemRust::get_descuento(), se esperaba {:?} y se recibió {:?}, esperado, resultado");

            let esperado = 25; 
            let club = ClubSemRust::new(25, 999, 400, 300, 200, 10);
            let resultado = club.get_descuento();
            
            assert_eq!(esperado, resultado, "Error en ClubSemRust::get_descuento(), se esperaba {:?} y se recibió {:?}, esperado, resultado");
        }

        #[test]
        fn set_descuento_test() {
            let esperado = ClubSemRust{
                socios: Vec::new(),
                descuento: 25,
                precio_categorias: (5000, 3000, 2000),
                duracion_deadline: 999,
                pagos_consecutivos_bono: 3,
                cuentas_habilitadas: Vec::new(),
                esta_bloqueado: false
            };
            let mut club = ClubSemRust::default();
            club.set_descuento(25);
            
            assert_eq!(esperado, resultado, "Error en ClubSemRust::set_duracion_deadline(), se esperaba {:?} y se recibió {:?}, esperado, resultado");
        }

        #[test]
        fn registrar_nuevo_socio_test() {
            let esperado = ClubSemRust{
                socios: Vec::from([Socio{
                    id_deporte: None,
                    id_categoria: 3,
                    dni: 44044044,
                    nombre: "Juancito".to_string(),
                    pagos: Vec::new(),
                }]),
                descuento: 15,
                precio_categorias: (5000, 3000, 2000),
                duracion_deadline: 864000000,
                pagos_consecutivos_bono: 3,
                cuentas_habilitadas: Vec::new(),
                esta_bloqueado: false
            };
            let mut club = ClubSemRust::default();
            club.registrar_nuevo_socio("Juancito".to_string(), 44044044, 3, None);
            
            assert_eq!(esperado, resultado, "Error en ClubSemRust::registrar_nuevo_socio(), se esperaba {:?} y se recibió {:?}, esperado, resultado");

            
            let esperado = ClubSemRust{
                socios: Vec::from([Socio{
                    id_deporte: None,
                    id_categoria: 3,
                    dni: 44044044,
                    nombre: "Juancito".to_string(),
                    pagos: Vec::new(),
                }, Socio{
                    id_deporte: Some(5),
                    id_categoria: 3,
                    dni: 45045045,
                    nombre: "Roberto".to_string(),
                    pagos: Vec::new(),
                }]),
                descuento: 15,
                precio_categorias: (5000, 3000, 2000),
                duracion_deadline: 864000000,
                pagos_consecutivos_bono: 3,
                cuentas_habilitadas: Vec::new(),
                esta_bloqueado: false
            };
            let mut club = ClubSemRust::default();
            club.registrar_nuevo_socio("Juancito".to_string(), 44044044, 3, None);
            club.registrar_nuevo_socio("Roberto".to_string(), 45045045, 2, Some(5));
            
            assert_eq!(esperado, resultado, "Error en ClubSemRust::registrar_nuevo_socio(), se esperaba {:?} y se recibió {:?}, esperado, resultado");
        }
        
    }

}
