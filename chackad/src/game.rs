use crate::board::Board;
use crate::piece::Piece;
use crate::piece::PieceType;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Game {
    white_pieces: HashSet<Piece>,
    black_pieces: HashSet<Piece>,
    board: Board,
    all_moves: HashMap<(usize, usize), (usize, usize)>,
    whites_turn: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            white_pieces: HashSet::new(),
            black_pieces: HashSet::new(),
            board: Board::new(8, 8),
            all_moves: HashMap::new(),
            whites_turn: true,
        }
    }

    pub fn regular_chess_setup(&mut self) {
        self.board = Board::new(8, 8);

        // White pieces
        for i in 0..8 {
            self.white_pieces
                .insert(Piece::new(&mut self.board, PieceType::Pawn, i, 1, true));
        }
        self.white_pieces
            .insert(Piece::new(&mut self.board, PieceType::Rook, 0, 0, true));
        self.white_pieces
            .insert(Piece::new(&mut self.board, PieceType::Knight, 1, 0, true));
        self.white_pieces
            .insert(Piece::new(&mut self.board, PieceType::Bishop, 2, 0, true));
        self.white_pieces
            .insert(Piece::new(&mut self.board, PieceType::Queen, 3, 0, true));
        self.white_pieces
            .insert(Piece::new(&mut self.board, PieceType::King, 4, 0, true));
        self.white_pieces
            .insert(Piece::new(&mut self.board, PieceType::Bishop, 5, 0, true));
        self.white_pieces
            .insert(Piece::new(&mut self.board, PieceType::Knight, 6, 0, true));
        self.white_pieces
            .insert(Piece::new(&mut self.board, PieceType::Rook, 7, 0, true));

        // Black pieces
        for i in 0..8 {
            self.black_pieces
                .insert(Piece::new(&mut self.board, PieceType::Pawn, i, 6, false));
        }
        self.black_pieces
            .insert(Piece::new(&mut self.board, PieceType::Rook, 0, 7, false));
        self.black_pieces
            .insert(Piece::new(&mut self.board, PieceType::Knight, 1, 7, false));
        self.black_pieces
            .insert(Piece::new(&mut self.board, PieceType::Bishop, 2, 7, false));
        self.black_pieces
            .insert(Piece::new(&mut self.board, PieceType::Queen, 3, 7, false));
        self.black_pieces
            .insert(Piece::new(&mut self.board, PieceType::King, 4, 7, false));
        self.black_pieces
            .insert(Piece::new(&mut self.board, PieceType::Bishop, 5, 7, false));
        self.black_pieces
            .insert(Piece::new(&mut self.board, PieceType::Knight, 6, 7, false));
        self.black_pieces
            .insert(Piece::new(&mut self.board, PieceType::Rook, 7, 7, false));
    }

    fn calc_all_moves(&mut self) {
        self.all_moves = HashMap::new();
        let mut turn = HashSet::new();

        if self.whites_turn {
            turn = self.white_pieces;
        } else {
            turn = self.black_pieces;
        }
        for a in turn.iter() {
            for i in 0..self.board.size_x {
                for j in 0..self.board.size_y {
                    if a.check_to(i, j, &self.board, PieceType::Pawn) {
                        self.all_moves.insert((a.pos_x, a.pos_y), (i, j));
                    }
                    if a.check_to(i, j, &self.board, PieceType::Rook) {
                        self.all_moves.insert((a.pos_x, a.pos_y), (i, j));
                    }
                    if a.check_to(i, j, &self.board, PieceType::Knight) {
                        self.all_moves.insert((a.pos_x, a.pos_y), (i, j));
                    }
                    if a.check_to(i, j, &self.board, PieceType::Bishop) {
                        self.all_moves.insert((a.pos_x, a.pos_y), (i, j));
                    }
                    if a.check_to(i, j, &self.board, PieceType::Queen) {
                        self.all_moves.insert((a.pos_x, a.pos_y), (i, j));
                    }
                    if a.check_to(i, j, &self.board, PieceType::King) {
                        self.all_moves.insert((a.pos_x, a.pos_y), (i, j));
                    }
                }
            }
        }
    }
}
