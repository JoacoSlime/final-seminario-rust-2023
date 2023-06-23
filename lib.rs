#![cfg_attr(not(feature = "std"), no_std, no_main)]
mod structs;

#[ink::contract]
mod sobrenombre_pendiente {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct SobrenombrePendiente {
        /// Stores a single `bool` value on the storage.
        socios: Vec<Socio>,
        descuento: u128,
        
    }

    impl SobrenombrePendiente {

        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        #[ink(message)]
        pub fn set_descuento(&mut self, descuento: u128) {
            self.descuento = descuento;
        }

        #[ink(message)]
        pub fn get_recibos(&self, dni: u32) -> Result<Vec<Recibo>, ErrorCustom> {
            if let Some(socio) = self.socios.iter().find(|s| s.dni == dni){
                Ok(socio.generar_recibos())
            } else {
                Err(ErrorCustom::SocioNoEncontrado)
            }
        }

        #[ink(message)]
        pub fn registrar_nuevo_socio(&mut self, nombre: String, id_categoria: u32, id_deporte: Option<u32>) {
            let socio = Socio::new(nombre, id_categoria, id_deporte);
            self.socios.push(socio);
        }

        #[ink(message)]
        pub fn registrar_pago(&mut self, dni: u32, monto: u128) {
            todo!()
        }

        #[ink(message)]
        pub fn get_socios(&self) -> Vec<Socio> {
            self.socios.clone()
        }
    }
}
