extern crate glutin_window;

use std::path::Path;
use std::collections::HashMap;


use piston::{WindowSettings, RenderEvent, EventLoop};
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};

use chess::colors::Colors;
use chess::piece_types::PieceTypes;

pub use crate::chessview::{ChessView, ChessViewSettings};
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
    let mut _chessview_controller: ChessViewController = ChessViewController::new();
    
    // Initializing the textures from the png files and storing them as a reference
    // inside a hash map based on the chess struct attributes (color and type)
    let mut _texture_storage: HashMap<(PieceTypes, Colors), Texture> = HashMap::new();
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

    _texture_storage.insert((PieceTypes::King, Colors::White), w_king);
    _texture_storage.insert((PieceTypes::Queen, Colors::White), w_queen);
    _texture_storage.insert((PieceTypes::Knight, Colors::White), w_knight);
    _texture_storage.insert((PieceTypes::Bishop, Colors::White), w_bishop);
    _texture_storage.insert((PieceTypes::Rook, Colors::White), w_rook);
    _texture_storage.insert((PieceTypes::Pawn, Colors::White), w_pawn);

    _texture_storage.insert((PieceTypes::King, Colors::Black), b_king);
    _texture_storage.insert((PieceTypes::Queen, Colors::Black), b_queen);
    _texture_storage.insert((PieceTypes::Knight, Colors::Black), b_knight);
    _texture_storage.insert((PieceTypes::Bishop, Colors::Black), b_bishop);
    _texture_storage.insert((PieceTypes::Rook, Colors::Black), b_rook);
    _texture_storage.insert((PieceTypes::Pawn, Colors::Black), b_pawn);

    // Main event loop
    while let Some(e) = events.next(&mut window)
    {
        // The `ChessViewController` handles all the mouse clicks
        // and all other GUI events that are needed for the game to work
        _chessview_controller.event(&mut _chessview, &e);
        if let Some(args) = e.render_args()
        {
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};

                // Clear with white color for every frame
                clear([1.0; 4], g);
                
                // The drawing on the GUI canvas is handled by the
                // `ChessView` struct
                _chessview.draw(&_texture_storage,
                    &_chessview_controller, &c, g);
            });
        }
    }
}