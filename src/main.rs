extern crate glutin_window;

use std::path::Path;
use std::collections::HashMap;

use piston::{WindowSettings, RenderEvent, EventLoop};
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};

pub use crate::chessview::{ChessPiece, ChessView, ChessViewSettings};
pub use crate::chessview_controller::ChessViewController;

mod chessview;
mod chessview_controller;

const W_WIDTH: u32  = 640;
const W_HEIGHT: u32 = 640;

fn main() {
    let opengl = OpenGL::V3_2;
    let settings: WindowSettings = WindowSettings::new("Sudoku", (W_WIDTH, W_HEIGHT))
                                    .exit_on_esc(true)
                                    .graphics_api(opengl)
                                    .vsync(true);
    let mut window: GlutinWindow = settings.build().expect("COULD NOT CREATE WINDOW");

    // Render only on user input
    let mut events = Events::new(EventSettings::new().lazy(true));
    // gl interface. Stores shaders + buffers 
    let mut gl = GlGraphics::new(opengl);


    // Main chess gui components for event handling
    let _chessview_settings: ChessViewSettings = ChessViewSettings::new();
    let mut _chessview: ChessView = ChessView::new(_chessview_settings);
    let mut _chessview_controller: ChessViewController = ChessViewController::new(_chessview);
    
    let mut _texture_storage: HashMap<ChessPiece, Texture> = HashMap::new();

    let w_king: Texture = Texture::from_path(Path::new("assets/img/w_king.png"),
    &TextureSettings::new()).unwrap();
    let w_queen: Texture = Texture::from_path(Path::new("assets/img/w_queen.png"),
    &TextureSettings::new()).unwrap(); 
    let w_rook: Texture = Texture::from_path(Path::new("assets/img/w_rook.png"),
    &TextureSettings::new()).unwrap(); 
    let w_knight: Texture = Texture::from_path(Path::new("assets/img/w_knight.png"),
    &TextureSettings::new()).unwrap(); 
    let w_bishop: Texture = Texture::from_path(Path::new("assets/img/w_bishop.png"),
    &TextureSettings::new()).unwrap(); 
    let w_pawn: Texture = Texture::from_path(Path::new("assets/img/w_pawn.png"),
    &TextureSettings::new()).unwrap();

    let b_king: Texture = Texture::from_path(Path::new("assets/img/b_king.png"),
    &TextureSettings::new()).unwrap();
    let b_queen: Texture = Texture::from_path(Path::new("assets/img/b_queen.png"),
    &TextureSettings::new()).unwrap(); 
    let b_rook: Texture = Texture::from_path(Path::new("assets/img/b_rook.png"),
    &TextureSettings::new()).unwrap(); 
    let b_knight: Texture = Texture::from_path(Path::new("assets/img/b_knight.png"),
    &TextureSettings::new()).unwrap(); 
    let b_bishop: Texture = Texture::from_path(Path::new("assets/img/b_bishop.png"),
    &TextureSettings::new()).unwrap(); 
    let b_pawn: Texture = Texture::from_path(Path::new("assets/img/b_pawn.png"),
    &TextureSettings::new()).unwrap();       

    _texture_storage.insert(ChessPiece::WKing, w_king);
    _texture_storage.insert(ChessPiece::WQueen, w_queen);
    _texture_storage.insert(ChessPiece::WKnight, w_knight);
    _texture_storage.insert(ChessPiece::WBishop, w_bishop);
    _texture_storage.insert(ChessPiece::WRook, w_rook);
    _texture_storage.insert(ChessPiece::WPawn, w_pawn);

    _texture_storage.insert(ChessPiece::BKing, b_king);
    _texture_storage.insert(ChessPiece::BQueen, b_queen);
    _texture_storage.insert(ChessPiece::BKnight, b_knight);
    _texture_storage.insert(ChessPiece::BBishop, b_bishop);
    _texture_storage.insert(ChessPiece::BRook, b_rook);
    _texture_storage.insert(ChessPiece::BPawn, b_pawn);

    // Main event loop
    while let Some(e) = events.next(&mut window)
    {
        _chessview_controller.event(&mut _chessview, &e);
        if let Some(args) = e.render_args()
        {
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};

                clear([1.0; 4], g);
                _chessview.draw(&_texture_storage,
                    &_chessview_controller, &c, g);
            });
        }
    }
}