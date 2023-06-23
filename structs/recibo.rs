use super::categoria::Categoria;

pub struct Recibo {
    nombre: String,
    dni: u32,
    monto: u128,
    categoria: Categoria,
}