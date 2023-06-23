use super::deporte::Deporte;
use ink;

pub enum Categoria {
    A,
    B,
    C,
}

impl Categoria {
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
        todo!();
        match self {
            Categoria::A => 5000,
            Categoria::B => 3000,
            Categoria::C => 2000,
        }
    }
}