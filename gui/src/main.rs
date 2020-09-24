mod move_button;
use move_button::MoveButton;

use ggez;
use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::input::*;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use chackad::game::Game;
use chackad::piece::{Piece, PieceType};

struct Assets {
    chessboard: graphics::Image,
    pieces: Vec<graphics::Image>,
    checkmate_text: graphics::Text,
    checkmate_draw_param: graphics::DrawParam,
}

impl Assets {
    fn new(ctx: &mut Context, board_size: f32) -> GameResult<Assets> {
        let chessboard = graphics::Image::new(ctx, "/chessboard.png")?;
        const PIECE_TEXTURE_PATHS: [&str; 12] = [
            "/pieces/white_pawn.png",
            "/pieces/white_rook.png",
            "/pieces/white_knight.png",
            "/pieces/white_bishop.png",
            "/pieces/white_queen.png",
            "/pieces/white_king.png",
            "/pieces/black_pawn.png",
            "/pieces/black_rook.png",
            "/pieces/black_knight.png",
            "/pieces/black_bishop.png",
            "/pieces/black_queen.png",
            "/pieces/black_king.png",
        ];

        let pieces: Vec<graphics::Image> = PIECE_TEXTURE_PATHS
            .iter()
            .map(|path| graphics::Image::new(ctx, path).unwrap())
            .collect();

        let mut checkmate_text = graphics::Text::new("Checkmate");
        checkmate_text.set_font(
            graphics::Font::default(),
            graphics::Scale { x: 40.0, y: 40.0 },
        );

        let checkmate_draw_param =
            graphics::DrawParam::new()
                .color(graphics::BLACK)
                .dest(na::Point2::new(
                    board_size * 0.5 - checkmate_text.width(ctx) as f32 * 0.5,
                    board_size * 0.5 - checkmate_text.height(ctx) as f32 * 0.5,
                ));

        Ok(Assets {
            chessboard,
            pieces,
            checkmate_text,
            checkmate_draw_param,
        })
    }

    fn piece_image(&self, white: bool, piece: PieceType) -> &graphics::Image {
        let offset = if white { 0 } else { 6 };
        &self.pieces[piece as usize + offset]
    }
}

struct GameState {
    game: Game,
    selected_tile: Option<(usize, usize)>,
    move_buttons: Vec<MoveButton>,

    assets: Assets,
    // Graphical
    board_size: f32,
    tile_size: f32,
    scale_factor: f32,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let mut game = Game::new();
        game.regular_chess_setup();

        let size = graphics::size(ctx);
        let min = size.0.min(size.1);
        let s = GameState {
            game,
            selected_tile: None,
            move_buttons: Vec::with_capacity(10),

            assets: Assets::new(ctx, min)?,

            board_size: min,
            tile_size: min / 8.0,
            scale_factor: min / 1000.0,
        };
        Ok(s)
    }
}

impl GameState {
    fn select_tile(&mut self, x: usize, y: usize) {
        let board = self.game.board();

        if !board.is_empty_tile(x, y) && self.game.is_whites_turn() == board.is_piece_white(x, y) {
            self.selected_tile = Some((x, y));

            // Add move buttons
            self.move_buttons.clear();
            if let Some(moves) = self.game.moves_from((x, y)) {
                for m in moves {
                    self.move_buttons.push(MoveButton::new((x, y), *m));
                }
            }
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: mouse::MouseButton,
        x: f32,
        y: f32,
    ) {
        if self.game.no_moves() {
            return;
        }

        if button == mouse::MouseButton::Left && x < self.board_size {
            let x = (x / self.tile_size) as usize;
            let y = (y / self.tile_size) as usize;
            for button in &self.move_buttons {
                if button.inside(x, y) {
                    self.game.next(button.from(), button.to());
                    self.selected_tile = None;
                    self.move_buttons.clear();

                    return;
                }
            }
            self.select_tile(x, y);
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let texture_draw_param =
            graphics::DrawParam::new().scale([self.scale_factor, self.scale_factor]);

        // Draw the chessboard
        graphics::draw(ctx, &self.assets.chessboard, texture_draw_param)?;

        // Draw selected tile
        if let Some(tile) = self.selected_tile {
            let rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(
                    tile.0 as f32 * self.tile_size,
                    tile.1 as f32 * self.tile_size,
                    self.tile_size,
                    self.tile_size,
                ),
                graphics::Color::from_rgb_u32(0x89beb3),
            )?;

            graphics::draw(ctx, &rect, graphics::DrawParam::new())?;
        }

        // Draw the pieces
        let mut draw_pieces =
            |iter: std::collections::hash_set::Iter<'_, Piece>, white: bool| -> GameResult {
                for piece in iter {
                    let draw_param = texture_draw_param.dest(na::Point2::new(
                        self.tile_size * piece.pos_x as f32,
                        self.tile_size * piece.pos_y as f32,
                    ));

                    graphics::draw(
                        ctx,
                        self.assets.piece_image(white, piece.piece_type),
                        draw_param,
                    )?;
                }
                Ok(())
            };

        draw_pieces(self.game.white_pieces_iter(), true)?;
        draw_pieces(self.game.black_pieces_iter(), false)?;

        // Draw move buttons
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(self.tile_size * 0.5, self.tile_size * 0.5),
            self.tile_size / 4.0,
            1.0,
            graphics::Color::from_rgba_u32(0x89beb3aa),
        )?;

        for button in &self.move_buttons {
            let pos = button.to_f32();
            graphics::draw(
                ctx,
                &circle,
                graphics::DrawParam::new().dest(na::Point2::new(
                    pos.0 * self.tile_size,
                    pos.1 * self.tile_size,
                )),
            )?;
        }

        if self.game.no_moves() {
            graphics::draw(
                ctx,
                &self.assets.checkmate_text,
                self.assets.checkmate_draw_param,
            )?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        std::path::PathBuf::from("./resources")
    };

    let mode: conf::WindowMode = Default::default();
    let cb = ggez::ContextBuilder::new("chess", "ggez")
        .window_mode(mode.dimensions(1000.0, 1000.0))
        .add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut GameState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
