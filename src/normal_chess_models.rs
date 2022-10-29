use std::collections::HashMap;

pub enum NCPieceNames {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
}

pub enum PieceColor {
    BLACK,
    WHITE,
}

pub struct globalState {
    castled: bool,
    pawn_starting: Box<bool, BOARD_SIZE>,
    pawn_pushed_two: Box<bool, BOARD_SIZE>,

}


pub const BOARD_SIZE: u16 = 8;

pub trait Piece {
    fn legal_moves(&self, board_globals) -> bool;
    fn is_empty(&self) -> bool;
}

struct BoardDim(u16);

impl BoardDim {
    fn new(pos: u16) -> Option<Age> {
        if pos <= BOARD_SIZE {
            Some(Age(age))
        } else {
            None
        }
    }
}

struct PiecePos(BoardDim, BoardDim);

pub struct NCPiece {
    color: PieceColor,
    cur_pos: PiecePos,
    piece_name: NCPieceNames,
}

impl Piece for NCPiece {
    fn legal_moves(&self, board_globals: ) ->

}

pub struct Board {
    pieces: HashMap<PiecePos, NCPiece>,
    globals: HashMap<globalState, 
}

