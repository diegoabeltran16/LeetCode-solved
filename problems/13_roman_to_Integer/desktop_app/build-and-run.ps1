# RomanForge Build and Run Script
# Navigate to desktop_app folder first, then run this script

Write-Host "🏛️ RomanForge - Three Cities Builder" -ForegroundColor Cyan
Write-Host "====================================`n" -ForegroundColor Cyan

# Verify we're in the right directory
if (!(Test-Path ".\RomanForge.sln")) {
    Write-Host "❌ Error: Not in desktop_app directory!" -ForegroundColor Red
    Write-Host "   Please run: cd C:\Users\Ohana\Documents\Repositorios\LeetCode-solved\problems\13_roman_to_Integer\desktop_app" -ForegroundColor Yellow
    exit 1
}

# Step 1: Verify Rust toolchain
Write-Host "🔍 Step 1: Checking Rust toolchain..." -ForegroundColor Yellow
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Rust/Cargo not found!" -ForegroundColor Red
    Write-Host "   Please install Rust from: https://rustup.rs" -ForegroundColor Red
    Write-Host "   After installation, restart PowerShell and run this script again." -ForegroundColor Red
    exit 1
}
Write-Host "✓ Cargo found: $(cargo --version)" -ForegroundColor Green

# Step 2: Build Rust engine
Write-Host "`n🦀 Step 2: Building Rust core engine (Titan-Machine Citadel)..." -ForegroundColor Yellow
Push-Location .\roman_engine
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Rust build failed!" -ForegroundColor Red
    Pop-Location
    exit 1
}
Pop-Location
Write-Host "✓ Rust engine built successfully" -ForegroundColor Green

# Step 3: Verify Rust DLL exists
$rustDll = ".\roman_engine\target\release\roman_engine.dll"
if (!(Test-Path $rustDll)) {
    Write-Host "❌ roman_engine.dll not found at: $rustDll" -ForegroundColor Red
    exit 1
}
Write-Host "✓ Found roman_engine.dll" -ForegroundColor Green

# Step 4: Build C# UI
Write-Host "`n🏢 Step 3: Building C# UI (Humanoid City)..." -ForegroundColor Yellow
dotnet build .\RomanForge.UI\RomanForge.UI.csproj -c Debug
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ C# build failed!" -ForegroundColor Red
    exit 1
}
Write-Host "✓ C# UI built successfully" -ForegroundColor Green

# Step 5: Copy Rust DLL to C# output
Write-Host "`n🌉 Step 4: Copying Rust DLL to C# output directory..." -ForegroundColor Yellow
$outputDir = ".\RomanForge.UI\bin\Debug\net9.0-windows\"
if (!(Test-Path $outputDir)) {
    Write-Host "❌ C# output directory not found: $outputDir" -ForegroundColor Red
    exit 1
}

Copy-Item -Path $rustDll -Destination $outputDir -Force
if (Test-Path (Join-Path $outputDir "roman_engine.dll")) {
    Write-Host "✓ DLL copied successfully" -ForegroundColor Green
} else {
    Write-Host "❌ Failed to copy DLL" -ForegroundColor Red
    exit 1
}

# Step 6: Run the application
Write-Host "`n🚀 Step 5: Launching RomanForge..." -ForegroundColor Yellow
Write-Host "   Press Ctrl+C to stop`n" -ForegroundColor Gray

dotnet run --project .\RomanForge.UI\RomanForge.UI.csproj --no-build -c Debug
