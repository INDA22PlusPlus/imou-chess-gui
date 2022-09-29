use piston::{GenericEvent, MouseButton, Button};
use crate::ChessView;
use chess::piece::*;

#[derive(Clone)]
pub struct ChessViewController
{
    
    pub selected: u8,

    cursor_pos: [f64;2]
}

impl ChessViewController
{
    // Dummy class, holding the selected block's coordinates and the cursor's pos
    pub fn new() -> ChessViewController
    {
        // 255 = not selected
        ChessViewController{selected: 255,
                            cursor_pos: [0.0;2]}
    }

    // Controlls the events, s.a mouse clicks and so on..
    pub fn event<E: GenericEvent>(&mut self, chessview: &mut ChessView, e: &E)
    {
        let pos: [f64; 2] = chessview.settings._pos;
        let size: f64 = chessview.settings._size;

        if let Some(cursor_pos) = e.mouse_cursor_args()
        {
            self.cursor_pos = cursor_pos;
        }

        // If a mouse click happened, try to register it a selection or a move
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args()
        {

            // x and y coordinates relative to the actual chess board
            let x: f64 = self.cursor_pos[0] - pos[0];
            let y: f64 = self.cursor_pos[1] - pos[1];

            // If not inside - just leave
            if !(x>=0.0 && x < size && y >= 0.0 && y < size) { return; }

            // Compute the actual block in the chess board
            let block_x: u8 = (x / size * 8.0) as u8;
            let block_y: u8 = (y / size * 8.0) as u8;

            let u8_x_to_str: [&str; 8] = ["a","b","c","d","e","f","g","h"];
            let u8_y_to_str: [&str; 8] = ["1","2","3","4","5","6","7","8"];

            let block_x_str: &str = u8_x_to_str[block_x as usize];
            let block_y_str: &str = u8_y_to_str[7-block_y as usize];
            let block_coords_str: String = format!("{}{}", block_x_str, block_y_str);

            match chessview.board.get_selected()
            {
                // If there's a piece selected make the move if possible
                Some(_from) => {
                    chessview.board.play_selected_piece_with_notation(
                                        block_coords_str.as_str());
                    
                    // Not selected
                    self.selected = 255;
                },
                None => {

                    let item: Option<Piece> = chessview.board.get_piece_option_with_notation(
                                                            block_coords_str.as_str());
                    // If the target block selected is not empty, select it and register 
                    // the coordinates
                    if let Some(_item) = item
                    {
                        // Set the coordinate of the selected block
                        self.selected = block_x+8*block_y;
                        chessview.board.select_piece_notation(block_coords_str.as_str());
                    }
                }
            }
        }
    }
}