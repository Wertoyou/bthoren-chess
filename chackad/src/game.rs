use crate::board::Board;
use crate::piece::Piece;
use crate::piece::PieceType;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Game {
    white_pieces: HashSet<Piece>,
    black_pieces: HashSet<Piece>,
    board: Board,
    all_moves: HashMap<(usize, usize), Vec<(usize, usize, PieceType)>>,
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
        let mut turn = &HashSet::new();

        if self.whites_turn {
            turn = &self.white_pieces;
        } else {
            turn = &self.black_pieces;
        }
        for a in turn.iter() {
            for i in 0..self.board.size_x {
                for j in 0..self.board.size_y {
                    if a.check_to(i, j, &self.board, PieceType::Pawn) {
                        self.all_moves
                            .entry((a.pos_x, a.pos_y))
                            .or_insert_with(Vec::new)
                            .push((i, j, PieceType::Pawn));
                    }
                    if a.check_to(i, j, &self.board, PieceType::Rook) {
                        self.all_moves
                            .entry((a.pos_x, a.pos_y))
                            .or_insert_with(Vec::new)
                            .push((i, j, PieceType::Rook));
                    }
                    if a.check_to(i, j, &self.board, PieceType::Knight) {
                        self.all_moves
                            .entry((a.pos_x, a.pos_y))
                            .or_insert_with(Vec::new)
                            .push((i, j, PieceType::Knight));
                    }
                    if a.check_to(i, j, &self.board, PieceType::Bishop) {
                        self.all_moves
                            .entry((a.pos_x, a.pos_y))
                            .or_insert_with(Vec::new)
                            .push((i, j, PieceType::Bishop));
                    }
                    if a.check_to(i, j, &self.board, PieceType::Queen) {
                        self.all_moves
                            .entry((a.pos_x, a.pos_y))
                            .or_insert_with(Vec::new)
                            .push((i, j, PieceType::Queen));
                    }
                    if a.check_to(i, j, &self.board, PieceType::King) {
                        //TODO fix rockad check check
                        self.all_moves
                            .entry((a.pos_x, a.pos_y))
                            .or_insert_with(Vec::new)
                            .push((i, j, PieceType::King));
                    }
                }
            }
        }

        self.all_moves = self.remove_checks();
    }

    fn remove_checks(&self) -> HashMap<(usize, usize), Vec<(usize, usize, PieceType)>> {
        let mut turn = &HashSet::new();
        let mut turn_next = &HashSet::new();

        if self.whites_turn {
            turn = &self.white_pieces;
            turn_next = &self.black_pieces;
        } else {
            turn = &self.black_pieces;
            turn_next = &self.white_pieces;
        }

        let mut out: HashMap<(usize, usize), Vec<(usize, usize, PieceType)>> = HashMap::new();

        for a in turn.iter() {
            for b in self.all_moves.iter() {
                for f in b.1.iter() {
                    let mut new_board = self.board.clone();
                    let mut c = a.clone();
                    c.move_to((f).0, (f).1, &mut new_board, (f).2);
                    let mut king_pos_x: usize = 0;
                    let mut king_pos_y: usize = 0;

                    if c.piece_type == PieceType::King {
                        king_pos_x = (f).0;
                        king_pos_y = (f).1;
                    } else {
                        for e in turn.iter() {
                            if e.piece_type == PieceType::King {
                                king_pos_x = e.pos_x;
                                king_pos_y = e.pos_y;
                            }
                        }
                    }

                    for d in turn_next.iter() {
                        if d.check_to(king_pos_x, king_pos_y, &self.board, PieceType::Pawn) {
                            continue;
                        }
                        if a.check_to(king_pos_x, king_pos_y, &self.board, PieceType::Rook) {
                            continue;
                        }
                        if a.check_to(king_pos_x, king_pos_y, &self.board, PieceType::Knight) {
                            continue;
                        }
                        if a.check_to(king_pos_x, king_pos_y, &self.board, PieceType::Bishop) {
                            continue;
                        }
                        if a.check_to(king_pos_x, king_pos_y, &self.board, PieceType::Queen) {
                            continue;
                        }
                        if a.check_to(king_pos_x, king_pos_y, &self.board, PieceType::King) {
                            continue;
                        }
                        out.entry(b.0.clone())
                            .or_insert_with(Vec::new)
                            .push(f.clone());
                    }
                }
            }
        }

        out
    }

    fn move_now(&mut self, from: &(usize, usize), to: &(usize, usize, PieceType)) {
        if self.whites_turn {
            let muttable_white = self.white_pieces.clone();
            self.white_pieces.clear();
            for mut a in muttable_white {
                if a.pos_x == from.0 && a.pos_y == from.1 {
                    let muttable_black = self.black_pieces.clone();
                    self.black_pieces.clear();
                    a.move_to(to.0, to.1, &mut self.board, to.2);
                    for b in muttable_black {
                        if b.pos_x == to.0 && b.pos_y == to.1 {
                            continue;
                        }
                        self.black_pieces.insert(b);
                    }
                }
                self.white_pieces.insert(a);
            }
        } else {
            let muttable_black = self.black_pieces.clone();
            self.black_pieces.clear();
            for mut a in muttable_black {
                if a.pos_x == from.0 && a.pos_y == from.1 {
                    let muttable_white = self.white_pieces.clone();
                    self.white_pieces.clear();
                    a.move_to(to.0, to.1, &mut self.board, to.2);
                    for b in muttable_white {
                        if b.pos_x == to.0 && b.pos_y == to.1 {
                            continue;
                        }
                        self.white_pieces.insert(b);
                    }
                }
                self.black_pieces.insert(a);
            }
        }
    }

    pub fn next(&mut self, from: (usize, usize), to: (usize, usize, PieceType)) {
        self.calc_all_moves();
        if self.all_moves.contains_key(&from) {
            if self
                .all_moves
                .entry(from)
                .or_insert_with(Vec::new) // Will never happen
                .contains(&to)
            {
                self.move_now(&from, &to);
            }
        }
    }
}
