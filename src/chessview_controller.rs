use piston::{GenericEvent, MouseButton, Button};
use crate::ChessView;
use crate::ChessPiece;


#[derive(Clone, Copy)]
pub struct ChessViewController
{
    pub chessview: ChessView,
    
    pub from: Option<u8>,
    pub to: Option<u8>,

    cursor_pos: [f64;2]
}

impl ChessViewController
{
    pub fn new(chessview: ChessView) -> ChessViewController
    {
        ChessViewController{chessview: chessview, from: None, to: None,
                            cursor_pos: [0.0;2]}
    }

    pub fn event<E: GenericEvent>(&mut self, chessview: &mut ChessView, e: &E)
    {
        let pos: [f64; 2] = chessview.settings._pos;
        let size: f64 = chessview.settings._size;

        if let Some(cursor_pos) = e.mouse_cursor_args()
        {
            self.cursor_pos = cursor_pos;
        }

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

            match self.from
            {
                // Just doing some moves
                Some(from) => {
                    let from_el: ChessPiece = chessview.blocks[from as usize];
                    if from_el == ChessPiece::Empty { self.from = None; return; }
                    chessview.blocks[from as usize] = ChessPiece::Empty;
                    chessview.blocks[(block_y * 8 + block_x) as usize] = from_el;                    
                    self.from = None;
                },
                None => self.from = Some(block_y * 8 + block_x)
            }
        }
    }
}