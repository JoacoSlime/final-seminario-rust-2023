//! Trabajo final de Seminario de Lenguajes: Rust (2023)
//! 
//! Este modulo es un trabajo conceptual del uso de Ink! para la creación de smart-contracts.
//! No se recomienda su utilización en un escenario de producción.
//! 
//! [`Github`]: https://github.com/JoacoSlime/final-seminario-rust-2023
//! 

#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use self::club_sem_rust::{ClubSemRustRef, Recibo, Socio};
#[ink::contract]
mod club_sem_rust {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink_e2e::env_logger::fmt::Timestamp;

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
            if id_categoria == 2 && id_deporte == None{
                panic!("Categoria B debe elegir un deporte");
            }else{
                if id_categoria == 1 || id_categoria == 3 && id_deporte != None{
                    panic!("Categoria A y Categoria C no deben elegir un deporte  -- Este campo debe permanecer vacio");
                }else{
                    let pago_inicial:Vec<Pago> = vec![Pago::new(vencimiento, id_categoria)];
                    Socio {
                        id_deporte,
                        id_categoria,
                        dni,
                        nombre,
                        pagos: pago_inicial,
                    }
                }
            }
            // TODO: AVISO, esta función puede ser modificada en un futuro en caso de no querer llamar a Deportes::get_deportes(id_deporte) en otras funciones. - Joaco
        }
        ///
	    /// Verifica si un determinado usuario esta habilitado o no para realizar un determinado deporte
        ///
        /// Recibe el id_deporte que se quiere verificar
        ///
        pub fn puede_hacer_deporte(&self, id_deporte: u32) -> bool {
            match self.id_categoria {
            	1 => return true,
            	2 => match id_deporte{
                        2 => return true,   //si el id es gimnasio, Categoria B deberia devolver true
                        _=> if let Some(id_dep) = self.id_deporte { //si no es gimnasio, chequear que coincida
                                return id_dep == id_deporte;
                            }else{
                                return false;
                            },
                    },
        		3 => match id_deporte{
                        2 => return true,   //si el id es gimnasio, Categoria C deberia devolver true
                        _=> return false,
                    },
                _ => panic!("ID de categoría inválido, por favor revise el socio."),
    	    }
        }
        //El metodo generar_recibos recorre los pagos y en caso de que no figure como pendiente crea el recibo y lo agrega al vec de recibos
        pub fn generar_recibos(&mut self) -> Vec<Recibo> {
            let mut recibos = Vec::new();
            if self.pagos.len() != 0 {
                for i in 0..self.pagos.len(){
                    if self.pagos[i].pendiente == false{
                        match self.pagos[i].fecha_pago{
                            Some(fe) => {
                                if let Some(monto_pagado) = self.pagos[i].monto_pagado {
                                let recibo = Recibo::new(self.nombre, self.dni, monto_pagado, self.id_categoria, fe );
                                recibos.push(recibo);
                                }
                            },
                            None => panic!("ESTE SOCIO REGISTRA UN PAGO SIN FECHA")
                        }
                    }
                }
            }
            return recibos
        }
        ///
        /// Consulta el ultimo pago y devuelve si esta vencido y sin pagar
        /// Si devuelve true el socio se considera moroso
        /// 
        pub fn es_moroso(&self, current_time:Timestamp) -> bool {
            if let Some(ultimo_pago) = self.pagos.last(){
                return ultimo_pago.es_moroso(current_time);
            }else{
                panic!("No hay ningun pago registrado ni hecho ni por haber para este socio");
            }
        }
        ///
        /// Socio realiza un Pago, se crea un nuevo Pago pendiente con una nueva fecha de vencimiento
        /// 
        /// Socio siempre tendrá un único Pago pendiente en último índice de su lista de Pagos
        /// La creación de un nuevo Pago pendiente se da automáticamente una vez pagado el anterior
        /// 
        pub fn realizar_pago(&mut self, descuento: Option<u128>, monto: u128, fecha: Timestamp, precio_categorias: Vec<u128>, deadline:Timestamp){
            if let Some(i) = self.pagos.iter().position(|p| p.pendiente){
                self.pagos[i].realizar_pago(descuento, monto, fecha, precio_categorias);
                self.pagos.push(Pago::new(fecha+deadline, self.id_categoria));
            }else{
                panic!("Este socio no tiene pagos");
            }
        }
        ///
	    /// Consulta los pagos mas recientes del Socio y devuelve true si cumple los requisitos para la bonificacion
        ///
        /// Recibe por parametro la cantidad de pagos que figuren como pagados "a tiempo" necesarios para aplicar la bonificacion
        /// cumple_bonificacion funciona como un shor-circuit. Cuando encuentra un pago que no cumple devuelve false y termina su ejecucion
        ///
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
        ///
	    /// Permite al usuario cambiar su propia categoria
        ///
        /// Si el id_categoria y/o id_deporte ingresados son invalidos, no guarda ningun cambio y se genera un panic
        ///
        pub fn cambiar_categoria(&mut self, id_categoria: u32, id_deporte: Option<u32>) {
            //To Do: if id_categoria = 2 && Id_deporte = None -> Panics!  -L
            if id_categoria == 2 && id_deporte == None{
                panic!("Si se desea cambiar a Categoria B, se debe elegir un deporte");
            }else{
                if id_categoria == 3 || id_categoria == 1 && id_deporte != None{
                    panic!("Si se desea cambiar a Categoria A o C, no se debe elegir un deporte");
                }else{
                    self.id_categoria = id_categoria;
                    self.id_deporte = id_deporte;
                }
            }
        }

        ///
	    /// Devuelve todos los deportes que realiza un determinado Socio
        ///
        /// Si es de categoria 1 devuelve None
        ///
        pub fn get_mi_deporte(&self) -> Option<Vec<Deporte>>{
            match self.id_categoria {
                1 => return None,
                2 => Categoria::match_categoria(self.id_categoria).get_deporte(self.id_deporte),
                3 => return Categoria::match_categoria(self.id_categoria).get_deporte(None),
                _ => panic!("ID de categoría inválido, por favor revise el socio."),
            }
        }
        pub fn mi_categoria(&self, id_c:u32) -> bool {
            return self.id_categoria == id_c;
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
        /// 
        /// Recibe un nombre, un dni, un monto, un id_categoria y una fecha.
        /// Devuelve un Recibo.
        ///  
        pub fn new(nombre: String, dni:u32, monto:u128, id_categoria: u32, fecha:Timestamp) -> Recibo {
            Recibo { 
                nombre,
                dni,
                monto,
                categoria: Categoria::match_categoria(id_categoria),
                fecha,
            }
        }
        // Necesario para obtener la recaudacion en el Gestor - L
        pub fn get_monto(&self) -> u128 {
            return self.monto;
        }
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
        monto_pagado: Option<u128>, // Cambiar a monto a pagar
    }
    impl Pago {
        /// 
        /// Recibe una fecha de vencimiento y un id de categoría.
        /// Devuelve un nuevo Pago.
        /// 
        pub fn new(vencimiento:Timestamp, id_categoria: u32) -> Pago {
            Pago {
                vencimiento,
                categoria: Categoria::new(id_categoria),
                pendiente: true,
                a_tiempo: false,
                aplico_descuento: false,
                fecha_pago: None,
                monto_pagado: None // Cambiar a monto a pagar
            }
        }
        
        /// 
        /// Recibe un monto, un vector de precios de las categorias y un descuento.
        /// Devuelve true si el pago es válido en base a los parametros.
        /// 
        pub fn verificar_pago(&self, monto: u128, precio_categorias: Vec<u128>, descuento: Option<u128>) -> bool {
            let precio_categorias: Vec<u128> = if let Some(descuento) = descuento {
                let mut precio_categorias = precio_categorias;
                for p in &mut precio_categorias {
                    *p =  *p - *p * (descuento / 100)
                };
                precio_categorias
            } else {
                precio_categorias
            };
            self.categoria.mensual(precio_categorias) == monto
        }
    
        /// 
        /// Recibe un descuento, un monto, una fecha y un vector de precios de categorias.
        /// Sí el pago es válido y no está pendiente, lo establece como completado y llena los campos correspondientes.
        /// 
        pub fn realizar_pago(&mut self, descuento: Option<u128>, monto: u128, fecha: Timestamp, precio_categorias: Vec<u128>) {
            if !self.pendiente {
                panic!("El pago no está pendiente.");
            } else if !self.verificar_pago(monto, precio_categorias, descuento) {
                panic!("Monto incorrecto.");
            } else {
                self.monto_pagado = Some(monto);
                self.fecha_pago = Some(fecha);
                self.pendiente = false;
                if descuento.is_some() {
                    self.aplico_descuento = true;
                };
                self.a_tiempo = self.vencimiento > fecha;
            }
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
        ///
        /// Recibe por parametro un id_categoria y devuelve el tipo Categoria que le corresponde
        ///
        pub fn match_categoria(id_categoria: u32) -> Self {
            match id_categoria {
                1 => Self::A,
                2 => Self::B,
                3 => Self::C,
                _ => panic!("ID de categoría inválido, por favor revise el socio."),
            }
        }
        ///
        /// Consulta y devuelve el deporte que le corresponde categoria
        ///
        /// Recibe por parametro un Option<u32> del id_deporte
        ///
        pub fn get_deporte(&self, id_deporte: Option<u32>) -> Option<Vec<Deporte>> {
            match self {
                Self::A => Some(Deporte::get_deportes()),
                Self::B => {
                    if let Some(id) = id_deporte {
                        Some(vec![Deporte::match_deporte(id)])
                    }else{
                        None //o panic! ? -L
                    }
                },
                Self::C => None,
            }
        }
        ///  
        /// Consulta y devuelve el precio de la categoria
        ///
        /// Recibe por parametro la lista de precios, el indice se corresponde con el precio correspondiente a la categoria
        ///
        pub fn mensual(&self, precio_categorias: Vec<u128>) -> u128 {
            match self {
                Categoria::A => precio_categorias[0],
                Categoria::B => precio_categorias[1],
                Categoria::C => precio_categorias[2],
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
        precio_categorias: Vec<u128>,
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
                precio_categorias:vec![precio_categoria_a, precio_categoria_b ,precio_categoria_c],
                pagos_consecutivos_bono,
                cuentas_habilitadas: Vec::new(),
                esta_bloqueado: false,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            // 864_000_000 es 10 días 
            Self::new(15, 864_000_000, 5000, 3000, 2000, 3)
        }

        /// Setea un nuevo precio de matricula mensual para cierta categoria.
        ///
        /// Si el id_categoria pasado por parametro es invalido, no genera ningun cambio y ocurre un Panic!
        ///
        #[ink(message)]
        pub fn set_precio_categoria(&mut self, p_categoria: u128, id_categoria: u32) {
        	if id_categoria > 0 && id_categoria < 4 {
                    let i = id_categoria-1;
            		self.precio_categorias[i as usize] = p_categoria;
        	}else{
            		panic!("SE INGRESÓ MAL LA CATEGORIA!!"); //panics! -L
        	}
        }

        ///
	    /// Setea una nueva duracion de deadline
        ///
        /// Si se modifica este atributo, las fechas de vencimiento a futuro tambien se correran
        ///
        #[ink(message)]
        pub fn set_duracion_deadline(&mut self, d_deadline: Timestamp) {
            self.duracion_deadline = d_deadline;
        }
        
        #[ink(message)]
        pub fn get_duracion_deadline(&self) -> Timestamp {
            self.duracion_deadline
        }

        ///
	    /// Setea un porcentaje de descuento para los usuarios a los que aplica la bonificacion
        ///
        /// Si se ingresa un porcentaje mayor a 100 o menor que 1, panics
        ///
        #[ink(message)]
        pub fn set_descuento(&mut self, descuento: u128) {
        	if descuento > 0 && descuento < 101  {
            		self.descuento = descuento;
        	}else{
            		panic!("EL PORCENTAJE DE DESCUENTO INGRESADO ESTÁ MAL!"); // panics!
        	}
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
            /*
                - Busca el socio
                - Toma el último pago
                - Si no está pago:
                    - Llama a realizar pago
            */
            let hoy = self.env().block_timestamp();
            if self.socios.len() > 0{
                let i:i32 = 0;
                while (i as usize) < self.socios.len() && self.socios[i as usize].dni != dni{
                     i = i + 1;
                }
                if self.socios[i].dni != dni{
                    panic!("EL DNI INGRESADO NO ES VALIDO");
                } else{
                    if self.socios[i as usize].pagos[self.socios[i as usize].pagos.len() - 1].pendiente = true {
                        if self.socios[i as usize].cumple_bonificacion(){
                            self.socios[i as usize].pagos[self.socios[i as usize].pagos.len() - 1].realizar_pago(Some(self.descuento), monto, hoy, self.precio_categorias);
                        }else{
                            self.socios[i as usize].pagos[self.socios[i as usize].pagos.len() - 1].realizar_pago(None, monto, hoy, self.precio_categorias);
                        }
                        
                    }else{
                        panic!("EL PAGO YA FUE REGISTRADO");
                    }
                }
            }
            
           
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




    

    pub fn get_current_time() -> Timestamp {
        let since_the_epoch = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch.as_secs()
            + since_the_epoch.subsec_nanos() as u64 / 1_000_000_000
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
            assert!(
                match resultado {
                    Ok(_) => panic!("Panic no es el correcto."),
                    Err(e) => assert_eq!(e, "Id del deporte inválido, revise el ID del socio."),
                }
            );
            let resultado = std::panic::catch_unwind(|| Deporte::match_deporte(9));
            assert!(
                match resultado {
                    Ok(_) => panic!("Panic no es el correcto."),
                    Err(e) => assert_eq!(e, "Id del deporte inválido, revise el ID del socio."),
                }
            );
        }
    }

    #[cfg(test)]
    mod club_sem_rust_tests {
        use crate::club_sem_rust::*;

        #[test]
        fn new_test(){
            let esperado = ClubSemRust{
                socios: Vec::new(),
                descuento: 25,
                precio_categorias: vec![400, 300, 200],
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
                precio_categorias: vec![5000, 3000, 2000],
                duracion_deadline: 864_000_000,
                pagos_consecutivos_bono: 3,
                cuentas_habilitadas: Vec::new(),
                esta_bloqueado: false
            };
            let resultado = ClubSemRust::default();

            assert_eq!(esperado, resultado, "Error en ClubSemRust::default(), se esperaba {:?} y se recibió {:?}", esperado, resultado)
        }

        #[test]
        fn get_duracion_deadline_test() {
            let esperado = 864_000_000;
            let club = ClubSemRust::default();
            let resultado = club.get_duracion_deadline();

            assert_eq!(esperado, resultado, "Error en ClubSemRust::get_duracion_deadline(), se esperaba {:?} y se recibió {:?}", esperado, resultado);

            let esperado = 999; 
            let club = ClubSemRust::new(25, 999, 400, 300, 200, 10);
            let resultado = club.get_duracion_deadline();
            
            assert_eq!(esperado, resultado, "Error en ClubSemRust::get_duracion_deadline(), se esperaba {:?} y se recibió {:?}", esperado, resultado);
        }

        #[test]
        fn set_duracion_deadline_test() {
            let esperado = ClubSemRust{
                socios: Vec::new(),
                descuento: 15,
                precio_categorias: vec![5000, 3000, 2000],
                duracion_deadline: 999,
                pagos_consecutivos_bono: 3,
                cuentas_habilitadas: Vec::new(),
                esta_bloqueado: false
            };
            let mut resultado = ClubSemRust::default();
            resultado.set_duracion_deadline(999);
            
            assert_eq!(esperado, resultado, "Error en ClubSemRust::set_duracion_deadline(), se esperaba {:?} y se recibió {:?}", esperado, resultado);
        }

        #[test]
        fn get_descuento_test() {
            let esperado = 15;
            let club = ClubSemRust::default();
            let resultado = club.get_descuento();

            assert_eq!(esperado, resultado, "Error en ClubSemRust::get_descuento(), se esperaba {:?} y se recibió {:?}", esperado, resultado);

            let esperado = 25; 
            let club = ClubSemRust::new(25, 999, 400, 300, 200, 10);
            let resultado = club.get_descuento();
            
            assert_eq!(esperado, resultado, "Error en ClubSemRust::get_descuento(), se esperaba {:?} y se recibió {:?}", esperado, resultado);
        }

        #[test]
        fn set_descuento_test() {
            let esperado = ClubSemRust{
                socios: Vec::new(),
                descuento: 25,
                precio_categorias: vec![5000, 3000, 2000],
                duracion_deadline: 999,
                pagos_consecutivos_bono: 3,
                cuentas_habilitadas: Vec::new(),
                esta_bloqueado: false
            };
            let mut resultado = ClubSemRust::default();
            resultado.set_descuento(25);
            
            assert_eq!(esperado, resultado, "Error en ClubSemRust::set_duracion_deadline(), se esperaba {:?} y se recibió {:?}", esperado, resultado);
        }

        #[ink::test]
        fn registrar_nuevo_socio_test() {
            let now = super::get_current_time();
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(now); 
            let esperado = ClubSemRust{
                socios: Vec::from([Socio{
                    id_deporte: None,
                    id_categoria: 3,
                    dni: 44044044,
                    nombre: "Juancito".to_string(),
                    pagos: vec![Pago::new(now + 864_000_000, 2000)],
                }]),
                descuento: 15,
                precio_categorias: vec![5000, 3000, 2000],
                duracion_deadline: 864_000_000,
                pagos_consecutivos_bono: 3,
                cuentas_habilitadas: Vec::new(),
                esta_bloqueado: false
            };
            let mut resultado = ClubSemRust::default();
            resultado.registrar_nuevo_socio("Juancito".to_string(), 44044044, 3, None);
            
            assert_eq!(esperado, resultado, "Error en ClubSemRust::registrar_nuevo_socio(), se esperaba {:?} y se recibió {:?}", esperado, resultado);

            
            let esperado = ClubSemRust{
                socios: Vec::from([Socio{
                    id_deporte: None,
                    id_categoria: 3,
                    dni: 44044044,
                    nombre: "Juancito".to_string(),
                    pagos: vec![Pago::new(now + 864_000_000, 2000)],
                }, Socio{
                    id_deporte: Some(5),
                    id_categoria: 3,
                    dni: 45045045,
                    nombre: "Roberto".to_string(),
                    pagos: vec![Pago::new(now + 864_000_000, 2000)],
                }]),
                descuento: 15,
                precio_categorias: vec![5000, 3000, 2000],
                duracion_deadline: 864_000_000,
                pagos_consecutivos_bono: 3,
                cuentas_habilitadas: Vec::new(),
                esta_bloqueado: false
            };
            let mut resultado = ClubSemRust::default();
            resultado.registrar_nuevo_socio("Juancito".to_string(), 44044044, 3, None);
            resultado.registrar_nuevo_socio("Roberto".to_string(), 45045045, 2, Some(5));
            
            assert_eq!(esperado, resultado, "Error en ClubSemRust::registrar_nuevo_socio(), se esperaba {:?} y se recibió {:?}", esperado, resultado);
        }
        
        #[ink::test]
        fn registrar_pago_test() {
            let now = super::get_current_time();
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(now); 
            let esperado = ClubSemRust{
                socios: Vec::from([Socio{
                    id_deporte: None,
                    id_categoria: 3,
                    dni: 44044044,
                    nombre: "Juancito".to_string(),
                    pagos: Vec::from([
                        Pago::new(now + 864_000_000, 2000)
                    ]),
                }]),
                descuento: 15,
                precio_categorias: vec![5000, 3000, 2000],
                duracion_deadline: 864_000_000,
                pagos_consecutivos_bono: 3,
                cuentas_habilitadas: Vec::new(),
                esta_bloqueado: false
            };
            let mut resultado = ClubSemRust::default();
            resultado.registrar_nuevo_socio("Juancito".to_string(), 44044044, 3, None);
            resultado.registrar_pago(44044044, 2000);
            assert_eq!(esperado, resultado, "Error en ClubSemRust::registrar_pago(), se esperaba {:?} y se recibió {:?}", esperado, resultado);
        }

        #[test]
        fn get_socios_test() {
            let now = super::get_current_time();
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(now); 
            let esperado = Vec::from([Socio{
                id_deporte: None,
                id_categoria: 3,
                dni: 44044044,
                nombre: "Juancito".to_string(),
                pagos: Vec::from([
                    Pago::new(now, 2000)
                ]),
            }, Socio{
                id_deporte: Some(5),
                id_categoria: 3,
                dni: 45045045,
                nombre: "Roberto".to_string(),
                pagos: Vec::new(),
            }]);
            let club = ClubSemRust{
                socios: Vec::from([Socio{
                    id_deporte: None,
                    id_categoria: 3,
                    dni: 44044044,
                    nombre: "Juancito".to_string(),
                    pagos: Vec::from([
                        Pago::new(now, 2000)
                    ]),
                }]),
                descuento: 15,
                precio_categorias: vec![5000, 3000, 2000],
                duracion_deadline: 864_000_000,
                pagos_consecutivos_bono: 3,
                cuentas_habilitadas: Vec::new(),
                esta_bloqueado: false
            };
            let resultado = club.get_socios();
            assert_eq!(esperado, resultado, "Error en ClubSemRust::get_socios(), se esperaba {:?} y se recibió {:?}", esperado, resultado);
        }

        #[test]
        fn get_recibos_test() {
            let now = super::get_current_time();

            let esperado = Vec::from([
                Recibo {
                    nombre: "Juancito".to_string(),
                    dni: 44044044,
                    monto: 5000,
                    categoria: Categoria::A,
                    fecha: now,
                },
                Recibo {
                    nombre: "Juancito".to_string(),
                    dni: 44044044,
                    monto: 2000,
                    categoria: Categoria::C,
                    fecha: now + 1_000_000,
                },
            ]);
            let club = ClubSemRust{
                socios: Vec::from([Socio{
                    id_deporte: None,
                    id_categoria: 3,
                    dni: 44044044,
                    nombre: "Juancito".to_string(),
                    pagos: Vec::from([
                        Pago{
                            vencimiento: now + 1_000_000,
                            categoria: Categoria::A,
                            pendiente: false,
                            a_tiempo: true,
                            aplico_descuento: true,
                            fecha_pago: Some(now),
                            monto_pagado: Some(5000),
                        },
                        Pago{
                            vencimiento: now + 5_000_000,
                            categoria: Categoria::C,
                            pendiente: false,
                            a_tiempo: true,
                            aplico_descuento: true,
                            fecha_pago: Some(now + 1_000_000),
                            monto_pagado: Some(2000),
                        }
                    ])
                }, Socio{
                    id_deporte: Some(5),
                    id_categoria: 3,
                    dni: 45045045,
                    nombre: "Roberto".to_string(),
                    pagos: Vec::new(),
                }]),
                descuento: 15,
                precio_categorias: vec![5000, 3000, 2000],
                duracion_deadline: 864_000_000,
                pagos_consecutivos_bono: 3,
                cuentas_habilitadas: Vec::new(),
                esta_bloqueado: false
            };
            let resultado = club.get_recibos(44044044);
            assert!(resultado.is_some());
            assert_eq!(esperado, resultado.unwrap(), "Error en ClubSemRust::get_recibos(), se esperaba {:?} y se recibió {:?}", esperado, resultado);

            
            let resultado = club.get_recibos(45045045);
            assert!(resultado.is_none(), "Error en ClubSemRust::get_recibos(), se esperaba None y se recibió {:?}", resultado);
        }

        #[test]
        fn agregar_cuenta_test() {
            let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
            let esperado = ClubSemRust{
                socios: Vec::new(),
                descuento: 15,
                precio_categorias: vec![5000, 3000, 2000],
                duracion_deadline: 864_000_000,
                pagos_consecutivos_bono: 3,
                cuentas_habilitadas: Vec::from([
                    accounts.alice,
                    accounts.bob,
                    ]),
                esta_bloqueado: false
            };
            let mut resultado = ClubSemRust::default();
            assert_ne!(resultado, esperado);
            resultado.agregar_cuenta(accounts.alice);
            assert_ne!(resultado, esperado);
            resultado.agregar_cuenta(accounts.bob);
            assert_eq!(resultado, esperado, "Error en ClubSemRust::agregar_cuenta(), se esperaba {:?} y se recibió {:?}", esperado, resultado);
        }
        

        #[test]
        fn flip_bloqueo_test(){
            let esperado = ClubSemRust{
                socios: Vec::new(),
                descuento: 15,
                precio_categorias: vec![5000, 3000, 2000],
                duracion_deadline: 864_000_000,
                pagos_consecutivos_bono: 3,
                cuentas_habilitadas: Vec::new(),
                esta_bloqueado: true
            };
            let mut resultado = ClubSemRust::default();
            assert_ne!(resultado, esperado);
            resultado.flip_bloqueo();
            assert_eq!(resultado, esperado, "Error en ClubSemRust::flip_bloqueo(), se esperaba {:?} y se recibió {:?}", esperado, resultado);
            
        }

        #[test]
        fn esta_habilitada(){
            let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
            let club = ClubSemRust{
                socios: Vec::new(),
                descuento: 15,
                precio_categorias: vec![5000, 3000, 2000],
                duracion_deadline: 864_000_000,
                pagos_consecutivos_bono: 3,
                cuentas_habilitadas: Vec::from([
                    accounts.alice,
                    accounts.bob,
                    ]),
                esta_bloqueado: false
            };
            assert!(!club.esta_habilitada(accounts.charlie));
            assert!(club.esta_habilitada(accounts.alice));
            assert!(club.esta_habilitada(accounts.bob));
        }
    }

    #[cfg(test)]
    mod categoria_tests {
        use crate::club_sem_rust::Categoria;

        //CATEGORIA TEST
        #[test]
        fn match_categoria_test(){
            let categA = Categoria::new(1);
            let categB = Categoria::new(2);
            let categC = Categoria::new(3);

            assert_eq!(categA.match_categoria(1),Categoria::A);
            assert_eq!(categB.match_categoria(2),Categoria::B);
            assert_eq!(categC.match_categoria(3),Categoria::C);

            assert_ne!(categA.match_categoria(1),Categoria::B);
            assert_ne!(categB.match_categoria(2),Categoria::C);
            assert_ne!(categC.match_categoria(3),Categoria::A);

        }

        #[test]
        fn get_deporte_test(){
            let categA = Categoria::new(1);
            let categB = Categoria::new(2);
            let categC = Categoria::new(3);
            
            assert_eq!(categA.get_deporte(None),Some(Vec<Deporte>));
            for i in 1..9{
                assert_eq!(categB.get_deporte(i),Some(Vec[i]));
            }
            
            assert_eq!(categC.get_deporte(None),None);
            assert_ne!(categC.get_deporte(3), Some(Vec[i]));
        }
        #[test]
        fn mensual_test(){
            let categA = Categoria::new(1);
            let categB = Categoria::new(2);
            let categC:Categoria = Categoria::new(3);
            let mut valores = Vec::new();
            valores.push(5000);
            valores.push(3000);
            valores.push(2000);

            assert_eq!(categA.mensual(valores),5000);
            assert_eq!(categB.mensual(valores),3000);
            assert_eq!(categC.mensual(valores),2000);

            assert_ne!(categA.mensual(valores),2000);
            assert_ne!(categB.mensual(valores),5000);
            assert_ne!(categC.mensual(valores),3000);
        }
        #[test]
        #[should_panic]
        fn test_panic() {
            let categD = Categoria::new(4);

            let categA = Categoria::new(1);
            let vacio = Vec::new();
            categA.mensual(vacio);

        }
    }

    mod recibo_tests {
        use crate::club_sem_rust::Recibo;
        use crate::club_sem_rust::Socio;

        //RECIBO TEST --> es necesario? el test de Socios ya deberia encargarse de que funcione la creacion de recibos correctamente
        #[test]
        fn test_new(){
            let nombre:String = "Carlos".to_string();
            let dni:u32 = 44444444;
            let monto:u128 = 1234567;
            let id_categoria:u32 = 1;
            let fecha:Timestamp = 1_000_000_000;

            let r:Recibo= Recibo { nombre: "Carlos".to_string(),
            dni: 44444444,
            monto: 1234567,
            categoria: 1,
            fecha: 1_000_000_000 };

            assert_eq!(Recibo::new(nombre, dni, monto, id_categoria, fecha),r);
        }
        #[test]
        #[should_panic(expected = "id_categoria fuera de rango.")]
        fn test_new_panic() {
            let nombre:String = "Carlos".to_string();
            let dni:u32 = 44444444;
            let monto:u128 = 1234567;
            let id_categoria_invalida:u32 = 100;
            let fecha:Timestamp = 1_000_000_000;
            
            Recibo::new(nombre, dni, monto, id_categoria_invalida, fecha);
        }

    }

    mod pago_tests {
        use crate::club_sem_rust::Pago;
        use crate::club_sem_rust::Socio;

        #[test]
        #[should_panic(expected = "id_categoria fuera de rango.")]
        fn test_new_panic(){
            let vencimiento: Timestamp = 1_000_000_000;
            let id_categoria_invalida:u32 = 100;
            Pago::new(vencimiento, id_categoria_invalida);
        }

        #[test]
        fn test_verificar_pago(){
            let pago:Pago = Pago::new(1_000_000_000, 3);
            let precio_categorias:Vec<u128> = vec![3000, 2000, 1000];

            assert_eq!(pago.verificar_pago(900, precio_categorias, Some(10)), true);

            let pago_2:Pago = Pago::new(1_000_000_000, 1);
            let precio_categorias_2:Vec<u128> = vec![3000, 2000, 1000];
            
            assert_ne!(pago.verificar_pago(0, precio_categorias_2, None), true);
        }
        #[test]
        #[should_panic(expected = "El pago no está pendiente")]
        fn test_realizar_pago_panic_pendiente(){
            let current_time: Timestamp = 1_000_000;
            let mut pago:Pago = Pago::new(1_000_000_000, 3);
            let precio_categorias:Vec<u128> = vec![3000, 2000, 1000];
            pago.realizar_pago(None, 1000, current_time, precio_categorias);

            pago.realizar_pago(None, 1000, current_time+1_000_000, precio_categorias);
            
        }
        #[test]
        #[should_panic(expected = "Monto incorrecto.")]
        fn test_realizar_pago_panic_monto(){
            let current_time: Timestamp = 1_000_000;
            let mut pago:Pago = Pago::new(1_000_000_000, 3);
            let precio_categorias:Vec<u128> = vec![3000, 2000, 1000];

            pago.realizar_pago(None, 5000, current_time, precio_categorias);
            
        }
        #[ink::test]
        fn test_es_moroso(){
            let pago:Pago = Pago::new(1_000_000_000, 3);
            let current_time:Timestamp = 2_000_000_000;

            assert_eq!(pago.es_moroso(current_time), true);

            /* metodo que verifique si el pago esta pendiente después de la fecha de vencimiento
             nos va a servir a la hora de verificar los morosos - L */

        }


    }















}
