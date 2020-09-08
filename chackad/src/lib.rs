mod board;

#[cfg(test)]
mod tests {
    use crate::board::Board;

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
}
