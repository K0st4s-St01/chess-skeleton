extern crate sdl2;

mod chess;


use std::time::Duration;

use chess::board::{self, Board};
use sdl2::image::LoadTexture;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("chess", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut board = chess::board::Board::new();
    

    let texture_creator = canvas.texture_creator();

    let black_pieces_img=texture_creator.load_texture("assets/pieces/BlackPieces_Simplified.png").unwrap();
    let white_pieces_img=texture_creator.load_texture("assets/pieces/WhitePieces_Simplified.png").unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut board_x = 9;
    let mut board_y = 9;

    let mut v:Vec<[i32;2]>=Vec::<[i32;2]>::new();


    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x, y } => {
                    if mouse_btn == MouseButton::Left{
                        board_x = (x/64) ;
                        board_y = (y/64) ;
                        println!("{} {}",board_x,board_y);
                        if(Board::in_bounds(board_x as i32, board_y as i32)){
                            let select = board.select(board_x, board_y);
                            if select != None{
                                v = select.unwrap();
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        // The rest of the game loop goes here...
        for i in 0..=7 {
            for j in 0..=7{
                canvas.set_draw_color(Color::RGB(200, 200, 200));
                let _ =canvas.fill_rect(Rect::new(i*64+2,j*64+2,60,60));
            }
        }
        for value in &v{
            canvas.set_draw_color(Color::RGB(0, 255, 255));
            canvas.fill_rect(Rect::new(value[0]  *64,value[1] *64,64,64));
        }
        for i in 0..8{
            for j in 0..8{
                let pieces = board.pieces;
                if pieces[i][j] != None{
                    match pieces[i][j].unwrap().color{
                        chess::board::Color::black => {
                            match pieces[i][j].unwrap().piece{
                                chess::board::Piece_enum::king => {
                                    canvas.copy(&black_pieces_img, Rect::new(80,0,16,16), Rect::new(i as i32*64,j as i32*64,64,64));
                                },
                                chess::board::Piece_enum::queen => {canvas.copy(&black_pieces_img, Rect::new(64,0,16,16), Rect::new(i as i32*64,j as i32*64,64,64));},
                                chess::board::Piece_enum::bishop => {canvas.copy(&black_pieces_img, Rect::new(48,0,16,16), Rect::new(i as i32*64,j as i32*64,64,64));},
                                chess::board::Piece_enum::knight => {canvas.copy(&black_pieces_img, Rect::new(16,0,16,16), Rect::new(i as i32*64,j as i32*64,64,64));},
                                chess::board::Piece_enum::rook => {canvas.copy(&black_pieces_img, Rect::new(32,0,16,16), Rect::new(i as i32*64,j as i32*64,64,64));},
                                chess::board::Piece_enum::pawn => {canvas.copy(&black_pieces_img, Rect::new(0,0,16,16), Rect::new(i as i32*64,j as i32*64,64,64));},
                            }
                        },
                        chess::board::Color::white => {
                            match pieces[i][j].unwrap().piece{
                                chess::board::Piece_enum::king => {
                                    canvas.copy(&white_pieces_img, Rect::new(80,0,16,16), Rect::new(i as i32*64,j as i32*64,64,64));
                                },
                                chess::board::Piece_enum::queen => {canvas.copy(&white_pieces_img, Rect::new(64,0,16,16), Rect::new(i as i32*64,j as i32*64,64,64));},
                                chess::board::Piece_enum::bishop => {canvas.copy(&white_pieces_img, Rect::new(48,0,16,16), Rect::new(i as i32*64,j as i32*64,64,64));},
                                chess::board::Piece_enum::knight => {canvas.copy(&white_pieces_img, Rect::new(16,0,16,16), Rect::new(i as i32*64,j as i32*64,64,64));},
                                chess::board::Piece_enum::rook => {canvas.copy(&white_pieces_img, Rect::new(32,0,16,16), Rect::new(i as i32*64,j as i32*64,64,64));},
                                chess::board::Piece_enum::pawn => {canvas.copy(&white_pieces_img, Rect::new(0,0,16,16), Rect::new(i as i32*64,j as i32*64,64,64));},
                            }
                        },
                    }
                }
            }
        }
        //
        canvas.present();
        std::thread::sleep(Duration::from_millis(100));
    }
}