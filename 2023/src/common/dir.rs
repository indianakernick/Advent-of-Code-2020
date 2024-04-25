#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    pub const ALL: [Self; 4] = [
        Self::N,
        Self::E,
        Self::S,
        Self::W,
    ];

    pub fn opposite(self) -> Self {
        match self {
            Self::N => Self::S,
            Self::E => Self::W,
            Self::S => Self::N,
            Self::W => Self::E,
        }
    }

    pub fn to_vec_mag(self, magnitude: i32) -> (i32, i32) {
        match self {
            Self::N => (0, -magnitude),
            Self::E => (magnitude, 0),
            Self::S => (0, magnitude),
            Self::W => (-magnitude, 0),
        }
    }

    pub fn to_vec(self) -> (i32, i32) {
        self.to_vec_mag(1)
    }
}
