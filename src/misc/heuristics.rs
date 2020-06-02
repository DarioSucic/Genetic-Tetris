use crate::*;

pub fn surface_roughness_heuristic(board: &TetrisBoard) -> f64 {
    let mut prev = BOARD_HEIGHT as i32;
    let mut total: i32 = 0;

    for i in 0..BOARD_HEIGHT {
        if board[i][0] != PieceColor::Empty {
            prev = i as i32;
            break;
        }
    }

    for j in 1..BOARD_WIDTH {
        let mut curr = BOARD_HEIGHT as i32;

        for i in 0..BOARD_HEIGHT {
            if board[i][j] != PieceColor::Empty {
                curr = i as i32;
                break;
            }
        }
        total += (curr - prev).abs();
        prev = curr;
    }

    total as f64
}

pub fn height_heuristic(board: &TetrisBoard) -> f64 {
    for i in 0..BOARD_HEIGHT {
        for j in 0..BOARD_WIDTH {
            if board[i][j] != PieceColor::Empty {
                return (BOARD_HEIGHT - i) as f64;
            }
        }
    }
    0.0
}

pub fn line_completion_heuristic(board: &TetrisBoard) -> f64 {
    let mut n_lines = 0;
    for i in 0..BOARD_HEIGHT {
        n_lines += board[i].iter().all(|&v| v != PieceColor::Empty) as i32;
    }
    n_lines as f64
}

pub fn ceil_gap_heuristic(board: &TetrisBoard) -> f64 {
    let mut total = 0;
    for i in 0..BOARD_HEIGHT - 1 {
        for j in 0..BOARD_WIDTH {
            if board[i][j] != PieceColor::Empty {
                for ii in i + 1..BOARD_HEIGHT {
                    if board[ii][j] != PieceColor::Empty {
                        break;
                    }
                    total += 1;
                }
            }
        }
    }
    total as f64
}

pub const N_HEURISTICS: usize = 4;
pub const HEURISTICS: [fn(&TetrisBoard) -> f64; N_HEURISTICS] = [
    surface_roughness_heuristic,
    height_heuristic,
    line_completion_heuristic,
    ceil_gap_heuristic,
];
