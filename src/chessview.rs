use std::collections::HashMap;

use chess::colors::Colors;
use chess::piece_types::PieceTypes;
use graphics::types::Color;
use opengl_graphics::{Texture, GlyphCache};
use graphics::{Transformed, Image, Rectangle, Context, Graphics};
use graphics::rectangle::square;

use chess::chess_engine::*;

use crate::ChessViewController;
use crate::chessview_controller::GameState;


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


    pub fn draw<G: Graphics<Texture = Texture>>(&self, glyph: &mut GlyphCache,
                            textures: &HashMap<(PieceTypes, Colors), Texture>, 
                            controller: &ChessViewController, 
                            c: &Context, g: &mut G)
    {

        // Run the right draw function based on the game state
        match controller.state
        {
            GameState::Intro => {
                self._draw_intro(glyph, c, g);
            },
            GameState::SPIngame => {
                self._draw_sp(textures, controller, c, g);
            },
            GameState::MPIngame => {
                self._draw_mp(glyph, textures, controller, c, g);
            },
            _ => ()
        }



    }

    // Draws the game intro (SP & MP buttons + title)
    fn _draw_intro<G: Graphics<Texture = Texture>>(&self, 
        glyph: &mut GlyphCache, 
        c: &Context, g: &mut G)
    {
        let main_bg_rect: [f64; 4] = [
            0.0,
            0.0,
            self.settings._size + 2.0*self.settings._pos[0],
            self.settings._size + 2.0*self.settings._pos[1]
        ];

        let main_bg_color: [f32; 4] = [0.4,0.4,0.4, 1.0];

        Rectangle::new(main_bg_color).draw(
            main_bg_rect,
            &c.draw_state,
            c.transform,
            g
        );

        let _total_width: f64 = self.settings._size + 2.0*self.settings._pos[0];
        let _total_height: f64 = self.settings._size + 2.0*self.settings._pos[1];

        // Graphics context transform to try and center the text
        let text_transform: [[f64; 3]; 2] = c.transform.trans(
            self.settings._intro_text_coords[0],
            self.settings._intro_text_coords[1]);

        // Draw the main intro text
        graphics::text::Text::new_color([1.0; 4], 50)
            .draw(
                "Chess 2", 
                glyph, 
                &c.draw_state, 
                text_transform, 
            g).unwrap();


        // Coord and size struct for the SP and MP buttons
        let button1_data: [f64; 4] = [
            self.settings._intro_sp_button_pos[0],
            self.settings._intro_sp_button_pos[1],
            self.settings._intro_button_size[0],
            self.settings._intro_button_size[1]
        ];

        let button2_data: [f64; 4] = [
            self.settings._intro_mp_button_pos[0],
            self.settings._intro_mp_button_pos[1],
            self.settings._intro_button_size[0],
            self.settings._intro_button_size[1]
        ];
        // Draw the buttons
        Rectangle::new(self.settings._intro_button_col).draw(
            button1_data,
            &c.draw_state,
            c.transform,
            g  
        );
        Rectangle::new(self.settings._intro_button_col).draw(
            button2_data,
            &c.draw_state,
            c.transform,
            g  
        );

        // Draw text "SP" and "MP" on the buttons
        let text_sp_transform: [[f64; 3]; 2] = c.transform.trans(
            self.settings._intro_sp_button_pos[0]+50.0,
            self.settings._intro_sp_button_pos[1]+37.0);
        let text_mp_transform: [[f64; 3]; 2] = c.transform.trans(
            self.settings._intro_mp_button_pos[0]+45.0,
            self.settings._intro_mp_button_pos[1]+37.0
        );
        graphics::text::Text::new_color([1.0; 4], 30)
        .draw(
            "SP", 
            glyph, 
            &c.draw_state, 
            text_sp_transform, 
        g).unwrap();

        graphics::text::Text::new_color([1.0; 4], 30)
        .draw(
            "MP", 
            glyph, 
            &c.draw_state, 
            text_mp_transform, 
        g).unwrap();
    }

    fn _draw_sp<G: Graphics<Texture = Texture>>(&self, 
        textures: &HashMap<(PieceTypes, Colors), Texture>, 
        controller: &ChessViewController, 
        c: &Context, g: &mut G)
    {
        self._draw_board(textures, controller, c, g);
    }

    fn _draw_mp<G: Graphics<Texture = Texture>>(&self,
        glyph: &mut GlyphCache,
        textures: &HashMap<(PieceTypes, Colors), Texture>, 
        controller: &ChessViewController, 
        c: &Context, g: &mut G)
    {
        if !controller.mp_connection
        {
            let main_bg_rect: [f64; 4] = [
                0.0,
                0.0,
                self.settings._size+2.0*self.settings._pos[0],
                self.settings._size+2.0*self.settings._pos[1]
            ];

            // Draw the gray bg
            Rectangle::new([0.4, 0.4, 0.4, 1.0]).draw(
                main_bg_rect,
                &c.draw_state,
                c.transform,
                g
            );

            // Matrix for positioning the "waiting for connection text"
            let text_transform: [[f64; 3]; 2] = c.transform.trans(
                120.0, 250.0
            );

            graphics::text::Text::new_color([1.0; 4], 30)
            .draw(
                "Waiting for connection..", 
                glyph, 
                &c.draw_state, 
                text_transform, 
            g).unwrap();

            return; 
        }

        self._draw_board(textures, controller, c, g);
    }

    // Main funcion that draws the whole boards
    fn _draw_board<G: Graphics<Texture = Texture>>(&self, 
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

    pub _from_block_col: Color,

    pub _intro_text_coords: [f64; 2],
    pub _intro_button_col: Color,
    pub _intro_button_size: [f64; 2],
    pub _intro_sp_button_pos: [f64; 2],
    pub _intro_mp_button_pos: [f64; 2],
}

impl ChessViewSettings
{
    pub fn new() -> ChessViewSettings
    {
        ChessViewSettings{
            _pos: [20.0;2],
            _size: 600.0,
            // rgba
            _border_col: [0.0, 0.0, 0.4, 1.0],
            _border_radius: 2.0,
            _block_radius: 1.0,
            // rgba
            _from_block_col: [0.0, 128.0/255.0, 0.0, 1.0],
            // x,y
            _intro_text_coords: [220.0, 100.0],
            // rgba
            _intro_button_col: [0.0, 150.0/255.0, 0.0, 1.0],
            // width, height
            _intro_button_size: [150.0, 45.0],
            // x,y
            _intro_sp_button_pos: [245.0, 150.0],
            _intro_mp_button_pos: [245.0, 210.0]

        }
    }
}