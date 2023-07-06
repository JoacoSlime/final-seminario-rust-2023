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
    //use ink_e2e::env_logger::fmt::Timestamp;

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
        /// Construye un nuevo Socio con sus variables de a cuerdo a lo que le enviemos por parametro.
        /// Empieza con un Pago pendiente
        /// 
        /// # Panic
        /// 
        /// Puede devolver panic sino se corresponde el id_deporte con la categoria.
        pub fn new(nombre: String, dni:u32, id_categoria: u32, id_deporte: Option<u32>, vencimiento:Timestamp, precio_categorias: Vec<u128>) -> Socio {
            if id_categoria == 2 && id_deporte == None{
                panic!("Categoria B debe elegir un deporte");
            }else{
                if id_categoria == 1 || id_categoria == 3 && id_deporte != None{
                    panic!("Categoria A y Categoria C no deben elegir un deporte  -- Este campo debe permanecer vacio");
                }else{
                    if id_categoria == 2 && id_deporte == Some(2){
                        panic!("Categoria B debe elegir un deporte distinto a Gimnasio(id=2). Intente con id_deporte 1, 3, 4, 5, 6, 7, u 8");
                    }else{
                        let pago_inicial:Vec<Pago> = vec![Pago::new(vencimiento, id_categoria, None, precio_categorias)];
                        Socio {
                            id_deporte,
                            id_categoria,
                            dni,
                            nombre,
                            pagos: pago_inicial,
                        }
                    }
                }
            }
        }

	    /// Verifica si un determinado usuario esta habilitado o no para realizar un determinado deporte
        ///
        /// Recibe el id_deporte que se quiere verificar
        ///
        /// # Ejemplo
        /// ```
        /// let precio_categorias = vec![5000,4000,2000];
        /// let socio = Socio::new("Alice", 44044044, 2, 1, 0, precio_categorias);
        /// let habilitado = socio.puede_hacer_deporte(1):
        /// assert!(habilitado); 
        /// ```
        pub fn puede_hacer_deporte(&self, id_deporte: u32) -> bool {
            match self.id_categoria {
            	1 => return true,
            	2 => match id_deporte{
                        2 => return true,
                        _=> if let Some(id_dep) = self.id_deporte {
                                return id_dep == id_deporte;
                            }else{
                                return false;
                            },
                    },
        		3 => match id_deporte{
                        2 => return true,
                        _=> return false,
                    },
                _ => panic!("ID de categoría inválido, por favor revise el socio."),
    	    }
        }

        /// Recorre todos los Pagos completados de un Socio y crea un listado de recibos con los datos relevantes de cada Pago
        /// 
        /// # Panic
        /// 
        /// Devuelve panic si se arrastró algún error durante el procesamiento de algún Pago
        pub fn generar_recibos(&self) -> Vec<Recibo> {
            let mut recibos = Vec::new();
            if self.pagos.len() != 0 {
                for i in 0..self.pagos.len(){
                    if self.pagos[i].pendiente == false{
                        match self.pagos[i].fecha_pago{
                            Some(fe) => {
                                let recibo = Recibo::new(self.nombre.clone(), self.dni, self.pagos[i].monto, self.id_categoria, fe );
                                recibos.push(recibo);    
                            },
                            None => panic!("Este Socio registra un Pago sin fecha")
                        }
                    }
                }
            }else{
                panic!("Este Socio no tiene ningun pago registrado habido ni por haber")
            }
            return recibos
        }

        /// Consulta el ultimo pago y devuelve true si está vencido y sin pagar
        /// Si devuelve true el socio se considera moroso
        /// 
        /// # Panic
        /// 
        /// El método puede dar panic en caso de que el socio no tenga pagos registrados.
        pub fn es_moroso(&self, current_time:Timestamp) -> bool {
            if let Some(ultimo_pago) = self.pagos.last(){
                return ultimo_pago.es_moroso(current_time);
            }else{
                panic!("Este socio no tiene ningún pago registrado");
            }
        }

        /// Socio realiza un Pago, luego se crea un nuevo Pago pendiente con una nueva fecha de vencimiento
        /// 
        /// Socio siempre deberá tener un único Pago pendiente en el último índice de su lista de Pagos
        /// La creación de un nuevo Pago pendiente se da automáticamente una vez pagado el anterior
        /// 
        /// # Panics
        /// 
        /// Este método puede dar panic en caso de que el socio no tenga pagos registrados.
        pub fn realizar_pago(&mut self, descuento: Option<u128>, monto: u128, fecha: Timestamp, precio_categorias: Vec<u128>, deadline:Timestamp){
            if let Some(i) = self.pagos.iter().position(|p| p.pendiente){
                self.pagos[i].realizar_pago(descuento, monto, fecha);
                self.pagos.push(Pago::new(fecha+deadline, self.id_categoria, descuento, precio_categorias));
            }else{
                panic!("Este socio no tiene pagos habidos ni por haber");
            }
        }

	    /// Consulta los pagos mas recientes del Socio y devuelve true si cumple los requisitos para la bonificacion
        ///
        /// Recibe por parametro la cantidad de pagos consecutivos que deben figurar como pagados "a tiempo" para aplicar la bonificacion
        /// cumple_bonificacion funciona como un shor-circuit. Al encontrar un pago que no cumple devuelve false y termina su ejecución
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

	    /// Permite al usuario cambiar su propia categoria
        ///
        /// Si el id_categoria y/o id_deporte ingresados son invalidos, no guarda ningun cambio y se genera un panic
        /// 
        /// Si se cambia a Categoria A o C debe setear id_deporte = None
        /// Si se cambia a Categoria B debe setear id_Deporte = Some(...)
        /// Si se cambia a Categoria B id_Deporte != Some(2)
        ///
        /// # Panics
        /// 
        /// Puede llegar a dar panic en caso de que:
        /// - Se pasa un id_deporte 2 al cambiar a categoría B.
        /// - No se pasa un id_deporte al cambiar a categoría B.
        /// - Se elije un id_deporte al cambiar a categoría A o B.
        pub fn cambiar_categoria(&mut self, id_categoria: u32, id_deporte: Option<u32>) {
            if id_categoria == 2 && id_deporte == Some(2){
                panic!("Categoria B debe elegir un deporte distinto a Gimnasio(id=2). Intente con id_deporte 1, 3, 4, 5, 6, 7, u 8");
            }else{
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
        }

	    /// Devuelve todos los deportes que realiza un determinado Socio
        ///
        /// Si es de Categoria C, devuelve None
        ///
        /// # Panics
        /// 
        /// Puede llegar a dar panic en caso de que el id_categoria sea mayor que 3 o menor que 1.
        pub fn get_mi_deporte(&self) -> Option<Vec<Deporte>>{
            match self.id_categoria {
                3 => return None,
                2 => Categoria::match_categoria(self.id_categoria).get_deporte(self.id_deporte),
                1 => return Categoria::match_categoria(self.id_categoria).get_deporte(None),
                _ => panic!("ID de categoría inválido, por favor revise el socio."),
            }
        }

        /// Determina la categoria de un Socio
        /// 
        /// Si el ID ingresado por parametro coincide con la categoria del Socio devuleve true
        /// Caso contrario devuelve false
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
        /// Construye un nuevo Recibo.
        /// 
        /// Puede llegar a dar panic por Categoria::match_categoria(id_categoria).
        /// 
        /// # Ejemplo
        /// 
        /// ```
        /// let nombre = String::from("Alice"); 
        /// let recibo = Recibo::new(nombre, u32::default(), u128::default(), 1, u64::default());
        /// ```
        pub fn new(nombre: String, dni:u32, monto:u128, id_categoria: u32, fecha:Timestamp) -> Recibo {
            Recibo { 
                nombre,
                dni,
                monto,
                categoria: Categoria::match_categoria(id_categoria),
                fecha,
            }
        }
        
        /// Devuleve el monto de un Recibo
        /// 
        /// # Ejemplo
        /// 
        /// ```
        /// let nombre = String::from("Alice");
        /// let recibo = Recibo::new(nombre, u32::default(), 5000, 1, u64::default());
        /// assert_eq!(recibo.get_monto(), 5000)
        /// ```
        pub fn get_monto(&self) -> u128 {
            return self.monto;
        }
        
        /// Chequea si un Recibo fue realizado durante cierto período de tiempo.
        /// 
        /// Si la fecha en la que se realizó el pago está dentro de ese intervalo, se devuelve true
        /// Si la fecha está por fuera de ese intervalo, se devuelve false
        /// 
        /// # Ejemplo
        /// ```
        /// let fecha_min = 1000;
        /// let fecha_max = 2000;
        /// let socio = Recibo::new("Alice", 44044044, 5000, 1, 1500);
        /// let entre = socio.fecha_entre(fecha_min, fecha_max);
        /// ```
        pub fn fecha_entre(&self, fecha_min:Timestamp, fecha_max:Timestamp) -> bool {
            return self.fecha >= fecha_min && self.fecha <= fecha_max;
        }
        // Metodo que necesitaba para Recaudacion en el contrato Gestor -L
    }

    #[derive(scale::Decode, scale::Encode, Debug, Clone, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Pago {
        vencimiento: Timestamp, // if(current_time >= vencimiento) then vencido
        categoria: Categoria,   // vencimiento = now + deadline_duration
        monto: u128,
        pendiente: bool,
        a_tiempo: bool,
        aplico_descuento: bool,
        fecha_pago: Option<Timestamp>,
    }
    impl Pago {
        /// Construye un nuevo Pago.
        /// 
        /// # Panics
        /// 
        /// Puede llegar a dar panic:
        /// - Si la id_categoria es inválida.
        /// - Si el numero de la multiplicación es demasiado grande al aplicar el descuento.    
        /// 
        /// # Ejemplo
        /// ```
        /// let pago = Pago::new(u64::default(), 1);
        /// ```
        pub fn new(vencimiento:Timestamp, id_categoria: u32,
             descuento: Option<u128>, precio_categorias: Vec<u128>) -> Pago {
            let categoria = Categoria::new(id_categoria);
            let precio_categorias = if let Some(descuento) = descuento {
                let mut nuevos_precios = Vec::with_capacity(3);
                for i in 0..nuevos_precios.len() {
                    let multiplicado = (precio_categorias[i]).checked_mul(descuento);
                    if let Some(multiplicado) = multiplicado {
                        nuevos_precios[i] = multiplicado.checked_div(100).expect("La división causó un overflow.");
                    } else {
                        panic!("La multiplicación causó un overflow.")
                    }
                };
                nuevos_precios
            } else {
                precio_categorias
            };
            Pago {
                vencimiento,
                categoria: categoria.clone(),
                monto: categoria.mensual(precio_categorias),
                pendiente: true,
                a_tiempo: false,
                aplico_descuento: descuento.is_some(),
                fecha_pago: None,
            }
        }


        /// Retorna true en caso de que el pago sea moroso.
        /// 
        /// Un pago se considera "moroso" en caso de que esté vencido e impago.
        /// 
        /// # Ejemplo
        /// ```
        /// let pago = Pago::new(u64::default(), 1);
        /// assert!(pago.es_moroso(u64::default() + 1));
        /// ```
        pub fn es_moroso(&self, now: Timestamp) -> bool {
            self.pendiente && self.vencimiento < now
        }
        
        /// Cambia el estado de un pago a pagado si es válido.
        /// 
        /// Verifica que el monto a pagar sea el correcto y que el pago esté pendiente, luego camabia el estado del pago a pagado. 
        /// 
        /// # Panics
        /// 
        /// Puede llegar a dar panic si el pago no está pendiente, o el monto pagado es diferente al monto a pagar.
        /// 
        /// # Ejemplo
        /// ```
        /// let pago = Pago::new(u64::default()+1, 1);
        /// let precio_categorias = Vec::from([5000,3000,2000]);
        /// pago.realizar_pago(None, 5000, u64::default(), precio_categorias);
        /// ```
        pub fn realizar_pago(&mut self, descuento: Option<u128>, monto: u128, fecha: Timestamp) {
            if !self.pendiente {
                panic!("El pago no está pendiente.");
            } else if !self.monto == monto {
                panic!("Monto incorrecto.");
            } else {
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
        /// Construye una Categoría a partir del ID ingresado por parámetro.
        /// 
        /// # Panics
        /// 
        /// Puede devolver panic si el ID ingresado está por fuera del rango establecido
        /// 
        /// Categoría A --> ID = 1
        /// Categoría B --> ID = 2
        /// Categoría C --> ID = 3
        pub fn new(id_categoria:u32) -> Categoria {
            match id_categoria {
                1 => Self::A,
                2 => Self::B,
                3 => Self::C,
                _ => panic!("ID de categoría inválido, por favor revise el socio."),
            }
        }
        
        /// Recibe por parametro un id_categoria y devuelve el tipo Categoria que le corresponde
        /// 
        /// # Panic
        /// 
        /// Puede devolver panic si el ID ingresado está por fuera del rango establecido
        ///
        /// Categoría A --> ID = 1
        /// Categoría B --> ID = 2
        /// Categoría C --> ID = 3
        pub fn match_categoria(id_categoria: u32) -> Self {
            match id_categoria {
                1 => Self::A,
                2 => Self::B,
                3 => Self::C,
                _ => panic!("ID de categoría inválido, por favor revise el socio."),
            }
        }

        /// Consulta y devuelve el deporte que le corresponde categoria
        /// 
        /// Todas las categorías pueden acceder al Gimnasio por defecto.
        /// 
        /// Categoría A --> Devuelve una lista con todos los deportes practicables en el Club SemRust
        /// Categoría B --> Devuelve el deporte elegido por el Socio
        /// Categoría C --> No practica deportes por fuera del Gimnasio
        ///
        /// Recibe por parametro un Option<u32> del id_deporte
        /// 
        /// # Panic
        /// 
        /// Puede devolver panic si se envia por parámetro un id_deporte = None siendo la categoría actual Categoría B
        pub fn get_deporte(&self, id_deporte: Option<u32>) -> Option<Vec<Deporte>> {
            match self {
                Self::A => Some(Deporte::get_deportes()),
                Self::B => {
                    if let Some(id) = id_deporte {
                        Some(vec![Deporte::match_deporte(id)])
                    }else{
                        panic!("No se encontró un ID de deporte")
                    }
                },
                Self::C => None,
            }
        }

        /// Consulta y devuelve el precio de la categoría de acuerdo a la lista de precios asignada por el contrato
        ///
        /// Recibe por parametro la lista de precios, el indice se corresponde con el precio correspondiente a la categoría
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
        /// Devuelve el vector de todos los deportes existentes.
        ///
        /// # Ejemplo
        /// ```
        /// let deportes = Deporte::get_deportes();
        /// assert_eq!(deportes.len()-1, Deporte::Paddle);
        /// ```
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
    
        /// Devuelve el deporte correspondiente a un id_deporte.
        /// 
        /// # Panics
        /// 
        /// Puede dar panic en caso de que id_deporte sea mayor a 8 o menor a 1.
        /// 
        /// # Ejemplo
        /// ```
        /// let deporte = Deporte::match_deporte(1);
        /// assert_eq!(deporte, Deporte::Futbol);
        /// ```
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
        owner:Option<AccountId>,
    }

    impl ClubSemRust {
        #[ink(constructor)]
        pub fn new(descuento: u128, duracion_deadline: Timestamp, precio_categoria_a: u128, precio_categoria_b: u128, precio_categoria_c: u128, pagos_consecutivos_bono: u32) -> Self {
            let mut club = Self {
                socios: Vec::new(),
                descuento,
                duracion_deadline,
                precio_categorias:vec![precio_categoria_a, precio_categoria_b ,precio_categoria_c],
                pagos_consecutivos_bono,
                cuentas_habilitadas: Vec::new(),
                esta_bloqueado: false,
                owner:None,
            };
            club
        }
	    
         #[ink(message)]
        pub fn transfer_account(&mut self, owner:Option<AccountId>){
            self.owner = owner;
        }
	    
        #[ink(constructor)]
        pub fn default() -> Self {
            // 864_000_000 es 10 días 
            Self::new(15, 864_000_000, 5000, 3000, 2000, 3)
        }

        /// Setea un nuevo precio de matricula mensual para cierta categoria.
        ///
        /// # Panics
        /// 
        /// Puede ocurrir un panic en caso de que:
        /// - La categoría sea inválida.
        /// - El bloqueo esté activado y:
        ///     - El caller no esté en el vector de cuentas habilitadas.
        ///     - Ni sea owner el caller.
        #[ink(message)]
        pub fn set_precio_categoria(&mut self, p_categoria: u128, id_categoria: u32) {
            if self.esta_habilitada(self.env().caller()){
                    if id_categoria > 0 && id_categoria < 4 {
                        let i = id_categoria-1;
                        self.precio_categorias[i as usize] = p_categoria;
                }else{
                        panic!("SE INGRESÓ MAL LA CATEGORIA!!");
                }
            }else{
                panic!("No está habilitado para realizar esta operación.")
            }
        }

	    /// Setea una nueva duracion de deadline
        ///
        /// Si se modifica este atributo, las fechas de vencimiento a futuro tambien se correran
        ///
        /// # Panics
        /// 
        /// Puede ocurrir un panic en caso de que:
        /// - El bloqueo esté activado y:
        ///     - El caller no esté en el vector de cuentas habilitadas.
        ///     - Ni sea owner el caller.
        #[ink(message)]
        pub fn set_duracion_deadline(&mut self, d_deadline: Timestamp) {
            if self.esta_habilitada(self.env().caller()){
                self.duracion_deadline = d_deadline;
            }else{
                panic!("No está habilitado para realizar esta operación.")
            }
        }
        
        #[ink(message)]
        pub fn get_duracion_deadline(&self) -> Timestamp {
            self.duracion_deadline
        }

	    /// Setea un porcentaje de descuento para los usuarios a los que aplica la bonificacion
        ///
        /// # Panics
        /// 
        /// Puede ocurrir un panic en caso de que:
        /// - Se ingresa un porcentaje inválido.
        /// - El bloqueo esté activado y:
        ///     - El caller no esté en el vector de cuentas habilitadas.
        ///     - Ni sea owner el caller.
        #[ink(message)]
        pub fn set_descuento(&mut self, descuento: u128) {
            if self.esta_habilitada(self.env().caller()){
                if descuento > 0 && descuento < 101  {
            		self.descuento = descuento;
                }else{
                    panic!("EL PORCENTAJE DE DESCUENTO INGRESADO ESTÁ MAL!"); // panics!
                }
            }else{
                panic!("No está habilitado para realizar esta operación.")
            }
        }

        /// Establece el descuento aplicado a los pagos de socios con bono aplicable.
        #[ink(message)]
        pub fn get_descuento(&self) -> u128 {
            self.descuento
        }
        
        /// Crea un nuevo socio y lo agrega al vector de socios.
        /// 
        /// # Panics
        /// 
        /// Puede ocurrir un panic en caso de que:
        /// - El bloqueo esté activado y:
        ///     - El caller no esté en el vector de cuentas habilitadas.
        ///     - Ni sea owner el caller.
        #[ink(message)]
        pub fn registrar_nuevo_socio(&mut self, nombre: String, dni:u32, id_categoria: u32, id_deporte: Option<u32>) {
            if self.esta_habilitada(self.env().caller()){
                let hoy = self.env().block_timestamp() + self.duracion_deadline;
                let precios = self.precio_categorias.clone();
                let socio = Socio::new(nombre, dni, id_categoria, id_deporte, hoy, precios);
                self.socios.push(socio);
            }else{
                panic!("No está habilitado para realizar esta operación.")
            }
        }
        
        #[ink(message)]

        /// Busca al socio y realiza el pago de su último pago.
        /// 
        /// # Panics
        /// 
        /// Puede ocurrir un panic en caso de que:
        /// - El DNI Ingresado sea invalido.
        /// - El pago ya estuviera registrado.
        /// - El bloqueo esté activado y:
        ///     - El caller no esté en el vector de cuentas habilitadas.
        ///     - Ni sea owner el caller.
        pub fn registrar_pago(&mut self, dni: u32, monto: u128) {
            if self.esta_habilitada(self.env().caller()){
                let hoy = self.env().block_timestamp();
                let precios = self.precio_categorias.clone();
                let deadline: Timestamp = hoy+self.get_duracion_deadline();
                if self.socios.len() > 0{
                    if let Some(i) = self.socios.iter().position(|s| s.dni == dni){
                        if self.socios[i].pagos[self.socios[i].pagos.len() - 1].pendiente == true {
                            if self.socios[i].cumple_bonificacion(self.pagos_consecutivos_bono){
                                self.socios[i].realizar_pago(Some(self.descuento), monto, hoy, precios, deadline);
                            }else{
                                self.socios[i].realizar_pago(None, monto, hoy, precios, deadline);
                            }
                        }else{
                            panic!("No existe un Pago pendiente!");
                        }
                    }else{
                        panic!("El DNI ingresado no es válido!");
                    }
                }else{
                    panic!("No hay ningún socio registrado!");
                }
            }else{
                panic!("No está habilitado para realizar esta operación.")
            }
        }
        
        /// Devuelve el vector de Socios.
        #[ink(message)]
        pub fn get_socios(&self) -> Vec<Socio> {
            self.socios.clone()
        }
        
        /// Devuelve un Vector de todos los recibos generados.
        #[ink(message)]
        pub fn get_recibos(&self, dni: u32) -> Option<Vec<Recibo>> {
            if let Some(socio) = self.socios.iter().find(|s| s.dni == dni){
                Some(socio.generar_recibos())
            } else {
                None
            }
        }
        
        /// Agrega una cuenta al vector de cuentas habilitadas.
        /// 
        /// # Panics
        /// 
        /// Puede ocurrir un panic en caso de que:
        /// - La categoría sea inválida.
        /// - El bloqueo esté activado y:
        ///     - El caller no esté en el vector de cuentas habilitadas.
        ///     - Ni sea owner el caller.
        #[ink(message)]
        pub fn agregar_cuenta(&mut self, id: AccountId) {
            match self.owner{
                Some(a) =>{ if self.env().caller() == a {
                    self.cuentas_habilitadas.push(id);
                }},
                None => panic!("NO HAY OWNER!"),
            }
        }
            /* 
               if self.owners.iter().any(|owner_id| *owner_id == self.env().caller() ) {
                    self.cuentas_habilitadas.push(id);
                } else {
                    panic!("Solo un Owner está habilitado para realizar esta operación.")
                }

                esto es si consideramos a los owners como gente con mayores privilegios que 
                los simplemente "habilitados"
                                                -L
             */
        

        #[ink(message)]
        pub fn flip_bloqueo(&mut self) {
            // if caller is owner
            self.esta_bloqueado = !self.esta_bloqueado
        }
        
        /// Retorna true si una cuenta está habilitada.
        ///
        /// Itera sobre el vector de AccountId de la estructura y devuelve true si encuentra 
        /// una cuenta que concuerde con el id pasado por parámetro
        fn esta_habilitada(&self, id: AccountId) -> bool {

            /* 
                
                if self.esta_bloqueado == false { return true }
                else { self.cuentas_habilitadas.iter().any(|account_id| *account_id == id) }

                Porque si esta no está bloqueado, cualquier cuenta tiene acceso a todo,
                Si está bloqueado, se usa el protocolo de callers con permisos

             */

            self.cuentas_habilitadas.iter().any(|account_id| *account_id == id)
        }
    }




    
    /// Magia negra.
    /// 
    /// Se supone que devuelve el Timestamp del tiempo actual.
    /// 
    /// SOLO PARA USO EN TESTS.
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
            let precio_categorias = vec![5000, 3000, 2000];
            let esperado = ClubSemRust{
                socios: Vec::from([Socio{
                    id_deporte: None,
                    id_categoria: 3,
                    dni: 44044044,
                    nombre: "Juancito".to_string(),
                    pagos: vec![Pago::new(now + 864_000_000, 2000, None, precio_categorias)],
                }]),
                descuento: 15,
                precio_categorias,
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
                    pagos: vec![Pago::new(now + 864_000_000, 2000, None, precio_categorias)],
                }, Socio{
                    id_deporte: Some(5),
                    id_categoria: 3,
                    dni: 45045045,
                    nombre: "Roberto".to_string(),
                    pagos: vec![Pago::new(now + 864_000_000, 2000, None, precio_categorias)],
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

        #[ink::test]
        #[should_panic(expected = "No hay ningún socio registrado!")]
        fn registrar_pago_test_panic_socio() {
            let mut club:ClubSemRust = ClubSemRust{
                socios: Vec::new(),
                descuento: 15,
                precio_categorias: vec![5000, 3000, 2000],
                duracion_deadline: 864_000_000,
                pagos_consecutivos_bono: 3,
                cuentas_habilitadas: Vec::new(),
                esta_bloqueado: false
            };
            
            club.registrar_pago(44044044, 2000);

        }


        #[ink::test]
        #[should_panic(expected = "El DNI ingresado no es válido!")]
        fn registrar_pago_test_panic_dni() {
            let now = super::get_current_time();
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(now); 
            let mut club = ClubSemRust{
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
            club.registrar_pago(44444444, 2000);

        }


        #[ink::test]
        #[should_panic(expected = "No existe un Pago pendiente!")]
        fn registrar_pago_test_panic_pago() {
            let now = super::get_current_time();
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(now); 
            let mut club = ClubSemRust{
                socios: Vec::from([Socio{
                    id_deporte: None,
                    id_categoria: 3,
                    dni: 44044044,
                    nombre: "Juancito".to_string(),
                    pagos: Vec::new(),
                }]),
                descuento: 15,
                precio_categorias: vec![5000, 3000, 2000],
                duracion_deadline: 864_000_000,
                pagos_consecutivos_bono: 3,
                cuentas_habilitadas: Vec::new(),
                esta_bloqueado: false
            };
            club.registrar_pago(44044044, 2000);

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
                            aplico_descuento: false,
                            fecha_pago: Some(now),
                            monto: 5000,
                        },
                        Pago{
                            vencimiento: now + 5_000_000,
                            categoria: Categoria::C,
                            pendiente: false,
                            a_tiempo: true,
                            aplico_descuento: false,
                            fecha_pago: Some(now + 1_000_000),
                            monto: 2000,
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

        #[ink::test]
        fn esta_habilitada_test(){
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

        #[test]
        #[ink::test]
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
        #[ink::test]
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

        #[test]
        #[ink::test]
        #[should_panic(expected = "id_categoria fuera de rango.")]
        fn test_new_panic(){
            let vencimiento: Timestamp = 1_000_000_000;
            let id_categoria_invalida:u32 = 100;
            Pago::new(vencimiento, id_categoria_invalida);
        }
        
        #[test]
        #[ink::test]
        fn test_verificar_pago(){
            let pago:Pago = Pago::new(1_000_000_000, 3);
            let precio_categorias:Vec<u128> = vec![3000, 2000, 1000];

            assert_eq!(pago.verificar_pago(900, precio_categorias, Some(10)), true);

            let pago_2:Pago = Pago::new(1_000_000_000, 1);
            let precio_categorias_2:Vec<u128> = vec![3000, 2000, 1000];
            
            assert_ne!(pago.verificar_pago(0, precio_categorias_2, None), true);
        }
        
        #[test]
        #[ink::test]
        #[should_panic(expected = "El pago no está pendiente")]
        fn test_realizar_pago_panic_pendiente(){
            let current_time: Timestamp = 1_000_000;
            let mut pago:Pago = Pago::new(1_000_000_000, 3);
            let precio_categorias:Vec<u128> = vec![3000, 2000, 1000];
            pago.realizar_pago(None, 1000, current_time, precio_categorias);

            pago.realizar_pago(None, 1000, current_time+1_000_000, precio_categorias);
            
        }
        
        #[test]
        #[ink::test]
        #[should_panic(expected = "Monto incorrecto.")]
        fn test_realizar_pago_panic_monto(){
            let current_time: Timestamp = 1_000_000;
            let mut pago:Pago = Pago::new(1_000_000_000, 3);
            let precio_categorias:Vec<u128> = vec![3000, 2000, 1000];

            pago.realizar_pago(None, 5000, current_time, precio_categorias);
            
        }
        
        #[test]
        #[ink::test]
        fn test_es_moroso(){
            let pago:Pago = Pago::new(1_000_000_000, 3);
            let current_time:Timestamp = 2_000_000_000;

            assert_eq!(pago.es_moroso(current_time), true);

        }
    }


}
