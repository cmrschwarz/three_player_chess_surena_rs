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

const CASTLING_AVAILABLE_BONUS: i16 = 5;
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

fn add_location_score(
    score: &mut Score,
    tpc: &mut ThreePlayerChess,
    loc: FieldLocation,
    piece_type: PieceType,
    color: Color,
) {
    let mut sc = piece_score(piece_type);
    let afl = AnnotatedFieldLocation::from_with_origin(color, loc);
    let f = afl.file as usize - 1;
    let r = ROW_SIZE - afl.rank as usize;
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

fn add_castling_scores(score: &mut Score, tpc: &mut ThreePlayerChess) {
    for c in Color::iter() {
        let ci = usize::from(*c);
        for cr in tpc.possible_rooks_for_castling[ci] {
            if cr.is_some() {
                score[ci] += CASTLING_AVAILABLE_BONUS;
            }
        }
    }
}

pub fn evaluate_position(
    tpc: &mut ThreePlayerChess,
    perspective: Color,
    force_eval: bool,
) -> Option<(Eval, bool)> {
    let capturing_players = if tpc.turn == get_next_hb(perspective, true) {
        Some(tpc.turn)
    } else {
        None
    };
    let mut captures_exist = false;
    let eval = match tpc.game_status {
        GameStatus::Draw(_) => EVAL_DRAW,
        GameStatus::Win(winner, win_reason) => {
            if winner == perspective {
                EVAL_WIN
            } else {
                match win_reason {
                    WinReason::DoubleResign => EVAL_LOSS,
                    WinReason::Checkmate(looser) => {
                        if looser == perspective {
                            EVAL_LOSS
                        } else {
                            EVAL_NEUTRAL
                        }
                    }
                }
            }
        }
        GameStatus::Ongoing => {
            let mut board_score = [0; HB_COUNT];
            if !force_eval && tpc.is_king_capturable(None) {
                return None;
            }
            for i in 0..BOARD_SIZE {
                if let Some((color, piece_type)) = *FieldValue::from(tpc.board[i]) {
                    let loc = FieldLocation::from(i);
                    add_location_score(&mut board_score, tpc, loc, piece_type, color);
                    if !force_eval
                        && color == perspective
                        && tpc.turn != perspective
                        && tpc.is_piece_capturable_at(loc, color, capturing_players)
                    {
                        return None;
                    }
                    if color != tpc.turn
                        && !captures_exist
                        && tpc.is_piece_capturable_at(loc, color, Some(tpc.turn))
                    {
                        captures_exist = true;
                    }
                }
            }
            add_castling_scores(&mut board_score, tpc);
            let p = usize::from(perspective);
            2 * board_score[p] - board_score[(p + 1) % 3] - board_score[(p + 2) % 3]
        }
    };
    Some((eval, captures_exist))
}
