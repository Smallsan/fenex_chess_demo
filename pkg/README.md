# Fenex Chess Web Demo

This directory contains an interactive web demonstration of the Fenex chess library, using WebAssembly to run the actual Rust code in the browser.

## 🚀 Quick Start

### Building the Demo

1. **Install wasm-pack** (if not already installed):
   ```bash
   cargo install wasm-pack
   ```

2. **Build the WASM module**:
   ```bash
   # On Windows
   ./build-wasm.bat
   
   # On macOS/Linux
   ./build-wasm.sh
   ```

3. **Start a local server** (required for WASM loading):
   ```bash
   python -m http.server 8000
   # or any other static file server
   ```

4. **Open your browser** to:
   ```
   http://localhost:8000/docs
   ```

### Manual Build Steps

If the build scripts don't work, you can build manually:

```bash
wasm-pack build --target web --out-dir docs/pkg --no-typescript
```

## 📦 Deploying to GitHub Pages

1. **Commit all files**:
   ```bash
   git add docs/
   git commit -m "Add WASM chess demo"
   git push
   ```

2. **Enable GitHub Pages**:
   - Go to your repository Settings > Pages
   - Set source to "Deploy from a branch"
   - Select "main" branch and "/docs" folder
   - Click Save

3. **Access your demo**:
   Your site will be available at: `https://USERNAME.github.io/REPOSITORY/`

## 🎮 Features

- **Interactive Chess Board**: Click pieces to select them, click highlighted squares to move
- **Full Chess Rules**: Supports castling, en passant, pawn promotion, check/checkmate detection
- **FEN Support**: Load custom positions using FEN notation
- **Move History**: Track all moves made during the game
- **Example Positions**: Quick-load common chess positions
- **Board Flipping**: View the board from either side
- **Responsive Design**: Works on desktop and mobile devices

## 🏗️ Technical Details

- **Frontend**: Pure HTML, CSS, and JavaScript
- **Backend Logic**: Rust compiled to WebAssembly using wasm-pack
- **Chess Engine**: Uses the actual Fenex library code
- **Styling**: Modern CSS with responsive design
- **Dependencies**: No external JavaScript frameworks

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

## 🔧 Development

### Modifying the Chess Logic

The chess logic is in the main Rust library (`src/wasm.rs` contains the WASM bindings). After making changes:

1. Rebuild the WASM module: `./build-wasm.bat` (or `.sh`)
2. Refresh your browser

### Modifying the Interface

- Edit `index.html` for structure changes
- Edit `styles.css` for styling changes  
- Edit `chess-wasm.js` for JavaScript functionality
- No rebuild needed, just refresh your browser

### Adding New Features

To expose new Rust functionality to the web:

1. Add the function to `src/wasm.rs` with `#[wasm_bindgen]` attribute
2. Rebuild WASM module
3. Call the function from JavaScript in `chess-wasm.js`

## 🐛 Troubleshooting

### WASM Loading Issues

- **File not found errors**: Make sure you built the WASM files and they're in `docs/pkg/`
- **CORS errors**: Use a local server, don't open the HTML file directly
- **Build failures**: Ensure you have the latest `wasm-pack` and Rust toolchain

### Common Build Problems

```bash
# Update Rust and wasm-pack
rustup update
cargo install wasm-pack --force

# Clean and rebuild
cargo clean
./build-wasm.bat
```

### Browser Compatibility

- Modern browsers with WASM support required
- Tested on Chrome 90+, Firefox 88+, Safari 14+
- Mobile browsers supported

## 📖 API Reference

The WASM interface exposes these main methods:

- `new ChessGame()` - Create a new game
- `from_fen(fen)` - Load position from FEN string
- `get_game_state()` - Get current board state and game info
- `get_valid_moves(row, col)` - Get valid moves for a piece
- `make_move(from_row, from_col, to_row, to_col)` - Make a move
- `get_fen()` - Export current position as FEN
- `reset()` - Reset to starting position

## 🤝 Contributing

Feel free to improve the web interface:

1. Fork the repository
2. Make your changes in the `docs/` directory
3. Test locally using the build instructions above  
4. Submit a pull request

## 📄 License

Same as the main Fenex library - see the root LICENSE file.
