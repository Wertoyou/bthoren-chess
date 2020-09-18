// TODO create better logic for seperating White/Black case for pawns (now it is almost duplicate code)
use crate::board::Board;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub pos_x: usize,
    pub pos_y: usize,
    pub is_white: bool,
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

    pub fn move_to(&mut self, to_x: usize, to_y: usize, board: &mut Board, promotion: PieceType) {
        board.set_emptiness(to_x, to_y, false);
        board.set_emptiness(self.pos_x, self.pos_y, true);
        board.false_orig_piece(self.pos_x, self.pos_y);
        board.false_orig_piece(to_x, to_y);
        self.pos_x = to_x;
        self.pos_y = to_y;
        self.piece_type = promotion;
        board.set_piece_is_white(to_x, to_y, self.is_white);
    }

    pub fn check_to(&self, to_x: usize, to_y: usize, board: &Board, promotion: PieceType) -> bool {
        if !board.is_valid_tile(to_x, to_y) {
            return false;
        }
        match &self.piece_type {
            PieceType::Pawn => return self.check_pawn(to_x, to_y, board, promotion),
            PieceType::Rook => return self.check_rook(to_x, to_y, board, promotion),
            PieceType::Knight => return self.check_knight(to_x, to_y, board, promotion),
            PieceType::Bishop => return self.check_bishop(to_x, to_y, board, promotion),
            PieceType::Queen => return self.check_queen(to_x, to_y, board, promotion),
            PieceType::King => return self.check_king(to_x, to_y, board, promotion),
        }
    }
    fn check_pawn(&self, to_x: usize, to_y: usize, board: &Board, promotion: PieceType) -> bool {
        if self.is_white {
            // Regular move one step forward (including promotion)
            if to_x == self.pos_x && to_y == self.pos_y + 1 {
                if board.is_empty_tile(to_x, to_y) && to_y == board.size_y - 1 {
                    if promotion == PieceType::Pawn || promotion == PieceType::King {
                        return false;
                    }
                    return true;
                } else if board.is_empty_tile(to_x, to_y) && promotion == PieceType::Pawn {
                    return true;
                } else {
                    return false;
                }
            }

            if promotion != PieceType::Pawn {
                return false;
            }

            // Capture move
            if to_y == self.pos_y + 1 && (to_x + 1 == self.pos_x || to_x == self.pos_x + 1) {
                if !board.is_empty_tile(to_x, to_y) && !board.is_piece_white(to_x, to_y) {
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
                    return true;
                } else {
                    return false;
                }
            }
        } else {
            // Regular move one step forward (including promotion)
            if to_x == self.pos_x && to_y + 1 == self.pos_y {
                if board.is_empty_tile(to_x, to_y) && to_y == 0 {
                    if promotion == PieceType::Pawn || promotion == PieceType::King {
                        return false;
                    }
                    return true;
                } else if board.is_empty_tile(to_x, to_y) && promotion == PieceType::Pawn {
                    return true;
                } else {
                    return false;
                }
            }

            if promotion != PieceType::Pawn {
                return false;
            }

            // Capture move
            if to_y + 1 == self.pos_y && (to_x + 1 == self.pos_x || to_x == self.pos_x + 1) {
                if !board.is_empty_tile(to_x, to_y) && board.is_piece_white(to_x, to_y) {
                    return true; //No error TODO fix with rust things
                } else {
                    return false;
                }
            }

            // Double move
            if to_x == self.pos_x && to_y + 2 == self.pos_y {
                if board.is_empty_tile(to_x, to_y)
                    && board.is_empty_tile(to_x, to_y - 1)
                    && self.pos_y == board.size_y - 2
                {
                    return true; //No error TODO fix with rust things
                } else {
                    return false;
                }
            }
        }

        false
    }

    fn check_rook(&self, to_x: usize, to_y: usize, board: &Board, promotion: PieceType) -> bool {
        if promotion != PieceType::Rook {
            return false;
        }
        if !board.is_empty_tile(to_x, to_y) && board.is_piece_white(to_x, to_y) == self.is_white {
            return false;
        }

        if to_x == self.pos_x && to_y != self.pos_y {
            for i in self.pos_y..to_y {
                if i == self.pos_y {
                    continue;
                } else {
                    if !board.is_empty_tile(to_x, i) {
                        return false;
                    }
                }
            }
            return true;
        }

        if to_y == self.pos_y && to_x != self.pos_x {
            for i in self.pos_x..to_x {
                if i == self.pos_x {
                    continue;
                } else {
                    if !board.is_empty_tile(i, to_y) {
                        return false;
                    }
                }
            }
            return true;
        }

        false
    }

    fn check_knight(&self, to_x: usize, to_y: usize, board: &Board, promotion: PieceType) -> bool {
        if promotion != PieceType::Knight {
            return false;
        }
        if !board.is_empty_tile(to_x, to_y) && !board.is_piece_white(to_x, to_y) == self.is_white {
            if (to_x as i128 - self.pos_x as i128).abs() == 2
                && (to_y as i128 - self.pos_y as i128).abs() == 1
            {
                return true;
            }
            if (to_y as i128 - self.pos_y as i128).abs() == 2
                && (to_x as i128 - self.pos_x as i128).abs() == 1
            {
                return true;
            }
        }
        false
    }
    fn check_bishop(&self, to_x: usize, to_y: usize, board: &Board, promotion: PieceType) -> bool {
        if promotion != PieceType::Bishop {
            return false;
        }
        if !board.is_empty_tile(to_x, to_y) && board.is_piece_white(to_x, to_y) == self.is_white {
            return false;
        }

        if (to_x as i128 - self.pos_x as i128) != (to_y as i128 - self.pos_y as i128) {
            return false;
        }

        if self.pos_x != to_x && self.pos_y != to_y {
            for (i, j) in (self.pos_x..to_x).zip(self.pos_y..to_y) {
                if i == self.pos_x && j == self.pos_y {
                    continue;
                } else {
                    if !board.is_empty_tile(i, j) {
                        return false;
                    }
                }
            }
            return true;
        }

        false
    }

    fn check_queen(&self, to_x: usize, to_y: usize, board: &Board, promotion: PieceType) -> bool {
        if promotion != PieceType::Queen {
            return false;
        }
        if self.check_bishop(to_x, to_y, board, PieceType::Bishop)
            || self.check_rook(to_x, to_y, board, PieceType::Rook)
        {
            return true;
        }
        false
    }
    fn check_king(&self, to_x: usize, to_y: usize, board: &Board, promotion: PieceType) -> bool {
        if promotion != PieceType::King {
            return false;
        }
        if !board.is_empty_tile(to_x, to_y) && board.is_piece_white(to_x, to_y) == self.is_white {
            return false;
        }
        if (to_x as i128 - self.pos_x as i128).abs() == 1
            || (to_y as i128 - self.pos_y as i128).abs() == 1
        {
            return true;
        }
        if to_x == self.pos_x + 2
            && to_y == self.pos_y
            && board.check_orig(self.pos_x, self.pos_y)
            && board.check_orig(board.size_x - 1, to_y)
        {
            return true;
        }
        if to_x == self.pos_x - 2
            && to_y == self.pos_y
            && board.check_orig(self.pos_x, self.pos_y)
            && board.check_orig(0, to_y)
        {
            return true;
        }
        false
    }
}
