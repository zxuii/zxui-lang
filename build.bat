@echo off
setlocal enabledelayedexpansion

echo [1/2] Compiling zxui in release mode...
cargo build --release
if %ERRORLEVEL% neq 0 (
    echo Error: Compilation failed.
    exit /b %ERRORLEVEL%
)

echo [2/2] Copying zxui.exe to project root...
if exist "target\release\zxui.exe" (
    copy /Y "target\release\zxui.exe" ".\" >nul
    echo Success: zxui.exe copied to root.
) else (
    echo Error: Could not find target\release\zxui.exe
    exit /b 1
)
