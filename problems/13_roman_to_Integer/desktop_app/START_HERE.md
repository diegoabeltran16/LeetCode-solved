# 🏛️ RomanForge Desktop Application
## **A Three-City Collaboration: The Hero's Desktop Journey**

> *Where the hero learns to build bridges between worlds,*  
> *uniting C#, Rust, and the desktop realm.*

---

## 📋 **What is RomanForge?**

**RomanForge** is a desktop application that brings the hero's Roman numeral mastery to life through a graphical interface. Built using three programming language cities working in perfect harmony:

- **C# (The Humanoid City)**: WPF UI, validation, user experience
- **Rust (The Titan-Machine Citadel)**: Core conversion engine, safety guarantees  
- **C++ (The Hybrid City)**: Reserved for future interop optimizations

**Core Feature:** Real-time bi-directional Roman ↔ Integer conversion with validation

---

## 🚀 **Quick Start**

### **The Fastest Way (Automated Script)**

### **The Fastest Way (Automated Script)**

```powershell
# 1. Navigate to the desktop_app folder
cd C:\Users\Ohana\Documents\Repositorios\LeetCode-solved\problems\13_roman_to_Integer\desktop_app

# 2. Run the automated build script  
.\build-and-run.ps1
```

✨ The script automatically:
1. Checks if Rust/Cargo is installed
2. Builds the Rust core engine (`roman_engine.dll`)
3. Builds the C# WPF UI (`RomanForge.exe`)
4. Copies the Rust DLL to the correct output folder
5. Launches the application

---

## ⚙️ **Prerequisites**

Before running RomanForge, ensure you have:

- ✅ **Rust toolchain** ([Install from rustup.rs](https://rustup.rs))
- ✅ **.NET 9.0 SDK** (or .NET 8.0+)
- ✅ **Windows 10/11**
- 🎨 **Optional**: Visual Studio 2022 (for development)

---

---

## 🔧 **Manual Build Instructions**

If you prefer step-by-step control:

```powershell
# Navigate to desktop_app folder
cd C:\Users\Ohana\Documents\Repositorios\LeetCode-solved\problems\13_roman_to_Integer\desktop_app

# Build Rust engine
cd roman_engine
cargo build --release
cd ..

# Build C# UI
dotnet build .\RomanForge.UI\RomanForge.UI.csproj -c Debug

# Copy DLL (adjust for your .NET version: net9.0-windows or net8.0-windows)
Copy-Item -Path .\roman_engine\target\release\roman_engine.dll -Destination .\RomanForge.UI\bin\Debug\net9.0-windows\ -Force

# Run the application
dotnet run --project .\RomanForge.UI\RomanForge.UI.csproj --no-build -c Debug
```

### **Open in Visual Studio**

```powershell
# From desktop_app folder
start RomanForge.sln
```

Then press **F5** to build and run.

---

## 🏗️ **Project Architecture**

```
desktop_app/
├── build-and-run.ps1          ← Automated build script
├── RomanForge.sln             ← Visual Studio solution
├── START_HERE.md              ← This file
├── DESIGN.md                  ← Detailed architecture docs
│
├── roman_engine/              ← 🦀 Rust Core (Titan-Machine Citadel)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs            ← FFI exports for C# interop
│   │   ├── converter.rs      ← Core conversion algorithms
│   │   ├── validator.rs      ← Input validation
│   │   └── algorithms/       ← Multiple algorithm variants
│   └── target/release/
│       └── roman_engine.dll  ← Built DLL
│
└── RomanForge.UI/             ← 🏢 C# WPF UI (Humanoid City)
    ├── RomanForge.UI.csproj
    ├── App.xaml               ← Application entry
    ├── MainWindow.xaml        ← Main UI layout
    ├── MainWindow.xaml.cs     ← Code-behind
    ├── ViewModels/
    │   └── ConverterViewModel.cs  ← MVVM logic
    ├── Validators/
    │   └── RomanNumeralValidator.cs
    └── Interop/
        └── NativeEngine.cs    ← P/Invoke to Rust
```

---

## 🌆 **How the Three Cities Collaborate**

### **C# - The Humanoid City (UI Layer)**
- **Responsibilities**: User interface, validation, error handling, MVVM structure
- **Strengths**: Rich WPF framework, type safety, enterprise patterns
- **Files**: `RomanForge.UI/` folder

### **Rust - The Titan-Machine Citadel (Core Engine)**
- **Responsibilities**: Roman ↔ Integer conversion, memory-safe algorithms, FFI exports
- **Strengths**: Zero-cost abstractions, guaranteed correctness, no garbage collection
- **Files**: `roman_engine/` folder

### **C++ - The Hybrid City (Future Bridge)**
- **Reserved For**: Advanced interop optimizations, performance caching
- **Strengths**: Seamless native/managed bridging when needed

---

## 🎯 **Features**

✅ **Bi-directional Conversion**
- Roman → Integer (e.g., MCMXCIV → 1994)
- Integer → Roman (e.g., 2024 → MMXXIV)

✅ **Real-time Validation**
- Visual feedback (✓ valid / ✗ invalid)
- Range checking (1-3999)
- Input sanitization

✅ **Educational Design**
- Clean separation of concerns
- Demonstrates cross-language FFI
- MVVM pattern in action

---

## 🐛 **Troubleshooting**

### ❌ **Error: "cargo is not recognized"**
**Problem**: Rust toolchain not installed  
**Solution**:
1. Visit https://rustup.rs
2. Download and run `rustup-init.exe`
3. Choose default installation options
4. **Restart PowerShell**
5. Run the build script again

### ❌ **Error: "File not found" or "Path not found"**
**Problem**: Running commands from wrong directory  
**Solution**: Always navigate to `desktop_app` first:
```powershell
cd C:\Users\Ohana\Documents\Repositorios\LeetCode-solved\problems\13_roman_to_Integer\desktop_app
```

### ❌ **Error: "BadImageFormatException" or "DllNotFoundException"**
**Problem**: Architecture mismatch (x86 vs x64)  
**Solution**: Ensure Rust uses MSVC x64 toolchain:
```powershell
rustup default stable-x86_64-pc-windows-msvc
cd roman_engine
cargo clean
cargo build --release
```

### ❌ **Error: "This script is not digitally signed"**
**Problem**: PowerShell execution policy  
**Solution**:
```powershell
powershell -ExecutionPolicy Bypass -File .\build-and-run.ps1
```

---

## 📚 **Learn More**

- **DESIGN.md** - Detailed architecture documentation
- **13_roman_to_integer.md** - The hero's journey narrative

---

## 🏆 **The Hero's Desktop Achievement**

*This app represents the hero's ability to unite different cities into a single tool. By building RomanForge, the hero learned to bridge worlds, validate boundaries, and architect solutions where each city contributes its unique strength.*

**🚀 Ready? Run `.\build-and-run.ps1` and watch the three cities come to life!**
