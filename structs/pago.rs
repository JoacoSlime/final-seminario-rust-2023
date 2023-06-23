use ink_e2e::env_logger::fmt::Timestamp;
use super::categoria::Categoria;

pub struct Pago {
    vencimiento: Timestamp,
    categoria: Categoria,
    pendiente: bool,
    a_tiempo: bool,
}

impl Pago {
    pub fn new() {
        todo!()
    }

    pub fn verificar_pago(&self, monto: u128) -> bool {
        self.categoria.mensual() == monto
    }

    pub fn realizar_pago(&mut self, monto: u128) {
        todo!()
    }
}