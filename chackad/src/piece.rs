// TODO create better logic for seperating White/Black case for pawns (now it is almost duplicate code)
use crate::board::Board;

pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

pub struct Piece {
    type: PieceType,
    pos_x: isize,
    pos_y: isize,
    is_white: bool,
}

impl Piece {
    pub fn new(type: PieceType, pos_x: isize, pos_y: isize, is_white: bool) -> Piece {
        Piece {
            type: type,
            pos_x: pos_x,
            pos_y: pos_y,
            is_white: is_white,
        }
    }

    fn move_now(to_x: isize, to_y: isize, board: &Board) {
        board.set_emptiness(to_x, to_y, false);
        board.set_emptiness(Self.pos_x, Self.pos_y, true);
        Self.pos_x = to_x;
        Self.pos_y = to_y;
        board.set_piece_is_white(to_x, to_y, is_white);
    }

    pub fn move_to (to_x: isize, to_y: isize, board: &Board) -> bool {
        match Self.type {
            if board.is_valid_tile(to_x, to_y) {
                return false;
            }
            Pawn => return Self.move_pawn(to_x, to_y, board),
            Rook => return Self.move_rook(to_x, to_y, board),
            Knight => return Self.move_knight(to_x, to_y, board),
            Bishop => return Self.move_bishop(to_x, to_y, board),
            Queen => return Self.move_queen(to_x, to_y, board),
            King => return Self.move_king(to_x, to_y, board),
            _ => //Error
        }
    }
    fn move_pawn (to_x: isize, to_y: isize, board: &Board) -> bool{
        if is_white {
            // Regular move one step forward (including promotion)
            if to_x == Self.pos_x && to_y == Self.pos_y + 1 {
                if board.is_empty_tile(to_x, to_y) && to_y == board.size_y - 1{
                    // TODO implement choose piece for promotion
                    Self.move_now(to_x, to_y, board);
                    return true; //No error TODO fix with rust things
                } else if board.is_empty_tile(to_x, to_y) {
                    Self.move_now(to_x, to_y, board);
                    return true; //No error TODO fix with rust things
                } else {
                    return false;
                }
            }
            // Capture move
            if to_y == Self.pos_y + 1 && (to_x == Self.pos_x - 1 || to_x == Self.pos_x + 1) {
                if !board.is_empty_tile && !board.is_piece_white(to_x, to_y){
                    Self.move_now(to_x, to_y, board);
                    return true; //No error TODO fix with rust things
                }else {
                    return false;
                }
            }

            // Double move
            if to_x == Self.pos_x && to_y == Self.pos_y + 2 {
                if board.is_empty_tile(to_x, to_y) && board.is_empty_tile(to_x, to_y - 1) && pos_y == 1 {
                    Self.move_now(to_x, to_y, board);
                    return true; //No error TODO fix with rust things
                } else {
                    return false;
                }
            }
        } else {
            // Regular move one step forward (including promotion)
            if to_x == Self.pos_x && to_y == Self.pos_y - 1 {
                if board.is_empty_tile(to_x, to_y) && to_y == 0 {
                    // TODO implement choose piece for promotion
                    Self.move_now(to_x, to_y, board);
                    return true; //No error TODO fix with rust things
                } else if board.is_empty_tile(to_x, to_y) {
                    Self.move_now(to_x, to_y, board);
                    return true; //No error TODO fix with rust things
                } else {
                    return false;
                }
            }
            // Capture move
            if to_y == Self.pos_y - 1 && (to_x == Self.pos_x - 1 || to_x == Self.pos_x + 1) {
                if !board.is_empty_tile && board.is_piece_white(to_x, to_y){
                    Self.move_now(to_x, to_y, board);
                    return true; //No error TODO fix with rust things
                }else {
                    return false;
                }
            }

            // Double move
            if to_x == Self.pos_x && to_y == Self.pos_y - 2 {
                if board.is_empty_tile(to_x, to_y) && board.is_empty_tile(to_x, to_y - 1) && pos_y == board.size_y - 2 {
                    Self.move_now(to_x, to_y, board);
                    return true; //No error TODO fix with rust things
                } else {
                    return false;
                }
            }
        }

        false
    }

    fn move_rook (to_x: isize, to_y: isize, board: &Board) -> bool{
        if !board.is_empty_tile && board.is_piece_white == Self.is_white {
            return false;
        }
        
        if to_x == Self.pos_x && to_y != Self.pos_y{
            for i in Self.pos_x..to_x {
                if i == Self.pos_x {
                    continue;
                } else {
                    if !board.is_empty_tile(to_x, i) {
                        return false;
                    }
                }
            }
            move_now(to_x, to_y, board);
            return true;
        }

        if to_y == Self.pos_y && to_x != Self.pos_x{
            for i in Self.pos_y..to_y {
                if i == Self.pos_y {
                    continue;
                } else {
                    if !board.is_empty_tile(i, to_y) {
                        return false;
                    }
                }
            }
            move_now(to_x, to_y, board);
            return true;
        }

        false
    }

    fn move_knight (to_x: isize, to_y: isize, board: &Board) -> bool{
        //TODO implement
    }
    fn move_bishop (to_x: isize, to_y: isize, board: &Board) -> bool{
        
    }
    fn move_queen (to_x: isize, to_y: isize, board: &Board) -> bool{
        //TODO implement
    }
    fn move_king (to_x: isize, to_y: isize, board: &Board) -> bool{
        //TODO implement
    }
}