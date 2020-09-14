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

    pub fn get_coords_from_string(s: String) -> (usize, usize) {
        if s.len() > 2 {
            panic!("string with only two caracters supported");
        }

        let c: char = s.chars().nth(0).unwrap();
        let n: char = s.chars().nth(1).unwrap();

        if !((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) {
            panic!("first character in string must be a-z");
        }

        if !(n >= '1' && n <= '9') {
            panic!("second character must be 1-9");
        }

        if c >= 'a' && c <= 'z' {
            return (c as usize - 'a' as usize, n as usize - '1' as usize);
        } else {
            return (c as usize - 'A' as usize, n as usize - '1' as usize);
        }
    }

    pub fn get_string_from_coords(x: usize, y: usize) -> String {
        format!(
            "{}{}",
            (x + 'a' as usize) as u8 as char,
            (y + '1' as usize) as u8 as char
        )
    }
}
