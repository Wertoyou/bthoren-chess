use crate::board::Board;
use crate::piece::Piece;
use crate::piece::PieceType;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Game {
    all_pieces: HashSet<Piece>,
    board: Board,
    all_moves: HashMap<(usize, usize), (usize, usize)>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            all_pieces: HashSet::new(),
            board: Board::new(8, 8),
            all_moves: HashMap::new(),
        }
    }

    pub fn regular_chess_setup(&mut self) {
        self.board = Board::new(8, 8);

        // White pieces
        for i in 0..8 {
            self.all_pieces
                .insert(Piece::new(&mut self.board, PieceType::Pawn, i, 1, true));
        }
        self.all_pieces
            .insert(Piece::new(&mut self.board, PieceType::Rook, 0, 0, true));
        self.all_pieces
            .insert(Piece::new(&mut self.board, PieceType::Knight, 1, 0, true));
        self.all_pieces
            .insert(Piece::new(&mut self.board, PieceType::Bishop, 2, 0, true));
        self.all_pieces
            .insert(Piece::new(&mut self.board, PieceType::Queen, 3, 0, true));
        self.all_pieces
            .insert(Piece::new(&mut self.board, PieceType::King, 4, 0, true));
        self.all_pieces
            .insert(Piece::new(&mut self.board, PieceType::Bishop, 5, 0, true));
        self.all_pieces
            .insert(Piece::new(&mut self.board, PieceType::Knight, 6, 0, true));
        self.all_pieces
            .insert(Piece::new(&mut self.board, PieceType::Rook, 7, 0, true));

        // Black pieces
        for i in 0..8 {
            self.all_pieces
                .insert(Piece::new(&mut self.board, PieceType::Pawn, i, 6, false));
        }
        self.all_pieces
            .insert(Piece::new(&mut self.board, PieceType::Rook, 0, 7, false));
        self.all_pieces
            .insert(Piece::new(&mut self.board, PieceType::Knight, 1, 7, false));
        self.all_pieces
            .insert(Piece::new(&mut self.board, PieceType::Bishop, 2, 7, false));
        self.all_pieces
            .insert(Piece::new(&mut self.board, PieceType::Queen, 3, 7, false));
        self.all_pieces
            .insert(Piece::new(&mut self.board, PieceType::King, 4, 7, false));
        self.all_pieces
            .insert(Piece::new(&mut self.board, PieceType::Bishop, 5, 7, false));
        self.all_pieces
            .insert(Piece::new(&mut self.board, PieceType::Knight, 6, 7, false));
        self.all_pieces
            .insert(Piece::new(&mut self.board, PieceType::Rook, 7, 7, false));
    }

    fn calc_all_moves(&mut self) {
        self.all_moves = HashMap::new();
        for a in self.all_pieces.iter() {
            for i in 0..self.board.size_x {
                for j in 0..self.board.size_y {
                    if a.check_to(i, j, &self.board, PieceType::Queen) {
                        self.all_moves.insert((a.pos_x, a.pos_y), (i, j));
                    }
                }
            }
        }
    }
}
