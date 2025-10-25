#[derive(Clone, Default)]
pub struct Tetromino<'a> {
    pub data: &'a [u8],
    pub side: i32,
}

impl Tetromino<'_> {
    pub fn tetromino_get(&self, row: i32, col: i32, rotation: i32) -> u8 {
        let side: i32 = self.side;
        match rotation {
            0 => return self.data[(row * side + col) as usize],
            1 => return self.data[((side - col - 1) * side + row) as usize],
            2 => return self.data[((side - row - 1) * side + (side - col - 1)) as usize],
            3 => return self.data[(col * side + (side - row - 1)) as usize],
            _ => {}
        }
        return 0;
    }
}

pub static TETRINO_1: &[u8] = &[0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0];

pub static TETRINO_2: &[u8] = &[2, 2, 2, 2];

pub static TETRINO_3: &[u8] = &[0, 3, 0, 3, 3, 3, 0, 0, 0];

pub static TETRINO_4: &[u8] = &[0, 4, 4, 4, 4, 0, 0, 0, 0];

pub static TETRINO_5: &[u8] = &[5, 5, 0, 0, 5, 5, 0, 0, 0];

pub static TETRINO_6: &[u8] = &[6, 0, 0, 6, 6, 6, 0, 0, 0];

pub static TETRINO_7: &[u8] = &[0, 0, 7, 7, 7, 7, 0, 0, 0];

pub const TETR_1: Tetromino = Tetromino {
    data: &TETRINO_1,
    side: 4,
};

pub const TETR_2: Tetromino = Tetromino {
    data: &TETRINO_2,
    side: 2,
};

pub const TETR_3: Tetromino = Tetromino {
    data: &TETRINO_3,
    side: 3,
};

pub const TETR_4: Tetromino = Tetromino {
    data: &TETRINO_4,
    side: 3,
};

pub const TETR_5: Tetromino = Tetromino {
    data: &TETRINO_5,
    side: 3,
};

pub const TETR_6: Tetromino = Tetromino {
    data: &TETRINO_6,
    side: 3,
};

pub const TETR_7: Tetromino = Tetromino {
    data: &TETRINO_7,
    side: 3,
};

pub static TETROMINOS: [Tetromino; 7] = [TETR_1, TETR_2, TETR_3, TETR_4, TETR_5, TETR_6, TETR_7];
