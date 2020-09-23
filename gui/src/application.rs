use chackad::game::Game;
use piston_window::*;

pub struct Application {
    // Rendering
    chessboard_texture: G2dTexture,
    piece_textures: Vec<G2dTexture>,

    // State
    game: Game,
    selected_tile: Option<[usize; 2]>,

    // Input
    left_mouse_pressed: bool,
    right_mouse_pressed: bool,
    current_mouse_pos: [f64; 2],
}

impl Application {
    pub fn new(texture_context: &mut G2dTextureContext) -> Self {
        let chessboard = Texture::from_path(
            texture_context,
            "resources/chessboard.png",
            Flip::None,
            &TextureSettings::new(),
        )
        .unwrap();

        const PIECE_TEXTURE_PATHS: [&str; 12] = [
            "resources/pieces/white_pawn.png",
            "resources/pieces/white_rook.png",
            "resources/pieces/white_knight.png",
            "resources/pieces/white_bishop.png",
            "resources/pieces/white_queen.png",
            "resources/pieces/white_king.png",
            "resources/pieces/black_pawn.png",
            "resources/pieces/black_rook.png",
            "resources/pieces/black_knight.png",
            "resources/pieces/black_bishop.png",
            "resources/pieces/black_queen.png",
            "resources/pieces/black_king.png",
        ];

        let piece_textures: Vec<Texture<_>> = PIECE_TEXTURE_PATHS
            .iter()
            .map(|path| {
                Texture::from_path(texture_context, path, Flip::None, &TextureSettings::new())
                    .unwrap()
            })
            .collect();

        let mut game = Game::new();
        game.regular_chess_setup();

        Self {
            chessboard_texture: chessboard,
            piece_textures: piece_textures,
            game: game,
            selected_tile: None,
            current_mouse_pos: [0.0, 0.0],
            left_mouse_pressed: false,
            right_mouse_pressed: false,
        }
    }

    pub fn run(&mut self, window: &mut PistonWindow) {
        while let Some(event) = window.next() {
            self.update(&event);
            self.draw(window, &event);
        }
    }

    fn draw(&mut self, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |context, graphics, _device| {
            clear([0.1, 0.2, 0.3, 1.0], graphics);
            image(&self.chessboard_texture, context.transform, graphics);

            if let Some(tile) = self.selected_tile {
                rectangle(
                    [0.0, 0.7, 0.0, 0.7],
                    [125.0 * tile[0] as f64, 125.0 * tile[1] as f64, 125.0, 125.0],
                    context.transform,
                    graphics,
                );
            }

            let scale = context.transform.scale(0.625, 0.625);

            for piece in self.game.white_pieces_iter() {
                let texture = &self.piece_textures[piece.piece_type as usize];
                let transform = scale.trans(piece.pos_x as f64 * 200.0, piece.pos_y as f64 * 200.0);
                image(texture, transform, graphics);
            }

            for piece in self.game.black_pieces_iter() {
                let texture = &self.piece_textures[piece.piece_type as usize + 6];
                let transform = scale.trans(piece.pos_x as f64 * 200.0, piece.pos_y as f64 * 200.0);
                image(texture, transform, graphics);
            }

            if let Some(tile) = self.selected_tile {
                if let Some(moves) = self.game.moves_from((tile[0], tile[1])) {
                    for (x, y, _) in moves {
                        ellipse(
                            [0.0, 0.7, 0.0, 0.7],
                            [
                                125.0 * *x as f64 + 31.25,
                                125.0 * *y as f64 + 31.25,
                                62.5,
                                62.5,
                            ],
                            context.transform,
                            graphics,
                        );
                    }
                }
            }
        });
    }

    fn update(&mut self, event: &Event) {
        if let Some(pos) = event.mouse_cursor_args() {
            self.current_mouse_pos = pos;
        }

        if let Some(button) = event.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                if self.left_mouse_pressed == false {
                    if self.current_mouse_pos[0] <= 1000.0 {
                        let x = (self.current_mouse_pos[0] / 125.0) as usize;
                        let y = (self.current_mouse_pos[1] / 125.0) as usize;

                        let board = self.game.board();

                        if !board.is_empty_tile(x, y) {
                            println!("Selected: {}, {}", x, y);
                            let white = self.game.is_whites_turn();
                            if white && board.is_piece_white(x, y) {
                                self.selected_tile = Some([x, y]);
                            }

                            if !white && !board.is_piece_white(x, y) {
                                self.selected_tile = Some([x, y]);
                            }
                        }
                    }
                }
                self.left_mouse_pressed = true;
            }

            if button == Button::Mouse(MouseButton::Right) {
                if self.right_mouse_pressed == false {
                    self.selected_tile = None;
                }

                self.right_mouse_pressed = true;
            }
        }

        if let Some(button) = event.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                self.left_mouse_pressed = false;
            }
            if button == Button::Mouse(MouseButton::Right) {
                self.right_mouse_pressed = false;
            }
        }
    }
}
