mod board;
mod game;
mod piece;

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::game::Game;
    use crate::piece;

    #[test]
    fn board_size_x_valid() {
        let board: Board = Board::new(8, 1);
        assert_eq!(board.size_x, 8);
        let board: Board = Board::new(1, 1);
        assert_eq!(board.size_x, 1);
        let board: Board = Board::new(1000, 1);
        assert_eq!(board.size_x, 1000);
    }

    #[test]
    #[should_panic]
    fn board_size_x_invalid() {
        let _board: Board = Board::new(0, 1);
    }

    #[test]
    fn board_size_y_valid() {
        let board: Board = Board::new(1, 8);
        assert_eq!(board.size_y, 8);
        let board: Board = Board::new(1, 1);
        assert_eq!(board.size_y, 1);
        let board: Board = Board::new(1, 1000);
        assert_eq!(board.size_y, 1000);
    }

    #[test]
    #[should_panic]
    fn board_size_y_invalid() {
        let _board: Board = Board::new(1, 0);
    }

    #[test]
    fn test_pawn_forward() {
        let mut board = Board::new(8, 8);
        let mut pawn = piece::Piece::new(&mut board, piece::PieceType::Pawn, 0, 1, true);
        pawn.check_to(0, 2, &mut board, piece::PieceType::Queen);
    }

    #[test]
    fn test_pawn_forward2() {
        let mut board = Board::new(8, 8);
        let mut pawn = piece::Piece::new(&mut board, piece::PieceType::Pawn, 0, 1, true);
        pawn.check_to(0, 3, &mut board, piece::PieceType::Queen);
    }

    #[test]
    fn test_pawn_capture() {
        let mut board = Board::new(8, 8);
        let mut pawn1 = piece::Piece::new(&mut board, piece::PieceType::Pawn, 0, 1, true);
        let mut pawn2 = piece::Piece::new(&mut board, piece::PieceType::Pawn, 1, 2, true);
        pawn1.check_to(1, 2, &mut board, piece::PieceType::Queen);
    }

    #[test]
    fn test_pawn_promotion() {
        let mut board = Board::new(8, 8);
        let mut pawn1 = piece::Piece::new(&mut board, piece::PieceType::Pawn, 0, 6, true);
        pawn1.check_to(0, 7, &mut board, piece::PieceType::Queen);
    }

    #[test]
    fn create_regular_game() {
        let mut game = Game::new();
        game.regular_chess_setup();
    }

    #[test]
    fn make_moves_in_game() {
        let mut game = Game::new();
        game.regular_chess_setup();
    }
}
