# Debug Deployment Guide

This guide covers deploying debug builds of World Foundry to all supported platforms for development and testing purposes.

## Overview

Debug deployments are essential for:
- **Development Testing**: Quick iteration and testing during development
- **Feature Validation**: Verifying new features work correctly across platforms
- **Bug Reproduction**: Reproducing and debugging issues in realistic environments
- **Performance Profiling**: Analyzing performance characteristics with debug symbols
- **Integration Testing**: Testing platform-specific integrations

## Prerequisites

Before deploying debug builds, ensure you have completed the development setup for your target platforms:
- [Development Setup Guide](../setup/DEVELOPMENT_SETUP.md)
- [Windows Development Guide](../setup/WINDOWS_DEVELOPMENT.md)
- [iOS Development Guide](../setup/IOS_DEVELOPMENT.md)
- [Android Development Guide](../setup/ANDROID_DEVELOPMENT.md)

## Core Engine Debug Build

### Building Debug Core

```bash
# Navigate to core engine
cd core

# Build debug version with full symbols
cargo build --target x86_64-pc-windows-msvc    # Windows
cargo build --target x86_64-apple-darwin       # macOS Intel
cargo build --target aarch64-apple-darwin      # macOS Apple Silicon
cargo build --target aarch64-apple-ios         # iOS Device
cargo build --target x86_64-apple-ios          # iOS Simulator
cargo build --target aarch64-linux-android     # Android ARM64
cargo build --target armv7-linux-androideabi   # Android ARM32
cargo build --target x86_64-linux-android      # Android x86_64
cargo build --target i686-linux-android        # Android x86

# Enable debug logging
export RUST_LOG=debug
export RUST_BACKTRACE=full

# Test core functionality
cargo test
cargo run --example basic_usage
```

### Debug Configuration

```toml
# Cargo.toml - Debug profile configuration
[profile.dev]
debug = true          # Include debug symbols
opt-level = 0         # No optimizations for faster builds
overflow-checks = true # Runtime integer overflow checks
lto = false          # Disable link-time optimization
panic = "unwind"     # Enable stack unwinding for better debugging
incremental = true   # Enable incremental compilation

[profile.dev.package."*"]
opt-level = 2        # Optimize dependencies for better performance

# Custom debug profile with some optimizations
[profile.debug-optimized]
inherits = "dev"
opt-level = 1        # Basic optimizations
debug = true         # Keep debug symbols
```

## Windows Debug Deployment

### Local Development Deployment

```powershell
# Navigate to Windows platform
cd platforms\windows

# Build debug version
dotnet build --configuration Debug

# Run locally
dotnet run --configuration Debug

# Or run with additional debugging
$env:RUST_LOG = "debug"
$env:RUST_BACKTRACE = "full"
dotnet run --configuration Debug --verbosity diagnostic
```

### Debug Package Creation

```powershell
# Create debug package with symbols
dotnet publish --configuration Debug --runtime win-x64 --self-contained true --output .\publish\debug

# Create MSIX package for sideloading
dotnet publish --configuration Debug --runtime win-x64 /p:PublishProfile=win-x64-debug.pubxml

# Install debug package locally
Add-AppxPackage -Path ".\publish\debug\WorldFoundry_1.0.0.0_x64_Debug.msix" -ForceApplicationShutdown
```

### Windows Debugging Tools

```powershell
# Install debugging tools
winget install Microsoft.WinDbg
winget install Microsoft.ProcessMonitor
winget install Microsoft.ProcessExplorer

# Enable application verifier (for advanced debugging)
appverif.exe /verify WorldFoundry.exe

# Collect ETW traces
wpr -start GeneralProfile
# Run your application
wpr -stop trace.etl
```

### Deployment to Test Machines

```powershell
# Create deployment script
# Deploy-Debug.ps1
param(
    [string]$TargetMachine,
    [string]$Username,
    [string]$Password
)

# Copy files to target machine
$session = New-PSSession -ComputerName $TargetMachine -Credential (Get-Credential)
Copy-Item -Path ".\publish\debug\*" -Destination "C:\WorldFoundry\Debug\" -ToSession $session -Recurse -Force

# Install on target machine
Invoke-Command -Session $session -ScriptBlock {
    Add-AppxPackage -Path "C:\WorldFoundry\Debug\WorldFoundry_1.0.0.0_x64_Debug.msix" -ForceApplicationShutdown
}

Remove-PSSession $session
```

## iOS Debug Deployment

### Simulator Deployment

```bash
# Build for iOS Simulator
cd platforms/ios

# Build Rust core for simulator
cd ../../core
cargo build --target x86_64-apple-ios  # Intel Macs
cargo build --target aarch64-apple-ios-sim  # Apple Silicon Macs

# Copy library to iOS project
cp target/x86_64-apple-ios/debug/libworld_foundry.a ../platforms/ios/WorldFoundryCore/Libraries/libworld_foundry_sim.a

# Build and run in simulator
cd ../platforms/ios
xcodebuild -project WorldFoundry.xcodeproj -scheme WorldFoundry -destination 'platform=iOS Simulator,name=iPhone 15 Pro' build

# Or use Xcode
open WorldFoundry.xcodeproj
# Select simulator target and press Cmd+R
```

### Device Deployment

```bash
# Build for iOS device
cd core
cargo build --target aarch64-apple-ios

# Copy library to iOS project
cp target/aarch64-apple-ios/debug/libworld_foundry.a ../platforms/ios/WorldFoundryCore/Libraries/libworld_foundry_device.a

# Create universal library
cd ../platforms/ios
lipo -create \
    WorldFoundryCore/Libraries/libworld_foundry_sim.a \
    WorldFoundryCore/Libraries/libworld_foundry_device.a \
    -output WorldFoundryCore/Libraries/libworld_foundry.a

# Build and deploy to device
xcodebuild -project WorldFoundry.xcodeproj \
    -scheme WorldFoundry \
    -destination 'platform=iOS,name=Your iPhone' \
    -configuration Debug \
    build \
    install

# Or use Xcode with connected device
# Select your device and press Cmd+R
```

### iOS Debug Configuration

```swift
// Debug configuration in iOS project
#if DEBUG
import os.log

let logger = Logger(subsystem: "com.balexda.worldfoundry", category: "debug")

// Enable debug logging
func setupDebugLogging() {
    logger.info("Debug mode enabled")
    
    // Enable Rust debug logging
    setenv("RUST_LOG", "debug", 1)
    setenv("RUST_BACKTRACE", "full", 1)
}

// Debug UI overlays
struct DebugOverlay: View {
    @State private var showDebugInfo = false
    
    var body: some View {
        VStack {
            if showDebugInfo {
                VStack(alignment: .leading) {
                    Text("Debug Information")
                        .font(.headline)
                    Text("Memory Usage: \(getMemoryUsage())")
                    Text("FPS: \(getCurrentFPS())")
                    Text("Core Engine Status: \(getCoreEngineStatus())")
                }
                .padding()
                .background(Color.black.opacity(0.8))
                .foregroundColor(.white)
                .cornerRadius(8)
            }
        }
        .onTapGesture(count: 3) {
            showDebugInfo.toggle()
        }
    }
}
#endif
```

### TestFlight Deployment

```bash
# Create archive for TestFlight
xcodebuild -project WorldFoundry.xcodeproj \
    -scheme WorldFoundry \
    -configuration Debug \
    -destination generic/platform=iOS \
    archive \
    -archivePath WorldFoundry-Debug.xcarchive

# Export for TestFlight
xcodebuild -exportArchive \
    -archivePath WorldFoundry-Debug.xcarchive \
    -exportPath ./export \
    -exportOptionsPlist ExportOptions-Debug.plist

# Upload to TestFlight
xcrun altool --upload-app \
    --type ios \
    --file "./export/WorldFoundry.ipa" \
    --username "your-apple-id@example.com" \
    --password "app-specific-password"
```

## Android Debug Deployment

### Local Device/Emulator Deployment

```bash
# Build Rust core for Android
cd core
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 -t x86 \
    -o ../platforms/android/app/src/main/jniLibs \
    build

# Build Android debug APK
cd ../platforms/android
./gradlew assembleDebug

# Install on connected device/emulator
adb install app/build/outputs/apk/debug/app-debug.apk

# Or install and run
./gradlew installDebug
adb shell am start -n com.balexda.worldfoundry/.MainActivity
```

### Android Debug Configuration

```kotlin
// Application.kt - Debug configuration
class WorldFoundryApplication : Application() {
    override fun onCreate() {
        super.onCreate()
        
        if (BuildConfig.DEBUG) {
            setupDebugLogging()
            enableStrictMode()
            setupLeakCanary()
        }
    }
    
    private fun setupDebugLogging() {
        // Enable native logging
        System.setProperty("rust.log", "debug")
        System.setProperty("rust.backtrace", "full")
        
        // Enable Android logging
        if (BuildConfig.DEBUG) {
            Timber.plant(Timber.DebugTree())
        }
    }
    
    private fun enableStrictMode() {
        StrictMode.setThreadPolicy(
            StrictMode.ThreadPolicy.Builder()
                .detectAll()
                .penaltyLog()
                .build()
        )
        
        StrictMode.setVmPolicy(
            StrictMode.VmPolicy.Builder()
                .detectAll()
                .penaltyLog()
                .build()
        )
    }
}
```

### Debug Build Variants

```kotlin
// build.gradle.kts - Debug build variants
android {
    buildTypes {
        debug {
            isDebuggable = true
            isMinifyEnabled = false
            applicationIdSuffix = ".debug"
            versionNameSuffix = "-debug"
            
            buildConfigField("boolean", "ENABLE_LOGGING", "true")
            buildConfigField("String", "API_BASE_URL", "\"https://debug-api.worldfoundry.com\"")
            
            ndk {
                debugSymbolLevel = "FULL"
            }
        }
        
        create("staging") {
            initWith(debug)
            applicationIdSuffix = ".staging"
            versionNameSuffix = "-staging"
            isDebuggable = false
            isMinifyEnabled = true
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
        }
    }
}
```

### Firebase App Distribution

```bash
# Install Firebase CLI
npm install -g firebase-tools

# Login to Firebase
firebase login

# Build and distribute debug APK
./gradlew assembleDebug
firebase appdistribution:distribute \
    app/build/outputs/apk/debug/app-debug.apk \
    --app 1:123456789:android:abcdef \
    --groups "testers" \
    --release-notes "Debug build with latest features"
```

### Android Device Testing

```bash
# Install ADB tools
# Ubuntu/Debian: sudo apt install android-tools-adb
# macOS: brew install android-platform-tools
# Windows: Download from Android SDK

# Enable developer options on device
# Settings → About phone → Tap "Build number" 7 times
# Settings → Developer options → Enable "USB debugging"

# Connect device and verify
adb devices

# Install debug APK
adb install -r app/build/outputs/apk/debug/app-debug.apk

# View logs
adb logcat | grep WorldFoundry

# Debug with Chrome DevTools (for WebView debugging)
# Open chrome://inspect in Chrome browser
```

## Cross-Platform Debug Testing

### Automated Testing Pipeline

```yaml
# .github/workflows/debug-deployment.yml
name: Debug Deployment Testing

on:
  push:
    branches: [ develop, feature/* ]
  pull_request:
    branches: [ main, develop ]

jobs:
  build-core:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build core engine
        run: |
          cd core
          cargo build --all-targets
          cargo test

  test-windows:
    runs-on: windows-latest
    needs: build-core
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-dotnet@v3
        with:
          dotnet-version: '8.0.x'
      - name: Build Windows debug
        run: |
          cd platforms/windows
          dotnet build --configuration Debug
      - name: Run Windows tests
        run: |
          cd platforms/windows
          dotnet test --configuration Debug

  test-ios:
    runs-on: macos-latest
    needs: build-core
    steps:
      - uses: actions/checkout@v4
      - name: Build iOS debug
        run: |
          cd platforms/ios
          xcodebuild -project WorldFoundry.xcodeproj -scheme WorldFoundry -destination 'platform=iOS Simulator,name=iPhone 15 Pro' build

  test-android:
    runs-on: ubuntu-latest
    needs: build-core
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-java@v3
        with:
          java-version: '17'
          distribution: 'temurin'
      - name: Build Android debug
        run: |
          cd platforms/android
          ./gradlew assembleDebug
```

### Debug Deployment Scripts

```bash
#!/bin/bash
# deploy-debug.sh - Cross-platform debug deployment script

set -e

PLATFORMS=("windows" "ios" "android")
BUILD_TYPE="debug"

echo "🚀 Starting debug deployment for all platforms..."

# Build core engine
echo "📦 Building core engine..."
cd core
cargo build
cargo test
cd ..

# Deploy to each platform
for platform in "${PLATFORMS[@]}"; do
    echo "🔨 Deploying to $platform..."
    
    case $platform in
        "windows")
            cd platforms/windows
            dotnet build --configuration Debug
            dotnet test --configuration Debug
            echo "✅ Windows debug build completed"
            cd ../..
            ;;
        "ios")
            cd platforms/ios
            xcodebuild -project WorldFoundry.xcodeproj -scheme WorldFoundry -destination 'platform=iOS Simulator,name=iPhone 15 Pro' build
            echo "✅ iOS debug build completed"
            cd ../..
            ;;
        "android")
            cd platforms/android
            ./gradlew assembleDebug
            ./gradlew testDebugUnitTest
            echo "✅ Android debug build completed"
            cd ../..
            ;;
    esac
done

echo "🎉 All debug deployments completed successfully!"
```

## Debug Monitoring and Logging

### Centralized Logging

```rust
// Core engine logging configuration
use log::{debug, info, warn, error};
use env_logger;

pub fn init_debug_logging() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_secs()
        .format_module_path(true)
        .init();
    
    info!("Debug logging initialized");
    debug!("Core engine version: {}", env!("CARGO_PKG_VERSION"));
}

// Platform-specific logging bridges
#[no_mangle]
pub extern "C" fn world_foundry_set_log_level(level: i32) {
    let log_level = match level {
        0 => log::LevelFilter::Off,
        1 => log::LevelFilter::Error,
        2 => log::LevelFilter::Warn,
        3 => log::LevelFilter::Info,
        4 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };
    
    log::set_max_level(log_level);
}
```

### Performance Monitoring

```rust
// Performance monitoring for debug builds
use std::time::{Duration, Instant};
use std::collections::HashMap;

pub struct PerformanceMonitor {
    timers: HashMap<String, Instant>,
    measurements: HashMap<String, Vec<Duration>>,
}

impl PerformanceMonitor {
    pub fn start_timer(&mut self, name: &str) {
        self.timers.insert(name.to_string(), Instant::now());
    }
    
    pub fn end_timer(&mut self, name: &str) {
        if let Some(start_time) = self.timers.remove(name) {
            let duration = start_time.elapsed();
            self.measurements
                .entry(name.to_string())
                .or_insert_with(Vec::new)
                .push(duration);
            
            debug!("⏱️ {}: {:?}", name, duration);
        }
    }
    
    pub fn report(&self) {
        for (name, measurements) in &self.measurements {
            let avg = measurements.iter().sum::<Duration>() / measurements.len() as u32;
            let min = measurements.iter().min().unwrap();
            let max = measurements.iter().max().unwrap();
            
            info!("📊 {}: avg={:?}, min={:?}, max={:?}, samples={}", 
                  name, avg, min, max, measurements.len());
        }
    }
}
```

## Troubleshooting Debug Deployments

### Common Issues

#### 1. Native Library Loading Issues
```bash
# Verify library architecture
file target/debug/libworld_foundry.so  # Linux
file target/debug/libworld_foundry.dylib  # macOS
file target/debug/world_foundry.dll  # Windows

# Check dependencies
ldd target/debug/libworld_foundry.so  # Linux
otool -L target/debug/libworld_foundry.dylib  # macOS
dumpbin /dependents target/debug/world_foundry.dll  # Windows
```

#### 2. Debug Symbol Issues
```bash
# Verify debug symbols are present
objdump -h target/debug/libworld_foundry.so | grep debug  # Linux
dsymutil target/debug/libworld_foundry.dylib  # macOS
```

#### 3. Platform-Specific Debugging

**Windows:**
```powershell
# Check event logs
Get-WinEvent -LogName Application | Where-Object {$_.ProviderName -eq "WorldFoundry"}

# Use Process Monitor
procmon.exe /AcceptEula /Minimized /BackingFile C:\temp\worldfoundry.pml
```

**iOS:**
```bash
# View device logs
xcrun devicectl list devices
xcrun devicectl device logs --device YOUR_DEVICE_ID

# Simulator logs
xcrun simctl spawn booted log stream --predicate 'subsystem == "com.balexda.worldfoundry"'
```

**Android:**
```bash
# Filtered logcat
adb logcat -s WorldFoundry:D *:S

# Crash dumps
adb bugreport bugreport.zip
```

## Security Considerations for Debug Builds

### Debug-Only Features

```rust
// Conditional compilation for debug features
#[cfg(debug_assertions)]
pub fn enable_debug_features() {
    // Enable additional logging
    // Expose internal APIs for testing
    // Add performance counters
    // Enable memory debugging
}

#[cfg(not(debug_assertions))]
pub fn enable_debug_features() {
    // No-op in release builds
}
```

### Sensitive Data Handling

```rust
// Sanitize sensitive data in debug logs
use log::debug;

pub fn log_map_import(path: &str, success: bool) {
    #[cfg(debug_assertions)]
    {
        let sanitized_path = path.split('/').last().unwrap_or("unknown");
        debug!("Map import: {} -> {}", sanitized_path, success);
    }
    
    #[cfg(not(debug_assertions))]
    {
        // Minimal logging in release
        if !success {
            log::warn!("Map import failed");
        }
    }
}
```

## Next Steps

After successful debug deployment:

1. **Performance Testing**: [Performance Testing Guide](../testing/PERFORMANCE_TESTING.md)
2. **Integration Testing**: [Integration Testing Guide](../testing/INTEGRATION_TESTING.md)
3. **Release Deployment**: [Release Deployment Guide](RELEASE_DEPLOYMENT.md)
4. **Monitoring**: [Production Monitoring Guide](../monitoring/PRODUCTION_MONITORING.md)

## Additional Resources

- [Rust Debugging Guide](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [Visual Studio Debugging](https://docs.microsoft.com/en-us/visualstudio/debugger/)
- [Xcode Debugging](https://developer.apple.com/documentation/xcode/debugging)
- [Android Debugging](https://developer.android.com/studio/debug)
- [Cross-Platform Debugging Best Practices](https://docs.microsoft.com/en-us/dotnet/core/diagnostics/)