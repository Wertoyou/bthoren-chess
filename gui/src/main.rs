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

use std::env;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

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

#[derive(PartialEq)]
enum State {
    Playing,
    WaitingForOpponent,
    GameOver,
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

    state: State,

    // Networking
    next_move: Option<((usize, usize), (usize, usize, PieceType))>,
    stream: Option<TcpStream>,
}

impl GameState {
    fn new(ctx: &mut Context, stream: Option<TcpStream>, client: bool) -> GameResult<GameState> {
        let mut game = Game::new();
        game.regular_chess_setup();

        let state = if stream.is_some() {
            if client {
                State::WaitingForOpponent
            } else {
                State::Playing
            }
        } else {
            State::Playing
        };

        let size = graphics::size(ctx);
        let min = size.0.min(size.1);
        let mut s = GameState {
            game,
            selected_tile: None,
            move_buttons: Vec::with_capacity(10),

            assets: Assets::new(ctx, min)?,

            board_size: min,
            tile_size: min / 8.0,
            scale_factor: min / 1000.0,
            state,
            next_move: None,
            stream,
        };

        if let Some(stream) = &mut s.stream {
            stream.set_nonblocking(true).unwrap();
        }

        Ok(s)
    }

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

    fn handle_networking(&mut self) {
        let mut buf: [u8; 5] = [0; 5];
        let stream: &mut TcpStream = self.stream.as_mut().unwrap();
        let len = match stream.read(&mut buf) {
            Ok(len) => len,
            Err(e) => {
                if e.kind() != std::io::ErrorKind::WouldBlock {
                    panic!("Something bad happend: {}", e);
                }
                0
            }
        };

        if len == 0 {
            return;
        }

        if len == 1 {
            #[allow(dead_code)]
            #[repr(u8)]
            enum MessageType {
                Decline = 0,
                Undo = 2,
                Accept = 3,
                Checkmate = 4,
                Draw = 5,
                Resign = 6,
            }

            let message_type: MessageType = unsafe { std::mem::transmute(buf[0]) };

            match message_type {
                MessageType::Decline => self.state = State::Playing,
                MessageType::Undo => {
                    stream.write(&[1; 1]).unwrap();
                }
                MessageType::Draw => {
                    stream.write(&[1; 1]).unwrap();
                }
                MessageType::Checkmate => self.state = State::GameOver,
                MessageType::Resign => self.state = State::GameOver,
                _ => {}
            }
            stream.flush().unwrap();
        } else {
            #[allow(dead_code)]
            #[repr(u8)]
            enum MoveType {
                Standard = 0,
                EnPassant = 1,
                Promotion = 2,
                KingsideCastle = 3,
                QueensideCastle = 4,
            }

            #[derive(Copy, Clone)]
            struct Standard {
                origin: u8,
                target: u8,
            }

            #[derive(Copy, Clone)]
            struct Promotion {
                origin: u8,
                target: u8,
                r#type: u8,
            }

            #[repr(C)]
            union MoveData {
                standard: Standard,
                promotion: Promotion,
            }

            struct Move {
                _padding: u8,
                tag: MoveType,
                data: MoveData,
            }

            let r#move: Move = unsafe { std::mem::transmute(buf) };

            let to_coords = |pos: u8| {
                let x = (pos & 0b111) as usize;
                let y = (pos >> 3) as usize;
                (x, y)
            };

            if let Some((from, to)) = self.next_move {
                self.game.next(from, to);
                self.next_move = None;
            }

            let normal_move = |game: &mut Game,
                               stream: &mut TcpStream,
                               origin: (usize, usize),
                               target: (usize, usize)| {
                if let Some(moves) = game.moves_from(origin) {
                    let net_move = (target.0, target.1, moves[0].2);
                    let mut found = false;
                    for r#move in moves {
                        if r#move == &net_move {
                            found = true;
                            break;
                        }
                    }

                    if found {
                        game.next(origin, net_move);
                    } else {
                        stream.write(&[1; 1]).unwrap();
                        stream.flush().unwrap();
                    }
                }
            };

            unsafe {
                match r#move {
                    Move {
                        _padding: 1,
                        tag: MoveType::Standard,
                        data: MoveData { standard: data },
                    } => {
                        let origin = to_coords(data.origin);
                        let target = to_coords(data.target);

                        normal_move(&mut self.game, stream, origin, target);
                    }
                    Move {
                        _padding: 1,
                        tag: MoveType::EnPassant,
                        data: MoveData { standard: data },
                    } => {
                        let origin = to_coords(data.origin);
                        let target = to_coords(data.target);

                        normal_move(&mut self.game, stream, origin, target);
                    }
                    Move {
                        _padding: 1,
                        tag: MoveType::Promotion,
                        data: MoveData { promotion: data },
                    } => {
                        if data.r#type > 3 {
                            stream.write(&[1; 1]).unwrap();
                        }
                        let r#type = match data.r#type {
                            0 => PieceType::Knight,
                            1 => PieceType::Bishop,
                            2 => PieceType::Rook,
                            _ => PieceType::Queen,
                        };

                        let origin = to_coords(data.origin);
                        let target = to_coords(data.target);
                        if let Some(moves) = self.game.moves_from(origin) {
                            let net_move = (target.0, target.1, r#type);
                            let mut found = false;
                            for r#move in moves {
                                if r#move == &net_move {
                                    found = true;
                                    break;
                                }
                            }

                            if found {
                                self.game.next(origin, net_move);
                            } else {
                                stream.write(&[1; 1]).unwrap();
                                stream.flush().unwrap();
                            }
                        }
                    }
                    _ => {}
                }
            }

            self.state = State::Playing;
        }
    }

    fn send_next_move(&mut self) {
        let stream = self.stream.as_mut().unwrap();
        let (from, to) = self.next_move.unwrap();

        let from = (from.0 as u8, from.1 as u8);
        let to = (to.0 as u8, to.1 as u8);

        let to_index = |x: u8, y: u8| x | (y << 3);

        let buffer: [u8; 4] = [1, 0, to_index(from.0, from.1), to_index(to.0, to.1)];

        stream.write(&buffer).unwrap();
        stream.flush().unwrap();
        self.state = State::WaitingForOpponent;
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.game.no_moves() {
            return Ok(());
        }

        if self.state == State::WaitingForOpponent {
            self.handle_networking();
        }

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: mouse::MouseButton,
        x: f32,
        y: f32,
    ) {
        if self.game.no_moves() || self.state != State::Playing {
            return;
        }

        if button == mouse::MouseButton::Left && x < self.board_size {
            let x = (x / self.tile_size) as usize;
            let y = (y / self.tile_size) as usize;
            for button in &self.move_buttons {
                if button.inside(x, y) {
                    if self.stream.is_some() {
                        self.next_move = Some((button.from(), button.to()));
                        self.send_next_move();
                    } else {
                        self.game.next(button.from(), button.to());
                    }
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

        if self.state == State::WaitingForOpponent {
            let (w, h) = graphics::size(ctx);
            let rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, w, h),
                graphics::Color::from_rgba(100, 100, 100, 150),
            )?;

            graphics::draw(ctx, &rect, graphics::DrawParam::new())?;
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
    let mut args = env::args();
    let _ = args.next();
    let (stream, client) = if let Some(arg) = args.next() {
        if arg == "--host" {
            let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
            (Some(listener.incoming().next().unwrap().unwrap()), false)
        } else if arg == "--client" {
            if let Some(arg) = args.next() {
                (Some(TcpStream::connect(arg).unwrap()), true)
            } else {
                panic!("Expected ip address after --client");
            }
        } else {
            panic!("Unknown command: {}", arg);
        }
    } else {
        (None, false)
    };

    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        std::path::PathBuf::from("./resources")
    };

    let mode: conf::WindowMode = Default::default();
    let cb = ggez::ContextBuilder::new("chess", "ggez")
        .window_mode(mode.dimensions(500.0, 500.0))
        .window_setup(conf::WindowSetup {
            title: "Chess".to_owned(),
            icon: "".to_owned(),
            vsync: true,
            srgb: true,
            samples: conf::NumSamples::One,
        })
        .add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut GameState::new(ctx, stream, client)?;
    event::run(ctx, event_loop, state)
}
