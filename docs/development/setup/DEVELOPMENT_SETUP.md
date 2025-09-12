# World Foundry Development Setup Guide

This comprehensive guide will help you set up a complete development environment for World Foundry across all supported platforms.

## Prerequisites

### System Requirements

#### Minimum Requirements
- **RAM**: 8GB (16GB recommended)
- **Storage**: 20GB free space (50GB recommended)
- **CPU**: Multi-core processor (4+ cores recommended)
- **GPU**: DirectX 11/OpenGL 3.3 compatible (for graphics development)

#### Operating System Support
- **Windows**: Windows 10 version 1903 or later (Windows 11 recommended)
- **macOS**: macOS 12.0 Monterey or later (macOS 14.0 Sonoma recommended)
- **Linux**: Ubuntu 20.04 LTS or later, Fedora 35+, or equivalent

## Core Development Environment

### 1. Rust Toolchain Setup

#### Install Rust
```bash
# Install rustup (Rust installer and version manager)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload your shell or run:
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

#### Configure Rust for Cross-Platform Development
```bash
# Install stable and beta toolchains
rustup install stable
rustup install beta
rustup default stable

# Add target platforms
rustup target add x86_64-pc-windows-msvc     # Windows
rustup target add x86_64-apple-darwin        # macOS Intel
rustup target add aarch64-apple-darwin       # macOS Apple Silicon
rustup target add aarch64-apple-ios          # iOS Device
rustup target add x86_64-apple-ios           # iOS Simulator
rustup target add aarch64-linux-android      # Android ARM64
rustup target add armv7-linux-androideabi    # Android ARM32
rustup target add x86_64-linux-android       # Android x86_64
rustup target add i686-linux-android         # Android x86
rustup target add wasm32-unknown-unknown     # WebAssembly (future)

# Install essential Rust tools
cargo install cargo-watch          # File watching for development
cargo install cargo-expand         # Macro expansion debugging
cargo install cargo-audit          # Security vulnerability scanning
cargo install cargo-outdated       # Dependency update checking
cargo install cargo-tree           # Dependency tree visualization
cargo install cargo-llvm-cov       # Code coverage
cargo install cargo-criterion      # Benchmarking
cargo install cargo-cross          # Cross-compilation helper
```

### 2. Git Configuration

```bash
# Configure Git (replace with your information)
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"

# Set up useful Git aliases
git config --global alias.co checkout
git config --global alias.br branch
git config --global alias.ci commit
git config --global alias.st status
git config --global alias.unstage 'reset HEAD --'
git config --global alias.last 'log -1 HEAD'
git config --global alias.visual '!gitk'

# Configure line endings (important for cross-platform development)
git config --global core.autocrlf input    # Linux/macOS
git config --global core.autocrlf true     # Windows

# Set up GPG signing (optional but recommended)
git config --global commit.gpgsign true
git config --global user.signingkey YOUR_GPG_KEY_ID
```

### 3. Clone and Setup Repository

```bash
# Clone the repository
git clone https://github.com/Balexda/WorldFoundry.git
cd WorldFoundry

# Set up development branch
git checkout -b feature/your-feature-name

# Install pre-commit hooks (if available)
# This ensures code quality before commits
cargo install pre-commit
pre-commit install
```

### 4. Core Engine Development Setup

```bash
# Navigate to core engine
cd core

# Install dependencies and build
cargo build

# Run tests to verify setup
cargo test

# Run with optimizations (for performance testing)
cargo build --release

# Generate documentation
cargo doc --open

# Run benchmarks
cargo bench

# Check for security vulnerabilities
cargo audit

# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings
```

## Platform-Specific Setup

### Windows Development

#### Prerequisites
```powershell
# Install Visual Studio 2022 Community (or Professional/Enterprise)
# Download from: https://visualstudio.microsoft.com/downloads/
# Required workloads:
# - .NET desktop development
# - Universal Windows Platform development
# - Desktop development with C++

# Install Windows 11 SDK (latest version)
# Install Windows App SDK 1.4 or later

# Install Git for Windows
# Download from: https://git-scm.com/download/win

# Install Windows Terminal (recommended)
# Install from Microsoft Store or GitHub releases
```

#### Windows-Specific Rust Setup
```powershell
# Install Visual Studio Build Tools (if not using full VS)
# Download from: https://visualstudio.microsoft.com/visual-cpp-build-tools/

# Verify MSVC toolchain
rustup toolchain install stable-x86_64-pc-windows-msvc
rustup default stable-x86_64-pc-windows-msvc

# Install Windows-specific tools
cargo install cargo-wix  # For MSI installer creation
```

#### Windows Platform Development
```powershell
# Navigate to Windows platform
cd platforms/windows

# Restore NuGet packages
dotnet restore

# Build the application
dotnet build

# Run the application
dotnet run

# Create MSIX package
dotnet publish -c Release -r win-x64 --self-contained
```

#### Windows Development Tools
- **Visual Studio 2022**: Primary IDE for Windows development
- **Visual Studio Code**: Lightweight editor with excellent Rust support
- **Windows Terminal**: Modern terminal application
- **PowerShell 7**: Cross-platform PowerShell
- **WinUI 3 Project Templates**: Install via Visual Studio Installer

### macOS/iOS Development

#### Prerequisites
```bash
# Install Xcode from Mac App Store (required for iOS development)
# Xcode 15.0 or later required

# Install Xcode Command Line Tools
xcode-select --install

# Install Homebrew (package manager)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install essential development tools
brew install git
brew install cmake
brew install pkg-config
brew install llvm
```

#### iOS-Specific Rust Setup
```bash
# Install iOS targets
rustup target add aarch64-apple-ios
rustup target add x86_64-apple-ios

# Install cargo-lipo for universal binary creation
cargo install cargo-lipo

# Install cbindgen for header generation
cargo install cbindgen
```

#### iOS Development Setup
```bash
# Navigate to iOS platform
cd platforms/ios

# Generate Rust headers for Swift
cbindgen --config cbindgen.toml --crate world_foundry --output WorldFoundryCore/Headers/world_foundry.h

# Build Rust library for iOS
cd ../../core
cargo lipo --release

# Copy libraries to iOS project
cp target/universal/release/libworld_foundry.a ../platforms/ios/WorldFoundryCore/Libraries/

# Open Xcode project
cd ../platforms/ios
open WorldFoundry.xcodeproj
```

#### iOS Development Tools
- **Xcode**: Primary IDE for iOS development
- **iOS Simulator**: For testing without physical devices
- **Instruments**: Performance profiling and debugging
- **SF Symbols**: Apple's icon library
- **Create ML**: For machine learning features (future)

### Android Development

#### Prerequisites
```bash
# Install Java Development Kit (JDK 17 or later)
# Ubuntu/Debian:
sudo apt update
sudo apt install openjdk-17-jdk

# macOS:
brew install openjdk@17

# Windows:
# Download from Oracle or use OpenJDK distribution

# Verify Java installation
java -version
javac -version
```

#### Android Studio Setup
```bash
# Download and install Android Studio
# https://developer.android.com/studio

# After installation, open Android Studio and:
# 1. Install Android SDK (API level 34 recommended)
# 2. Install Android NDK (version 25.1.8937393 or later)
# 3. Install CMake
# 4. Install LLDB (for debugging)

# Set environment variables (add to ~/.bashrc or ~/.zshrc)
export ANDROID_HOME=$HOME/Android/Sdk
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/25.1.8937393
export PATH=$PATH:$ANDROID_HOME/tools:$ANDROID_HOME/platform-tools
```

#### Android-Specific Rust Setup
```bash
# Install Android targets
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
rustup target add i686-linux-android

# Install cargo-ndk for Android builds
cargo install cargo-ndk

# Create standalone NDK toolchain (if needed)
$ANDROID_NDK_HOME/build/tools/make_standalone_toolchain.py \
    --api 24 \
    --arch arm64 \
    --install-dir $HOME/android-ndk-arm64
```

#### Android Development Setup
```bash
# Navigate to Android platform
cd platforms/android

# Build Rust library for Android
cd ../../core

# Build for all Android architectures
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 -t x86 -o ../platforms/android/app/src/main/jniLibs build --release

# Open Android Studio project
cd ../platforms/android
# Open the project in Android Studio

# Or build from command line
./gradlew build

# Run on connected device/emulator
./gradlew installDebug
```

#### Android Development Tools
- **Android Studio**: Primary IDE for Android development
- **Android Emulator**: For testing without physical devices
- **ADB (Android Debug Bridge)**: Command-line debugging tool
- **Gradle**: Build system
- **ProGuard/R8**: Code obfuscation and optimization

## Development Workflow

### 1. Daily Development Routine

```bash
# Start of day - update and check status
git pull origin main
git status
cargo check  # Quick compile check

# During development - continuous testing
cargo watch -x test  # Auto-run tests on file changes
cargo watch -x "run --example basic_usage"  # Auto-run examples

# Before committing
cargo fmt        # Format code
cargo clippy     # Run linter
cargo test       # Run all tests
cargo doc        # Update documentation
```

### 2. Feature Development Workflow

```bash
# Create feature branch
git checkout -b feature/new-world-generator

# Make changes and test incrementally
cargo test
cargo build --release

# Test on multiple platforms (if available)
cargo build --target x86_64-pc-windows-msvc
cargo build --target x86_64-apple-darwin
cargo build --target aarch64-linux-android

# Commit changes
git add .
git commit -m "feat: implement new world generation algorithm

- Add Perlin noise-based terrain generation
- Implement biome classification system
- Add unit tests for generation algorithms
- Update documentation

Closes #123"

# Push and create pull request
git push origin feature/new-world-generator
```

### 3. Testing Strategy

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# Benchmarks
cargo bench

# Code coverage
cargo llvm-cov --html
# Open htmlcov/index.html to view coverage report

# Cross-platform testing
cargo test --target x86_64-pc-windows-msvc
cargo test --target x86_64-apple-darwin

# Memory leak detection (with valgrind on Linux)
cargo build
valgrind --tool=memcheck --leak-check=full ./target/debug/world_foundry_cli
```

## IDE and Editor Setup

### Visual Studio Code (Recommended for Rust)

#### Essential Extensions
```json
{
  "recommendations": [
    "rust-lang.rust-analyzer",      // Rust language server
    "vadimcn.vscode-lldb",          // Debugging support
    "serayuzgur.crates",            // Cargo.toml management
    "tamasfe.even-better-toml",     // TOML syntax highlighting
    "ms-vscode.cmake-tools",        // CMake support
    "ms-vscode.cpptools",           // C++ support for FFI
    "github.copilot",               // AI code completion (optional)
    "github.vscode-pull-request-github", // GitHub integration
    "eamodio.gitlens",              // Enhanced Git support
    "ms-vsliveshare.vsliveshare"    // Live collaboration
  ]
}
```

#### VS Code Settings
```json
{
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.procMacro.enable": true,
  "rust-analyzer.completion.addCallParentheses": false,
  "editor.formatOnSave": true,
  "editor.codeActionsOnSave": {
    "source.fixAll.rust-analyzer": true
  },
  "files.watcherExclude": {
    "**/target/**": true
  }
}
```

#### Debugging Configuration
```json
// .vscode/launch.json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug Core Engine",
      "cargo": {
        "args": ["build", "--bin=world_foundry_cli"],
        "filter": {
          "name": "world_foundry_cli",
          "kind": "bin"
        }
      },
      "args": ["--help"],
      "cwd": "${workspaceFolder}/core"
    },
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug Tests",
      "cargo": {
        "args": ["test", "--no-run", "--bin=world_foundry_cli"],
        "filter": {
          "name": "world_foundry_cli",
          "kind": "bin"
        }
      },
      "args": [],
      "cwd": "${workspaceFolder}/core"
    }
  ]
}
```

### JetBrains IDEs

#### CLion (for Rust development)
- Install Rust plugin
- Configure Rust toolchain in Settings → Languages & Frameworks → Rust
- Set up run configurations for different targets

#### IntelliJ IDEA (for Android development)
- Install Android plugin
- Configure Android SDK path
- Set up run configurations for Android builds

## Environment Variables

### Development Environment Variables

```bash
# Add to ~/.bashrc, ~/.zshrc, or equivalent

# Rust configuration
export RUST_BACKTRACE=1              # Enable backtraces
export RUST_LOG=debug                # Enable debug logging
export CARGO_INCREMENTAL=1           # Enable incremental compilation

# Platform-specific paths
export ANDROID_HOME=$HOME/Android/Sdk
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/25.1.8937393
export JAVA_HOME=/usr/lib/jvm/java-17-openjdk-amd64  # Linux
# export JAVA_HOME=/Library/Java/JavaVirtualMachines/openjdk-17.jdk/Contents/Home  # macOS

# Development tools
export PATH=$PATH:$HOME/.cargo/bin
export PATH=$PATH:$ANDROID_HOME/tools:$ANDROID_HOME/platform-tools

# Graphics development (Linux)
export DISPLAY=:0                    # For X11 applications
export WAYLAND_DISPLAY=wayland-0     # For Wayland applications

# Performance profiling
export CARGO_PROFILE_RELEASE_DEBUG=true  # Include debug info in release builds
```

## Troubleshooting Common Setup Issues

### Rust Compilation Issues

#### Linker Errors
```bash
# Linux: Install build essentials
sudo apt install build-essential

# macOS: Install Xcode command line tools
xcode-select --install

# Windows: Install Visual Studio Build Tools
# Download from Microsoft website
```

#### Target Not Found
```bash
# Reinstall target
rustup target remove x86_64-pc-windows-msvc
rustup target add x86_64-pc-windows-msvc
```

### Platform-Specific Issues

#### Windows: MSVC Not Found
```powershell
# Install Visual Studio Build Tools
# Or set environment variable
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"
```

#### macOS: iOS Simulator Issues
```bash
# Reset iOS Simulator
xcrun simctl erase all

# Rebuild iOS targets
cargo clean
rustup target add x86_64-apple-ios --force
```

#### Android: NDK Issues
```bash
# Verify NDK installation
ls $ANDROID_NDK_HOME

# Reinstall NDK through Android Studio
# Tools → SDK Manager → SDK Tools → NDK (Side by side)
```

### Performance Issues

#### Slow Compilation
```bash
# Enable parallel compilation
export CARGO_BUILD_JOBS=8  # Adjust based on CPU cores

# Use faster linker (Linux)
sudo apt install lld
export RUSTFLAGS="-C link-arg=-fuse-ld=lld"

# Use faster linker (macOS)
export RUSTFLAGS="-C link-arg=-fuse-ld=/usr/bin/ld"
```

#### Memory Issues During Build
```bash
# Reduce parallel jobs
export CARGO_BUILD_JOBS=2

# Increase system swap space (Linux)
sudo fallocate -l 4G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
```

## Next Steps

After completing the development setup:

1. **Read the Architecture Documentation**: [Architecture Overview](../architecture/ARCHITECTURE.md)
2. **Follow Platform-Specific Guides**:
   - [Windows Development](WINDOWS_DEVELOPMENT.md)
   - [iOS Development](IOS_DEVELOPMENT.md)
   - [Android Development](ANDROID_DEVELOPMENT.md)
3. **Learn the Deployment Process**: [Deployment Guide](../deployment/DEPLOYMENT_GUIDE.md)
4. **Set Up Debugging**: [Debugging Guide](../debugging/DEBUGGING_GUIDE.md)

## Getting Help

- **GitHub Issues**: Report bugs and request features
- **GitHub Discussions**: Ask questions and share ideas
- **Documentation**: Comprehensive guides and API reference
- **Community**: Join our development community

Remember to keep your development environment updated and follow the project's contribution guidelines for the best development experience.