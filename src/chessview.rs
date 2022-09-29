use std::collections::HashMap;

use chess::colors::Colors;
use chess::piece_types::PieceTypes;
use graphics::types::Color;
use opengl_graphics::{Texture};
use graphics::{Image, Rectangle, Context, Graphics};
use graphics::rectangle::square;

use chess::chess_engine::*;

use crate::ChessViewController;


#[derive(Clone)]
pub struct ChessView
{
    pub board: ChessEngine,
    pub settings: ChessViewSettings,
}

impl ChessView
{
    pub fn new(settings: ChessViewSettings) -> ChessView
    { 

        ChessView{board: ChessEngine::new(), settings: settings}
    }

    // Main funcion that draws the whole board
    pub fn draw<G: Graphics<Texture = Texture>>(&self, 
                            textures: &HashMap<(PieceTypes, Colors), Texture>, 
                            controller: &ChessViewController, 
                            c: &Context, g: &mut G)
    {
        
        let board_rect: [f64; 4] = [
            self.settings._pos[0],
            self.settings._pos[1],
            self.settings._size,
            self.settings._size,
        ];

        let _col_bl: [f32; 4] = [128.0/255.0, 64.0/255.0, 0.0, 1.0];
        let _col_wh: [f32; 4] = [1.0; 4];
        let block_size: f64 = self.settings._size as f64 / 8.0;

        let mut i: u8 = 0;
        // Loop through every element in the board
        for row in self.board.get_board()
        {
            for piece in row
            {
                // The actual coordinates inside the GUI canvas
                let x1: f64 = self.settings._pos[0] + (i%8) as f64 * block_size;
                let y1: f64 = self.settings._pos[1] + (i/8) as f64 * block_size;

                // Rectangle parameters for drawing, width,height, start x & y
                let chess_cell: [f64; 4] = [x1,y1,block_size, block_size];
                let mut chess_cell_col: [f32; 4] = if (i/8+i+1)%2==0 {_col_wh} else {_col_bl};
                
                // If looping though the selected block, set the bg color to green to mark it
                // as the selected one
                if i==controller.selected
                {
                    chess_cell_col = self.settings._from_block_col;
                }

                // And draw the chess cell
                Rectangle::new(chess_cell_col).draw(
                    chess_cell,
                    &c.draw_state,
                    c.transform,
                    g
                );

                // If there's actually a piece at this block (not empty)
                // try to draw it
                if let Some(piece) = piece
                {
                    // Get the target texture from the hash map
                    let target_texture: &Texture = textures.get(
                        &(piece.piece_type, piece.color)).unwrap();

                    // Draw the chess piece png
                    let visual_piece = Image::new().rect(square(x1,y1, block_size));
                    visual_piece.draw(target_texture, &c.draw_state, c.transform, g);
                }

                // Holding track of the linear coordinates
                i += 1;
            }
        }

        // Draw a border around the chess board
        Rectangle::new_border(self.settings._border_col,
                                self.settings._border_radius).draw(
                                    board_rect,
                                    &c.draw_state,
                                    c.transform,
                                    g
                                );
    }
}


// Could be later implemented for this struct to be constructed from a config file
#[derive(Clone, Copy)]
pub struct ChessViewSettings
{
    pub _pos: [f64; 2],
    pub _size: f64,
    pub _border_col: Color,
    pub _border_radius: f64,
    pub _block_radius: f64,

    pub _from_block_col: Color
}

impl ChessViewSettings
{
    pub fn new() -> ChessViewSettings
    {
        ChessViewSettings{
            _pos: [20.0;2],
            _size: 600.0,
            _border_col: [0.0, 0.0, 0.4, 1.0],
            _border_radius: 2.0,
            _block_radius: 1.0,
            _from_block_col: [0.0, 128.0/255.0, 0.0, 1.0]
        }
    }
}