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