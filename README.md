# Fenex Chess Web Demo

## 📁 File Structure

```
docs/
├── index.html          # Main HTML page
├── styles.css          # Styling and responsive design
├── chess-wasm.js       # JavaScript WASM interface
├── pkg/               # Generated WASM files (created by build)
│   ├── fenex.js       # WASM JavaScript bindings
│   ├── fenex_bg.wasm  # Compiled WebAssembly module
│   └── ...
└── README.md          # This file
```

## Development

### Adding New Features

To expose new Rust functionality to the web:

1. Add the function to `src/wasm.rs` with `#[wasm_bindgen]` attribute
2. Rebuild WASM module
3. Call the function from JavaScript in `chess-wasm.js`

## API Reference

The WASM interface exposes these main methods:

- `new ChessGame()` - Create a new game
- `from_fen(fen)` - Load position from FEN string
- `get_game_state()` - Get current board state and game info
- `get_valid_moves(row, col)` - Get valid moves for a piece
- `make_move(from_row, from_col, to_row, to_col)` - Make a move
- `get_fen()` - Export current position as FEN
- `reset()` - Reset to starting position

