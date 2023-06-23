use super::recibo::Recibo;
use super::pago::Pago;

pub struct Socio{
    id_deporte: Option<u32>,
    id_categoria: u32,
    dni: u32,
    nombre: String,
    pagos: Vec<Pago>,
}

impl Socio{
    pub fn generar_recibo(&self) -> Vec<Recibo> {
        todo!()
    }

    pub fn cumple_bonificacion(&self) -> bool {
        todo!()
    }

    pub fn cambiar_categoria(&mut self, id_categoria: u32, id_deporte: Option<u32>) {
        self.id_categoria = id_categoria;
        self.id_deporte = id_deporte;
    }
}