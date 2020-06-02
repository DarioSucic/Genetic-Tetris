pub use ggez::event::{self, EventHandler};
pub use ggez::{
    conf,
    graphics::{self, Color},
    input::keyboard::{is_key_pressed, KeyCode},
    Context, ContextBuilder, GameResult,
};

pub use rand::prelude::*;
pub use rand::{rngs::SmallRng, Rng};

pub use std::collections::HashMap;

pub const PIECE_SIZE: f32 = 30.0;
pub const UNIT: f32 = PIECE_SIZE;
pub const PIECE_SPAWN_OFFSET: i32 = 2;
pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

mod piece;
pub use piece::*;

pub type TetrisBoard = [[PieceColor; BOARD_WIDTH]; BOARD_HEIGHT];

#[derive(Clone)]
pub struct TetrisState {
    pub current_piece: Piece,
    pub ghost_pos: [i32; 2],
    pub next_piece: Piece,
    pub pos: [i32; 2],
    pub board: TetrisBoard,
    pub drop_count: u64,
    pub sub_count: u64,
    pub pressed_map: HashMap<KeyCode, bool>,
    pub rng: SmallRng,
    pub score: u32,
    pub is_over: bool,
}

impl TetrisState {
    pub fn new(_ctx: &mut Context) -> TetrisState {
        let mut rng = SmallRng::from_entropy();

        let mut pressed_map = HashMap::new();
        for &key in &[KeyCode::Left, KeyCode::Right, KeyCode::Up, KeyCode::Down] {
            pressed_map.insert(key, false);
        }

        TetrisState {
            current_piece: Piece::random(&mut rng),
            next_piece: Piece::random(&mut rng),
            pos: [BOARD_WIDTH as i32 / 2 - 2, PIECE_SPAWN_OFFSET],
            ghost_pos: [0, 0],
            board: [[PieceColor::Empty; BOARD_WIDTH]; BOARD_HEIGHT],
            drop_count: 0,
            sub_count: 0,
            pressed_map,
            rng,
            score: 0,
            is_over: false,
        }
    }

    pub fn reset(&mut self) {
        self.rng = SmallRng::from_entropy();
        self.current_piece = Piece::random(&mut self.rng);
        self.next_piece = Piece::random(&mut self.rng);
        self.pos = [BOARD_WIDTH as i32 / 2 - 2, PIECE_SPAWN_OFFSET];
        self.ghost_pos = [0, 0];
        self.board
            .iter_mut()
            .for_each(|line| line.iter_mut().for_each(|x| *x = PieceColor::Empty));
        self.drop_count = 0;
        self.sub_count = 0;
        self.pressed_map.clear();
        self.score = 0;
        self.is_over = false;
    }

    pub fn pick_move_by_key<F>(&mut self, f: F) -> Option<KeyCode>
    where
        F: Fn(&TetrisBoard) -> f64,
    {
        fn with_locked(mut board: TetrisBoard, pos: [i32; 2], piece: &Piece) -> TetrisBoard {
            let color = piece.color;
            let [px, py] = pos;
            for &(x, y) in &piece.shape {
                let x = px + x as i32;
                let y = py + y as i32;
                board[y as usize][x as usize] = color;
            }
            board
        }

        let mut piece = self.current_piece;
        let mut pos = self.pos;
        let mut best = (1e9, 0, self.pos);

        let xmax = BOARD_WIDTH as i32;
        for rotation in 0..4 {
            let (p_xmin, p_xmax) = piece.x_bounds();
            for x in 0 - p_xmin..xmax - p_xmax {
                pos[0] = x;
                if !self.is_valid_move(pos, &piece) {
                    continue;
                }
                let drop = self.calc_drop_pos(pos, &piece);
                let board = with_locked(self.board, drop, &piece);
                let loss = f(&board);
                // println!("[x, rot] = {:?} => {:.2}", [x, rotation], loss);
                if loss < best.0 {
                    best = (loss, rotation, pos);
                }
            }

            if rotation < 3 {
                piece.rotate();
            }
        }

        let (_best_score, best_rotation, best_pos) = best;

        if best_rotation > 0 {
            return Some(KeyCode::Up);
        }

        use std::cmp::Ordering;
        match best_pos[0].cmp(&self.pos[0]) {
            Ordering::Equal => None,
            Ordering::Less => Some(KeyCode::Left),
            Ordering::Greater => Some(KeyCode::Right),
        }
    }

    pub fn key_handler<F>(&mut self, ctx: &mut Context, key: KeyCode, mut handler: F)
    where
        F: FnMut(&mut TetrisState),
    {
        let is_pressed = self.pressed_map.entry(key).or_default();

        if is_key_pressed(ctx, key) {
            if !*is_pressed {
                *is_pressed = true;
                handler(self);
                self.sub_count = 0;
            }
        } else {
            *is_pressed = false;
        }
    }

    pub fn rotate_current_piece(&mut self) {
        let mut new_piece = self.current_piece;
        new_piece.rotate();

        if self.is_valid_move(self.pos, &new_piece) {
            self.current_piece = new_piece;
        }
    }

    pub fn movement_handler(&mut self, ctx: &mut Context) {
        self.key_handler(ctx, KeyCode::Left, |state| state.move_current_piece(-1, 0));
        self.key_handler(ctx, KeyCode::Right, |state| state.move_current_piece(1, 0));
        self.key_handler(ctx, KeyCode::Down, |state| state.move_current_piece(0, 1));
    }

    pub fn rotation_handler(&mut self, ctx: &mut Context) {
        self.key_handler(ctx, KeyCode::Up, |state| {
            state.rotate_current_piece();
        });
    }

    pub fn drop_handler(&mut self, ctx: &mut Context) {
        self.key_handler(ctx, KeyCode::Space, |state| {
            state.pos = state.calc_drop_pos(state.pos, &state.current_piece);
            state.drop_count += 144 - state.drop_count % 144 - 1;
        });
    }

    pub fn collides(&self, x: usize, y: usize) -> bool {
        self.board[y][x] != PieceColor::Empty
    }

    pub fn is_in_map(&self, x: usize, y: usize) -> bool {
        x < BOARD_WIDTH && y < BOARD_HEIGHT
    }

    pub fn is_valid_move(&self, pos: [i32; 2], piece: &Piece) -> bool {
        let [nx, ny] = pos;

        for &(px, py) in &piece.shape {
            let x = (nx as i8 + px) as usize;
            let y = (ny as i8 + py) as usize;
            if !self.is_in_map(x, y) || self.collides(x, y) {
                return false;
            }
        }

        true
    }

    pub fn move_current_piece(&mut self, dx: i32, dy: i32) {
        let pos = [self.pos[0] + dx, self.pos[1] + dy];
        if self.is_valid_move(pos, &self.current_piece) {
            self.pos = pos;
        }
    }

    pub fn lock_current_piece(&mut self) {
        let color = self.current_piece.color;
        for &(x, y) in &self.current_piece.shape {
            let x = self.pos[0] + x as i32;
            let y = self.pos[1] + y as i32;
            self.board[y as usize][x as usize] = color;
        }
    }

    pub fn propagate_lines(&mut self) {
        for y in 0..BOARD_HEIGHT {
            if self.board[y].iter().all(|&v| v != PieceColor::Empty) {
                self.score += 100;
                for x in 0..BOARD_WIDTH {
                    self.board[y][x] = PieceColor::Empty;
                }
                self.board[0..=y].rotate_right(1);
            }
        }
    }

    pub fn draw_map(&self, ctx: &mut Context) -> GameResult<()> {
        let mut mesh_builder = graphics::MeshBuilder::new();

        let mut n = 0;

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let piece_color = self.board[y][x];
                if piece_color == PieceColor::Empty {
                    continue;
                }
                let color = piece_color.to_color();

                let bounds =
                    graphics::Rect::new(UNIT * x as f32, UNIT * y as f32, UNIT - 1.0, UNIT - 1.0);
                mesh_builder.rectangle(graphics::DrawMode::fill(), bounds, color);
                n += 1;
            }
        }

        if n < 1 {
            Ok(())
        } else {
            let mesh = mesh_builder.build(ctx).unwrap();
            graphics::draw(ctx, &mesh, graphics::DrawParam::new())
        }
    }

    pub fn calc_drop_pos(&self, mut pos: [i32; 2], piece: &Piece) -> [i32; 2] {
        loop {
            pos[1] += 1;
            if !self.is_valid_move(pos, &piece) {
                pos[1] -= 1;
                break;
            }
        }

        pos
    }

    pub fn update_ghost_pos(&mut self) {
        self.ghost_pos = self.calc_drop_pos(self.pos, &self.current_piece);
    }

    pub fn fill_screen(&self, ctx: &mut Context, color: Color) -> GameResult<()> {
        let rect = graphics::Rect::new(
            0.0,
            0.0,
            UNIT * BOARD_WIDTH as f32,
            UNIT * BOARD_HEIGHT as f32,
        );
        let mesh =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color).unwrap();
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())
    }

    pub fn draw_centered(&self, ctx: &mut Context, text: &str, color: Color) -> GameResult<()> {
        let mut score_text = graphics::Text::new(text);
        score_text.set_font(graphics::Font::default(), graphics::Scale::uniform(36.0));

        let text_width = score_text.width(ctx) as f32;
        let screen_width = UNIT * BOARD_WIDTH as f32;
        let width_padding = (screen_width - text_width) / 2.0;

        let text_height = score_text.height(ctx) as f32;
        let screen_height = UNIT * BOARD_HEIGHT as f32;
        let height_padding = (screen_height - text_height) / 2.0;

        graphics::draw(
            ctx,
            &score_text,
            graphics::DrawParam::new()
                .color(color)
                .dest([width_padding, height_padding]),
        )
    }
}
