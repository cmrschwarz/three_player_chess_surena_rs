use crate::*;
use three_player_chess::board::PieceType::*;

const FIELD_BONUS_PAWN: [[i16; ROW_SIZE]; ROW_SIZE] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [50, 50, 50, 50, 50, 50, 50, 50],
    [10, 10, 20, 30, 30, 20, 10, 10],
    [5, 5, 10, 25, 25, 10, 5, 5],
    [0, 0, 0, 20, 20, 0, 0, 0],
    [5, -5, -10, 0, 0, -10, -5, 5],
    [5, 10, 10, -20, -20, 10, 10, 5],
    [0, 0, 0, 0, 0, 0, 0, 0],
];

const FIELD_BONUS_KNIGHT: [[i16; ROW_SIZE]; ROW_SIZE] = [
    [-50, -40, -30, -30, -30, -30, -40, -50],
    [-40, -20, 0, 0, 0, 0, -20, -40],
    [-30, 0, 10, 15, 15, 10, 0, -30],
    [-30, 5, 15, 20, 20, 15, 5, -30],
    [-30, 0, 15, 20, 20, 15, 0, -30],
    [-30, 5, 10, 15, 15, 10, 5, -30],
    [-40, -20, 0, 5, 5, 0, -20, -40],
    [-50, -40, -30, -30, -30, -30, -40, -50],
];

const FIELD_BONUS_BISHOP: [[i16; ROW_SIZE]; ROW_SIZE] = [
    [-20, -10, -10, -10, -10, -10, -10, -20],
    [-10, 0, 0, 0, 0, 0, 0, -10],
    [-10, 0, 5, 10, 10, 5, 0, -10],
    [-10, 5, 5, 10, 10, 5, 5, -10],
    [-10, 0, 10, 10, 10, 10, 0, -10],
    [-10, 10, 10, 10, 10, 10, 10, -10],
    [-10, 5, 0, 0, 0, 0, 5, -10],
    [-20, -10, -10, -10, -10, -10, -10, -20],
];

const FIELD_BONUS_ROOK: [[i16; ROW_SIZE]; ROW_SIZE] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [5, 10, 10, 10, 10, 10, 10, 5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [0, 0, 0, 5, 5, 0, 0, 0],
];

const FIELD_BONUS_QUEEN: [[i16; ROW_SIZE]; ROW_SIZE] = [
    [-20, -10, -10, -5, -5, -10, -10, -20],
    [-10, 0, 0, 0, 0, 0, 0, -10],
    [-10, 0, 5, 5, 5, 5, 0, -10],
    [-5, 0, 5, 5, 5, 5, 0, -5],
    [0, 0, 5, 5, 5, 5, 0, -5],
    [-10, 5, 5, 5, 5, 5, 0, -10],
    [-10, 0, 5, 0, 0, 0, 0, -10],
    [-20, -10, -10, -5, -5, -10, -10, -20],
];

const FIELD_BONUS_KING_MIDDLEGAME: [[i16; ROW_SIZE]; ROW_SIZE] = [
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [30, -40, -40, -50, -50, -40, -40, -30],
    [30, -40, -40, -50, -50, -40, -40, -30],
    [30, -40, -40, -50, -50, -40, -40, -30],
    [20, -30, -30, -40, -40, -30, -30, -20],
    [10, -20, -20, -20, -20, -20, -20, -10],
    [20, 20, 0, 0, 0, 0, 20, 20],
    [20, 30, 10, 0, 0, 10, 30, 20],
];

const FIELD_BONUS_KING_ENDGAME: [[i16; ROW_SIZE]; ROW_SIZE] = [
    [-50, -40, -30, -20, -20, -30, -40, -50],
    [-30, -20, -10, 0, 0, -10, -20, -30],
    [-30, -10, 20, 30, 30, 20, -10, -30],
    [-30, -10, 30, 40, 40, 30, -10, -30],
    [-30, -10, 30, 40, 40, 30, -10, -30],
    [-30, -10, 20, 30, 30, 20, -10, -30],
    [-30, -30, 0, 0, 0, 0, -30, -30],
    [-50, -30, -30, -30, -30, -30, -30, -50],
];

fn piece_score(pt: PieceType) -> Eval {
    match pt {
        Pawn => 100,
        Knight => 300,
        Bishop => 400,
        Rook => 500,
        Queen => 900,
        King => 0,
    }
}

fn add_location_score(score: &mut Score, tpc: &mut ThreePlayerChess, loc: FieldLocation) {
    if let Some((color, piece_type)) = *FieldValue::from(tpc.board[usize::from(loc)]) {
        let mut sc = piece_score(piece_type);
        let afl = AnnotatedFieldLocation::from_with_origin(color, loc);
        let f = afl.file as usize - 1;
        let r = afl.rank as usize - 1;
        sc += match piece_type {
            Pawn => FIELD_BONUS_PAWN[r][f],
            Knight => FIELD_BONUS_KNIGHT[r][f],
            Bishop => FIELD_BONUS_BISHOP[r][f],
            Rook => FIELD_BONUS_ROOK[r][f],
            Queen => FIELD_BONUS_QUEEN[r][f],
            King => {
                //TODO: use a proper endgame detection using the piece count
                if tpc.move_index > 30 {
                    FIELD_BONUS_KING_ENDGAME[r][f]
                } else {
                    FIELD_BONUS_KING_MIDDLEGAME[r][f]
                }
            }
        };
        score[usize::from(color)] += sc;
    }
}

pub fn evaluate_position(tpc: &mut ThreePlayerChess) -> Score {
    match tpc.game_status {
        GameStatus::Draw(_) => [EVAL_DRAW; HB_COUNT],
        GameStatus::Win(winner, win_reason) => {
            let mut score = [0 as Eval; HB_COUNT];
            let windex = usize::from(winner);
            score[windex] = EVAL_WIN;
            match win_reason {
                WinReason::DoubleResign => {
                    score[(windex + 1) % 3] = EVAL_LOSS;
                    score[(windex + 2) % 3] = EVAL_LOSS;
                }
                WinReason::Checkmate(looser) => {
                    score[usize::from(looser)] = EVAL_LOSS;
                    let neutral = if get_next_hb(winner, true) == looser {
                        windex + 2
                    } else {
                        windex + 1
                    };
                    score[neutral % 3] = EVAL_NEUTRAL;
                }
            }
            score
        }
        GameStatus::Ongoing => {
            let mut board_score = [0; HB_COUNT];
            for i in 0..BOARD_SIZE {
                add_location_score(&mut board_score, tpc, FieldLocation::from(i));
            }
            let mut score = [0; HB_COUNT];
            for i in 0..HB_COUNT {
                score[i] = 2 * board_score[i] - board_score[(i + 1) % 3] - board_score[(i + 2) % 3];
            }
            score
        }
    }
}