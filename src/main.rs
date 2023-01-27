mod chess;

extern crate core;

use std::{env, path, usize};
use std::path::{Path, PathBuf};
use std::thread::Thread;
use array2d::Array2D;
use ggez::graphics::Image;
use ggez::Context;
use ggez::*;
use ggez::graphics::Drawable;
use crate::chess::position::position::Position;
use crate::chess::piece::piece::Piece;
use crate::chess::color::color::Color;
use crate::chess::game::r#move::chess_move::{ChessMove, MoveType};
use crate::chess::game::game::Game;

#[derive(Clone)]
struct State {
    game: Game,
    current_available_moves: Vec<ChessMove>,

    pos_x: f32,
    pos_y: f32,
    mouse_down: bool,

    selected: Option<Position>,

    white_pawn: Image,
    black_pawn: Image,

    white_bishop: Image,
    black_bishop: Image,

    white_knight: Image,
    black_knight: Image,

    white_rook: Image,
    black_rook: Image,

    white_queen: Image,
    black_queen: Image,

    white_king: Image,
    black_king: Image,
}

impl State {
    fn new(ctx: &mut ggez::Context) -> GameResult<State> {
        let pawn_white_image = Image::from_path(ctx, "/images/pawn_white.png").unwrap();
        let pawn_black_image = Image::from_path(ctx, "/images/pawn_black.png").unwrap();

        let bishop_white_image = Image::from_path(ctx, "/images/bishop_white.png").unwrap();
        let bishop_black_image = Image::from_path(ctx, "/images/bishop_black.png").unwrap();

        let knight_white_image = Image::from_path(ctx, "/images/knight_white.png").unwrap();
        let knight_black_image = Image::from_path(ctx, "/images/knight_black.png").unwrap();

        let rook_white_image = Image::from_path(ctx, "/images/rook_white.png").unwrap();
        let rook_black_image = Image::from_path(ctx, "/images/rook_black.png").unwrap();

        let queen_white_image = Image::from_path(ctx, "/images/queen_white.png").unwrap();
        let queen_black_image = Image::from_path(ctx, "/images/queen_black.png").unwrap();

        let king_white_image = Image::from_path(ctx, "/images/king_white.png").unwrap();
        let king_black_image = Image::from_path(ctx, "/images/king_black.png").unwrap();


        let state = State {
            game: Game::default(),
            current_available_moves: vec![],
            pos_x: 100.0,
            pos_y: 100.0,
            mouse_down: false,
            selected: None,

            white_pawn: pawn_white_image,
            black_pawn: pawn_black_image,
            white_bishop: bishop_white_image,
            black_bishop: bishop_black_image,
            white_knight: knight_white_image,
            black_knight: knight_black_image,
            white_rook: rook_white_image,
            black_rook: rook_black_image,
            white_queen: queen_white_image,
            black_queen: queen_black_image,
            white_king: king_white_image,
            black_king: king_black_image,
        }.set_available_moves();

        return state;
    }

    fn set_available_moves(&mut self) -> GameResult<State> {
        self.current_available_moves = self.game.get_all_turn_available_moves();

        Ok(self.clone())
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut ggez::Context,
        x: f32,
        y: f32,
        xrel: f32,
        yrel: f32,
    ) -> GameResult {
        if self.mouse_down {
            self.pos_x = x;
            self.pos_y = y;
        }
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        button: ggez::input::mouse::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        let xdiv = x / 100.0;
        let ydiv = y / 100.0;
        let column = if xdiv < 1.0 && xdiv >= 0.0 {
            0_usize
        } else if xdiv < 2.0 && xdiv >= 1.0 {
            1_usize
        } else if xdiv < 3.0 && xdiv >= 2.0 {
            2_usize
        } else if xdiv < 4.0 && xdiv >= 3.0 {
            3_usize
        } else if xdiv < 5.0 && xdiv >= 4.0 {
            4_usize
        } else if xdiv < 6.0 && xdiv >= 5.0 {
            5_usize
        } else if xdiv < 7.0 && xdiv >= 6.0 {
            6_usize
        } else if xdiv < 8.0 && xdiv >= 7.0 {
            7_usize
        } else {
            8_usize
        };

        let row = if ydiv < 1.0 && ydiv >= 0.0 {
            0_usize
        } else if ydiv < 2.0 && ydiv >= 1.0 {
            1_usize
        } else if ydiv < 3.0 && ydiv >= 2.0 {
            2_usize
        } else if ydiv < 4.0 && ydiv >= 3.0 {
            3_usize
        } else if ydiv < 5.0 && ydiv >= 4.0 {
            4_usize
        } else if ydiv < 6.0 && ydiv >= 5.0 {
            5_usize
        } else if ydiv < 7.0 && ydiv >= 6.0 {
            6_usize
        } else if ydiv < 8.0 && ydiv >= 7.0 {
            7_usize
        } else {
            8_usize
        };

        if column < 8 && row < 8 {
            let pos = Position::new(column, row);

            match self.game.get_piece_from_position(&pos) {
                None => {
                    match &self.selected {
                        None => (),
                        Some(t) => {
                            match self.game.try_move_piece( t, &pos) {
                                Ok(t) => (),
                                Err(err) => panic!("{}", err)
                            };
                            self.set_available_moves();
                            ()
                        }
                    }
                },
                Some((piece, color)) => {
                    let turn = self.game.get_turn();

                    if color == turn {
                        self.selected = Some(pos)
                    }
                    else {
                        match &self.selected {
                            None => {
                                self.selected = Some(pos);
                            },
                            Some(t) => {
                                self.game.try_move_piece(t, &pos);
                                self.set_available_moves();
                                ()
                            }
                        }
                    }
                }
            }
        }
        //println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        let mut filler = true;

        let string =  Game::available_moves_to_string(&self.current_available_moves);
        let text = graphics::Text::new(string);

        canvas.draw(&text,
                    graphics::DrawParam::new()
                        .color((1.0, 1.0, 1.0, 1.0))
                        .scale([1.0, 1.0])
                        .dest([100.0 * 8.0 + 20.0, 15.0])
        );

        let mut string = String::new();
        string.push_str("Selected: ");
        match &self.selected {
            None => string.push_str("None"),
            Some(pos) => string.push_str(&pos.to_string())
        }

        let text_selected = graphics::Text::new(string);

        canvas.draw(&text_selected,
                    graphics::DrawParam::new()
                        .color((1.0, 1.0, 1.0, 1.0))
                        .scale([1.0, 1.0])
                        .dest([15.0, 815.0])
        );

        for x in 0..8 {
            for y in 0..8 {
                let image = match self.game.board.get(y, x) {
                    None => panic!("shfsdkgjsdgf"),
                    Some(t) => match t {
                        1 => Some(&self.white_pawn),
                        2 => Some(&self.white_bishop),
                        3 => Some(&self.white_knight),
                        4 => Some(&self.white_rook),
                        5 => Some(&self.white_queen),
                        6 => Some(&self.white_king),
                        9 => Some(&self.black_pawn),
                        10 => Some(&self.black_bishop),
                        11 => Some(&self.black_knight),
                        12 => Some(&self.black_rook),
                        13 => Some(&self.black_queen),
                        14 => Some(&self.black_king),
                        _ => None
                    }
                };

                match image {
                    None => (),
                    Some(t) => {
                        let draw_params = ggez::graphics::DrawParam::new();
                        let recto = ggez::graphics::Rect::new((x * 100_usize) as f32, (y * 100_usize) as f32, 1.6, 1.6);
                        let draw_params = draw_params.dest_rect(recto);
                        let draw_params = draw_params.z(32);
                        t.draw(&mut canvas, draw_params)
                    }
                }


                let rect = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new((x * 100) as f32, (y * 100) as f32, 100.0 as f32, 100.0 as f32),
                    if filler { graphics::Color::WHITE } else { graphics::Color::BLACK },
                )?;
                canvas.draw(&rect, graphics::DrawParam::default());

                filler = !filler;
            }
            filler = !filler;
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() {

    let win_setup = ggez::conf::WindowMode {
        width: 1920.0,
        height: 1080.0,
        borderless: false,
        min_width: 1.0,
        max_width: 2000.0,
        min_height: 1.0,
        max_height: 2000.0,
        maximized: false,
        resizable: true,
        fullscreen_type: ggez::conf::FullscreenType::Windowed,
        logical_size: None,
        visible: true,
        transparent: false,
        resize_on_scale_factor_change: false
    };

    let (mut ctx, event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
        .window_mode(win_setup)
        .build()
        .unwrap();


    let state = State::new(&mut ctx).unwrap();

    event::run(ctx, event_loop, state);

    /*let mut game = Game::create_board_from_string("2b1kb2/8/8/8/8/8/8/2B1KB2", 1);

    chess::print_board(game.clone());
    let position_from = position::new(2,0);
    let position_to = position::new(4,2);

    //let position_from = position::new(2,7);
    //let position_to = position::new(4,5);
    match game.try_move_piece(&position_from, &position_to) {
        Ok(new_game) => (),
        Err(err) => println!("{}", err)
    };
    chess::print_board(game.clone());
    ()*/
}