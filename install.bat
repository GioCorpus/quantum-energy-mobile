@echo off
echo ========================================
echo Quantum Energy Mobile - Installation
echo ========================================
echo.

echo [1/4] Checking prerequisites...
echo.

where node >nul 2>nul
if %errorlevel% neq 0 (
    echo ERROR: Node.js is not installed or not in PATH
    echo Please install Node.js 18+ from https://nodejs.org/
    pause
    exit /b 1
)

where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo ERROR: Rust/Cargo is not installed or not in PATH
    echo Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

echo Node.js and Rust found!
echo.

echo [2/4] Installing npm dependencies...
call npm install
if %errorlevel% neq 0 (
    echo ERROR: Failed to install npm dependencies
    pause
    exit /b 1
)
echo.

echo [3/4] Installing Tauri CLI...
cargo install tauri-cli --version "^2.0.0"
if %errorlevel% neq 0 (
    echo WARNING: Tauri CLI installation failed or already installed
)
echo.

echo [4/4] Verifying installation...
echo.
echo Checking Rust dependencies...
cargo check --manifest-path src-tauri/Cargo.toml
if %errorlevel% neq 0 (
    echo WARNING: Some Rust dependencies may need attention
)
echo.

echo ========================================
echo Installation Complete!
echo ========================================
echo.
echo To run the application:
echo   npm run tauri dev
echo.
echo To build for production:
echo   npm run tauri build
echo.
echo For Android build:
echo   npm run tauri android build
echo.
echo For iOS build:
echo   npm run tauri ios build
echo.
pause
