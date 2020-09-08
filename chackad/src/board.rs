mod tile {
    pub enum Color {
        Black,
        White,
    }

    pub struct Tile {
        pub color: Color,

        pub pos_x: usize,
        pub pos_y: usize,
    }
}

pub struct Board {
    tiles: Vec<Vec<tile::Tile> >,
    pub size_x: usize,
    pub size_y: usize,
}

impl Board {
    pub fn new (size_x: usize, size_y: usize) -> Board {
        
        if size_x == 0 || size_y == 0 {
            panic!("Size of board must be positive integers");
        }

        let mut tiles: Vec<Vec<tile::Tile>> = Vec::new();
        
        for i in 0..size_y {
            tiles.push(Vec::new());
            for j in 0..size_x {
                let next;
                if (i + j) % 2 == 0 {
                    next = tile::Tile {color: tile::Color::White, pos_x: i, pos_y: j};
                } else {
                    next = tile::Tile {color: tile::Color::Black, pos_x: i, pos_y: j};
                }

                tiles[i].push(next);
            }
        }

        let tiles = tiles;
        Board {tiles: tiles, size_x: size_x, size_y: size_y}
    }
}