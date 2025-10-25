use super::game::*;
use super::tetromino::*;
use sdl2::pixels::Color;
use sdl2::render::*;
use sdl2::video::*;

const fn color(r: u8, g: u8, b: u8, a: u8) -> Color {
    let result: Color = Color {
        r: r,
        g: g,
        b: b,
        a: a,
    };
    return result;
}

static BASE_COLORS: [Color; 8] = [
    color(0x28, 0x28, 0x28, 0xFF),
    color(0x2D, 0x99, 0x99, 0xFF),
    color(0x99, 0x99, 0x2D, 0xFF),
    color(0x99, 0x2D, 0x99, 0xFF),
    color(0x2D, 0x99, 0x51, 0xFF),
    color(0x99, 0x2D, 0x2D, 0xFF),
    color(0x2D, 0x63, 0x99, 0xFF),
    color(0x99, 0x63, 0x2D, 0xFF),
];

static LIGHT_COLORS: [Color; 8] = [
    color(0x28, 0x28, 0x28, 0xFF),
    color(0x44, 0xE5, 0xE5, 0xFF),
    color(0xE5, 0xE5, 0x44, 0xFF),
    color(0xE5, 0x44, 0xE5, 0xFF),
    color(0x44, 0xE5, 0x7A, 0xFF),
    color(0xE5, 0x44, 0x44, 0xFF),
    color(0x44, 0x95, 0xE5, 0xFF),
    color(0xE5, 0x95, 0x44, 0xFF),
];

static DARK_COLORS: [Color; 8] = [
    color(0x28, 0x28, 0x28, 0xFF),
    color(0x1E, 0x66, 0x66, 0xFF),
    color(0x66, 0x66, 0x1E, 0xFF),
    color(0x66, 0x1E, 0x66, 0xFF),
    color(0x1E, 0x66, 0x36, 0xFF),
    color(0x66, 0x1E, 0x1E, 0xFF),
    color(0x1E, 0x42, 0x66, 0xFF),
    color(0x66, 0x42, 0x1E, 0xFF),
];

enum Text_Allignment {
    TEXT_ALLIGN_LEFT,
    TEXT_ALLIGN_CENTER,
    TEXT_ALLIGN_RIGHT,
}

fn draw_string(
    canvas: &mut Canvas<Window>,
    font: &sdl2::ttf::Font,
    text: &str,
    x: i32,
    y: i32,
    allignment: Text_Allignment,
    color: Color,
) {
    let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();

    let surface = font
        .render(text)
        .blended(color)
        .map_err(|e| e.to_string())
        .unwrap();

    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())
        .unwrap();

    let mut rect = sdl2::rect::Rect::new(0, 0, 0, 0);

    rect.w = surface.width() as i32;
    rect.h = surface.height() as i32;
    rect.y = y;
    match allignment {
        Text_Allignment::TEXT_ALLIGN_LEFT => rect.x = x,
        Text_Allignment::TEXT_ALLIGN_CENTER => rect.x = x - surface.width() as i32 / 2,
        Text_Allignment::TEXT_ALLIGN_RIGHT => rect.x = x - surface.width() as i32,
    }
    canvas.copy(&texture, None, Some(rect)).unwrap();
}

fn draw_cell(
    canvas: &mut Canvas<Window>,
    row: i32,
    col: i32,
    value: u8,
    offset_x: i32,
    offset_y: i32,
    outline: bool,
) {
    let base_color: Color = BASE_COLORS[value as usize];
    let light_color: Color = LIGHT_COLORS[value as usize];
    let dark_color: Color = DARK_COLORS[value as usize];
    let edge: i32 = GRID_SIZE as i32 / 8;
    let x: i32 = col * GRID_SIZE as i32 + offset_x;
    let y: i32 = row * GRID_SIZE as i32 + offset_y;
    if outline {
        canvas.set_draw_color(dark_color);
        canvas
            .draw_rect(sdl2::rect::Rect::new(
                x,
                y,
                GRID_SIZE as u32,
                GRID_SIZE as u32,
            ))
            .unwrap();
        return;
    }
    canvas.set_draw_color(dark_color);
    canvas
        .fill_rect(sdl2::rect::Rect::new(
            x,
            y,
            GRID_SIZE as u32,
            GRID_SIZE as u32,
        ))
        .unwrap();
    canvas.set_draw_color(light_color);
    canvas
        .fill_rect(sdl2::rect::Rect::new(
            x + edge,
            y,
            GRID_SIZE as u32 - edge as u32,
            GRID_SIZE as u32 - edge as u32,
        ))
        .unwrap();
    canvas.set_draw_color(base_color);
    canvas
        .fill_rect(sdl2::rect::Rect::new(
            x + edge,
            y + edge,
            GRID_SIZE as u32 - edge as u32 * 2,
            GRID_SIZE as u32 - edge as u32 * 2,
        ))
        .unwrap();
}

fn draw_piece(
    canvas: &mut Canvas<Window>,
    piece: &Piece_State,
    offset_x: i32,
    offset_y: i32,
    outline: bool,
) {
    let tetromino: &Tetromino = &TETROMINOS[piece.get_tetr_index() as usize];
    for row in 0..tetromino.side {
        for col in 0..tetromino.side {
            let value: u8 = tetromino.tetromino_get(row, col, piece.get_rotation());
            if value != 0 {
                draw_cell(
                    canvas,
                    row + piece.get_offset_row(),
                    col + piece.get_offset_col(),
                    value,
                    offset_x,
                    offset_y,
                    outline,
                );
            }
        }
    }
}

fn draw_next_piece(canvas: &mut Canvas<Window>, piece_next: &Piece_State, outline: bool) {
    let tetromino: &Tetromino = &TETROMINOS[piece_next.get_tetr_index() as usize];
    for row in 0..tetromino.side {
        for col in 0..tetromino.side {
            let value: u8 = tetromino.tetromino_get(row, col, piece_next.get_rotation());
            if value != 0 {
                draw_cell(canvas, row + 1, col + 5, value, 0, 0, outline);
            }
        }
    }
}

fn draw_board(
    canvas: &mut Canvas<Window>,
    game: &Game_State,
    width: i32,
    height: i32,
    offset_x: i32,
    offset_y: i32,
) {
    canvas.set_draw_color(BASE_COLORS[0]);
    canvas
        .fill_rect(sdl2::rect::Rect::new(
            offset_x,
            offset_y,
            width as u32 * GRID_SIZE as u32,
            height as u32 * GRID_SIZE as u32,
        ))
        .unwrap();
    for row in 0..height {
        for col in 0..width {
            let value: u8 = game.matrix_get(width, row, col);
            if value != 0 {
                draw_cell(canvas, row, col, value, offset_x, offset_y, false);
            }
        }
    }
}

pub fn render_game(game: &mut Game_State, canvas: &mut Canvas<Window>, font: &sdl2::ttf::Font) {
    let highlight_color = color(0xFF, 0xFF, 0xFF, 0xFF);
    let padding_y: i32 = 60;
    draw_board(canvas, &game, WIDTH as i32, HEIGHT as i32, 0, padding_y);
    if game.phase == Game_Phase::GAME_PHASE_PLAY {
        draw_piece(canvas, &game.piece, 0, padding_y, false);
        let mut piece: Piece_State = game.piece;
        while piece.check_piece_valid(&game, WIDTH as i32, HEIGHT as i32) {
            piece.move_down();
        }
        piece.move_up();
        draw_piece(canvas, &piece, 0, padding_y, true);
    }
    if game.phase == Game_Phase::GAME_PHASE_LINE {
        for row in 0..HEIGHT {
            if game.lines[row] > 0 {
                let x: i32 = 0;
                let y: i32 = row as i32 * GRID_SIZE as i32 + padding_y;

                canvas.set_draw_color(highlight_color);
                let _ = canvas
                    .fill_rect(sdl2::rect::Rect::new(
                        x,
                        y,
                        WIDTH as u32 * GRID_SIZE as u32,
                        GRID_SIZE as u32,
                    ))
                    .unwrap();
            }
        }
    } else if game.phase == Game_Phase::GAME_PHASE_GAMEOVER {
        let x: i32 = WIDTH as i32 * GRID_SIZE as i32 / 2;
        let y: i32 = (HEIGHT as i32 * GRID_SIZE as i32 + padding_y) / 2;
        draw_string(
            canvas,
            font,
            "GAME OVER",
            x,
            y,
            Text_Allignment::TEXT_ALLIGN_CENTER,
            highlight_color,
        );
    } else if game.phase == Game_Phase::GAME_PHASE_START {
        let x: i32 = WIDTH as i32 * GRID_SIZE as i32 / 2;
        let y: i32 = (HEIGHT as i32 * GRID_SIZE as i32 + padding_y) / 2;
        draw_string(
            canvas,
            font,
            "PRESS START",
            x,
            y,
            Text_Allignment::TEXT_ALLIGN_CENTER,
            highlight_color,
        );
        let start_level_text = format!("STARTING LEVEL: {}", game.start_level);
        draw_string(
            canvas,
            font,
            &start_level_text,
            x,
            y + 32,
            Text_Allignment::TEXT_ALLIGN_CENTER,
            highlight_color,
        );
    }
    let margin_y: i32 = 60;
    canvas.set_draw_color(color(0x00, 0x00, 0x00, 0x00));
    let _ = canvas
        .fill_rect(sdl2::rect::Rect::new(
            0,
            margin_y,
            WIDTH as u32 * GRID_SIZE as u32,
            (HEIGHT as u32 - VISIBLE_HEIGHT as u32) * GRID_SIZE as u32,
        ))
        .unwrap();
    draw_next_piece(canvas, &game.piece_next, false);
    let level_string = format!("LEVEL: {}", game.level);
    draw_string(
        canvas,
        font,
        &level_string,
        5,
        5,
        Text_Allignment::TEXT_ALLIGN_LEFT,
        highlight_color,
    );
    let score_string = format!("SCORE: {}", game.points);
    draw_string(
        canvas,
        font,
        &score_string,
        5,
        40,
        Text_Allignment::TEXT_ALLIGN_LEFT,
        highlight_color,
    );
    let lines_string = format!("LINES: {}", game.line_count);
    draw_string(
        canvas,
        font,
        &lines_string,
        5,
        75,
        Text_Allignment::TEXT_ALLIGN_LEFT,
        highlight_color,
    );
    draw_string(
        canvas,
        font,
        "NEXT",
        150,
        5,
        Text_Allignment::TEXT_ALLIGN_LEFT,
        highlight_color,
    );
}
