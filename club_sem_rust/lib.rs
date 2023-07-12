//! Trabajo final de Seminario de Lenguajes: Rust (2023)
//! 
//! Este modulo es un trabajo conceptual del uso de Ink! para la creación de smart-contracts.
//! No se recomienda su utilización en un escenario de producción.
//! 
//! [`Github`]: https://github.com/JoacoSlime/final-seminario-rust-2023
//! 

#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use self::club_sem_rust::*;
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
        account: AccountId,
        nombre: String,
        pagos: Vec<Pago>,
    }
    impl Socio{
        /// Construye un nuevo Socio con sus variables de a cuerdo a lo que le enviemos por parametro.
        /// Empieza con un Pago pendiente.
        /// 
        /// # Panic
        /// 
        /// - Puede devolver panic sino se corresponde el id_deporte con la categoria:
        ///     - Categoría B, con deporte None.
        ///     - Categoria A y C, con deporte Some.
        ///     - Categoria B con deporte igual a 2 o fuera de rango.
        pub fn new(nombre: String, dni:u32, account: AccountId, id_categoria: u32, id_deporte: Option<u32>, vencimiento:Timestamp, precio_categorias: Vec<u128>) -> Socio {
            if id_categoria == 2 && id_deporte == None{
                panic!("Categoria B debe elegir un deporte");
            }else{
                if (id_categoria == 1 || id_categoria == 3) && id_deporte != None{ // Agregar a los tests - J
                    panic!("Categoria A y Categoria C no deben elegir un deporte  -- Este campo debe permanecer vacio");
                }else{
                    if (id_categoria == 2) && ((id_deporte == Some(2)) || (id_deporte < Some(1)) || (id_deporte > Some(8))) { 
                        panic!("Categoria B debe elegir un deporte distinto a Gimnasio(id=2) y dentro del rango 1 a 8");
                    }else{
                        let pago_inicial:Vec<Pago> = Vec::from([Pago::new(vencimiento, id_categoria, None, precio_categorias)]);
                        Socio {
                            id_deporte,
                            id_categoria,
                            dni,
                            nombre,
                            account,
                            pagos: pago_inicial,
                        }
                    }
                }
            }
        }

	    /// Verifica si un determinado usuario esta habilitado o no para realizar un determinado deporte.
        ///
        /// Recibe el id_deporte que se quiera verificar.
        /// 
        /// # Panic
        /// 
        /// - Si el id_categoría del Socio está fuera de rango.
        ///
        /// # Ejemplo
        /// 
        /// ```
        /// let account = [0; 32];
        /// use crate::club_sem_rust::Socio;
        /// 
        /// let precio_categorias = vec![5000,4000,2000];
        /// let socio = Socio::new("Alice".to_string(), 44044044, account.into(), 2, Some(1), 0, precio_categorias);
        /// let habilitado = socio.puede_hacer_deporte(1);
        /// assert!(habilitado); 
        /// ```
        pub fn puede_hacer_deporte(&self, id_deporte: u32) -> bool {
            if id_deporte > 8 || id_deporte < 1 {
                panic!("ID de deporte inválido.")
            }else{
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
        }

        /// Recorre todos los Pagos completados de un Socio y crea un vector de Recibos con los datos relevantes de cada Pago
        /// Si el Socio no tiene ningún Pago realizado, devuelve un vector vacío.
        /// 
        /// # Panic
        /// 
        /// - Si el pago no presenta una fecha.
        /// - Si el socio no tiene ningún Pago registrado.
        pub fn generar_recibos(&self) -> Vec<Recibo> {
            let mut recibos = Vec::new();
            if self.pagos.is_empty() {
                panic!("Este socio no tiene ningún Pago registrado");
            }else{
                for i in 0..self.pagos.len(){
                    if self.pagos[i].pendiente == false{
                        match self.pagos[i].fecha_pago{
                            Some(fe) => {
                                let recibo = Recibo::new(self.nombre.clone(), self.dni, self.pagos[i].monto, self.pagos[i].categoria.clone(), fe );
                                recibos.push(recibo);    
                            },
                            None => panic!("Este Socio registra un Pago sin fecha")
                        }
                    }
                }
            }
            return recibos
        }

        /// Consulta el ultimo pago y devuelve true si está vencido y sin pagar.
        /// Si devuelve true el socio se considera moroso.
        /// 
        /// # Panic
        /// 
        /// - Si el socio no tiene pagos registrados.
        pub fn es_moroso(&self, current_time:Timestamp) -> bool {
            if let Some(ultimo_pago) = self.pagos.last(){
                return ultimo_pago.es_moroso(current_time);
            }else{
                panic!("Este socio no tiene ningún pago registrado");
            }
        }

        /// Socio realiza un Pago, inmediatamente después se crea un nuevo Pago pendiente con una nueva fecha de vencimiento
        /// 
        /// Todo Socio deberá tener al menos un Pago pendiente en el último índice de su lista de Pagos.
        /// La creación de un nuevo Pago pendiente se da automáticamente una vez pagado el anterior.
        /// 
        /// # Panics
        /// 
        /// - Si el socio no tiene pagos registrados.
        pub fn realizar_pago(&mut self, descuento: Option<u128>, pagos_consecutivos: u32, monto: u128, fecha: Timestamp, precio_categorias: Vec<u128>, deadline:Timestamp){
            if let Some(i) = self.pagos.iter().position(|p| p.pendiente){
                self.pagos[i].realizar_pago(monto, fecha);
                if self.cumple_bonificacion(pagos_consecutivos){
                    self.pagos.push(Pago::new(self.pagos[i].vencimiento.checked_add(deadline).expect("Error al sumar el tiempo"), self.id_categoria, descuento, precio_categorias));
                } else {
                    self.pagos.push(Pago::new(self.pagos[i].vencimiento.checked_add(deadline).expect("Error al sumar el tiempo"), self.id_categoria, None, precio_categorias));
                }
            }else{
                panic!("Este socio no tiene ningún Pago registrado");
            }
        }

	    /// Consulta los pagos mas recientes del Socio y devuelve true si cumple los requisitos para la bonificación.
        ///
        /// Recibe por parametro la cantidad de pagos consecutivos que deben figurar como pagados "a tiempo" para aplicar la bonificacion.
        /// cumple_bonificacion funciona como un short-circuit. Al encontrar un pago que no cumple, o al ser los pagos_consecutivos 0,
        /// devuelve false y termina su ejecución.
        pub fn cumple_bonificacion(&self, pagos_consecutivos: u32) -> bool {
            if self.pagos.len() < pagos_consecutivos as usize || pagos_consecutivos == 0 {
                return false
            }else{
                let m = self.pagos.len().checked_sub(pagos_consecutivos as usize).expect("Error al restar los pagos_consecutivos.");
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
        /// Si el id_categoria y/o id_deporte ingresados son invalidos, no guarda ningun cambio y se devuelve un panic
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
            if (id_categoria == 2) && (id_deporte == Some(2)) {
                panic!("Categoria B debe elegir un deporte distinto a Gimnasio(id=2). Intente con id_deporte 1, 3, 4, 5, 6, 7, u 8");
            } else {
                if (id_categoria == 2) && (id_deporte == None) {
                    panic!("Si se desea cambiar a Categoria B, se debe elegir un deporte");
                } else {
                    if (id_categoria == 3 || id_categoria == 1) && (id_deporte != None) {
                        panic!("Si se desea cambiar a Categoria A o C, no se debe elegir un deporte");
                    } else {
                        self.id_categoria = id_categoria;
                        self.id_deporte = id_deporte;
                    }
                }
            }
        }

	    /// Devuelve todos los deportes que realiza un determinado Socio.
        ///
        /// Si es de Categoria C, devuelve None.
        ///
        /// # Panics
        /// 
        /// Devolverá panic en caso de que el id_categoria sea mayor que 3 o menor que 1.
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
        /// use crate::club_sem_rust::Recibo;
        /// use crate::club_sem_rust::Categoria;
        /// 
        /// let nombre = String::from("Alice"); 
        /// let recibo = Recibo::new(nombre, u32::default(), u128::default(), Categoria::A, u64::default());
        /// ```
        pub fn new(nombre: String, dni:u32, monto:u128, categoria: Categoria, fecha:Timestamp) -> Recibo {
            Recibo { 
                nombre,
                dni,
                monto,
                categoria,
                fecha,
            }
        }
        
        /// Devuleve el monto de un Recibo
        /// 
        /// # Ejemplo
        /// 
        /// ```
        /// use crate::club_sem_rust::Recibo;
        /// use crate::club_sem_rust::Categoria;
        /// 
        /// let nombre = String::from("Alice");
        /// let recibo = Recibo::new(nombre, u32::default(), 5000, Categoria::A, u64::default());
        /// assert_eq!(recibo.get_monto(), 5000)
        /// ```
        pub fn get_monto(&self) -> u128 {
            return self.monto;
        }
        
        /// Chequea si un Recibo fue realizado durante cierto período de tiempo.
        /// 
        /// Si la fecha en la que se realizó el Pago está dentro de ese intervalo, se devuelve true.
        /// Si la fecha está por fuera de ese intervalo, se devuelve false.
        /// 
        /// # Ejemplo
        /// ```
        /// use crate::club_sem_rust::Recibo;
        /// use crate::club_sem_rust::Categoria;
        /// 
        /// let fecha_min = 1000;
        /// let fecha_max = 2000;
        /// let socio = Recibo::new("Alice".to_string(), 44044044, 5000, Categoria::A, 1500);
        /// let entre = socio.fecha_entre(fecha_min, fecha_max);
        /// ```
        pub fn fecha_entre(&self, fecha_min:Timestamp, fecha_max:Timestamp) -> bool {
            return self.fecha >= fecha_min && self.fecha <= fecha_max;
        }
    }

    #[derive(scale::Decode, scale::Encode, Debug, Clone, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Pago {
        vencimiento: Timestamp,
        categoria: Categoria,
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
        /// - Si el precio de la categoria o el descuento son demasiado grande al aplicar el descuento.
        /// 
        /// # Ejemplo
        /// ```
        /// use crate::club_sem_rust::Pago;
        /// 
        /// let precios = Vec::from([5000,4000,2000]);
        /// let pago = Pago::new(u64::default(), 1, None, precios);
        /// ```
        pub fn new(vencimiento:Timestamp, id_categoria: u32,
             descuento: Option<u128>, precio_categorias: Vec<u128>) -> Pago {
            let categoria = Categoria::new(id_categoria);
            let precio_categorias = if let Some(descuento) = descuento {
                let mut nuevos_precios = Vec::with_capacity(3);
                nuevos_precios.resize(precio_categorias.len(), 0);
                for i in 0..nuevos_precios.len() {
                    let multiplicado = (precio_categorias[i]).checked_mul(100_u128.checked_sub(descuento).expect("La resta causó un overflow"));
                    if let Some(multiplicado) = multiplicado {
                        nuevos_precios[i] = multiplicado.checked_div(100).unwrap(); // Nunca puede dar None.
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
        /// 
        /// ```
        /// use crate::club_sem_rust::Pago;
        /// 
        /// let precios = Vec::from([5000,4000,2000]);
        /// let pago = Pago::new(u64::default(), 1, None, precios);
        /// assert!(pago.es_moroso(u64::default() + 1));
        /// ```
        pub fn es_moroso(&self, now: Timestamp) -> bool {
            self.pendiente && (self.vencimiento < now)
        }
        
        /// Cambia el estado de un pago a pagado si es válido.
        /// 
        /// Verifica que el monto a pagar sea el correcto y que el pago esté pendiente, luego camabia el estado del pago a pagado. 
        /// 
        /// # Panics
        /// 
        /// - Si el pago no está pendiente.
        /// - Si el monto pagado es diferente al monto a pagar.
        /// 
        /// # Ejemplo
        /// 
        /// ```
        /// use crate::club_sem_rust::Pago;
        /// 
        /// let precios = Vec::from([5000,4000,2000]);
        /// let mut pago = Pago::new(u64::default(), 1, None, precios);
        /// pago.realizar_pago(5000, u64::default());
        /// ```
        pub fn realizar_pago(&mut self, monto: u128, fecha: Timestamp) {
            if !self.pendiente {
                panic!("El pago no está pendiente.");
            } else if self.monto != monto {
                panic!("Monto incorrecto.");
            } else {
                self.fecha_pago = Some(fecha);
                self.pendiente = false;
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
        /// La Categoría A corresponde al id_categoria 1.
        /// La Categoría B corresponde al id_categoria 2.
        /// La Categoría C corresponde al id_categoria 3.
        /// 
        /// # Panics
        /// 
        /// - Si el ID ingresado está por fuera del rango establecido.
        pub fn new(id_categoria:u32) -> Categoria {
            match id_categoria {
                1 => Self::A,
                2 => Self::B,
                3 => Self::C,
                _ => panic!("ID de categoría inválido, por favor revise el socio."),
            }
        }
        
        /// Recibe por parámetro un id_categoria y devuelve el tipo Categoria que le corresponde.
        /// 
        /// # Panic
        /// 
        /// - Si el ID ingresado está por fuera del rango establecido.
        pub fn match_categoria(id_categoria: u32) -> Self {
            match id_categoria {
                1 => Self::A,
                2 => Self::B,
                3 => Self::C,
                _ => panic!("ID de categoría inválido, por favor revise el socio."),
            }
        }

        /// Consulta y devuelve el deporte que le corresponde a una Categoria
        /// 
        /// Todas las categorías pueden acceder al Gimnasio por defecto.
        /// 
        /// Si es Categoría A, devuelve una lista con todos los deportes practicables en el Club SemRust.
        /// Si es Categoría B, devuelve el deporte elegido por el Socio.
        /// Si es Categoría C, no practica deportes por fuera del Gimnasio.
        ///
        /// 
        /// # Panic
        /// 
        /// - Si se envia por parámetro un id_deporte = None siendo la Categoría actual Categoría B
        pub fn get_deporte(&self, id_deporte: Option<u32>) -> Option<Vec<Deporte>> {
            match self {
                Self::A => Some(Deporte::get_deportes()),
                Self::B => {
                    if let Some(id) = id_deporte {
                        Some(Vec::from([Deporte::match_deporte(id)]))
                    }else{
                        panic!("No se encontró un ID de deporte")
                    }
                },
                Self::C => None,
            }
        }

        /// Consulta y devuelve el precio de la Categoría de acuerdo a la lista de precios asignada por el contrato.
        ///
        /// Recibe por parametro la lista de precios, el indice se corresponde con el precio correspondiente a la categoría
        /// # Panic
        /// 
        /// Puede devolver panic si:
        ///  - La lista de precios enviada por parámetro está vacia.
        ///  - La lista de precios tiene una longitud mayor o menos que 3.
        pub fn mensual(&self, precio_categorias: Vec<u128>) -> u128 {
            if precio_categorias.len()==3 {
                match self {
                    Categoria::A => precio_categorias[0],
                    Categoria::B => precio_categorias[1],
                    Categoria::C => precio_categorias[2],
                }
            } else {
                panic!("El formato del vector de precios es incorrecto.")
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
        /// use crate::club_sem_rust::Deporte;
        /// 
        /// let deportes = Deporte::get_deportes();
        /// assert_eq!(deportes[deportes.len()-1], Deporte::Paddle);
        /// ```
        pub fn get_deportes() -> Vec<Deporte> {
            Vec::from([
                Self::Futbol,
                Self::Gimnasio,
                Self::Basquet,
                Self::Rugby,
                Self::Hockey,
                Self::Natacion,
                Self::Tenis,
                Self::Paddle
            ])
        }
    
        /// Devuelve el deporte correspondiente a un id_deporte.
        /// 
        /// # Panics
        /// 
        /// Puede dar panic en caso de que id_deporte sea mayor a 8 o menor a 1.
        /// 
        /// # Ejemplo
        /// ```
        /// use crate::club_sem_rust::Deporte;
        /// 
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
    /// - La cantidad de pagos consecutivos necesarios para dar un descuento. (0 = desactivado)
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
        /// Crea un nuevo club en base a los parámetros dados
        /// 
        /// # Panics:
        /// - Si el descuento es mayor a 100
        #[ink(constructor)]
        pub fn new(descuento: u128, duracion_deadline: Timestamp, precio_categoria_a: u128, precio_categoria_b: u128, precio_categoria_c: u128, pagos_consecutivos_bono: u32) -> Self {
            if descuento > 100 {
                panic!("Porcentaje de descuento inválido");
            };
            let mut club = Self {
                socios: Vec::new(),
                descuento,
                duracion_deadline,
                precio_categorias:Vec::from([
                    precio_categoria_a * 1_000_000_000_000,
                    precio_categoria_b * 1_000_000_000_000,
                    precio_categoria_c * 1_000_000_000_000
                ]),
                pagos_consecutivos_bono,
                cuentas_habilitadas: Vec::new(),
                esta_bloqueado: false,
                owner:None,
            };
            club.transfer_account(None);
            club
        }
	    
        
        /// Crea un club con valores por defecto arbitrarios.
        #[ink(constructor)]
        pub fn default() -> Self {
            // 864_000_000 es 10 días 
            Self::new(15, 864_000_000, 5, 3, 2, 3)
        }
        
        /// Transfiere la cuenta de un owner a otro pasado por parámetro
        /// 
        /// # Panics
        /// - Si el caller no es el owner
        #[ink(message)]
        pub fn transfer_account(&mut self, new_owner:Option<AccountId>){
            let caller = self.env().caller();
            if let Some(owner) = self.owner {
                if owner == caller {
                    self.owner = new_owner;
                } else {
                    panic!("NO ES EL OWNER")
                }
            } else {
                self.owner = Some(caller);
            }
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
                        let i = id_categoria.checked_sub(1).expect("Error al restar el índice");
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
        
        /// Devuelve el tiempo que tiene un usuario desde el registro de un pago para verificarlo.
        /// 
        /// La deadline está en formato Unix Timestamp
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
                if descuento > 100  {
                    panic!("EL PORCENTAJE DE DESCUENTO INGRESADO ESTÁ MAL!"); // panics!
                } else {
            		self.descuento = descuento;
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
        pub fn registrar_nuevo_socio(&mut self, nombre: String, dni:u32, account: AccountId, id_categoria: u32, id_deporte: Option<u32>) {
            if self.esta_habilitada(self.env().caller()){
                let vencimiento = self.env().block_timestamp().checked_add(self.duracion_deadline).expect("Overflow en la suma de tiempo");
                let precios = self.precio_categorias.clone();
                if (id_categoria == 2) && ((id_deporte == Some(2)) || (id_deporte < Some(1)) || (id_deporte > Some(8))) {
                    panic!("El ID de deporte ingresado es inválido");
                } else {
                    let socio = Socio::new(nombre, dni, account, id_categoria, id_deporte, vencimiento, precios);
                    self.socios.push(socio);
                }
            }else{
                panic!("No está habilitado para realizar esta operación.")
            }
        }
        
        
        /// Busca al socio y realiza el pago de su último pago.
        /// Utiliza el dni como indice y requiere de permisos.
        /// 
        /// # Panics
        /// 
        /// Puede ocurrir un panic en caso de que:
        /// - El DNI Ingresado sea invalido.
        /// - El pago ya estuviera registrado.
        /// - El bloqueo esté activado y:
        ///     - El caller no esté en el vector de cuentas habilitadas.
        ///     - Ni sea owner el caller.
        #[ink(message)]
        pub fn registrar_pago_dni(&mut self, dni: u32, monto: u128) {
            let hoy = self.env().block_timestamp();
            let precios = self.precio_categorias.clone();
            let deadline: Timestamp = self.get_duracion_deadline();
            if self.esta_habilitada(self.env().caller()) {
                if self.socios.len() > 0{
                    if let Some(socio) = self.socios.iter_mut().find(|s| s.dni == dni){
                        if socio.pagos.last().is_some() {
                            socio.realizar_pago(Some(self.descuento), self.pagos_consecutivos_bono, monto, hoy, precios, deadline);
                        }else{
                            panic!("No existen pagos para esta cuenta!");
                        }
                    }else{
                        panic!("El DNI ingresado no es válido!");
                    }
                }else{
                    panic!("No hay ningún socio registrado!");
                }
            } else {
                panic!("No está habilitado para realizar esta operación.")
            }
        }
        
        
        /// Busca al socio y realiza el pago de su último pago.
        /// Utiliza el AccountId como indice y no requiere de permisos.
        /// 
        /// # Panics
        /// 
        /// Puede ocurrir un panic en caso de que:
        /// - El AccountId sea invalido.
        /// - El pago ya estuviera registrado.
        fn registrar_pago_account(&mut self, account: AccountId, monto: u128) {
            let hoy = self.env().block_timestamp();
            let precios = self.precio_categorias.clone();
            let deadline: Timestamp = hoy.checked_add(self.get_duracion_deadline()).expect("Overflow en la suma de tiempo");
            if self.socios.len() > 0{
                if let Some(socio) = self.socios.iter_mut().find(|s| s.account == account){
                    if socio.pagos.last().is_some() {
                        socio.realizar_pago(Some(self.descuento), self.pagos_consecutivos_bono, monto, hoy, precios, deadline);
                    }else{
                        panic!("No existen pagos para esta cuenta!");
                    }
                }else{
                    panic!("El AccountId no es válido!");
                }
            }else{
                panic!("No hay ningún socio registrado!");
            }
        }

        /// Permite al usuario pagar manualmente.
        /// (PROOF OF CONCEPT)
        /// 
        /// # Panics:
        /// 
        /// - Si el usuario no está registrado.
        /// - Si el pago ya estuviera registrado.
        /// - Si hubiera un descuento que cause tokens con punto fijo. ("Monto incorrecto.")
        #[ink(message, payable)]
        pub fn pagar(&mut self) {
            let monto: Balance = self.env().transferred_value();
            let cuenta = self.env().caller();
            self.registrar_pago_account(cuenta, monto);
        }

        /// Retira el valor específicado del contrato.
        /// 
        /// # Panics
        /// 
        /// - Si no tiene suficiente dinero.
        /// - Si el caller no está habilitado.
        #[ink(message)]
        pub fn withdraw_this(&mut self, valor: Balance) {
            if self.esta_habilitada(self.env().caller()){
                if valor <= self.env().balance() {
                    if self.env().transfer(self.env().caller(), valor).is_err() {
                        panic!("El balance mínimo fue sobrepasado");
                    }
                } else {
                    panic!("No hay balance suficiente en la cuenta.")
                }
            } else {
                panic!("No está habilitado para realizar esta operación.")
            }
        }

        /// Devuelve el balance del contrato.
        #[ink(message)]
        pub fn get_balance(&self) -> Balance {
            self.env().balance()
        }

        /// Devuelve el vector de Socios.
        #[ink(message)]
        pub fn get_socios(&self) -> Vec<Socio> {
            self.socios.clone()
        }
        
        /// Devuelve un Vector de todos los recibos generados.
        /// 
        /// # Panics
        /// - Si el socio no existe.
        #[ink(message)]
        pub fn get_recibos(&self, dni: u32) -> Vec<Recibo> {
            if let Some(socio) = self.socios.iter().find(|s| s.dni == dni){
                socio.generar_recibos()
            } else {
                panic!("Socio no encontrado.");
            }
        }

        
        /// Agrega una cuenta al vector de cuentas habilitadas.
        /// 
        /// # Panics
        /// 
        /// Puede ocurrir un panic en caso de que:
        /// - Si el caller no sea owner.
        /// - Si no hay owner.
        /// - Si ya existiera la cuenta.
        #[ink(message)]
        pub fn agregar_cuenta(&mut self, id: AccountId) {
            match self.owner{
                Some(o) => {
                    if self.env().caller() == o {
                        if !self.cuentas_habilitadas.iter().any(|a| *a == id){
                            self.cuentas_habilitadas.push(id);
                        } else {
                            panic!("La cuenta ya está habilitada");
                        };
                    } else {
                        panic!("El caller no es el owner.");
                    }
                },
                None => panic!("NO HAY OWNER!"),
            }
        }
                
        /// Quita una cuenta del vector de cuentas habilitadas.
        /// 
        /// # Panics
        /// 
        /// Puede ocurrir un panic en caso de que:
        /// - Si el caller no sea owner.
        /// - Si no hay owner.
        /// - Si no existiera la cuenta.
        #[ink(message)]
        pub fn quitar_cuenta(&mut self, id: AccountId) {
            match self.owner{
                Some(o) => {
                    if self.env().caller() == o {
                        let cuenta = self.cuentas_habilitadas.iter().position(|c| *c == id);
                        match cuenta {
                            Some(i) => self.cuentas_habilitadas.remove(i),
                            None => panic!("Esta cuenta no se encuentra entre las habilitadas."),
                        };
                    } else {
                        panic!("El caller no es el owner.");
                    }
                },
                None => panic!("NO HAY OWNER!"),
            }
        }
        
        /// Bloquea la estructura para que solo pueda ser modificada por las cuentas habilitadas o el owner
        #[ink(message)]
        pub fn flip_bloqueo(&mut self) {
            if Some(self.env().caller()) == self.owner {
                self.esta_bloqueado = !self.esta_bloqueado
            } else {
                panic!("NO ES EL OWNER!");
            }
        }
        
        /// Retorna true si una cuenta está habilitada.
        ///
        /// Itera sobre el vector de AccountId de la estructura y devuelve true si encuentra 
        /// una cuenta que concuerde con el id pasado por parámetro
        fn esta_habilitada(&self, id: AccountId) -> bool {
            !self.esta_bloqueado
            || self.cuentas_habilitadas.iter().any(|account_id| *account_id == id)
            || self.owner == Some(id)
        }
    }






    #[cfg(test)]
    mod tests {
        use crate::club_sem_rust::*;

        #[cfg(test)]
        mod deporte_tests {
            use super::*;
            
            #[ink::test]
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
                
            #[ink::test]
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
                
            }

            #[ink::test]
            #[should_panic(expected = "Id del deporte inválido, revise el ID del socio.")]
            fn match_deporte_panic_test() {
                let _ = Deporte::match_deporte(0);
                let _ = Deporte::match_deporte(9);
            }
        }
        
        #[cfg(test)]
        mod club_sem_rust_tests {
            use super::*;

            #[ink::test]
            fn new_test(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                let canon = 1_000_000_000_000;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner.clone());
                let esperado = ClubSemRust{
                    socios: Vec::new(),
                    descuento: 25,
                    precio_categorias: vec![4*canon, 3*canon, 2*canon],
                    duracion_deadline: 999,
                    pagos_consecutivos_bono: 10,
                    owner: Some(owner),
                    cuentas_habilitadas: Vec::new(),
                    esta_bloqueado: false
                };
                let resultado = ClubSemRust::new(25, 999, 4, 3, 2, 10);
                
                assert_eq!(esperado, resultado, "Error en ClubSemRust::new(), se esperaba {:?} y se obtuvo {:?}", esperado, resultado)
            }

            
            #[ink::test]
            #[should_panic(expected = "Porcentaje de descuento inválido")]
            fn new_panic_test(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner.clone());
                let _ = ClubSemRust::new(101, 999, 400, 300, 200, 10);
            }

            #[ink::test]
            fn default_test() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                let canon = 1_000_000_000_000;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner.clone());
                let esperado = ClubSemRust{
                    socios: Vec::new(),
                    descuento: 15,
                    precio_categorias: vec![5*canon, 3*canon, 2*canon],
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: Some(owner),
                    cuentas_habilitadas: Vec::new(),
                    esta_bloqueado: false
                };
                let resultado = ClubSemRust::default();
                
                assert_eq!(esperado, resultado, "Error en ClubSemRust::default(), se esperaba {:?} y se recibió {:?}", esperado, resultado)
            }

             #[ink::test]
            fn set_precio_categoria_test(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner.clone());
                let esperado = ClubSemRust{
                    socios: Vec::new(),
                    descuento: 25,
                    precio_categorias: vec![50000, 10000, 20000],
                    duracion_deadline: 999,
                    pagos_consecutivos_bono: 10,
                    owner: Some(owner),
                    cuentas_habilitadas: Vec::new(),
                    esta_bloqueado: false
                };
                let mut resultado = ClubSemRust::new(25, 999, 400, 300, 200, 10);
                resultado.set_precio_categoria(10000, 2);
                resultado.set_precio_categoria(20000, 3);
                resultado.set_precio_categoria(50000, 1);
                assert_eq!(resultado, esperado);

            }

            #[ink::test]
            #[should_panic(expected = "SE INGRESÓ MAL LA CATEGORIA!!")]

            fn set_precio_categoria_test_panic_first(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner.clone());
                let mut esperado = ClubSemRust{
                    socios: Vec::new(),
                    descuento: 25,
                    precio_categorias: vec![400, 300, 200],
                    duracion_deadline: 999,
                    pagos_consecutivos_bono: 10,
                    owner: Some(owner),
                    cuentas_habilitadas: Vec::new(),
                    esta_bloqueado: false
                };
             
                esperado.set_precio_categoria(10000, 5);
               
            }
		
            #[ink::test]
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
            
            #[ink::test]
            fn set_duracion_deadline_test_panic() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner);
                let mut resultado = ClubSemRust::default();
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
                resultado.set_duracion_deadline(999);
            }
            
            #[ink::test]
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

            #[ink::test]
            fn set_descuento_test() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                let canon = 1_000_000_000_000;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner.clone());
                let esperado = ClubSemRust{
                    socios: Vec::new(),
                    descuento: 25,
                    precio_categorias: vec![5*canon, 3*canon, 2*canon],
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: Some(owner),
                    cuentas_habilitadas: Vec::new(),
                    esta_bloqueado: false
                };
                let mut resultado = ClubSemRust::default();
                resultado.set_descuento(25);
                
                assert_eq!(esperado, resultado, "Error en ClubSemRust::set_duracion_deadline(), se esperaba {:#?} y se recibió {:#?}", esperado, resultado);
            }
            
            #[ink::test]
            #[should_panic(expected = "EL PORCENTAJE DE DESCUENTO INGRESADO ESTÁ MAL!")]
            fn set_descuento_test_panic_value() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner.clone());
                let mut club = ClubSemRust::default();
                club.set_descuento(101);
            }

            #[ink::test]
            #[should_panic(expected = "No está habilitado para realizar esta operación.")]
            fn set_descuento_test_panic_permissions() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner.clone());
                let mut club = ClubSemRust::default();
                club.flip_bloqueo();
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
                club.set_descuento(25);
            }

            #[ink::test]
            fn registrar_nuevo_socio_test() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner.clone());
                let now = 5000;
                let canon = 1_000_000_000_000;
                let precio_categorias = Vec::from([5*canon,3*canon,2*canon]);
                ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(now);
                let esperado = ClubSemRust{
                    socios: Vec::from([Socio{
                        id_deporte: None,
                        id_categoria: 3,
                        dni: 44044044,
                        nombre: "Juancito".to_string(),
                        account: accounts.django,
                        pagos: Vec::from([Pago::new(now + 864_000_000, 3, None, precio_categorias.clone())]),
                    }]),
                    descuento: 15,
                    precio_categorias: precio_categorias.clone(),
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: Some(owner.clone()),
                    cuentas_habilitadas: Vec::new(),
                    esta_bloqueado: false
                };
                let mut resultado = ClubSemRust::default();
                resultado.registrar_nuevo_socio("Juancito".to_string(), 44044044, accounts.django, 3, None);
                
                assert_eq!(esperado, resultado, "Error en ClubSemRust::registrar_nuevo_socio(), se esperaba {:#?} y se recibió {:#?}", esperado, resultado);
                
                
                let esperado = ClubSemRust{
                    socios: Vec::from([Socio{
                        id_deporte: None,
                        id_categoria: 3,
                        dni: 44044044,
                        nombre: "Juancito".to_string(),
                        account: accounts.django,
                        pagos: Vec::from([Pago::new(now + 864_000_000, 3, None, precio_categorias.clone())]),
                    }, Socio{
                        id_deporte: Some(5),
                        id_categoria: 2,
                        dni: 45045045,
                        nombre: "Roberto".to_string(),
                        account: accounts.bob,
                        pagos: Vec::from([Pago::new(now + 864_000_000, 2, None, precio_categorias.clone())]),
                    }]),
                    descuento: 15,
                    precio_categorias: precio_categorias.clone(),
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: Some(owner),
                    cuentas_habilitadas: Vec::new(),
                    esta_bloqueado: false
                };
                let mut resultado = ClubSemRust::default();
                resultado.registrar_nuevo_socio("Juancito".to_string(), 44044044, accounts.django, 3, None);
                resultado.registrar_nuevo_socio("Roberto".to_string(), 45045045,accounts.bob, 2, Some(5));
                
                assert_eq!(esperado, resultado, "Error en ClubSemRust::registrar_nuevo_socio(), se esperaba {:#?} y se recibió {:#?}", esperado, resultado);
            }

            #[should_panic(expected = "No está habilitado para realizar esta operación.")]
            #[ink::test]
            fn registrar_nuevo_socio_test_panic() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.frank);
                let mut club = ClubSemRust::default();
                club.flip_bloqueo();
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.django);
                club.registrar_nuevo_socio("Juancito".to_string(), 44044044, accounts.django, 3, None);
            }
            
            #[ink::test]
            fn registrar_pago_dni_test() {
                let now = 5000;
                let canon = 1_000_000_000_000;
                ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(now); 
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner.clone());
                let precio_categorias = Vec::from([5*canon, 3*canon, 2*canon]);

                let esperado = ClubSemRust{
                    socios: Vec::from([
                        Socio{
                            id_deporte: None,
                            id_categoria: 3,
                            dni: 44044044,
                            account: accounts.django,
                            nombre: "Juancito".to_string(),
                            pagos: Vec::from([
                                Pago{
                                    vencimiento: 864_000_000 + now,
                                    categoria: Categoria::C,
                                    monto: 2*canon,
                                    pendiente: false,
                                    a_tiempo: true,
                                    aplico_descuento: false,
                                    fecha_pago: Some(now),
                                },
                                Pago{
                                    vencimiento: 864_000_000 * 2 + now,
                                    categoria: Categoria::C,
                                    monto: 2*canon,
                                    pendiente: true,
                                    a_tiempo: false,
                                    aplico_descuento: false,
                                    fecha_pago: None,
                                },
                            ]),
                        }]),
                        descuento: 15,
                        precio_categorias: precio_categorias.clone(),
                        duracion_deadline: 864_000_000,
                        pagos_consecutivos_bono: 3,
                        owner: Some(owner),
                        cuentas_habilitadas: Vec::new(),
                        esta_bloqueado: false
                    };
                    let mut resultado = ClubSemRust::default();
                    resultado.registrar_nuevo_socio("Juancito".to_string(), 44044044, accounts.django, 3, None);
                    resultado.registrar_pago_dni(44044044, 2*canon);
                    assert_eq!(esperado, resultado, "Error en ClubSemRust::registrar_pago_dni(), se esperaba {:#?} y se recibió {:#?}", esperado, resultado);
                }

            #[ink::test]
            #[should_panic(expected = "No hay ningún socio registrado!")]
            fn registrar_pago_dni_test_panic_socio() {
                let mut club:ClubSemRust = ClubSemRust{
                    socios: Vec::new(),
                    descuento: 15,
                    precio_categorias: vec![5000, 3000, 2000],
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: None,
                    cuentas_habilitadas: Vec::new(),
                    esta_bloqueado: false
                };
                
                club.registrar_pago_dni(44044044, 2000);
                
            }
            
            
            #[ink::test]
            #[should_panic(expected = "El DNI ingresado no es válido!")]
            fn registrar_pago_dni_test_panic_dni() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let now = 5000;
                ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(now); 
                let precio_categorias = Vec::from([5000, 3000, 2000]);
                let mut club = ClubSemRust{
                    socios: Vec::from([Socio{
                        id_deporte: None,
                        id_categoria: 3,
                        dni: 44044044,
                        account: accounts.django,
                        nombre: "Juancito".to_string(),
                        pagos: Vec::from([
                            Pago::new(now + 864_000_000, 3, None, precio_categorias.clone())
                            ]),
                        }]),
                        descuento: 15,
                        precio_categorias: precio_categorias.clone(),
                        duracion_deadline: 864_000_000,
                        pagos_consecutivos_bono: 3,
                        owner: None,
                        cuentas_habilitadas: Vec::new(),
                        esta_bloqueado: false
                    };
                    club.registrar_pago_dni(44444444, 2000);
                    
                }
                
                
            #[ink::test]
            #[should_panic(expected = "No existen pagos para esta cuenta!")]
            fn registrar_pago_dni_test_panic_pago() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let now = 5000;
                ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(now); 
                let precio_categorias = Vec::from([5000, 3000, 2000]);
                let mut club = ClubSemRust{
                    socios: Vec::from([Socio{
                        id_deporte: None,
                        id_categoria: 3,
                        dni: 44044044,
                        account: accounts.django,
                        nombre: "Juancito".to_string(),
                        pagos: Vec::new(),
                    }]),
                    descuento: 15,
                    precio_categorias: precio_categorias.clone(),
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: None,
                    cuentas_habilitadas: Vec::new(),
                    esta_bloqueado: false
                };
                club.registrar_pago_dni(44044044, 2000);
                
            }

            
            #[ink::test]
            fn registrar_pago_account_test() {
                let now = 5000;
                let canon = 1_000_000_000_000;
                ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(now); 
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner.clone());
                let precio_categorias = Vec::from([5*canon, 3*canon, 2*canon]);

                let esperado = ClubSemRust{
                    socios: Vec::from([
                        Socio{
                            id_deporte: None,
                            id_categoria: 3,
                            dni: 44044044,
                            account: accounts.django,
                            nombre: "Juancito".to_string(),
                            pagos: Vec::from([
                                Pago{
                                    vencimiento: 864_000_000 + now,
                                    categoria: Categoria::C,
                                    monto: 2*canon,
                                    pendiente: false,
                                    a_tiempo: true,
                                    aplico_descuento: false,
                                    fecha_pago: Some(now),
                                },
                                Pago{
                                    vencimiento: 864_000_000 * 2 + now * 2,
                                    categoria: Categoria::C,
                                    monto: 2*canon,
                                    pendiente: true,
                                    a_tiempo: false,
                                    aplico_descuento: false,
                                    fecha_pago: None,
                                },
                            ]),
                        }]),
                        descuento: 15,
                        precio_categorias: precio_categorias.clone(),
                        duracion_deadline: 864_000_000,
                        pagos_consecutivos_bono: 3,
                        owner: Some(owner),
                        cuentas_habilitadas: Vec::new(),
                        esta_bloqueado: false
                    };
                let mut resultado = ClubSemRust::default();
                resultado.registrar_nuevo_socio("Juancito".to_string(), 44044044, accounts.django, 3, None);
                resultado.registrar_pago_account(accounts.django, 2*canon);
                assert_eq!(esperado, resultado, "Error en ClubSemRust::registrar_pago_dni(), se esperaba {:#?} y se recibió {:#?}", esperado, resultado);
                }

            #[ink::test]
            #[should_panic(expected = "No hay ningún socio registrado!")]
            fn registrar_pago_account_test_panic_socio() {
                let mut club:ClubSemRust = ClubSemRust{
                    socios: Vec::new(),
                    descuento: 15,
                    precio_categorias: vec![5000, 3000, 2000],
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: None,
                    cuentas_habilitadas: Vec::new(),
                    esta_bloqueado: false
                };
                
                club.registrar_pago_dni(44044044, 2000);
                
            }
            
            
            #[ink::test]
            #[should_panic(expected = "El AccountId no es válido!")]
            fn registrar_pago_account_test_panic_dni() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let now = 5000;
                ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(now); 
                let precio_categorias = Vec::from([5000, 3000, 2000]);
                let mut club = ClubSemRust{
                    socios: Vec::from([Socio{
                        id_deporte: None,
                        id_categoria: 3,
                        dni: 44044044,
                        account: accounts.django,
                        nombre: "Juancito".to_string(),
                        pagos: Vec::from([
                            Pago::new(now + 864_000_000, 3, None, precio_categorias.clone())
                            ]),
                        }]),
                        descuento: 15,
                        precio_categorias: precio_categorias.clone(),
                        duracion_deadline: 864_000_000,
                        pagos_consecutivos_bono: 3,
                        owner: None,
                        cuentas_habilitadas: Vec::new(),
                        esta_bloqueado: false
                    };
                    club.registrar_pago_account(accounts.bob, 2000);
                    
                }
                
                
            #[ink::test]
            #[should_panic(expected = "No existen pagos para esta cuenta!")]
            fn registrar_pago_account_test_panic_pago() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let now = 5000;
                ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(now); 
                let precio_categorias = Vec::from([5000, 3000, 2000]);
                let mut club = ClubSemRust{
                    socios: Vec::from([Socio{
                        id_deporte: None,
                        id_categoria: 3,
                        dni: 44044044,
                        account: accounts.django,
                        nombre: "Juancito".to_string(),
                        pagos: Vec::new(),
                    }]),
                    descuento: 15,
                    precio_categorias: precio_categorias.clone(),
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: None,
                    cuentas_habilitadas: Vec::new(),
                    esta_bloqueado: false
                };
                club.registrar_pago_account(accounts.django, 2000);
            }

            #[ink::test]
            fn pagar_test() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let now = 5000;
                let canon = 1_000_000_000_000;
                ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(now); 
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.frank);
                let precio_categorias = Vec::from([5*canon, 3*canon, 2*canon]);
                let esperado = ClubSemRust{
                    socios: Vec::from([
                        Socio{
                            id_deporte: None,
                            id_categoria: 3,
                            dni: 44044044,
                            account: accounts.django,
                            nombre: "Juancito".to_string(),
                            pagos: Vec::from([
                                Pago{
                                    vencimiento: 864_000_000 + now,
                                    categoria: Categoria::C,
                                    monto: 2*canon,
                                    pendiente: false,
                                    a_tiempo: true,
                                    aplico_descuento: false,
                                    fecha_pago: Some(now),
                                },
                                Pago{
                                    vencimiento: 864_000_000 * 2 + now * 2,
                                    categoria: Categoria::C,
                                    monto: 2*canon,
                                    pendiente: false,
                                    a_tiempo: true,
                                    aplico_descuento: false,
                                    fecha_pago: Some(now),
                                },
                                Pago{
                                    vencimiento: 864_000_000 * 3 + now * 3,
                                    categoria: Categoria::C,
                                    monto: 2*canon,
                                    pendiente: false,
                                    a_tiempo: true,
                                    aplico_descuento: false,
                                    fecha_pago: Some(now),
                                },
                                Pago{
                                    vencimiento: 864_000_000 * 4 + now * 4,
                                    categoria: Categoria::C,
                                    monto: 17*(canon/10),
                                    pendiente: false,
                                    a_tiempo: true,
                                    aplico_descuento: true,
                                    fecha_pago: Some(now),
                                },
                                Pago{
                                    vencimiento: 864_000_000 * 5 + now * 5,
                                    categoria: Categoria::C,
                                    monto: 2*canon,
                                    pendiente: true,
                                    a_tiempo: false,
                                    aplico_descuento: false,
                                    fecha_pago: None,
                                },
                            ]),
                        }]),
                        descuento: 15,
                        precio_categorias: precio_categorias.clone(),
                        duracion_deadline: 864_000_000,
                        pagos_consecutivos_bono: 3,
                        owner: Some(accounts.frank),
                        cuentas_habilitadas: Vec::new(),
                        esta_bloqueado: false
                    };
                let mut resultado = ClubSemRust::default();
                resultado.registrar_nuevo_socio("Juancito".to_string(), 44044044, accounts.django, 3, None);
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.django);
                ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(2*canon);
                resultado.pagar();
                resultado.pagar();
                resultado.pagar();
                ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(17*(canon/10));
                resultado.pagar();
                assert_eq!(esperado, resultado, "Error en ClubSemRust::registrar_pago_dni(), se esperaba {:#?} y se recibió {:#?}", esperado, resultado);
            }

            #[ink::test]
            #[should_panic(expected = "No hay ningún socio registrado!")]
            fn pagar_test_panic_socios() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let now = 5000;
                ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(now); 
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.frank);
                let mut club = ClubSemRust::default();
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.django);
                ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(0);
                club.pagar();
            }
            
            #[ink::test]
            #[should_panic(expected = "Monto incorrecto.")]
            fn pagar_test_panic_monto() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let now = 5000;
                ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(now); 
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.frank);
                let mut club = ClubSemRust::default();
                club.registrar_nuevo_socio("Juancito".to_string(), 44044044, accounts.django, 3, None);
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.django);
                ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(0);
                club.pagar();
            }
            
            #[ink::test]
            #[should_panic(expected = "No existen pagos para esta cuenta!")]
            fn pagar_test_panic_sin_pagos() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let now = 5000;
                ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(now); 
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.frank);
                let precio_categorias = Vec::from([5000, 3000, 2000]);
                let mut club = ClubSemRust{
                    socios: Vec::from([Socio{
                        id_deporte: None,
                        id_categoria: 3,
                        dni: 44044044,
                        account: accounts.django,
                        nombre: "Juancito".to_string(),
                        pagos: Vec::new(),
                    }]),
                    descuento: 15,
                    precio_categorias: precio_categorias.clone(),
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: None,
                    cuentas_habilitadas: Vec::new(),
                    esta_bloqueado: false
                };
                club.registrar_nuevo_socio("Juancito".to_string(), 44044044, accounts.django, 3, None);
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.django);
                ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(2000);
                club.pagar();
            }

            #[ink::test]
            fn withdraw_this() {
                let contract_balance = 100;
                let contract_id = ink::env::test::callee::<ink::env::DefaultEnvironment>();
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();

                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.frank);
                ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(contract_id, contract_balance);
                ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(accounts.frank, 0);
                
                let mut club = ClubSemRust::default();
                club.withdraw_this(80);

                let balance_frank = ink::env::test::get_account_balance::<ink::env::DefaultEnvironment>(accounts.frank).expect("No se pudo obtener el balance, algo salió terriblemente mal.");
                let balance_contrato = ink::env::test::get_account_balance::<ink::env::DefaultEnvironment>(contract_id).expect("No se pudo obtener el balance, algo salió terriblemente mal.");

                assert_eq!(balance_frank, 80);
                assert_eq!(balance_contrato, 20);
            }

            #[ink::test]
            #[should_panic(expected = "No hay balance suficiente en la cuenta.")]
            fn withdraw_this_panic_surpassed_balance() {
                let contract_balance = 100;
                let contract_id = ink::env::test::callee::<ink::env::DefaultEnvironment>();
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();

                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.frank);
                ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(contract_id, contract_balance);
                ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(accounts.frank, 0);
                
                let mut club = ClubSemRust::default();
                club.withdraw_this(120);
            }

            #[ink::test]
            #[should_panic(expected = "No está habilitado para realizar esta operación.")]
            fn withdraw_this_panic_permission_error() {
                let contract_balance = 100;
                let contract_id = ink::env::test::callee::<ink::env::DefaultEnvironment>();
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();

                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.frank);
                ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(contract_id, contract_balance);
                ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(accounts.frank, 0);
                let mut club = ClubSemRust::default();
                club.flip_bloqueo();

                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
                club.withdraw_this(80);
            }

            #[ink::test]
            fn get_balance_test() {
                let contract_balance = 100;
                let contract_id = ink::env::test::callee::<ink::env::DefaultEnvironment>();

                ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(contract_id, contract_balance);
                
                let club = ClubSemRust::default();
                assert_eq!(club.get_balance(), 100);
            }
                
            #[ink::test]
            fn get_socios_test() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let now = 5000;
                ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(now); 
                let precio_categorias = Vec::from([5000, 3000, 2000]);
                let esperado = Vec::from([
                    Socio{
                        id_deporte: None,
                        id_categoria: 3,
                        dni: 44044044,
                        nombre: "Juancito".to_string(),
                        account: accounts.django,
                        pagos: Vec::from([
                            Pago::new(now, 3, None, precio_categorias.clone())
                        ]),
                    }, Socio{
                        id_deporte: Some(5),
                        id_categoria: 3,
                        dni: 45045045,
                        account: accounts.bob,
                        nombre: "Roberto".to_string(),
                        pagos: Vec::new(),
                    }]);
                let club = ClubSemRust{
                    socios: Vec::from([Socio{
                            id_deporte: None,
                            id_categoria: 3,
                            dni: 44044044,
                            account: accounts.django,
                            nombre: "Juancito".to_string(),
                            pagos: Vec::from([Pago::new(now, 3, None, precio_categorias.clone())])
                        }, Socio {
                            id_deporte: Some(5),
                            id_categoria: 3,
                            dni: 45045045,
                            account: accounts.bob,
                            nombre: "Roberto".to_string(),
                            pagos: Vec::new(),
                        }
                        ]),
                        descuento: 15,
                        precio_categorias,
                        duracion_deadline: 864_000_000,
                        pagos_consecutivos_bono: 3,
                        owner: None,
                        cuentas_habilitadas: Vec::new(),
                        esta_bloqueado: false
                    };
                let resultado = club.get_socios();
                assert_eq!(esperado, resultado, "Error en ClubSemRust::get_socios(), se esperaba {:#?} y se recibió {:#?}", esperado, resultado);
            }
            
            #[ink::test]
            fn get_recibos_test() {
                let now = 5000;
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                
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
                            account: accounts.django,
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
                                id_deporte: None,
                                id_categoria: 3,
                                dni: 45045045,
                                account: accounts.django,
                                nombre: "Roberto".to_string(),
                                pagos:  Vec::from([
                                    Pago{
                                        vencimiento: now + 1_000_000,
                                        categoria: Categoria::C,
                                        pendiente: true,
                                        a_tiempo: false,
                                        aplico_descuento: false,
                                        fecha_pago: None,
                                        monto: 2000,
                                    }
                                ]),
                            }]),
                    descuento: 15,
                    precio_categorias: vec![5000, 3000, 2000],
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: None,
                    cuentas_habilitadas: Vec::new(),
                    esta_bloqueado: false
                };
                let resultado = club.get_recibos(44044044);
                assert_eq!(esperado, resultado, "Error en ClubSemRust::get_recibos(), se esperaba {:#?} y se recibió {:#?}", esperado, resultado);
                
                let esperado: Vec<Recibo> = Vec::new();
                let resultado = club.get_recibos(45045045);
                assert_eq!(esperado, resultado, "Error en ClubSemRust::get_recibos(), se esperaba {:#?} y se recibió {:#?}", esperado, resultado);
            }


            #[ink::test]
            #[should_panic(expected = "Este socio no tiene ningún Pago registrado")]
            fn get_recibos_panic_test_pago_vacio() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let esperado: Vec<Recibo> = Vec::new();
                let club = ClubSemRust{
                    socios: Vec::from([Socio{
                            id_deporte: None,
                            id_categoria: 3,
                            dni: 45045045,
                            account: accounts.django,
                            nombre: "Roberto".to_string(),
                            pagos:  Vec::new(),
                        }]),
                    descuento: 15,
                    precio_categorias: vec![5000, 3000, 2000],
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: None,
                    cuentas_habilitadas: Vec::new(),
                    esta_bloqueado: false
                };
                let resultado = club.get_recibos(45045045);
                assert_eq!(esperado, resultado, "Error en ClubSemRust::get_recibos(), se esperaba {:#?} y se recibió {:#?}", esperado, resultado);
            }

            #[ink::test]
            #[should_panic(expected = "Socio no encontrado")]
            fn get_recibos_panic_test() {
                let club = ClubSemRust{
                    socios: Vec::new(),
                    descuento: 15,
                    precio_categorias: vec![5000, 3000, 2000],
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: None,
                    cuentas_habilitadas: Vec::new(),
                    esta_bloqueado: false
                };
                let _ = club.get_recibos(46046046);
            }
            
            #[ink::test]
            fn agregar_cuenta_test() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                let canon = 1_000_000_000_000;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner.clone());
                let esperado = ClubSemRust{
                    socios: Vec::new(),
                    descuento: 15,
                    precio_categorias: vec![5*canon, 3*canon, 2*canon],
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: Some(owner.clone()),
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
            
            #[ink::test]
            #[should_panic(expected = "NO HAY OWNER!")]
            fn agregar_cuenta_panic_no_owner_test() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner.clone());
                let mut club = ClubSemRust{
                    socios: Vec::new(),
                    descuento: 15,
                    precio_categorias: vec![5000, 3000, 2000],
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: None,
                    cuentas_habilitadas: Vec::from([
                            accounts.alice,
                            accounts.bob,
                    ]),
                    esta_bloqueado: false
                };
                club.agregar_cuenta(accounts.alice);
            }
            
            #[ink::test]
            #[should_panic(expected = "El caller no es el owner.")]
            fn agregar_cuenta_panic_permission_test() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner.clone());
                let mut club = ClubSemRust{
                    socios: Vec::new(),
                    descuento: 15,
                    precio_categorias: vec![5000, 3000, 2000],
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: Some(owner),
                    cuentas_habilitadas: Vec::from([
                            accounts.alice,
                            accounts.bob,
                    ]),
                    esta_bloqueado: false
                };
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
                club.agregar_cuenta(accounts.alice);
            }

            
            
            #[ink::test]
            #[should_panic(expected = "La cuenta ya está habilitada")]
            fn agregar_cuenta_panic_already_exists_test() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner.clone());
                let mut club = ClubSemRust{
                    socios: Vec::new(),
                    descuento: 15,
                    precio_categorias: vec![5000, 3000, 2000],
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: Some(owner),
                    cuentas_habilitadas: Vec::from([
                            accounts.alice,
                            accounts.bob,
                    ]),
                    esta_bloqueado: false
                };
                club.agregar_cuenta(accounts.alice);
            }

            
            
            #[ink::test]
            fn quitar_cuenta_test() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                let canon = 1_000_000_000_000;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner.clone());
                let esperado = ClubSemRust{
                    socios: Vec::new(),
                    descuento: 15,
                    precio_categorias: vec![5*canon, 3*canon, 2*canon],
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: Some(owner.clone()),
                    cuentas_habilitadas: Vec::from([
                            accounts.alice,
                    ]),
                    esta_bloqueado: false
                };
                let mut resultado = ClubSemRust::default();
                assert_ne!(resultado, esperado);
                resultado.agregar_cuenta(accounts.alice);
                resultado.agregar_cuenta(accounts.bob);
                assert_ne!(resultado, esperado);
                resultado.quitar_cuenta(accounts.bob);
                assert_eq!(resultado, esperado, "Error en ClubSemRust::agregar_cuenta(), se esperaba {:?} y se recibió {:?}", esperado, resultado);
            }

            
            
            #[ink::test]
            #[should_panic(expected = "NO HAY OWNER!")]
            fn quitar_cuenta_panic_no_owner_test() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner.clone());
                let mut club = ClubSemRust{
                    socios: Vec::new(),
                    descuento: 15,
                    precio_categorias: vec![5000, 3000, 2000],
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: None,
                    cuentas_habilitadas: Vec::from([
                            accounts.alice,
                            accounts.bob,
                    ]),
                    esta_bloqueado: false
                };
                club.quitar_cuenta(accounts.alice);
            }
            
            #[ink::test]
            #[should_panic(expected = "El caller no es el owner.")]
            fn quitar_cuenta_panic_permission_test() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner.clone());
                let mut club = ClubSemRust{
                    socios: Vec::new(),
                    descuento: 15,
                    precio_categorias: vec![5000, 3000, 2000],
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: Some(owner),
                    cuentas_habilitadas: Vec::from([
                            accounts.alice,
                            accounts.bob,
                    ]),
                    esta_bloqueado: false
                };
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
                club.quitar_cuenta(accounts.alice);
            }

            
            
            #[ink::test]
            #[should_panic(expected = "Esta cuenta no se encuentra entre las habilitadas.")]
            fn quitar_cuenta_panic_does_not_exists_test() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner.clone());
                let mut club = ClubSemRust{
                    socios: Vec::new(),
                    descuento: 15,
                    precio_categorias: vec![5000, 3000, 2000],
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: Some(owner),
                    cuentas_habilitadas: Vec::from([
                            accounts.alice,
                            accounts.bob,
                    ]),
                    esta_bloqueado: false
                };
                club.quitar_cuenta(accounts.frank);
            }
                
            #[ink::test]
            fn flip_bloqueo_test(){
                let canon = 1_000_000_000_000;
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner.clone());
                let esperado = ClubSemRust{
                    socios: Vec::new(),
                    descuento: 15,
                    precio_categorias: vec![5*canon, 3*canon, 2*canon],
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: Some(owner.clone()),
                    cuentas_habilitadas: Vec::new(),
                    esta_bloqueado: true
                };
                let mut resultado = ClubSemRust::default();
                assert_ne!(resultado, esperado);
                resultado.flip_bloqueo();
                assert_eq!(resultado, esperado, "Error en ClubSemRust::flip_bloqueo(), se esperaba {:?} y se recibió {:?}", esperado, resultado);
            
            }

            
            #[ink::test]
            #[should_panic(expected = "NO ES EL OWNER!")]
            fn flip_bloqueo_panic_test(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.frank);
                let mut club = ClubSemRust::default();
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
                club.flip_bloqueo();
            }

            #[ink::test]
            fn esta_habilitada_test(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner.clone());
                let mut club = ClubSemRust{
                    socios: Vec::new(),
                    descuento: 15,
                    precio_categorias: vec![5000, 3000, 2000],
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: Some(owner.clone()),
                    cuentas_habilitadas: Vec::from([
                        accounts.alice,
                        accounts.bob,
                    ]),
                    esta_bloqueado: false
                };
                assert!(club.esta_habilitada(accounts.charlie));
                club.flip_bloqueo();
                assert!(!club.esta_habilitada(accounts.charlie));
                assert!(club.esta_habilitada(accounts.alice));
                assert!(club.esta_habilitada(accounts.bob));
                assert!(club.esta_habilitada(owner));
            }

            #[ink::test]
            fn transfer_account_test(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner.clone());
                let esperado1 = ClubSemRust{
                    socios: Vec::new(),
                    descuento: 15,
                    precio_categorias: vec![5000, 3000, 2000],
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: Some(owner),
                    cuentas_habilitadas: Vec::from([
                        accounts.alice,
                        accounts.bob,
                    ]),
                    esta_bloqueado: false
                };
                let esperado2 = ClubSemRust{
                    socios: Vec::new(),
                    descuento: 15,
                    precio_categorias: vec![5000, 3000, 2000],
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: Some(accounts.alice),
                    cuentas_habilitadas: Vec::from([
                        accounts.alice,
                        accounts.bob,
                    ]),
                    esta_bloqueado: false
                };
                let mut club = ClubSemRust{
                    socios: Vec::new(),
                    descuento: 15,
                    precio_categorias: vec![5000, 3000, 2000],
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: None,
                    cuentas_habilitadas: Vec::from([
                        accounts.alice,
                        accounts.bob,
                    ]),
                    esta_bloqueado: false
                };
                club.transfer_account(None);
                assert_eq!(esperado1, club);
                club.transfer_account(Some(accounts.alice));
                assert_eq!(esperado2, club);
            }

            #[should_panic(expected = "NO ES EL OWNER")]
            #[ink::test]
            fn transfer_account_test_panic() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let owner = accounts.frank;
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(owner.clone());
                let mut club = ClubSemRust{
                    socios: Vec::new(),
                    descuento: 15,
                    precio_categorias: vec![5000, 3000, 2000],
                    duracion_deadline: 864_000_000,
                    pagos_consecutivos_bono: 3,
                    owner: Some(accounts.alice),
                    cuentas_habilitadas: Vec::from([
                        accounts.alice,
                        accounts.bob,
                    ]),
                    esta_bloqueado: false
                };
                club.transfer_account(Some(owner));
            }
        }
            
        #[cfg(test)]
        mod categoria_tests {
            use super::*;
            
            //CATEGORIA TEST
            #[ink::test]
            fn match_categoria_test(){
                
                assert_eq!(Categoria::match_categoria(1), Categoria::A);
                assert_eq!(Categoria::match_categoria(2), Categoria::B);
                assert_eq!(Categoria::match_categoria(3), Categoria::C);
                
                assert_ne!(Categoria::match_categoria(1), Categoria::B);
                assert_ne!(Categoria::match_categoria(3), Categoria::B);
                assert_ne!(Categoria::match_categoria(1), Categoria::C);
                assert_ne!(Categoria::match_categoria(2), Categoria::C);
                assert_ne!(Categoria::match_categoria(2), Categoria::A);
                assert_ne!(Categoria::match_categoria(3), Categoria::A);
                
            }

            
            #[ink::test]
            #[should_panic(expected = "ID de categoría inválido, por favor revise el socio.")]
            fn match_categoria_panic_low_test(){
                let _ = Categoria::match_categoria(0);           
            }

            
            #[ink::test]
            #[should_panic(expected = "ID de categoría inválido, por favor revise el socio.")]
            fn match_categoria_panic_high_test(){
                let _ = Categoria::match_categoria(4);                
            }
            
            #[ink::test]
            fn get_deporte_test(){
                let categ_a = Categoria::new(1);
                let categ_b = Categoria::new(2);
                let categ_c = Categoria::new(3);
                let deportes = Deporte::get_deportes();

                assert_eq!(categ_a.get_deporte(None),Some(deportes.clone()));
                for i in 1..9{
                    assert_eq!(categ_b.get_deporte(Some(i)),Some(Vec::from([deportes[(i-1) as usize].clone()])));
                }
                
                assert_eq!(categ_c.get_deporte(None),None);
                assert_ne!(categ_c.get_deporte(Some(3)), Some(Vec::from([deportes[1].clone()]))); 
            }

            #[ink::test]
            fn mensual_test(){
                let categ_a = Categoria::new(1);
                let categ_b = Categoria::new(2);
                let categ_c:Categoria = Categoria::new(3);
                let mut valores = Vec::new();
                valores.push(5000);
                valores.push(3000);
                valores.push(2000);
                
                assert_eq!(categ_a.mensual(valores.clone()),5000);
                assert_eq!(categ_b.mensual(valores.clone()),3000);
                assert_eq!(categ_c.mensual(valores.clone()),2000);
                
                assert_ne!(categ_a.mensual(valores.clone()),2000);
                assert_ne!(categ_b.mensual(valores.clone()),5000);
                assert_ne!(categ_c.mensual(valores),3000);
            }

            
            #[ink::test]
            #[should_panic(expected = "ID de categoría inválido, por favor revise el socio.")]
            fn test_new_panic() {
                let _ = Categoria::new(4);
            }

            #[ink::test]
            #[should_panic(expected = "El formato del vector de precios es incorrecto.")]
            fn test_mensual_panic() {
                let categ_a = Categoria::new(1);
                let vacio = Vec::new();
                let _ = categ_a.mensual(vacio);
            }

            
            #[ink::test]
            #[should_panic(expected = "No se encontró un ID de deporte")]
            fn test_get_deporte_panic() {
                let categ_b = Categoria::new(2);
                let _ = categ_b.get_deporte(None);
            }
        }
        
        #[cfg(test)]
        mod recibo_tests {
            use super::*;

            #[ink::test]
            fn test_new(){
                let nombre:String = "Carlos".to_string();
                let dni:u32 = 44444444;
                let monto:u128 = 1234567;
                let fecha:Timestamp = 1_000_000_000;
                
                let esperado:Recibo= Recibo { nombre: "Carlos".to_string(),
                dni: 44444444,
                monto: 1234567,
                categoria: Categoria::match_categoria(1),
                fecha: 1_000_000_000 };
                
                assert_eq!(Recibo::new(nombre, dni, monto, Categoria::A, fecha), esperado);
            }

            #[ink::test]
            fn test_get_monto(){
                let nombre:String = "Carlos".to_string();
                let dni:u32 = 44444444;
                let monto:u128 = 5000;
                let fecha:Timestamp = 1_000_000_000;

                let esperado:u128 = 5000;
                let recibo:Recibo = Recibo::new(nombre, dni, monto, Categoria::A, fecha);
                
                assert_eq!(recibo.get_monto(), esperado);
            }

            #[ink::test]
            fn test_fecha_entre(){
                let nombre:String = "Carlos".to_string();
                let dni:u32 = 44444444;
                let monto:u128 = 5000;
                let fecha:Timestamp = 1_000_000;
                let recibo:Recibo = Recibo::new(nombre, dni, monto, Categoria::A, fecha);

                let fecha_min: Timestamp = 500_000;
                let fecha_max: Timestamp = 1_500_000;

                assert_eq!(recibo.fecha_entre(fecha_min.clone(), fecha_max.clone()), true);
                assert_eq!(recibo.fecha_entre(fecha_min+1_000_000, fecha_max+1_000_000), false);
            }

        }
        
        #[cfg(test)]
        mod pago_tests {
            use super::*;

            #[ink::test]
            #[ink::test]
            #[should_panic(expected = "ID de categoría inválido, por favor revise el socio.")]
            fn test_new_id_panic(){
                let vencimiento: Timestamp = 1_000_000_000;
                let id_categoria_invalida:u32 = 100;
                Pago::new(vencimiento, id_categoria_invalida, None, Vec::from([5000,4000,2000]));
            }

            #[ink::test]
            #[should_panic(expected = "La resta causó un overflow")]
            fn test_new_overflow_sub_panic(){
                let vencimiento: Timestamp = 1_000_000_000;
                let id_categoria_invalida:u32 = 3;
                Pago::new(vencimiento, id_categoria_invalida, Some(u128::MAX), Vec::from([5000,4000,2000]));
            }

            #[ink::test]
            #[should_panic(expected = "La multiplicación causó un overflow.")]
            fn test_new_overflow_mul_panic(){
                let vencimiento: Timestamp = 1_000_000_000;
                let id_categoria_invalida:u32 = 3;
                Pago::new(vencimiento, id_categoria_invalida, Some(10), Vec::from([u128::MAX,u128::MAX,u128::MAX]));
            }

            #[ink::test]
            fn test_new_pago(){
                let pago_con_descuento:Pago = Pago::new(1_000_000_000, 3, Some(10), Vec::from([5000,4000,2000]));
                let pago_sin_descuento:Pago = Pago::new(1_000_000_000, 3, None, Vec::from([5000,4000,2000]));
                let pago_gratis:Pago = Pago::new(1_000_000_000, 3, Some(100), Vec::from([5000,4000,2000]));

                let esperado_con_descuento:Pago = Pago { vencimiento: 1_000_000_000,
                    categoria: Categoria::C,
                    monto: 1800,
                    pendiente: true,
                    a_tiempo: false,
                    aplico_descuento: true,
                    fecha_pago: None,
                };
                let esperado_sin_descuento:Pago = Pago { vencimiento: 1_000_000_000,
                    categoria: Categoria::C,
                    monto: 2000,
                    pendiente: true,
                    a_tiempo: false,
                    aplico_descuento: false,
                    fecha_pago: None,
                };
                let esperado_gratis:Pago = Pago { vencimiento: 1_000_000_000,
                    categoria: Categoria::C,
                    monto: 0,
                    pendiente: true,
                    a_tiempo: false,
                    aplico_descuento: true,
                    fecha_pago: None,
                };

                assert_eq!(pago_con_descuento, esperado_con_descuento);
                assert_eq!(pago_sin_descuento, esperado_sin_descuento);
                assert_eq!(pago_gratis, esperado_gratis);
            }
            
            #[ink::test]
            #[ink::test]
            #[should_panic(expected = "El pago no está pendiente")]
            fn test_realizar_pago_panic_pendiente(){
                let current_time: Timestamp = 1_000_000;
                let precio_categorias:Vec<u128> = Vec::from([5000,4000,2000]);
                let mut pago:Pago = Pago::new(1_000_000_000, 3, None, precio_categorias.clone());
                pago.realizar_pago(2000, current_time);
                
                pago.realizar_pago(2000, current_time+1_000_000);
                
            }
            
            #[ink::test]
            #[ink::test]
            #[should_panic(expected = "Monto incorrecto.")]
            fn test_realizar_pago_panic_monto(){
                let current_time: Timestamp = 1_000_000;
                let precio_categorias:Vec<u128> = Vec::from([5000,4000,2000]);
                let mut pago:Pago = Pago::new(1_000_000_000, 3, None, precio_categorias);
                
                pago.realizar_pago(0, current_time);
                
            }

            #[ink::test]
            fn test_realizar_pago(){
                let mut pago_con_descuento:Pago = Pago::new(1_000_000_000, 3, Some(10), Vec::from([5000,4000,2000]));
                let mut pago_sin_descuento:Pago = Pago::new(1_000_000_000, 3, None, Vec::from([5000,4000,2000]));

                let esperado_con_descuento:Pago = Pago {
                    vencimiento: 1_000_000_000,
                    categoria: Categoria::C,
                    monto: 1800,
                    pendiente: false,
                    a_tiempo: true,
                    aplico_descuento: true,
                    fecha_pago: Some(1_000_000),
                };
                let esperado_sin_descuento:Pago = Pago {
                    vencimiento: 1_000_000_000,
                    categoria: Categoria::C,
                    monto: 2000,
                    pendiente: false,
                    a_tiempo: true,
                    aplico_descuento: false,
                    fecha_pago: Some(1_000_000),
                };

                pago_con_descuento.realizar_pago(1800, 1_000_000);
                pago_sin_descuento.realizar_pago(2000, 1_000_000);

                assert_eq!(pago_con_descuento, esperado_con_descuento);
                assert_eq!(pago_sin_descuento, esperado_sin_descuento);
            }

            #[ink::test]
            #[ink::test]
            fn test_es_moroso(){
                let precio_categorias:Vec<u128> = Vec::from([5000,4000,2000]);
                let pago:Pago = Pago::new(1_000_000_000, 3, None, precio_categorias);
                let current_time:Timestamp = 2_000_000_000;
                
                assert_eq!(pago.es_moroso(current_time), true);
                
            }
        }
        #[cfg(test)]
        mod socio_tests {
            use super::*;
            
            #[ink::test]
            fn new_socio_test(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let precios_categorias = Vec::from([5000, 3000, 2000]);
                let en30dias =  864_000_000 + 864_000_000 + 864_000_000;
                let resultado1 = Socio::new("Luis".to_string(), 2345, accounts.alice, 2, Some(3), en30dias, precios_categorias.clone() );
                let resultado2 = Socio::new("Juana".to_string(), 23245, accounts.bob, 1, None, en30dias, precios_categorias.clone() );
                let resultado3 = Socio::new("Carlos".to_string(), 23445, accounts.charlie, 3, None, en30dias, precios_categorias.clone() );
                let esperado1 = Socio {
                    id_deporte: Some(3),
                    id_categoria: 2,
                    dni: 2345,
                    account: accounts.alice,
                    nombre: "Luis".to_string(),
                    pagos: Vec::from([Pago::new(en30dias, 2, None, precios_categorias.clone())]),
                };
                let esperado2 = Socio {
                    id_deporte: None,
                    id_categoria: 1,
                    dni: 23245,
                    account: accounts.bob,
                    nombre: "Juana".to_string(),
                    pagos: Vec::from([Pago::new(en30dias, 1, None, precios_categorias.clone())]),
                };
                let esperado3 = Socio {
                    id_deporte: None,
                    id_categoria: 3,
                    dni: 23445,
                    account: accounts.charlie,
                    nombre: "Carlos".to_string(),
                    pagos: Vec::from([Pago::new(en30dias, 3, None, precios_categorias.clone())]),
                };
                assert_eq!(resultado1, esperado1);
                assert_eq!(resultado2, esperado2);
                assert_eq!(resultado3, esperado3);
            }

            #[ink::test]
            #[should_panic(expected = "Categoria B debe elegir un deporte")]
            fn new_socio_test_panic_first(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let en30dias = 864_000_000 + 864_000_000 + 864_000_000;
                let precios_categorias = Vec::from([5000, 3000, 2000]);
                let _ = Socio::new("Juana".to_string(), 23245, accounts.bob, 2, None, en30dias, precios_categorias.clone() );
            }

            #[ink::test]
            #[should_panic(expected = "Categoria A y Categoria C no deben elegir un deporte  -- Este campo debe permanecer vacio")] 
            fn new_socio_test_panic_second(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let en30dias = 864_000_000 + 864_000_000 + 864_000_000;
                let precios_categorias = Vec::from([5000, 3000, 2000]);
                let _ = Socio::new("Luis".to_string(), 2345, accounts.alice, 1, Some(3), en30dias, precios_categorias.clone() );
                let _ = Socio::new("Carlos".to_string(), 23445, accounts.charlie, 3, Some(4), en30dias, precios_categorias );
            }

            #[ink::test]
            #[should_panic(expected = "Categoria B debe elegir un deporte distinto a Gimnasio(id=2) y dentro del rango 1 a 8")]
            fn new_socio_test_panic_third(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let en30dias = 864_000_000 + 864_000_000 + 864_000_000;
                let precios_categorias = Vec::from([5000, 3000, 2000]);
                let _ = Socio::new("Juana".to_string(), 23245, accounts.bob, 2, Some(2), en30dias, precios_categorias.clone() );
            }
            
            #[ink::test]
            fn puede_hacer_deporte_test(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                
                let en30dias = 864_000_000 + 864_000_000 + 864_000_000;
                let precios_categorias = Vec::from([5000, 3000, 2000]);
                let socio1 = Socio::new("Luis".to_string(), 2345, accounts.alice, 2, Some(3), en30dias, precios_categorias.clone() );
                let socio2 = Socio::new("Juana".to_string(), 23245, accounts.bob, 1, None, en30dias, precios_categorias.clone() );
                let socio3 = Socio::new("Carlos".to_string(), 23445, accounts.charlie, 3, None, en30dias, precios_categorias.clone() );
                
                assert!(socio1.puede_hacer_deporte(3));
                assert!(socio1.puede_hacer_deporte(2));
                for i in 1..9{
                    assert!(socio2.puede_hacer_deporte(i));
                } 
                assert!(socio3.puede_hacer_deporte(2));
                assert!(!socio3.puede_hacer_deporte(1));
                assert!(!socio3.puede_hacer_deporte(3));
            }
            
            #[ink::test]
            #[should_panic(expected = "ID de deporte inválido.")]
            fn puede_hacer_deporte_test_panic_deporte(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
    
                let en30dias = 864_000_000 + 864_000_000 + 864_000_000;
                let precios_categorias = Vec::from([5000, 3000, 2000]);
                let socio1 = Socio::new("Luis".to_string(), 2345, accounts.alice, 2, Some(3), en30dias, precios_categorias.clone() );
                
                socio1.puede_hacer_deporte(100);
            }
            
            #[ink::test]
            #[should_panic(expected = "ID de categoría inválido, por favor revise el socio.")]
            fn puede_hacer_deporte_test_panic_categoria(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let socio = Socio{
                    id_deporte: Some(3),
                    id_categoria: 0,
                    dni: 5432,
                    account: accounts.alice,
                    nombre: "Alicia".to_string(),
                    pagos: Vec::new(),
                };
                socio.puede_hacer_deporte(2);
            }

            #[ink::test]
            fn generar_recibos() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let precios_categorias = Vec::from([5000, 3000, 2000]);
                let en30dias = 864_000_000 + 864_000_000 + 864_000_000;
                let mut socio = Socio::new("Luis".to_string(), 2345, accounts.alice, 2, Some(3), en30dias+1, precios_categorias.clone() );

                socio.realizar_pago(Some(15), 3, 3000, en30dias, precios_categorias.clone(), 0);
                socio.realizar_pago(Some(15), 3, 3000, en30dias, precios_categorias.clone(), 0);
                socio.realizar_pago(Some(15), 3, 3000, en30dias, precios_categorias.clone(), 0);
                socio.realizar_pago(Some(15), 3, 2550, en30dias, precios_categorias.clone(), 0);

                let esperado = Vec::from([Recibo{
                    nombre: "Luis".to_string(),
                    dni: 2345,
                    monto: 3000,
                    categoria: Categoria::B,
                    fecha: en30dias
                }, Recibo {
                    nombre: "Luis".to_string(),
                    dni: 2345,
                    monto: 3000,
                    categoria: Categoria::B,
                    fecha: en30dias
                }, Recibo {
                    nombre: "Luis".to_string(),
                    dni: 2345,
                    monto: 3000,
                    categoria: Categoria::B,
                    fecha: en30dias
                }, Recibo {
                    nombre: "Luis".to_string(),
                    dni: 2345,
                    monto: 2550,
                    categoria: Categoria::B,
                    fecha: en30dias
                }]);
                let resultado = socio.generar_recibos();
                assert_eq!(esperado, resultado);
            }

            #[ink::test]
            #[should_panic(expected = "Este socio no tiene ningún Pago registrado")]
            fn generar_recibos_panic_first(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let socio = Socio{
                    id_deporte: Some(3),
                    id_categoria: 3,
                    dni: 5432,
                    account: accounts.alice,
                    nombre: "Alicia".to_string(),
                    pagos: Vec::new(),
                };
                socio.generar_recibos();
            }

            #[ink::test]
            #[should_panic(expected = "Este Socio registra un Pago sin fecha")]
            fn generar_recibos_panic_second(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let mut socio = Socio{
                    id_deporte: Some(3),
                    id_categoria: 3,
                    dni: 5432,
                    account: accounts.alice,
                    nombre: "Alicia".to_string(),
                    pagos: Vec::new(),
                };
                let un_pago = Pago {
                    vencimiento: 500_000_000,
                    categoria: Categoria::A,
                    monto: 5000,
                    pendiente: false,
                    a_tiempo: true,
                    aplico_descuento: false,
                    fecha_pago: None
                };
                socio.pagos.push(un_pago);
                socio.generar_recibos();
            }

            #[ink::test]
            fn es_moroso_test(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let precios_categorias = Vec::from([5000, 3000, 2000]);
                let en30dias = 864_000_000 + 864_000_000 + 864_000_000;
                let mut socio = Socio::new("Luis".to_string(), 2345, accounts.alice, 2, Some(3), en30dias, precios_categorias.clone() );
                assert!(!socio.es_moroso(500));
                socio.realizar_pago(None, 3, 3000, en30dias, precios_categorias, en30dias );
                assert!(socio.es_moroso(en30dias*2+1));
            }
            
            #[ink::test]
            #[should_panic(expected = "Este socio no tiene ningún pago registrado")]
            fn es_moroso_panic_test(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let socio = Socio{
                    id_deporte: Some(3),
                    id_categoria: 3,
                    dni: 5432,
                    account: accounts.alice,
                    nombre: "Alicia".to_string(),
                    pagos: Vec::new(),
                };
                socio.es_moroso(0);
            }

            #[ink::test]
            fn realizar_pago_test() {
                let now = 5000;
                let precios_categorias = Vec::from([5000, 3000, 2000]);
                let deadline = 20000;
                ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(now); 
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let esperado = Socio{
                    id_deporte: None,
                    id_categoria: 1,
                    dni: 5432,
                    account: accounts.alice,
                    nombre: "Alicia".to_string(),
                    pagos: Vec::from([Pago {
                        vencimiento: deadline,
                        categoria: Categoria::A,
                        monto: 5000,
                        pendiente: false,
                        a_tiempo: true,
                        aplico_descuento: false,
                        fecha_pago: Some(now)
                    }, Pago {
                        vencimiento: deadline*2,
                        categoria: Categoria::A,
                        monto: 5000,
                        pendiente: false,
                        a_tiempo: true,
                        aplico_descuento: false,
                        fecha_pago: Some(now)
                    }, Pago {
                        vencimiento: deadline*3,
                        categoria: Categoria::A,
                        monto: 5000,
                        pendiente: false,
                        a_tiempo: true,
                        aplico_descuento: false,
                        fecha_pago: Some(now)
                    }, Pago {
                        vencimiento: deadline*4,
                        categoria: Categoria::A,
                        monto: 4250,
                        pendiente: false,
                        a_tiempo: true,
                        aplico_descuento: true,
                        fecha_pago: Some(now)
                    }, Pago {
                        vencimiento: deadline*5,
                        categoria: Categoria::A,
                        monto: 5000,
                        pendiente: true,
                        a_tiempo: false,
                        aplico_descuento: false,
                        fecha_pago: None
                    }]),
                };
                
                let mut resultado = Socio::new("Alicia".to_string(), 5432, accounts.alice, 1, None, deadline, precios_categorias.clone());
                resultado.realizar_pago(Some(15), 3, 5000, now, precios_categorias.clone(), deadline);
                resultado.realizar_pago(Some(15), 3, 5000, now, precios_categorias.clone(), deadline);
                resultado.realizar_pago(Some(15), 3, 5000, now, precios_categorias.clone(), deadline);
                resultado.realizar_pago(Some(15), 3, 4250, now, precios_categorias.clone(), deadline);
                assert_eq!(esperado, resultado);
            }

            #[ink::test]
            #[should_panic(expected = "Este socio no tiene ningún Pago registrado")]
            fn realizar_pago_test_panic() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let precios_categorias = Vec::from([5000, 3000, 2000]);
                let mut socio = Socio{
                    id_deporte: Some(3),
                    id_categoria: 3,
                    dni: 5432,
                    account: accounts.alice,
                    nombre: "Alicia".to_string(),
                    pagos: Vec::new(),
                };
                socio.realizar_pago(None, 0, 0, 0, precios_categorias, 0);
            }

            #[ink::test]
            fn cumple_bonificacion_test() {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let precios_categorias = Vec::from([5000, 3000, 2000]);
                let en30dias = 864_000_000 + 864_000_000 + 864_000_000;
                let mut socio = Socio::new("Luis".to_string(), 2345, accounts.alice, 2, Some(3), en30dias, precios_categorias.clone() );
                assert!(!socio.cumple_bonificacion(3));
                socio.realizar_pago(Some(15), 3, 3000, 0, precios_categorias.clone(), 1);
                assert!(!socio.cumple_bonificacion(3));
                socio.realizar_pago(Some(15), 3, 3000, 0, precios_categorias.clone(), 1);
                assert!(!socio.cumple_bonificacion(3));
                socio.pagos.iter_mut().last().unwrap().a_tiempo=true; // Un pago solo está a tiempo si no está pendiente.
                assert!(socio.cumple_bonificacion(3));
                assert!(!socio.cumple_bonificacion(0));
            }

            #[ink::test]
            fn cambiar_categoria_test(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let precios_categorias = Vec::from([5000, 3000, 2000]);
                let en30dias = 864_000_000 + 864_000_000 + 864_000_000;
                let mut socio = Socio::new("Luis".to_string(), 2345, accounts.alice, 2, Some(3), en30dias, precios_categorias );
                socio.cambiar_categoria(3, None);
                assert_eq!(socio.id_categoria, 3);
                socio.cambiar_categoria(1, None);
                assert_eq!(socio.id_categoria, 1);
                socio.cambiar_categoria(2, Some(5));
                assert_eq!(socio.id_categoria, 2);
            }

            #[ink::test]
            #[should_panic(expected = "Categoria B debe elegir un deporte distinto a Gimnasio(id=2). Intente con id_deporte 1, 3, 4, 5, 6, 7, u 8")]
            fn cambiar_categoria_test_panic_first(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let precios_categorias = Vec::from([5000, 3000, 2000]);
                let en30dias = 864_000_000 + 864_000_000 + 864_000_000;
                let mut socio = Socio::new("Luis".to_string(), 2345, accounts.alice, 1, None, en30dias, precios_categorias );
                socio.cambiar_categoria(2, Some(2));
            }

            #[ink::test]
            #[should_panic(expected = "Si se desea cambiar a Categoria B, se debe elegir un deporte")]
            fn cambiar_categoria_test_panic_second(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let precios_categorias = Vec::from([5000, 3000, 2000]);
                let en30dias = 864_000_000 + 864_000_000 + 864_000_000;
                let mut socio = Socio::new("Luis".to_string(), 2345, accounts.alice, 1, None, en30dias, precios_categorias );
                socio.cambiar_categoria(2, None);
            }

            #[ink::test]
            #[should_panic(expected = "Si se desea cambiar a Categoria A o C, no se debe elegir un deporte")]
            fn cambiar_categoria_test_panic_third(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let precios_categorias = Vec::from([5000, 3000, 2000]);
                let en30dias = 864_000_000 + 864_000_000 + 864_000_000;
                let mut socio = Socio::new("Luis".to_string(), 2345, accounts.alice, 1, None, en30dias, precios_categorias );
                socio.cambiar_categoria(3, Some(3));
            }

            #[ink::test]
            fn get_mi_deporte_test(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let precios_categorias = Vec::from([5000, 3000, 2000]);
                let en30dias = 864_000_000 + 864_000_000 + 864_000_000;
                let mut socio = Socio::new("Luis".to_string(), 2345, accounts.alice, 2, Some(3), en30dias, precios_categorias );
                let resultado = socio.get_mi_deporte();
                let esperado = Some(Vec::from([Deporte::Basquet]));
                assert_eq!(resultado, esperado);

                socio.cambiar_categoria(3, None);
                let resultado = socio.get_mi_deporte();
                let esperado = None;
                assert_eq!(resultado, esperado);

                socio.cambiar_categoria(1, None);
                let resultado = socio.get_mi_deporte();
                let esperado = Some(Deporte::get_deportes());
                assert_eq!(resultado, esperado);

                socio.cambiar_categoria(2, Some(5));
                let resultado = socio.get_mi_deporte();
                let esperado = Some(Vec::from([Deporte::Hockey]));
                assert_eq!(resultado, esperado);
            }

            #[ink::test]
            #[should_panic(expected = "ID de categoría inválido, por favor revise el socio.")]
            fn get_mi_deporte_test_panic () {
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let socio = Socio{
                    id_deporte: Some(3),
                    id_categoria: 0,
                    dni: 5432,
                    account: accounts.alice,
                    nombre: "Alicia".to_string(),
                    pagos: Vec::new(),
                };
                socio.get_mi_deporte();
            }
    
            #[ink::test]
            fn mi_categoria_test(){
                let accounts: ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> = ink::env::test::default_accounts();
                let precios_categorias = Vec::from([5000, 3000, 2000]);
                let en30dias = 864_000_000 + 864_000_000 + 864_000_000;
                let socio = Socio::new("Luis".to_string(), 2345, accounts.alice, 2, Some(3), en30dias, precios_categorias );
                assert!(socio.mi_categoria(2));
                assert!(!socio.mi_categoria(3));
            }
        
        }
    }
}

