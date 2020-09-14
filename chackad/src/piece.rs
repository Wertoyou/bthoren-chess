// TODO create better logic for seperating White/Black case for pawns (now it is almost duplicate code)
use crate::board::Board;

#[derive(PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

pub struct Piece {
    piece_type: PieceType,
    pos_x: usize,
    pos_y: usize,
    is_white: bool,
}

impl Piece {
    pub fn new(
        board: &mut Board,
        piece_type: PieceType,
        pos_x: usize,
        pos_y: usize,
        is_white: bool,
    ) -> Piece {
        board.set_emptiness(pos_x, pos_y, false);
        Piece {
            piece_type: piece_type,
            pos_x: pos_x,
            pos_y: pos_y,
            is_white: is_white,
        }
    }

    fn move_now(&mut self, to_x: usize, to_y: usize, board: &mut Board) {
        board.set_emptiness(to_x, to_y, false);
        board.set_emptiness(self.pos_x, self.pos_y, true);
        self.pos_x = to_x;
        self.pos_y = to_y;
        board.set_piece_is_white(to_x, to_y, self.is_white);
    }

    pub fn move_to(
        &mut self,
        to_x: usize,
        to_y: usize,
        board: &mut Board,
        promotion: PieceType,
    ) -> bool {
        if board.is_valid_tile(to_x, to_y) {
            return false;
        }
        match &self.piece_type {
            PieceType::Pawn => return self.move_pawn(to_x, to_y, board, promotion),
            PieceType::Rook => return self.move_rook(to_x, to_y, board),
            PieceType::Knight => return self.move_knight(to_x, to_y, board),
            PieceType::Bishop => return self.move_bishop(to_x, to_y, board),
            PieceType::Queen => return self.move_queen(to_x, to_y, board),
            PieceType::King => return self.move_king(to_x, to_y, board),
        }
    }
    fn move_pawn(
        &mut self,
        to_x: usize,
        to_y: usize,
        board: &mut Board,
        promotion: PieceType,
    ) -> bool {
        if self.is_white {
            // Regular move one step forward (including promotion)
            if to_x == self.pos_x && to_y == self.pos_y + 1 {
                if board.is_empty_tile(to_x, to_y) && to_y == board.size_y - 1 {
                    if promotion == PieceType::Pawn || promotion == PieceType::King {
                        return false;
                    }
                    self.piece_type = promotion;
                    self.move_now(to_x, to_y, board);
                    return true;
                } else if board.is_empty_tile(to_x, to_y) {
                    self.move_now(to_x, to_y, board);
                    return true;
                } else {
                    return false;
                }
            }
            // Capture move
            if to_y == self.pos_y + 1 && (to_x == self.pos_x - 1 || to_x == self.pos_x + 1) {
                if !board.is_empty_tile(to_x, to_y) && !board.is_piece_white(to_x, to_y) {
                    self.move_now(to_x, to_y, board);
                    return true;
                } else {
                    return false;
                }
            }

            // Double move
            if to_x == self.pos_x && to_y == self.pos_y + 2 {
                if board.is_empty_tile(to_x, to_y)
                    && board.is_empty_tile(to_x, to_y - 1)
                    && self.pos_y == 1
                {
                    self.move_now(to_x, to_y, board);
                    return true;
                } else {
                    return false;
                }
            }
        } else {
            // Regular move one step forward (including promotion)
            if to_x == self.pos_x && to_y == self.pos_y - 1 {
                if board.is_empty_tile(to_x, to_y) && to_y == 0 {
                    if promotion == PieceType::Pawn || promotion == PieceType::King {
                        return false;
                    }
                    self.piece_type = promotion;
                    self.move_now(to_x, to_y, board);
                    return true;
                } else if board.is_empty_tile(to_x, to_y) {
                    self.move_now(to_x, to_y, board);
                    return true;
                } else {
                    return false;
                }
            }
            // Capture move
            if to_y == self.pos_y - 1 && (to_x == self.pos_x - 1 || to_x == self.pos_x + 1) {
                if !board.is_empty_tile(to_x, to_y) && board.is_piece_white(to_x, to_y) {
                    self.move_now(to_x, to_y, board);
                    return true; //No error TODO fix with rust things
                } else {
                    return false;
                }
            }

            // Double move
            if to_x == self.pos_x && to_y == self.pos_y - 2 {
                if board.is_empty_tile(to_x, to_y)
                    && board.is_empty_tile(to_x, to_y - 1)
                    && self.pos_y == board.size_y - 2
                {
                    self.move_now(to_x, to_y, board);
                    return true; //No error TODO fix with rust things
                } else {
                    return false;
                }
            }
        }

        false
    }

    fn move_rook(&mut self, to_x: usize, to_y: usize, board: &mut Board) -> bool {
        if !board.is_empty_tile(to_x, to_y) && board.is_piece_white(to_x, to_y) == self.is_white {
            return false;
        }

        if to_x == self.pos_x && to_y != self.pos_y {
            for i in self.pos_x..to_x {
                if i == self.pos_x {
                    continue;
                } else {
                    if !board.is_empty_tile(to_x, i) {
                        return false;
                    }
                }
            }
            self.move_now(to_x, to_y, board);
            return true;
        }

        if to_y == self.pos_y && to_x != self.pos_x {
            for i in self.pos_y..to_y {
                if i == self.pos_y {
                    continue;
                } else {
                    if !board.is_empty_tile(i, to_y) {
                        return false;
                    }
                }
            }
            self.move_now(to_x, to_y, board);
            return true;
        }

        false
    }

    fn move_knight(&mut self, to_x: usize, to_y: usize, board: &mut Board) -> bool {
        //TODO implement
        false
    }
    fn move_bishop(&mut self, to_x: usize, to_y: usize, board: &mut Board) -> bool {
        //TODO implement
        false
    }
    fn move_queen(&mut self, to_x: usize, to_y: usize, board: &mut Board) -> bool {
        //TODO implement
        false
    }
    fn move_king(&mut self, to_x: usize, to_y: usize, board: &mut Board) -> bool {
        //TODO implement
        false
    }
}
