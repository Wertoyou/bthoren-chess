use std::collections::HashSet;

mod board {
    mod tile {
        enum Color {
            Black,
            White,
        }

        #[derive(Eq)]
        struct Tile {
            pub color: Color,

            pub pos_x: u32,
            pub pos_y: u32,
        }
    }

    struct Board {
        tiles: Vec<Vec<Tile> >,
    }

    impl New for Board {
        fn new (&self, size_x: u32, size_y: u32) {
            let mut tiles = Vec::new();
            
            for i in (0..size_y) {
                for j in (0..size_x) {
                    if (i + j) % 2 == 0 {
                        let next: tile::Tile = {color: tile::Color::White, pos_x: i, pos_y: j};
                    } else {
                        let next: tile::Tile = {color: tile::Color::Black, pos_x: i, pos_y: j};
                    }
                    tiles[i].push(next);
                }
                tiles.push(Vec::new());
            }

            let tiles = tiles;
        }
    }
}