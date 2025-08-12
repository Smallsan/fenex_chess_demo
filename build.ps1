Set-Location "c:\Users\minec\Desktop\Repositories\fenex_chess_demo"
Write-Host "Building WASM module..."
wasm-pack build --target web --out-dir pkg --no-typescript
Write-Host "Build complete!"
