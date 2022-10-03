extern crate json;

use std::net::TcpStream;
use std::fs;

use piston::{GenericEvent, MouseButton, Button};
use crate::ChessView;
use chess::piece::*;

pub mod items {
    include!("inet/protobuf.rs");
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum GameState
{
    Intro = 0,
    SPIngame = 1,
    MPIngame = 2,

    SPPause = 3,
    MPPause = 4
}

#[derive(Clone)]
pub struct ChessViewController
{
    pub mp_connection: bool,
    pub state: GameState,
    pub selected: u8,
    cursor_pos: [f64;2]
}

impl ChessViewController
{
    // Dummy class, holding the selected block's coordinates and the cursor's pos
    pub fn new() -> ChessViewController
    {
        // 255 = not selected
        ChessViewController{mp_connection: false, state: GameState::Intro,
            selected: 255, cursor_pos: [0.0;2]}
    }

    // Controlls the events, s.a mouse clicks and so on..
    pub fn event<E: GenericEvent>(&mut self, chessview: &mut ChessView, e: &E)
    {
        let pos: [f64; 2] = chessview.settings._pos;

        if let Some(cursor_pos) = e.mouse_cursor_args()
        {
            self.cursor_pos = cursor_pos;
        }

        // If a mouse click happened, try to register it a selection or a move
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args()
        {

            // Run the right click event handler based on the game state
            match self.state
            {
                GameState::Intro => self._click_intro_handler(chessview),
                GameState::SPIngame => self._click_sp_ingame_handler(pos, chessview),
                GameState::MPIngame => self._click_mp_ingame_handler(pos, chessview),
                _ => ()
            }


        }
    }

    fn _click_intro_handler(&mut self, chessview: &ChessView)
    {
        let x: f64 = self.cursor_pos[0];
        let y: f64 = self.cursor_pos[1];
        let b_sp_xy: [f64; 2] = chessview.settings._intro_sp_button_pos;
        let b_mp_xy: [f64; 2] = chessview.settings._intro_mp_button_pos;
        let b_wh: [f64; 2] = chessview.settings._intro_button_size;

        // Calculate if click pos inside the bounds of the SP button
        let clicked_sp: bool = 0.0 <= x - b_sp_xy[0] && x - b_sp_xy[0] <= b_wh[0]
                                && 0.0 <= y - b_sp_xy[1] && y - b_sp_xy[1] <= b_wh[1];


        // Calculate if click pos inside the bounds of the MP button
        let clicked_mp: bool = 0.0 <= x - b_mp_xy[0] && x - b_mp_xy[0] <= b_wh[0]
                                && 0.0 <= y - b_mp_xy[1] && y - b_mp_xy[1] <= b_wh[1];


        // Change the game state to either mp or sp based on the button clicked
        if clicked_sp { self.state = GameState::SPIngame; }
        if clicked_mp { 
            self.state = GameState::MPIngame; 
            
            let _config_str = fs::read_to_string("config/server.json")
                                                    .expect("CANNOT OPEN MAIN CONFIG FILE");

            let config = json::parse(_config_str.as_str()).unwrap();
            // Connect to server & initialize the connection as detailed in `src/inet/chess.proto`
            _connect2server_and_init(config["ip-port"].as_str().unwrap());
        }
    }

    fn _click_sp_ingame_handler(&mut self, pos: [f64; 2], chessview: &mut ChessView)
    {
        // Total board size
        let size: f64 = chessview.settings._size;

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

    fn _click_mp_ingame_handler(&mut self, pos: [f64; 2], chessview: &mut ChessView)
    {
        // Don't allow any moves when connection to server is not established
        if !self.mp_connection { return; }
    }
}

fn _connect2server_and_init(ip_port: &str)
{
    let mut stream = TcpStream::connect(ip_port)
                                    .expect("CANNOT CONNECT TO SERVER!");
    //....
    // follow the `chess.proto` to init a successfull connection
       
}