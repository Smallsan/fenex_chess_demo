// WASM Chess Game Interface
class WasmChessGame {
    constructor() {
        this.game = null;
        this.selectedSquare = null;
        this.validMoves = [];
        this.isFlipped = false;
    }

    async init() {
        try {
            // Load the WASM module
            const wasm = await import('./pkg/fenex.js');
            await wasm.default(); // Initialize WASM
            
            // Create new game instance
            this.game = new wasm.ChessGame();
            console.log('WASM Chess game initialized successfully');
            return true;
        } catch (error) {
            console.error('Failed to initialize WASM:', error);
            return false;
        }
    }

    getGameState() {
        if (!this.game) return null;
        return this.game.get_game_state();
    }

    getFen() {
        if (!this.game) return '';
        return this.game.get_fen();
    }

    getValidMoves(row, col) {
        if (!this.game) return [];
        return this.game.get_valid_moves(row, col);
    }

    makeMove(fromRow, fromCol, toRow, toCol) {
        if (!this.game) return false;
        return this.game.make_move(fromRow, fromCol, toRow, toCol);
    }

    loadFromFen(fen) {
        try {
            this.game = this.game.constructor.from_fen(fen);
            return true;
        } catch (error) {
            console.error('Invalid FEN:', error);
            return false;
        }
    }

    reset() {
        if (!this.game) return;
        this.game.reset();
        this.clearSelection();
    }

    clearSelection() {
        this.selectedSquare = null;
        this.validMoves = [];
    }
}

// Chess piece symbols for display
const PIECE_SYMBOLS = {
    'WhitePawn': '♙',    // White filled
    'WhiteRook': '♖',    // White filled
    'WhiteKnight': '♘',  // White filled
    'WhiteBishop': '♗',  // White filled
    'WhiteQueen': '♕',   // White filled
    'WhiteKing': '♔',    // White filled
    'BlackPawn': '♟︎',    // Black outlined
    'BlackRook': '♜',    // Black outlined
    'BlackKnight': '♞',  // Black outlined
    'BlackBishop': '♝',  // Black outlined
    'BlackQueen': '♛',   // Black outlined
    'BlackKing': '♚'     // Black outlined
};

// Main chess interface
let chessGame;
let moveHistory = [];

async function initializeGame() {
    const loadingMessage = document.getElementById('game-status');
    loadingMessage.textContent = 'Loading WASM module...';
    loadingMessage.className = 'status-display loading';

    chessGame = new WasmChessGame();
    const initialized = await chessGame.init();

    if (initialized) {
        createBoard();
        updateDisplay();
        setupEventListeners();
        loadingMessage.textContent = 'Game ready!';
        loadingMessage.className = 'status-display ready';
        setTimeout(() => {
            updateGameStatus();
        }, 1000);
    } else {
        loadingMessage.textContent = 'Failed to load WASM module. Please check console for errors.';
        loadingMessage.className = 'status-display error';
        showFallbackMessage();
    }
}

function showFallbackMessage() {
    const container = document.querySelector('.game-container');
    container.innerHTML = `
        <div class="error-container">
            <h2>WASM Loading Failed</h2>
            <p>The WebAssembly module couldn't be loaded. This might be because:</p>
            <ul>
                <li>The WASM files haven't been built yet</li>
                <li>The page isn't being served from a web server</li>
                <li>Your browser doesn't support WASM</li>
            </ul>
            <h3>To build and run this demo:</h3>
            <ol>
                <li>Install wasm-pack: <code>cargo install wasm-pack</code></li>
                <li>Build WASM: <code>wasm-pack build --target web --out-dir docs/pkg</code></li>
                <li>Serve with a local server: <code>python -m http.server 8000</code></li>
                <li>Visit: <code>http://localhost:8000/docs</code></li>
            </ol>
            <p>Check the <a href="https://github.com/Smallsan/fenex">repository README</a> for detailed instructions.</p>
        </div>
    `;
}

function createBoard() {
    const chessboard = document.getElementById('chessboard');
    chessboard.innerHTML = '';

    for (let row = 0; row < 8; row++) {
        for (let col = 0; col < 8; col++) {
            const square = document.createElement('div');
            square.className = `square ${(row + col) % 2 === 0 ? 'light' : 'dark'}`;
            square.dataset.row = row;
            square.dataset.col = col;
            square.addEventListener('click', handleSquareClick);
            chessboard.appendChild(square);
        }
    }
}

function updateDisplay() {
    const gameState = chessGame.getGameState();
    if (!gameState) return;

    const squares = document.querySelectorAll('.square');
    
    squares.forEach((square, index) => {
        const row = Math.floor(index / 8);
        const col = index % 8;
        
        // Clear previous styling
        square.classList.remove('selected', 'valid-move', 'has-piece', 'piece-white', 'piece-black');
        square.textContent = '';
        
        // Display row and column (adjust for flip)
        const displayRow = chessGame.isFlipped ? 7 - row : row;
        const displayCol = chessGame.isFlipped ? 7 - col : col;
        
        // Add piece if present
        const piece = gameState.board[displayRow][displayCol];
        if (piece) {
            square.textContent = PIECE_SYMBOLS[piece] || '?';
            square.classList.add('has-piece');
            
            // Add color class for better styling
            if (piece.toLowerCase().includes('white')) {
                square.classList.add('piece-white');
            } else {
                square.classList.add('piece-black');
            }
        }
        
        // Highlight selected square
        if (chessGame.selectedSquare && 
            chessGame.selectedSquare.row === displayRow && 
            chessGame.selectedSquare.col === displayCol) {
            square.classList.add('selected');
        }
        
        // Highlight valid moves
        if (chessGame.validMoves.some(move => 
            move.to_row === displayRow && move.to_col === displayCol)) {
            square.classList.add('valid-move');
            if (piece) {
                square.classList.add('has-piece');
            }
        }
    });

    // Update FEN display
    document.getElementById('fen-display').value = gameState.fen;
    
    // Update current turn
    document.getElementById('current-turn').textContent = 
        gameState.current_player.charAt(0).toUpperCase() + gameState.current_player.slice(1);
}

function updateGameStatus() {
    const gameState = chessGame.getGameState();
    if (!gameState) return;

    const statusElement = document.getElementById('game-status');
    
    if (gameState.is_checkmate) {
        const winner = gameState.current_player === 'white' ? 'Black' : 'White';
        statusElement.textContent = `Checkmate! ${winner} wins!`;
        statusElement.className = 'status-display checkmate';
    } else if (gameState.is_stalemate) {
        statusElement.textContent = 'Stalemate! Game is a draw.';
        statusElement.className = 'status-display stalemate';
    } else if (gameState.in_check) {
        statusElement.textContent = `${gameState.current_player.charAt(0).toUpperCase() + gameState.current_player.slice(1)} is in check!`;
        statusElement.className = 'status-display check';
    } else {
        statusElement.textContent = 'Game in progress';
        statusElement.className = 'status-display';
    }
}

function handleSquareClick(event) {
    const square = event.target;
    const row = parseInt(square.dataset.row);
    const col = parseInt(square.dataset.col);
    
    // Adjust for flipped board
    const actualRow = chessGame.isFlipped ? 7 - row : row;
    const actualCol = chessGame.isFlipped ? 7 - col : col;
    
    const gameState = chessGame.getGameState();
    if (!gameState || gameState.is_checkmate || gameState.is_stalemate) return;

    // If clicking on a valid move square
    if (chessGame.selectedSquare && 
        chessGame.validMoves.some(move => 
            move.to_row === actualRow && move.to_col === actualCol)) {
        
        // Make the move
        const success = chessGame.makeMove(
            chessGame.selectedSquare.row,
            chessGame.selectedSquare.col,
            actualRow,
            actualCol
        );
        
        if (success) {
            // Record move in history
            const fromNotation = String.fromCharCode(97 + chessGame.selectedSquare.col) + (8 - chessGame.selectedSquare.row);
            const toNotation = String.fromCharCode(97 + actualCol) + (8 - actualRow);
            addMoveToHistory(`${fromNotation} → ${toNotation}`);
            
            chessGame.clearSelection();
            updateDisplay();
            updateGameStatus();
        }
        return;
    }

    // If clicking on own piece, select it
    const piece = gameState.board[actualRow][actualCol];
    if (piece) {
        const pieceColor = piece.toLowerCase().includes('white') ? 'white' : 'black';
        if (pieceColor === gameState.current_player) {
            chessGame.selectedSquare = { row: actualRow, col: actualCol };
            chessGame.validMoves = chessGame.getValidMoves(actualRow, actualCol);
            updateDisplay();
            return;
        }
    }
    
    // Otherwise, clear selection
    chessGame.clearSelection();
    updateDisplay();
}

function addMoveToHistory(moveNotation) {
    moveHistory.push(moveNotation);
    const historyElement = document.getElementById('move-history');
    
    const moveElement = document.createElement('div');
    moveElement.className = 'move-entry';
    moveElement.textContent = `${Math.ceil(moveHistory.length / 2)}. ${moveNotation}`;
    
    historyElement.appendChild(moveElement);
    historyElement.scrollTop = historyElement.scrollHeight;
}

function setupEventListeners() {
    // Reset button
    document.getElementById('reset-btn').addEventListener('click', () => {
        chessGame.reset();
        moveHistory = [];
        document.getElementById('move-history').innerHTML = '';
        updateDisplay();
        updateGameStatus();
    });

    // Copy FEN button
    document.getElementById('fen-btn').addEventListener('click', () => {
        const fen = chessGame.getFen();
        navigator.clipboard.writeText(fen).then(() => {
            const btn = document.getElementById('fen-btn');
            const originalText = btn.textContent;
            btn.textContent = 'Copied!';
            setTimeout(() => {
                btn.textContent = originalText;
            }, 2000);
        });
    });

    // Flip board button
    document.getElementById('flip-btn').addEventListener('click', () => {
        chessGame.isFlipped = !chessGame.isFlipped;
        updateDisplay();
    });

    // Load FEN functionality
    document.getElementById('load-fen-btn').addEventListener('click', () => {
        const input = document.getElementById('fen-input');
        const applyBtn = document.getElementById('apply-fen-btn');
        const cancelBtn = document.getElementById('cancel-fen-btn');
        const loadBtn = document.getElementById('load-fen-btn');
        
        input.style.display = 'block';
        applyBtn.style.display = 'inline-block';
        cancelBtn.style.display = 'inline-block';
        loadBtn.style.display = 'none';
        input.focus();
    });

    document.getElementById('apply-fen-btn').addEventListener('click', () => {
        const fen = document.getElementById('fen-input').value.trim();
        if (fen) {
            const success = chessGame.loadFromFen(fen);
            if (success) {
                chessGame.clearSelection();
                moveHistory = [];
                document.getElementById('move-history').innerHTML = '';
                updateDisplay();
                updateGameStatus();
            } else {
                alert('Invalid FEN string!');
            }
        }
        cancelFenInput();
    });

    document.getElementById('cancel-fen-btn').addEventListener('click', cancelFenInput);

    // Clear history button
    document.getElementById('clear-history-btn').addEventListener('click', () => {
        moveHistory = [];
        document.getElementById('move-history').innerHTML = '';
    });

    // Example position buttons
    document.querySelectorAll('.example-btn').forEach(btn => {
        btn.addEventListener('click', () => {
            const fen = btn.dataset.fen;
            const success = chessGame.loadFromFen(fen);
            if (success) {
                chessGame.clearSelection();
                moveHistory = [];
                document.getElementById('move-history').innerHTML = '';
                updateDisplay();
                updateGameStatus();
            }
        });
    });

    // Enter key for FEN input
    document.getElementById('fen-input').addEventListener('keypress', (e) => {
        if (e.key === 'Enter') {
            document.getElementById('apply-fen-btn').click();
        } else if (e.key === 'Escape') {
            cancelFenInput();
        }
    });
}

function cancelFenInput() {
    const input = document.getElementById('fen-input');
    const applyBtn = document.getElementById('apply-fen-btn');
    const cancelBtn = document.getElementById('cancel-fen-btn');
    const loadBtn = document.getElementById('load-fen-btn');
    
    input.style.display = 'none';
    input.value = '';
    applyBtn.style.display = 'none';
    cancelBtn.style.display = 'none';
    loadBtn.style.display = 'inline-block';
}

// Initialize the game when the page loads
document.addEventListener('DOMContentLoaded', initializeGame);
