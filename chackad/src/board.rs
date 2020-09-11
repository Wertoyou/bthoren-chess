mod tile {
    pub enum Color {
        Black,
        White,
    }

    pub struct Tile {
        pub color: Color,

        pub pos_x: usize,
        pub pos_y: usize,
        pub is_empty: bool,
        pub piece_is_white: bool,
    }
}

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
                    };
                } else {
                    next = tile::Tile {
                        color: tile::Color::Black,
                        pos_x: i,
                        pos_y: j,
                        is_empty: true,
                        piece_is_white: false,
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

    pub fn is_valid_tile(pos_x: isize, pos_y: isize) -> bool {
        if pos_x >= Self.size_x || pos_y >= Self.size_y || pos_x < 0 || pos_y < 0 {
            return false;
        }else {
            return true;
        }
    }

    pub fn is_empty_tile(pos_x: usize, pos_y: usize) -> bool {
        Self.tiles[pos_x][pos_y].is_empty
    }

    fn set_emptines(pos_x: usize, pos_y: usize, to_empty: bool) {
        Self.tiles[pos_x][pos_y].is_empty = to_empty;
    }

    fn set_piece_is_white(pos_x: usize, pos_y: usize, is_white: bool) {
        Self.tiles[pos_x][pos_y].piece_is_white = is_white;
    }

    fn is_piece_white(pos_x: usize, pos_y: usize) -> bool {
        Self.tiles[pos_x][pos_y].piece_is_white
    }
}
