use crate::*;

pub const PIECE_SIZE: f32 = 30.0;

#[derive(Clone, Copy)]
pub struct Piece {
    pub color: PieceColor,
    pub shape: [(i8, i8); 4],
    pub center: (f32, f32),
}

impl Piece {
    pub fn random(rng: &mut SmallRng) -> Piece {
        *PIECES.choose(rng).unwrap()
    }

    pub fn rotate(&mut self) {
        let (cx, cy) = self.center;

        for xy in self.shape.iter_mut() {
            let mut x = xy.0 as f32 - cx;
            let mut y = xy.1 as f32 - cy;

            let tmp = x;
            x = -y;
            y = tmp;

            xy.0 = (x + cx) as i8;
            xy.1 = (y + cy) as i8;
        }
    }

    pub fn x_bounds(&self) -> (i32, i32) {
        let xmin = self.shape.iter().map(|&(x, _y)| x).min().unwrap();
        let xmax = self.shape.iter().map(|&(x, _y)| x).max().unwrap();
        (xmin as i32, xmax as i32)
    }

    pub fn gen_mesh(&self, ctx: &mut Context) -> graphics::Mesh {
        let color = self.color.to_color();

        let mut mesh_builder = graphics::MeshBuilder::new();
        for &(x, y) in &self.shape {
            const UNIT: f32 = PIECE_SIZE;
            let bounds =
                graphics::Rect::new(UNIT * x as f32, UNIT * y as f32, UNIT - 1.0, UNIT - 1.0);
            mesh_builder.rectangle(graphics::DrawMode::fill(), bounds, color);
        }

        mesh_builder.build(ctx).unwrap()
    }

    pub fn draw(&self, ctx: &mut Context, pos: [i32; 2], alpha: f32) -> GameResult<()> {
        let mesh = self.gen_mesh(ctx);
        let [x, y] = pos;
        graphics::draw(
            ctx,
            &mesh,
            graphics::DrawParam::new()
                .dest([PIECE_SIZE * x as f32, PIECE_SIZE * y as f32])
                .color(Color::new(1.0, 1.0, 1.0, alpha)),
        )
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceColor {
    Teal,
    Blue,
    Orange,
    Yellow,
    Green,
    Purple,
    Red,
    Empty,
}

impl PieceColor {
    pub fn to_color(&self) -> Color {
        match self {
            PieceColor::Teal => Color::from_rgb_u32(0x008080),
            PieceColor::Blue => Color::from_rgb_u32(0x0341AE),
            PieceColor::Orange => Color::from_rgb_u32(0xFF971C),
            PieceColor::Yellow => Color::from_rgb_u32(0xFFD500),
            PieceColor::Green => Color::from_rgb_u32(0x72CB3B),
            PieceColor::Purple => Color::from_rgb_u32(0x800080),
            PieceColor::Red => Color::from_rgb_u32(0xFF3213),
            PieceColor::Empty => panic!(),
        }
    }
}

pub const PIECES: [Piece; 7] = [
    Piece {
        color: PieceColor::Teal,
        shape: [(0, 0), (1, 0), (2, 0), (3, 0)],
        center: (2.0, 1.0),
    },
    Piece {
        color: PieceColor::Blue,
        shape: [(0, 0), (0, 1), (1, 1), (2, 1)],
        center: (1.0, 1.0),
    },
    Piece {
        color: PieceColor::Orange,
        shape: [(2, 0), (0, 1), (1, 1), (2, 1)],
        center: (2.0, 1.0),
    },
    Piece {
        color: PieceColor::Yellow,
        shape: [(0, 0), (1, 0), (0, 1), (1, 1)],
        center: (1.0, 1.0),
    },
    Piece {
        color: PieceColor::Green,
        shape: [(1, 0), (2, 0), (0, 1), (1, 1)],
        center: (1.0, 1.0),
    },
    Piece {
        color: PieceColor::Purple,
        shape: [(1, 0), (0, 1), (1, 1), (2, 1)],
        center: (1.0, 1.0),
    },
    Piece {
        color: PieceColor::Red,
        shape: [(0, 0), (1, 0), (1, 1), (2, 1)],
        center: (1.0, 1.0),
    },
];
