use std::collections::HashMap;

use graphics::types::Color;
use opengl_graphics::{Texture, GlGraphics};
use graphics::{Image, Rectangle, Context, Graphics};
use graphics::rectangle::square;
use piston::controller;

use crate::ChessViewController;

#[repr(u8)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum ChessPiece
{
    Empty   =   0,
    BKing   =   1,
    BQueen  =   2,
    BRook   =   3,
    BBishop =   4,
    BKnight =   5,
    BPawn   =   6,

    WKing   =   7,
    WQueen  =   8,
    WRook   =   9,
    WBishop =   10,
    WKnight =   11,
    WPawn   =   12,
}

#[derive(Clone, Copy)]
pub struct ChessView
{
    // Preliminary, waiting for the chess lib
    pub blocks: [ChessPiece; 64],
    pub settings: ChessViewSettings,
}

impl ChessView
{
    pub fn new(settings: ChessViewSettings) -> ChessView
    { 
        // Visual representation of the board
        let blocks: [ChessPiece; 64] = [
            ChessPiece::BRook, ChessPiece::BKnight, ChessPiece::BBishop, 
            ChessPiece::BQueen, ChessPiece::BKing, ChessPiece::BBishop, 
            ChessPiece::BKnight, ChessPiece::BRook, ChessPiece::BPawn,
            ChessPiece::BPawn, ChessPiece::BPawn, ChessPiece::BPawn,
            ChessPiece::BPawn, ChessPiece::BPawn, ChessPiece::BPawn,
            ChessPiece::BPawn, ChessPiece::Empty, ChessPiece::Empty, 
            ChessPiece::Empty, ChessPiece::Empty, ChessPiece::Empty, 
            ChessPiece::Empty, ChessPiece::Empty, ChessPiece::Empty, 
            ChessPiece::Empty, ChessPiece::Empty, ChessPiece::Empty, 
            ChessPiece::Empty, ChessPiece::Empty, ChessPiece::Empty, 
            ChessPiece::Empty, ChessPiece::Empty, ChessPiece::Empty, 
            ChessPiece::Empty, ChessPiece::Empty, ChessPiece::Empty, 
            ChessPiece::Empty, ChessPiece::Empty, ChessPiece::Empty, 
            ChessPiece::Empty, ChessPiece::Empty, ChessPiece::Empty, 
            ChessPiece::Empty, ChessPiece::Empty, ChessPiece::Empty, 
            ChessPiece::Empty, ChessPiece::Empty, ChessPiece::Empty, 
            ChessPiece::WPawn, ChessPiece::WPawn, ChessPiece::WPawn, 
            ChessPiece::WPawn, ChessPiece::WPawn, ChessPiece::WPawn, 
            ChessPiece::WPawn, ChessPiece::WPawn, ChessPiece::WRook,
            ChessPiece::WKnight, ChessPiece::WBishop, ChessPiece::WQueen,
            ChessPiece::WKing, ChessPiece::WBishop, ChessPiece::WKnight,
            ChessPiece::WRook
        ];
        ChessView{blocks: blocks, settings: settings}
    }

    pub fn draw<G: Graphics<Texture = Texture>>(&self, textures: &HashMap<ChessPiece, Texture>, 
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
        for i in 0..64
        {
            let x1: f64 = self.settings._pos[0] + (i%8) as f64 * block_size;
            let y1: f64 = self.settings._pos[1] + (i/8) as f64 * block_size;

            // Draw the cell
            let chess_cell: [f64; 4] = [x1,y1,block_size, block_size];
            let mut chess_cell_col: [f32; 4] = if (i/8+i+1)%2==0 {_col_wh} else {_col_bl};
            
            match controller.from
            {
                Some(from) => {
                    // Set the color to green if rendering the selected chess piece
                    if from==i && self.blocks[i as usize] != ChessPiece::Empty
                    {
                        chess_cell_col = self.settings._from_block_col;
                    }

                },
                None => ()
            }

            Rectangle::new(chess_cell_col).draw(
                chess_cell,
                &c.draw_state,
                c.transform,
                g
            );

            // Draw the chess piece png
            let visual_piece = Image::new().rect(square(x1,y1, block_size));
            let _piece: ChessPiece= self.blocks[i as usize];
            if _piece == ChessPiece::Empty { continue; }
            let target_texture: &Texture = textures.get(&_piece).unwrap();
            visual_piece.draw(target_texture, &c.draw_state, c.transform, g);
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