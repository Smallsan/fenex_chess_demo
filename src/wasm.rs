use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use fenex::chess::board::board::Board;
use fenex::chess::board::coordinates::Coordinates;
use fenex::chess::piece::piece::Color;

// Enable logging in WASM
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Serialize, Deserialize)]
pub struct Move {
    pub from_row: usize,
    pub from_col: usize,
    pub to_row: usize,
    pub to_col: usize,
}

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub board: Vec<Vec<Option<String>>>,
    pub current_player: String,
    pub in_check: bool,
    pub is_checkmate: bool,
    pub is_stalemate: bool,
    pub fen: String,
}

#[wasm_bindgen]
pub struct ChessGame {
    board: Board,
}

#[wasm_bindgen]
impl ChessGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ChessGame {
        console_log!("Creating new chess game");
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .unwrap_or_else(|_| Board::new());
        ChessGame { board }
    }

    #[wasm_bindgen]
    pub fn from_fen(fen: &str) -> Result<ChessGame, JsValue> {
        console_log!("Loading from FEN: {}", fen);
        match Board::from_fen(fen) {
            Ok(board) => Ok(ChessGame { board }),
            Err(e) => Err(JsValue::from_str(&format!("Invalid FEN: {:?}", e))),
        }
    }

    #[wasm_bindgen]
    pub fn get_fen(&self) -> String {
        self.board.to_fen()
    }

    #[wasm_bindgen]
    pub fn get_game_state(&self) -> JsValue {
        let board_state: Vec<Vec<Option<String>>> = (0..8)
            .map(|row| {
                (0..8)
                    .map(|col| {
                        // Convert 0-indexed to 1-indexed coordinates
                        let coords = Coordinates::new((col + 1) as i8, (8 - row) as i8);
                        self.board.get(coords)
                            .map(|piece| format!("{:?}{:?}", piece.color, piece.piece_type))
                    })
                    .collect()
            })
            .collect();

        let current_player = match self.board.color_to_move {
            Color::White => "white",
            Color::Black => "black",
        };

        let in_check = self.board.is_in_check();
        let is_checkmate = self.board.is_checkmate();
        let is_stalemate = self.board.is_stalemate();

        let state = GameState {
            board: board_state,
            current_player: current_player.to_string(),
            in_check,
            is_checkmate,
            is_stalemate,
            fen: self.board.to_fen(),
        };

        serde_wasm_bindgen::to_value(&state).unwrap()
    }

    #[wasm_bindgen]
    pub fn get_valid_moves(&self, row: usize, col: usize) -> JsValue {
        console_log!("Getting valid moves for {},{}", row, col);
        
        // Convert from 0-indexed to 1-indexed coordinates
        let from_coords = Coordinates::new((col + 1) as i8, (8 - row) as i8);
        
        let legal_moves = self.board.generate_legal_moves();
        console_log!("Total legal moves generated: {}", legal_moves.len());
        
        let moves_from_square: Vec<Move> = legal_moves
            .iter()
            .filter(|(from, _to)| *from == from_coords)
            .map(|(_from, to)| Move {
                from_row: row,  // Use the original row parameter
                from_col: col,  // Use the original col parameter
                to_row: (8 - to.y) as usize,  // Convert back to 0-indexed
                to_col: (to.x - 1) as usize,
            })
            .collect();

        console_log!("Valid moves from this square: {}", moves_from_square.len());
        for mv in &moves_from_square {
            console_log!("Move: {},{} -> {},{}", mv.from_row, mv.from_col, mv.to_row, mv.to_col);
        }

        serde_wasm_bindgen::to_value(&moves_from_square).unwrap()
    }

    #[wasm_bindgen]
    pub fn make_move(&mut self, from_row: usize, from_col: usize, to_row: usize, to_col: usize) -> bool {
        console_log!("Attempting move from {},{} to {},{}", from_row, from_col, to_row, to_col);
        
        // Convert from 0-indexed to 1-indexed coordinates
        let from_coords = Coordinates::new((from_col + 1) as i8, (8 - from_row) as i8);
        let to_coords = Coordinates::new((to_col + 1) as i8, (8 - to_row) as i8);
        
        match self.board.apply_move(from_coords, to_coords) {
            Ok(_) => {
                console_log!("Move successful");
                true
            }
            Err(e) => {
                console_log!("Move failed: {:?}", e);
                false
            }
        }
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        console_log!("Resetting board");
        self.board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .unwrap_or_else(|_| Board::new());
    }

    #[wasm_bindgen]
    pub fn display(&self) {
        self.board.display();
    }
}
