use fenex::chess::board::board::Board;
use fenex::chess::board::coordinates::Coordinates;
use fenex::chess::piece::piece::Color;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// Enable logging in WASM
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (unsafe { log(&format_args!($($t)*).to_string()) })
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
                        self.board
                            .get(coords)
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
        console_log!("Getting valid moves for piece at row={}, col={}", row, col);

        // Convert JS coordinates (0-indexed) to Fenex coordinates (1-indexed)
        let from_coords = Coordinates::new((col + 1) as i8, (8 - row) as i8);
        console_log!("Fenex coords: x={}, y={}", from_coords.x, from_coords.y);

        // Check if there's a piece at this position
        if let Some(piece) = self.board.get(from_coords) {
            console_log!("Found piece: {:?} {:?}", piece.color, piece.piece_type);
        } else {
            console_log!("No piece at this position");
            return serde_wasm_bindgen::to_value(&Vec::<Move>::new()).unwrap();
        }

        // Use the engine's legal move generation (which we know works correctly)
        let legal_moves = self.board.generate_legal_moves();
        console_log!("Total legal moves in position: {}", legal_moves.len());

        // Filter moves that start from the selected square
        let valid_moves: Vec<Move> = legal_moves
            .iter()
            .filter(|(from, _to)| *from == from_coords)
            .map(|(_from, to)| {
                // Convert back to JS coordinates (0-indexed)
                Move {
                    from_row: row,
                    from_col: col,
                    to_row: (8 - to.y) as usize,
                    to_col: (to.x - 1) as usize,
                }
            })
            .collect();

        console_log!("Valid moves for this piece: {}", valid_moves.len());

        // Log some example moves for debugging
        for (i, mv) in valid_moves.iter().take(5).enumerate() {
            console_log!(
                "Move {}: ({},{}) -> ({},{})",
                i + 1,
                mv.from_row,
                mv.from_col,
                mv.to_row,
                mv.to_col
            );
        }

        serde_wasm_bindgen::to_value(&valid_moves).unwrap()
    }

    #[wasm_bindgen]
    pub fn make_move(
        &mut self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
    ) -> bool {
        console_log!(
            "Attempting move from ({},{}) to ({},{})",
            from_row,
            from_col,
            to_row,
            to_col
        );

        // Convert from JS coordinates (0-indexed) to Fenex coordinates (1-indexed)
        let from_coords = Coordinates::new((from_col + 1) as i8, (8 - from_row) as i8);
        let to_coords = Coordinates::new((to_col + 1) as i8, (8 - to_row) as i8);

        console_log!(
            "Fenex coordinates: from ({},{}) to ({},{})",
            from_coords.x,
            from_coords.y,
            to_coords.x,
            to_coords.y
        );

        // Check if this move is in the legal moves list
        let legal_moves = self.board.generate_legal_moves();
        let is_legal = legal_moves.contains(&(from_coords, to_coords));
        console_log!("Move is in legal moves list: {}", is_legal);

        if !is_legal {
            console_log!("Move rejected: not in legal moves list");
            return false;
        }

        // Get piece info before move
        if let Some(piece) = self.board.get(from_coords) {
            console_log!("Moving piece: {:?} {:?}", piece.color, piece.piece_type);
        }

        // Apply the move
        match self.board.apply_move(from_coords, to_coords) {
            Ok(_) => {
                console_log!("Move successful!");

                // Check if the move resulted in check
                if self.board.is_in_check() {
                    let current_player = match self.board.color_to_move {
                        Color::White => "White",
                        Color::Black => "Black",
                    };
                    console_log!("CHECK! {} king is under attack", current_player);
                }

                true
            }
            Err(e) => {
                console_log!("Move failed: {:?}", e);
                false
            }
        }
    }

    #[wasm_bindgen]
    pub fn make_promotion_move(
        &mut self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
        promotion_piece: &str,
    ) -> bool {
        console_log!(
            "Attempting promotion move from ({},{}) to ({},{}) promoting to {}",
            from_row,
            from_col,
            to_row,
            to_col,
            promotion_piece
        );

        // Convert from JS coordinates (0-indexed) to Fenex coordinates (1-indexed)
        let from_coords = Coordinates::new((from_col + 1) as i8, (8 - from_row) as i8);
        let to_coords = Coordinates::new((to_col + 1) as i8, (8 - to_row) as i8);

        console_log!(
            "Fenex coordinates: from ({},{}) to ({},{})",
            from_coords.x,
            from_coords.y,
            to_coords.x,
            to_coords.y
        );

        // Check if this is a pawn promotion move
        if let Some(piece) = self.board.get(from_coords) {
            console_log!("Moving piece: {:?} {:?}", piece.color, piece.piece_type);

            // Check if it's a pawn moving to the promotion rank
            let is_promotion = match piece.piece_type {
                fenex::chess::piece::piece::PieceType::Pawn => match piece.color {
                    Color::White => to_coords.y == 8,
                    Color::Black => to_coords.y == 1,
                },
                _ => false,
            };

            if !is_promotion {
                console_log!("Not a promotion move");
                return self.make_move(from_row, from_col, to_row, to_col);
            }
        } else {
            console_log!("No piece at source position");
            return false;
        }

        // Check if fenex 0.1.10 has updated promotion handling
        console_log!("Using fenex 0.1.10 - attempting to use promotion piece: {}", promotion_piece);
        
        // Try to find a promotion move in the legal moves that matches our desired piece
        let legal_moves = self.board.generate_legal_moves();
        
        // Look for promotion moves from this position
        let mut found_promotion = false;
        for (from, to) in &legal_moves {
            if *from == from_coords && *to == to_coords {
                console_log!("Found matching legal move for promotion");
                found_promotion = true;
                break;
            }
        }
        
        if !found_promotion {
            console_log!("No legal promotion move found");
            return false;
        }
        
        console_log!("Using fenex 0.1.10 specific promotion methods");
        
        // Use fenex 0.1.10 specific promotion methods
        let result = match promotion_piece.to_lowercase().as_str() {
            "queen" => {
                console_log!("Promoting to Queen");
                self.board.promote_to_queen(from_coords, to_coords)
            },
            "rook" => {
                console_log!("Promoting to Rook");
                self.board.promote_to_rook(from_coords, to_coords)
            },
            "bishop" => {
                console_log!("Promoting to Bishop");
                self.board.promote_to_bishop(from_coords, to_coords)
            },
            "knight" => {
                console_log!("Promoting to Knight");
                self.board.promote_to_knight(from_coords, to_coords)
            },
            _ => {
                console_log!("Unknown piece type, defaulting to Queen");
                self.board.promote_to_queen(from_coords, to_coords)
            }
        };
        
        match result {
            Ok(_) => {
                console_log!("Promotion move successful! (using fenex 0.1.10 specific methods)");
                
                // Log what piece is actually at the destination after the move
                if let Some(piece) = self.board.get(to_coords) {
                    console_log!("Promoted piece is: {:?} {:?}", piece.color, piece.piece_type);
                    console_log!("SUCCESS: Promoted to {:?}!", piece.piece_type);
                } else {
                    console_log!("WARNING: No piece found at promotion square after move");
                }

                // Check if the move resulted in check
                if self.board.is_in_check() {
                    let current_player = match self.board.color_to_move {
                        Color::White => "White",
                        Color::Black => "Black",
                    };
                    console_log!("CHECK! {} king is under attack", current_player);
                }

                true
            }
            Err(e) => {
                console_log!("Promotion move failed: {:?}", e);
                false
            }
        }
    }

    #[wasm_bindgen]
    pub fn is_promotion_move(
        &self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
    ) -> bool {
        // Convert from JS coordinates (0-indexed) to Fenex coordinates (1-indexed)
        let from_coords = Coordinates::new((from_col + 1) as i8, (8 - from_row) as i8);
        let to_coords = Coordinates::new((to_col + 1) as i8, (8 - to_row) as i8);

        // Check if there's a pawn at the source position
        if let Some(piece) = self.board.get(from_coords) {
            match piece.piece_type {
                fenex::chess::piece::piece::PieceType::Pawn => {
                    // Check if moving to promotion rank
                    match piece.color {
                        Color::White => to_coords.y == 8,
                        Color::Black => to_coords.y == 1,
                    }
                }
                _ => false,
            }
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        console_log!("Resetting board");
        self.board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .unwrap_or_else(|_| Board::new());
    }

    #[wasm_bindgen]
    pub fn debug_check_moves(&self, fen: &str) -> JsValue {
        console_log!("=== DEBUGGING CHECK MOVES ===");
        console_log!("Testing FEN: {}", fen);

        // Load the position
        let board = match Board::from_fen(fen) {
            Ok(b) => b,
            Err(e) => {
                console_log!("Invalid FEN: {:?}", e);
                return serde_wasm_bindgen::to_value(&Vec::<String>::new()).unwrap();
            }
        };

        console_log!("Current player: {:?}", board.color_to_move);
        console_log!("King in check: {}", board.is_in_check());

        // Get all legal moves
        let legal_moves = board.generate_legal_moves();
        console_log!("Total legal moves: {}", legal_moves.len());

        let mut check_giving_moves = Vec::new();
        let mut debug_info = Vec::new();

        // Test each move to see if it gives check
        for (from, to) in &legal_moves {
            let mut test_board = board.clone();

            // Get piece info
            let piece_info = if let Some(piece) = board.get(*from) {
                format!("{:?} {:?}", piece.color, piece.piece_type)
            } else {
                "No piece".to_string()
            };

            // Apply the move
            match test_board.apply_move(*from, *to) {
                Ok(_) => {
                    // Check if opponent is now in check
                    if test_board.is_in_check() {
                        let move_desc = format!(
                            "{} from ({},{}) to ({},{}) gives check!",
                            piece_info, from.x, from.y, to.x, to.y
                        );
                        console_log!("{}", move_desc);
                        check_giving_moves.push(move_desc.clone());
                        debug_info.push(move_desc);
                    }
                }
                Err(e) => {
                    console_log!("Move failed: {:?}", e);
                }
            }
        }

        console_log!("Found {} check-giving moves", check_giving_moves.len());
        console_log!("=== END DEBUG ===");

        serde_wasm_bindgen::to_value(&debug_info).unwrap()
    }

    #[wasm_bindgen]
    pub fn test_specific_check_move(&self, from_x: i8, from_y: i8, to_x: i8, to_y: i8) -> JsValue {
        console_log!("=== TESTING SPECIFIC MOVE ===");
        console_log!("Move: ({},{}) to ({},{})", from_x, from_y, to_x, to_y);

        let from_coords = Coordinates::new(from_x, from_y);
        let to_coords = Coordinates::new(to_x, to_y);

        // Check if piece exists
        if let Some(piece) = self.board.get(from_coords) {
            console_log!("Piece: {:?} {:?}", piece.color, piece.piece_type);
        } else {
            console_log!("No piece at source");
            return serde_wasm_bindgen::to_value(&"No piece at source").unwrap();
        }

        // Check if move is in legal moves
        let legal_moves = self.board.generate_legal_moves();
        let is_legal = legal_moves.contains(&(from_coords, to_coords));
        console_log!("Move is legal: {}", is_legal);

        if !is_legal {
            console_log!("Move not in legal moves list");
            return serde_wasm_bindgen::to_value(&"Move not legal").unwrap();
        }

        // Test the move
        let mut test_board = self.board.clone();
        match test_board.apply_move(from_coords, to_coords) {
            Ok(_) => {
                let gives_check = test_board.is_in_check();
                console_log!("Move successful, gives check: {}", gives_check);
                console_log!("Result FEN: {}", test_board.to_fen());

                let result = format!("Move successful, gives check: {}", gives_check);
                serde_wasm_bindgen::to_value(&result).unwrap()
            }
            Err(e) => {
                console_log!("Move failed: {:?}", e);
                serde_wasm_bindgen::to_value(&format!("Move failed: {:?}", e)).unwrap()
            }
        }
    }

    #[wasm_bindgen]
    pub fn load_check_test_position(&mut self) -> String {
        console_log!("Loading test position where white can easily give check");

        // Position where white bishop on c4 can take f7 giving check
        let fen = "rnbqkb1r/pppp1ppp/5n2/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 2 3";

        match Board::from_fen(fen) {
            Ok(board) => {
                self.board = board;
                console_log!("Test position loaded: {}", fen);

                // Test if Bxf7+ is available
                let bishop_from = Coordinates::new(3, 4); // c4
                let f7_square = Coordinates::new(6, 7); // f7

                let legal_moves = self.board.generate_legal_moves();
                let bxf7_legal = legal_moves.contains(&(bishop_from, f7_square));

                console_log!("Bxf7+ is in legal moves: {}", bxf7_legal);

                if bxf7_legal {
                    // Test the move
                    let mut test_board = self.board.clone();
                    match test_board.apply_move(bishop_from, f7_square) {
                        Ok(_) => {
                            console_log!("Bxf7+ executed successfully");
                            console_log!("Gives check: {}", test_board.is_in_check());
                            console_log!("Resulting FEN: {}", test_board.to_fen());
                        }
                        Err(e) => {
                            console_log!("Bxf7+ failed: {:?}", e);
                        }
                    }
                } else {
                    console_log!("WARNING: Bxf7+ is not in legal moves list!");
                    console_log!("Available moves from bishop:");
                    for (from, to) in &legal_moves {
                        if *from == bishop_from {
                            console_log!("  c4 to ({},{})", to.x, to.y);
                        }
                    }
                }

                fen.to_string()
            }
            Err(e) => {
                console_log!("Failed to load test position: {:?}", e);
                format!("Error: {:?}", e)
            }
        }
    }
}
