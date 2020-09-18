mod tile {
    #[derive(Clone)]
    pub enum Color {
        Black,
        White,
    }

    #[derive(Clone)]
    pub struct Tile {
        pub color: Color,

        pub pos_x: usize,
        pub pos_y: usize,
        pub is_empty: bool,
        pub piece_is_white: bool,
        pub has_orig_piece: bool,
    }
}

#[derive(Clone)]
pub struct Board {
    tiles: Vec<Vec<tile::Tile>>,
    pub size_x: usize,
    pub size_y: usize,
}

impl Board {
    pub fn new(size_x: usize, size_y: usize) -> Board {
        if size_x <= 0 || size_y <= 0 {
            panic!("Size of board must be positive integers");
        }

        if size_x > 1_000_000 || size_y > 1_000_000 {
            panic!("Size of board must be below 1'000'000");
        }

        let mut tiles: Vec<Vec<tile::Tile>> = Vec::new();

        for i in 0..size_y {
            tiles.push(Vec::new());
            for j in 0..size_x {
                let next;
                if (i + j) % 2 == 0 {
                    next = tile::Tile {
                        color: tile::Color::White,
                        pos_x: i,
                        pos_y: j,
                        is_empty: true,
                        piece_is_white: false,
                        has_orig_piece: true,
                    };
                } else {
                    next = tile::Tile {
                        color: tile::Color::Black,
                        pos_x: i,
                        pos_y: j,
                        is_empty: true,
                        piece_is_white: false,
                        has_orig_piece: true,
                    };
                }

                tiles[i].push(next);
            }
        }

        let tiles = tiles;
        Board {
            tiles: tiles,
            size_x: size_x,
            size_y: size_y,
        }
    }

    pub fn is_valid_tile(&self, pos_x: usize, pos_y: usize) -> bool {
        if pos_x >= self.size_x || pos_y >= self.size_y {
            return false;
        } else {
            return true;
        }
    }

    pub fn is_empty_tile(&self, pos_x: usize, pos_y: usize) -> bool {
        self.tiles[pos_x][pos_y].is_empty
    }

    pub fn set_emptiness(&mut self, pos_x: usize, pos_y: usize, to_empty: bool) {
        self.tiles[pos_x][pos_y].is_empty = to_empty;
    }

    pub fn set_piece_is_white(&mut self, pos_x: usize, pos_y: usize, is_white: bool) {
        self.tiles[pos_x][pos_y].piece_is_white = is_white;
    }

    pub fn is_piece_white(&self, pos_x: usize, pos_y: usize) -> bool {
        self.tiles[pos_x][pos_y].piece_is_white
    }

    pub fn false_orig_piece(&mut self, x: usize, y: usize) {
        self.tiles[x][y].has_orig_piece = false;
    }

    pub fn check_orig(&self, x: usize, y: usize) -> bool {
        self.tiles[x][y].has_orig_piece
    }
}
