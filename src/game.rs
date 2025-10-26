use super::tetromino::*;
use rand::Rng;
pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 22;
pub const VISIBLE_HEIGHT: usize = 20;
pub const GRID_SIZE: usize = 30;

pub static FRAMES_PER_DROP: [u32; 30] = [
    48, 43, 38, 33, 28, 23, 18, 13, 8, 6, 5, 5, 5, 4, 4, 4, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    1,
];

pub const TARGET_SECONDS_PER_FRAME: f32 = 1.0 / 60.0;

pub fn random_int(min: u8, max: u8) -> u8 {
    let range: u8 = max - min;
    let mut rng = rand::rng();
    let random_num: u8 = rng.random();
    return min + random_num % range;
}

pub fn min(x: i32, y: i32) -> i32 {
    if x <= y {
        return x;
    } else {
        return y;
    }
}

pub fn max(x: i32, y: i32) -> i32 {
    if x >= y {
        return x;
    } else {
        return y;
    }
}

#[derive(PartialEq, Default)]
pub enum Game_Phase {
    #[default]
    GAME_PHASE_START,
    GAME_PHASE_PLAY,
    GAME_PHASE_LINE,
    GAME_PHASE_GAMEOVER,
}

struct IndexBuffer {
    current: u8,
    next: u8,
}

#[derive(Clone, Copy)]
pub struct Piece_State {
    tetromino_index: u8,
    offset_row: i32,
    offset_col: i32,
    rotation: i32,
}

impl Piece_State {
    pub fn new(index: u8) -> Self {
        Piece_State {
            tetromino_index: index,
            offset_row: 0,
            offset_col: WIDTH as i32 / 2,
            rotation: 0,
        }
    }
    pub fn get_tetr_index(&self) -> u8 {
        return self.tetromino_index;
    }
    pub fn get_rotation(&self) -> i32 {
        return self.rotation;
    }
    pub fn get_offset_col(&self) -> i32 {
        return self.offset_col;
    }
    pub fn get_offset_row(&self) -> i32 {
        return self.offset_row;
    }
    pub fn move_up(&mut self) {
        self.offset_row -= 1;
    }
    pub fn move_down(&mut self) {
        self.offset_row += 1;
    }
    pub fn check_piece_valid(&self, game: &Game_State, width: i32, height: i32) -> bool {
        let tetromino: &Tetromino = &TETROMINOS[(self.tetromino_index) as usize];
        for row in 0..tetromino.side {
            for col in 0..tetromino.side {
                let value: u8 = tetromino.tetromino_get(row, col, self.rotation);
                if value > 0 {
                    let board_row: i32 = self.offset_row + row;
                    let board_col: i32 = self.offset_col + col;
                    if board_row < 0 || board_row >= height || board_col < 0 || board_col >= width {
                        return false;
                    }
                    if game.matrix_get(width, board_row, board_col) != 0 {
                        return false;
                    }
                }
            }
        }
        return true;
    }
}

pub struct Game_State {
    board: [u8; WIDTH * HEIGHT],
    pub lines: [u8; HEIGHT],
    pub pending_line_count: i32,
    pub line_count: i32,
    pub points: i32,
    pub piece: Piece_State,
    pub piece_next: Piece_State,
    pub phase: Game_Phase,
    pub start_level: i32,
    pub level: i32,
    pub next_drop_time: f32,
    pub highlight_end_time: f32,
    time: f32,
    index_buffer: IndexBuffer,
}

impl Game_State {
    pub fn new() -> Self {
        Game_State {
            board: [0; WIDTH * HEIGHT],
            lines: [0; HEIGHT],
            pending_line_count: 0,
            line_count: 0,
            points: 0,
            piece: Piece_State::new(0),
            piece_next: Piece_State::new(1),
            phase: Game_Phase::GAME_PHASE_START,
            start_level: 1,
            level: 1,
            next_drop_time: 0.0,
            highlight_end_time: 0.0,
            time: 0.0,
            index_buffer: IndexBuffer {
                current: random_int(0, 7),
                next: random_int(0, 7),
            },
        }
    }
    pub fn set_time(&mut self, value: f32) {
        self.time = value;
    }
    pub fn matrix_get(&self, width: i32, row: i32, col: i32) -> u8 {
        let index: i32 = row * width + col;
        return self.board[index as usize];
    }
    fn matrix_set(&mut self, width: i32, row: i32, col: i32, value: u8) {
        let index: i32 = row * width + col;
        self.board[index as usize] = value;
    }
    fn check_row_filled(&self, width: i32, row: i32) -> u8 {
        for col in 0..width {
            if self.matrix_get(width, row, col) == 0 {
                return 0;
            }
        }
        return 1;
    }
    fn check_row_empty(&self, width: i32, row: i32) -> u8 {
        for col in 0..width {
            if self.matrix_get(width, row, col) != 0 {
                return 0;
            }
        }
        return 1;
    }
    fn find_lines(&mut self, width: i32, height: i32) -> i32 {
        let mut count: i32 = 0;
        for row in 0..height {
            let filled: u8 = self.check_row_filled(width, row);
            self.lines[row as usize] = filled;
            count += filled as i32;
        }
        return count;
    }
    fn clear_lines(&mut self, width: i32, height: i32) {
        let mut src_row: i32 = height - 1;
        for dst_row in (1..height).rev() {
            while src_row >= 0 && self.lines[src_row as usize] != 0 {
                src_row -= 1;
            }
            if src_row < 0 {
                let start: usize = (dst_row * width - 1) as usize;
                let end: usize = (dst_row * width + width - 1) as usize;
                self.board[start..=end].fill(0);
            } else {
                if src_row != dst_row {
                    let dst_start: usize = (dst_row * width) as usize;
                    let dst_end: usize = (dst_row * width + width) as usize;
                    let src_start: usize = (src_row * width) as usize;
                    let src_end: usize = (src_row * width + width) as usize;
                    let slice: &Vec<u8> = &self.board[src_start..src_end].to_vec();
                    self.board[dst_start..dst_end].copy_from_slice(slice);
                }
                src_row -= 1;
            }
        }
    }
    fn merge_piece(&mut self) {
        let tetromino: &Tetromino = &TETROMINOS[self.piece.tetromino_index as usize];
        for row in 0..tetromino.side {
            for col in 0..tetromino.side {
                let value: u8 = tetromino.tetromino_get(row, col, self.piece.rotation);
                if value != 0 {
                    let board_row: i32 = self.piece.offset_row + row;
                    let board_col: i32 = self.piece.offset_col + col;
                    if board_row >= 0
                        && board_row < HEIGHT as i32
                        && board_col >= 0
                        && board_col < WIDTH as i32
                    {
                        self.matrix_set(WIDTH as i32, board_row, board_col, value);
                    }
                }
            }
        }
        self.index_buffer.next = random_int(0, 7);
    }
    fn get_time_to_next_drop(&self) -> f32 {
        let mut faux_level = 0;
        if self.level > 29 {
            faux_level = 29;
        } else {
            faux_level = self.level;
        }
        return (FRAMES_PER_DROP[faux_level as usize]) as f32 * TARGET_SECONDS_PER_FRAME;
    }

    fn spawn_piece(&mut self) {
        self.piece_next = Piece_State::new(self.index_buffer.next);
        self.piece = Piece_State::new(self.index_buffer.current);
        self.index_buffer.current = self.index_buffer.next;
        self.piece.offset_col =
            (WIDTH as i32) / 2 - (TETROMINOS[self.piece.tetromino_index as usize].side as i32 / 2);
        self.next_drop_time = self.time + self.get_time_to_next_drop();
    }
    fn soft_drop(&mut self) -> bool {
        self.piece.offset_row += 1;
        if !self
            .piece
            .check_piece_valid(&self, WIDTH as i32, HEIGHT as i32)
        {
            self.piece.offset_row -= 1;
            self.merge_piece();
            self.spawn_piece();
            return false;
        }
        self.next_drop_time = self.time + self.get_time_to_next_drop();
        return true;
    }
    fn hard_drop(&mut self) {
        while self.soft_drop() {}
    }
    fn compute_points(&self, line_count: i32) -> i32 {
        match line_count {
            1 => return 40 * (self.level + 1),
            2 => return 100 * (self.level + 1),
            3 => return 300 * (self.level + 1),
            4 => return 1200 * (self.level + 1),
            _ => {}
        }
        return 0;
    }
    fn get_lines_for_next_level(&self, level: i32) -> i32 {
        let first_level_up_limit: i32 = min(
            self.start_level * 10 + 10,
            max(100, self.start_level * 10 - 50),
        );
        if level == self.start_level {
            return first_level_up_limit;
        }
        let diff: i32 = level - self.start_level;
        return first_level_up_limit + diff * 10;
    }
    fn update_game_start(&mut self, input: &Input_State) {
        if input.dup > 0 {
            self.start_level += 1;
        }
        if input.ddown > 0 && self.start_level > 0 {
            self.start_level -= 1;
        }
        if input.da > 0 {
            self.board.fill(0);
            self.level = self.start_level;
            self.line_count = 0;
            self.points = 0;
            self.spawn_piece();
            self.phase = Game_Phase::GAME_PHASE_PLAY;
        }
    }
    fn update_game_gameover(&mut self, input: &Input_State) {
        if input.da > 0 {
            self.phase = Game_Phase::GAME_PHASE_START;
        }
    }
    fn update_game_line(&mut self) {
        if self.time >= self.highlight_end_time {
            self.clear_lines(WIDTH as i32, HEIGHT as i32);
            self.line_count += self.pending_line_count;
            self.points += self.compute_points(self.pending_line_count);
            let lines_for_next_level = self.get_lines_for_next_level(self.level);
            if self.line_count >= lines_for_next_level {
                self.level += 1;
            }
            self.phase = Game_Phase::GAME_PHASE_PLAY;
        }
    }
    fn update_game_play(&mut self, input: &Input_State) {
        if input.dleft > 0 {
            self.piece.offset_col -= 1;
            if !self
                .piece
                .check_piece_valid(self, WIDTH as i32, HEIGHT as i32)
            {
                self.piece.offset_col += 1;
            }
        }
        if input.dright > 0 {
            self.piece.offset_col += 1;
            if !self
                .piece
                .check_piece_valid(self, WIDTH as i32, HEIGHT as i32)
            {
                self.piece.offset_col -= 1;
            }
        }
        if input.dup > 0
            && self
                .piece
                .check_piece_valid(self, WIDTH as i32, HEIGHT as i32)
        {
            self.piece.rotation = (self.piece.rotation + 1) % 4;
            if !self
                .piece
                .check_piece_valid(self, WIDTH as i32, HEIGHT as i32)
            {
                self.piece.rotation = (self.piece.rotation + 3) % 4;
            }
        }
        if self
            .piece
            .check_piece_valid(self, WIDTH as i32, HEIGHT as i32)
        {}
        if input.ddown > 0 {
            self.soft_drop();
        }
        if input.da > 0 {
            self.hard_drop();
        }
        while self.time >= self.next_drop_time {
            self.soft_drop();
        }
        self.pending_line_count = self.find_lines(WIDTH as i32, HEIGHT as i32);
        if self.pending_line_count > 0 {
            self.phase = Game_Phase::GAME_PHASE_LINE;
            self.highlight_end_time = self.time + 0.5;
        }
        let game_over_row: i32 = 0;
        if self.check_row_empty(WIDTH as i32, game_over_row) == 0 {
            self.phase = Game_Phase::GAME_PHASE_GAMEOVER;
        }
    }
    pub fn update(&mut self, input: &Input_State) {
        match self.phase {
            Game_Phase::GAME_PHASE_START => {
                self.update_game_start(input);
            }
            Game_Phase::GAME_PHASE_PLAY => {
                self.update_game_play(input);
            }
            Game_Phase::GAME_PHASE_LINE => {
                self.update_game_line();
            }
            Game_Phase::GAME_PHASE_GAMEOVER => {
                self.update_game_gameover(input);
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Input_State {
    pub left: u8,
    pub right: u8,
    pub up: u8,
    pub down: u8,
    pub a: u8,

    pub dleft: i8,
    pub dright: i8,
    pub dup: i8,
    pub ddown: i8,
    pub da: i8,
}

impl Input_State {
    pub fn new() -> Self {
        Input_State {
            left: 0,
            right: 0,
            up: 0,
            down: 0,
            a: 0,
            dleft: 0,
            dright: 0,
            dup: 0,
            ddown: 0,
            da: 0,
        }
    }
}
