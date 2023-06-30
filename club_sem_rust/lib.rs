#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use self::club_sem_rust::ClubSemRustRef;
#[ink::contract]
mod club_sem_rust {
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
    }
    impl Socio{
        /// Construye un nuevo Socio con sus variables de a cuerdo a lo que le enviemos por parametro.
        ///
        /// Empieza con un Pago pendiente
        ///
<<<<<<< HEAD
        /// # Ejemplos
=======
        /// # Ejemplo
>>>>>>> 094aa59 (Eliminados failsafes. Agregados test de Deporte.)
        ///
        /// ```
        /// let nuevo_socio = Socio::new("Carlos".to_string(), 44555888, 2, Some<02>);
        /// ```
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
<<<<<<< HEAD
                _ => Self::C, // Fail-safe
=======
                3 => Self::C,
                _ => panic!("ID de categoría inválido, por favor revise el socio."),
>>>>>>> 094aa59 (Eliminados failsafes. Agregados test de Deporte.)
            }
        }

        pub fn match_categoria(id_categoria: u32) -> Self {
            match id_categoria {
                1 => Self::A,
                2 => Self::B,
<<<<<<< HEAD
                _ => Self::C, // Fail-safe
=======
                3 => Self::C,
                _ => panic!("ID de categoría inválido, por favor revise el socio."),
>>>>>>> 094aa59 (Eliminados failsafes. Agregados test de Deporte.)
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
<<<<<<< HEAD
=======
        Gimnasio,
>>>>>>> 094aa59 (Eliminados failsafes. Agregados test de Deporte.)
        Basquet,
        Rugby,
        Hockey,
        Natacion,
        Tenis,
        Paddle
    }
    impl Deporte {
<<<<<<< HEAD
        pub fn get_deportes() -> Vec<Deporte> {
            vec![
                Self::Futbol,
=======
        /// Devuelve el vector de todos los deportes existentes.
        ///
        /// TODO: Explicación a rellenear por el implementador el método
        ///
        /// # Ejemplo
        ///
        /// ```
        /// let deportes = Deporte::get_deportes();
        /// 
        /// assert_eq(deportes[0], Deporte::Futbol);
        /// assert_eq(deportes[1], Deporte::Gimnasio);
        /// assert_eq(deportes[7], Deporte::Paddle);
        /// ```
        pub fn get_deportes() -> Vec<Deporte> {
            vec![
                Self::Futbol,
                Self::Gimnasio,
>>>>>>> 094aa59 (Eliminados failsafes. Agregados test de Deporte.)
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
<<<<<<< HEAD
                2 => Self::Basquet,
                3 => Self::Rugby,
                4 => Self::Hockey,
                5 => Self::Natacion,
                6 => Self::Tenis,
                _ => Self::Paddle,
=======
                2 => Self::Gimnasio,
                3 => Self::Basquet,
                4 => Self::Rugby,
                5 => Self::Hockey,
                6 => Self::Natacion,
                7 => Self::Tenis,
                8 => Self::Paddle,
                _ => panic!("Id del deporte inválido, revise el ID del socio."),
>>>>>>> 094aa59 (Eliminados failsafes. Agregados test de Deporte.)
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
    #[ink(storage)]
    pub struct ClubSemRust {
        socios: Vec<Socio>,
        descuento: u128,
        precio_categorias: (u128, u128, u128),
        duracion_deadline: Timestamp,
        pagos_consecutivos_bono: u32,
    }

    impl ClubSemRust {
        #[ink(constructor)]
        pub fn new(descuento: u128, precio_categoria_a: u128, precio_categoria_b: u128, precio_categoria_c: u128, pagos_consecutivos_bono: u32) -> Self {
            Self {
                socios: Vec::new(),
                descuento,
                duracion_deadline: 999,
                precio_categorias:(precio_categoria_a, precio_categoria_b ,precio_categoria_c),
                pagos_consecutivos_bono
            }
        }

        ///
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(15, 5000, 3000, 2000, 3)
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
                Some(socio.generar_recibo())
            } else {
                None
            }
        }

        #[ink(message)]
        pub fn registrar_nuevo_socio(&mut self, nombre: String, dni:u32, id_categoria: u32, id_deporte: u32) {
            let hoy = self.env().block_timestamp() + self.duracion_deadline;
            let socio = Socio::new(nombre, dni, id_categoria, Some(id_deporte), hoy);
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
<<<<<<< HEAD
=======

#[cfg(test)]
mod deporte_tests {
    use super::club_sem_rust::Deporte;

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

        assert_eq!(esperado, recibido, "Los deportes devueltos por el enum Deporte fueron distintos a los esperados.")
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
>>>>>>> 094aa59 (Eliminados failsafes. Agregados test de Deporte.)
