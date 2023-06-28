#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[ink::contract]
mod sobrenombre_pendiente {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;

    #[derive(scale::Decode, scale::Encode, Debug, Clone)]
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

        deportes: Option<Vec<Deporte>>,
    }
    impl Socio{
        /// Construye un nuevo Socio con sus variables de a cuerdo a lo que le enviemos por parametro.
        ///
        /// Empieza con un Pago pendiente
        ///
        /// # Examples
        ///
        /// ```
        /// # #![allow(unused_mut)]
        /// let nuevo_socio = Socio::new("Carlos".to_string(), 44555888, 2, Some<02>);
        /// ```
        pub fn new(nombre: String, dni:u32, id_categoria: Option<u32>, id_deporte: Option<u32>, vencimiento:Timestamp) -> Socio {

            match id_categoria {
                3 =>
                    Socio { id_deporte: None, id_categoria, dni, nombre, pagos: vec![Pago::new(vencimiento, id_categoria,)], deportes: Some(Deporte::get_deportes())},
                2 =>
                    Socio { id_deporte: Some(id_deporte), id_categoria, dni, nombre, pagos: vec![Pago::new(vencimiento, id_categoria,)], deportes: Some(vec![Deporte::match_deporte(id_deporte)])},
                
                _ => 
                    Socio { id_deporte:None, id_categoria, dni, nombre, pagos: vec![Pago::new(vencimiento, id_categoria,)], deportes: None },
            }
            
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
                let mut i:usize;
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
            self.deportes

        }
    }

    

    #[derive(scale::Decode, scale::Encode, Debug, Clone)]
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



    #[derive(scale::Decode, scale::Encode, Debug, Clone)]
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


    #[derive(scale::Decode, scale::Encode, Debug, Clone)]
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
                _ => Self::C, // Fail-safe
            }
        }

        pub fn match_categoria(id_categoria: u32) -> Self {
            match id_categoria {
                1 => Self::A,
                2 => Self::B,
                _ => Self::C, // Fail-safe
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



    #[derive(scale::Decode, scale::Encode, Debug, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Deporte {
        Futbol,
        Basquet,
        Rugby,
        Hockey,
        Natacion,
        Tenis,
        Paddle
    }
    impl Deporte {
        pub fn get_deportes() -> Vec<Deporte> {
            vec![
                Self::Futbol,
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
                2 => Self::Basquet,
                3 => Self::Rugby,
                4 => Self::Hockey,
                5 => Self::Natacion,
                6 => Self::Tenis,
                _ => Self::Paddle,
            }
        }
    }









    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct SobrenombrePendiente {
        /// Stores a single `bool` value on the storage.
        socios: Vec<Socio>,
        descuento: u128,
        precio_categorias: (u128, u128, u128),
        duracion_deadline: Timestamp,
        pagos_consecutivos_bono: u32,
    }

    impl SobrenombrePendiente {

        #[ink(constructor)]
        pub fn new(descuento: u128, precio_categoria_a: u128, precio_categoria_b: u128, precio_categoria_c: u128) -> Self {
            Self { socios: Vec::new(),
                descuento,
                duracion_deadline: 999,
                precio_categorias:(precio_categoria_a, precio_categoria_b ,precio_categoria_c)
            }
        }

        ///
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(u128::default(), u128::default(), u128::default(), u128::default())
        }

        #[ink(message)]
        pub fn set_duracion_deadline(&mut self, d_deadline: Timestamp) {
            self.duracion_deadline = d_deadline;
        }


        #[ink(message)]
        pub fn set_descuento(&mut self, descuento: u128) {
            self.descuento = descuento;
        }

        #[ink(message)]
        pub fn get_recibos(&self, dni: u32) -> Option<Vec<Recibo>> {
            if let Some(socio) = self.socios.iter().find(|s| s.dni == dni){
                Some(socio.generar_recibos())
            } else {
                None
            }
        }

        #[ink(message)]
        pub fn registrar_nuevo_socio(&mut self, nombre: String, dni:u32, id_categoria: u32, id_deporte: u32) {
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
    }
}
