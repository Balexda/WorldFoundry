# iOS Development Guide

This guide covers setting up and developing the iOS platform implementation of World Foundry using SwiftUI and the iOS SDK.

## Prerequisites

### System Requirements
- **OS**: macOS 12.0 Monterey or later
- **Recommended**: macOS 14.0 Sonoma or later
- **RAM**: 8GB minimum, 16GB recommended
- **Storage**: 15GB free space for Xcode and tools
- **Hardware**: Intel Mac or Apple Silicon Mac

### Required Software

#### 1. Xcode
Download and install Xcode from the Mac App Store:
- **Version**: Xcode 15.0 or later
- **Size**: ~10GB download, ~15GB installed
- **Includes**: iOS SDK, Simulator, Interface Builder, Instruments

**Installation Steps:**
1. Open Mac App Store
2. Search for "Xcode"
3. Click "Get" or "Install"
4. Wait for download and installation (may take 1-2 hours)
5. Launch Xcode and accept license agreements

#### 2. Command Line Tools
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Verify installation
xcode-select -p
# Should output: /Applications/Xcode.app/Contents/Developer
```

#### 3. Additional Development Tools
```bash
# Install Homebrew (package manager)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install essential tools
brew install git
brew install cmake
brew install pkg-config
brew install llvm

# Install iOS-specific tools
brew install ios-deploy        # Deploy to iOS devices
brew install libimobiledevice  # Communicate with iOS devices
brew install ideviceinstaller  # Install apps on iOS devices
```

## Development Environment Setup

### 1. Clone and Setup Repository

```bash
# Clone the repository
git clone https://github.com/Balexda/WorldFoundry.git
cd WorldFoundry

# Navigate to iOS platform
cd platforms/ios
```

### 2. Rust Core Engine Setup

```bash
# Navigate to core engine
cd ../../core

# Install iOS-specific Rust targets
rustup target add aarch64-apple-ios      # iOS Device (ARM64)
rustup target add x86_64-apple-ios       # iOS Simulator (Intel)
rustup target add aarch64-apple-ios-sim  # iOS Simulator (Apple Silicon)

# Install cargo-lipo for universal binary creation
cargo install cargo-lipo

# Install cbindgen for header generation
cargo install cbindgen

# Build the core engine for iOS
cargo lipo --release

# Generate C headers for Swift integration
cbindgen --config cbindgen.toml --crate world_foundry --output ../platforms/ios/WorldFoundryCore/Headers/world_foundry.h
```

### 3. iOS Project Setup

```bash
# Navigate to iOS platform
cd ../platforms/ios

# Copy Rust libraries to iOS project
cp ../../core/target/universal/release/libworld_foundry.a WorldFoundryCore/Libraries/

# Open Xcode project
open WorldFoundry.xcodeproj
```

## Project Structure Deep Dive

### Xcode Project Structure
```
platforms/ios/
├── WorldFoundry.xcodeproj/          # Xcode project file
│   ├── project.pbxproj             # Project configuration
│   └── xcshareddata/               # Shared project data
├── WorldFoundry/                   # Main iOS application
│   ├── App/                        # Application lifecycle
│   │   ├── WorldFoundryApp.swift   # App entry point
│   │   ├── ContentView.swift       # Root view
│   │   └── AppDelegate.swift       # App delegate (if needed)
│   ├── Views/                      # SwiftUI views
│   │   ├── MapView/               # Map display views
│   │   ├── CultureView/           # Culture management views
│   │   ├── SettingsView/          # Settings and preferences
│   │   └── SharedComponents/      # Reusable UI components
│   ├── ViewModels/                # MVVM view models
│   │   ├── MapViewModel.swift     # Map state management
│   │   ├── CultureViewModel.swift # Culture state management
│   │   └── SettingsViewModel.swift # Settings state management
│   ├── Models/                    # Swift data models
│   │   ├── WorldModel.swift       # World data structures
│   │   ├── CultureModel.swift     # Culture data structures
│   │   └── MapModel.swift         # Map data structures
│   ├── Services/                  # Business logic services
│   │   ├── CoreEngineService.swift # Rust core integration
│   │   ├── FileService.swift      # File management
│   │   ├── CloudService.swift     # iCloud integration
│   │   └── ExportService.swift    # Export functionality
│   ├── Resources/                 # App resources
│   │   ├── Assets.xcassets        # Images and colors
│   │   ├── Localizable.strings    # Localization
│   │   └── Info.plist             # App configuration
│   └── Extensions/                # Swift extensions
│       ├── Color+Extensions.swift # Color utilities
│       └── View+Extensions.swift  # View utilities
├── WorldFoundryCore/              # Rust core wrapper
│   ├── Headers/                   # C headers for Swift
│   │   └── world_foundry.h        # Generated header
│   ├── Libraries/                 # Rust libraries
│   │   └── libworld_foundry.a     # Universal library
│   └── Sources/                   # Swift wrapper code
│       ├── CoreEngine.swift       # High-level Swift API
│       ├── NativeBridge.swift     # Low-level FFI bridge
│       └── DataTypes.swift        # Swift data type definitions
├── WorldFoundryTests/             # Unit tests
│   ├── WorldFoundryTests.swift    # Main test suite
│   ├── CoreEngineTests.swift      # Core engine tests
│   └── ViewModelTests.swift       # View model tests
├── WorldFoundryUITests/           # UI tests
│   ├── WorldFoundryUITests.swift  # UI test suite
│   └── WorldFoundryUITestsLaunchTests.swift # Launch tests
└── Frameworks/                    # External frameworks
    └── (Third-party dependencies)
```

### Key Configuration Files

#### Info.plist
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDisplayName</key>
    <string>World Foundry</string>
    <key>CFBundleIdentifier</key>
    <string>com.balexda.worldfoundry</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>LSRequiresIPhoneOS</key>
    <true/>
    <key>UILaunchScreen</key>
    <dict>
        <key>UIColorName</key>
        <string>AccentColor</string>
    </dict>
    <key>UISupportedInterfaceOrientations</key>
    <array>
        <string>UIInterfaceOrientationPortrait</string>
        <string>UIInterfaceOrientationLandscapeLeft</string>
        <string>UIInterfaceOrientationLandscapeRight</string>
    </array>
    <key>UISupportedInterfaceOrientations~ipad</key>
    <array>
        <string>UIInterfaceOrientationPortrait</string>
        <string>UIInterfaceOrientationPortraitUpsideDown</string>
        <string>UIInterfaceOrientationLandscapeLeft</string>
        <string>UIInterfaceOrientationLandscapeRight</string>
    </array>
    <key>UTExportedTypeDeclarations</key>
    <array>
        <dict>
            <key>UTTypeIdentifier</key>
            <string>com.balexda.worldfoundry.map</string>
            <key>UTTypeDescription</key>
            <string>World Foundry Map</string>
            <key>UTTypeConformsTo</key>
            <array>
                <string>public.data</string>
            </array>
            <key>UTTypeTagSpecification</key>
            <dict>
                <key>public.filename-extension</key>
                <array>
                    <string>wfmap</string>
                </array>
            </dict>
        </dict>
    </array>
</dict>
</plist>
```

## Development Workflow

### 1. Daily Development Setup

```bash
# Start Xcode with the project
open platforms/ios/WorldFoundry.xcodeproj

# Or use command line development
# Terminal 1: Watch for Rust changes
cd core
cargo watch -x "lipo --release"

# Terminal 2: Auto-copy libraries to iOS project
cd platforms/ios
fswatch -o ../../core/target/universal/release/ | xargs -n1 -I{} cp ../../core/target/universal/release/libworld_foundry.a WorldFoundryCore/Libraries/
```

### 2. Building the Application

#### Debug Build
```bash
# Build for iOS Simulator
xcodebuild -project WorldFoundry.xcodeproj \
    -scheme WorldFoundry \
    -destination 'platform=iOS Simulator,name=iPhone 15 Pro' \
    -configuration Debug \
    build

# Run in simulator
xcodebuild -project WorldFoundry.xcodeproj \
    -scheme WorldFoundry \
    -destination 'platform=iOS Simulator,name=iPhone 15 Pro' \
    -configuration Debug \
    run
```

#### Release Build
```bash
# Build for iOS Device
xcodebuild -project WorldFoundry.xcodeproj \
    -scheme WorldFoundry \
    -destination generic/platform=iOS \
    -configuration Release \
    build

# Create archive for distribution
xcodebuild -project WorldFoundry.xcodeproj \
    -scheme WorldFoundry \
    -configuration Release \
    -destination generic/platform=iOS \
    archive \
    -archivePath WorldFoundry.xcarchive
```

### 3. Debugging

#### Xcode Debugging
1. Set breakpoints in Swift code
2. Press Cmd+R or Product → Run
3. Use Debug Navigator for memory and CPU profiling
4. Use Console for log output

#### Rust Core Debugging
```bash
# Build Rust core with debug symbols
cd core
cargo build --target aarch64-apple-ios

# Copy debug version to iOS project
cp target/aarch64-apple-ios/debug/libworld_foundry.a ../platforms/ios/WorldFoundryCore/Libraries/libworld_foundry_debug.a

# Use Xcode's LLDB for mixed Swift/Rust debugging
# In Xcode: Debug → Attach to Process
```

#### Logging and Diagnostics
```swift
// Add to WorldFoundryApp.swift for comprehensive logging
import os.log

@main
struct WorldFoundryApp: App {
    private let logger = Logger(subsystem: "com.balexda.worldfoundry", category: "app")
    
    var body: some Scene {
        WindowGroup {
            ContentView()
                .onAppear {
                    setupLogging()
                }
        }
    }
    
    private func setupLogging() {
        #if DEBUG
        logger.info("Debug mode enabled")
        
        // Enable Rust debug logging
        setenv("RUST_LOG", "debug", 1)
        setenv("RUST_BACKTRACE", "full", 1)
        #endif
    }
}
```

### 4. Testing

#### Unit Tests
```bash
# Run all tests
xcodebuild test -project WorldFoundry.xcodeproj -scheme WorldFoundry -destination 'platform=iOS Simulator,name=iPhone 15 Pro'

# Run specific test class
xcodebuild test -project WorldFoundry.xcodeproj -scheme WorldFoundry -destination 'platform=iOS Simulator,name=iPhone 15 Pro' -only-testing:WorldFoundryTests/CoreEngineTests
```

#### UI Tests
```bash
# Run UI tests
xcodebuild test -project WorldFoundry.xcodeproj -scheme WorldFoundry -destination 'platform=iOS Simulator,name=iPhone 15 Pro' -only-testing:WorldFoundryUITests
```

## Core Engine Integration

### 1. Swift/Rust FFI Bridge

```swift
// NativeBridge.swift - Low-level FFI bridge
import Foundation

// C function declarations from Rust
@_silgen_name("world_foundry_create")
func world_foundry_create() -> OpaquePointer?

@_silgen_name("world_foundry_destroy")
func world_foundry_destroy(_ handle: OpaquePointer)

@_silgen_name("world_foundry_import_azgaar")
func world_foundry_import_azgaar(_ handle: OpaquePointer, _ path: UnsafePointer<CChar>) -> Int32

@_silgen_name("world_foundry_render_map")
func world_foundry_render_map(_ handle: OpaquePointer, _ map: OpaquePointer, _ width: Int32, _ height: Int32) -> NativeImageData

// Native data structures
struct NativeImageData {
    let data: UnsafeMutablePointer<UInt8>?
    let width: UInt32
    let height: UInt32
    let bytesPerPixel: UInt32
}
```

### 2. High-Level Swift Wrapper

```swift
// CoreEngine.swift - High-level Swift API
import Foundation
import UIKit

@MainActor
public class CoreEngine: ObservableObject {
    private var handle: OpaquePointer?
    
    @Published public var currentWorld: WorldModel?
    @Published public var isLoading = false
    @Published public var errorMessage: String?
    
    public init() {
        handle = world_foundry_create()
        guard handle != nil else {
            fatalError("Failed to create core engine")
        }
    }
    
    deinit {
        if let handle = handle {
            world_foundry_destroy(handle)
        }
    }
    
    public func importAzgaarMap(from url: URL) async throws -> WorldModel {
        guard let handle = handle else {
            throw CoreEngineError.engineNotInitialized
        }
        
        isLoading = true
        errorMessage = nil
        
        defer {
            isLoading = false
        }
        
        return try await withCheckedThrowingContinuation { continuation in
            Task.detached {
                let result = url.path.withCString { path in
                    world_foundry_import_azgaar(handle, path)
                }
                
                await MainActor.run {
                    if result == 0 {
                        let world = self.convertNativeWorld(result)
                        self.currentWorld = world
                        continuation.resume(returning: world)
                    } else {
                        let error = CoreEngineError.importFailed(code: result)
                        self.errorMessage = error.localizedDescription
                        continuation.resume(throwing: error)
                    }
                }
            }
        }
    }
    
    public func renderMap(world: WorldModel, size: CGSize) async throws -> UIImage {
        guard let handle = handle else {
            throw CoreEngineError.engineNotInitialized
        }
        
        return try await withCheckedThrowingContinuation { continuation in
            Task.detached {
                let nativeWorld = self.convertToNativeWorld(world)
                let imageData = world_foundry_render_map(
                    handle,
                    nativeWorld,
                    Int32(size.width),
                    Int32(size.height)
                )
                
                await MainActor.run {
                    if let image = self.convertToUIImage(imageData) {
                        continuation.resume(returning: image)
                    } else {
                        continuation.resume(throwing: CoreEngineError.renderFailed)
                    }
                }
            }
        }
    }
    
    private func convertToUIImage(_ imageData: NativeImageData) -> UIImage? {
        guard let data = imageData.data else { return nil }
        
        let colorSpace = CGColorSpaceCreateDeviceRGB()
        let bitmapInfo = CGBitmapInfo(rawValue: CGImageAlphaInfo.premultipliedLast.rawValue)
        
        guard let context = CGContext(
            data: data,
            width: Int(imageData.width),
            height: Int(imageData.height),
            bitsPerComponent: 8,
            bytesPerRow: Int(imageData.width * imageData.bytesPerPixel),
            space: colorSpace,
            bitmapInfo: bitmapInfo.rawValue
        ) else { return nil }
        
        guard let cgImage = context.makeImage() else { return nil }
        return UIImage(cgImage: cgImage)
    }
}

public enum CoreEngineError: LocalizedError {
    case engineNotInitialized
    case importFailed(code: Int32)
    case renderFailed
    case invalidData
    
    public var errorDescription: String? {
        switch self {
        case .engineNotInitialized:
            return "Core engine not initialized"
        case .importFailed(let code):
            return "Import failed with code: \(code)"
        case .renderFailed:
            return "Map rendering failed"
        case .invalidData:
            return "Invalid data provided"
        }
    }
}
```

## UI Development with SwiftUI

### 1. MVVM Pattern Implementation

```swift
// MapViewModel.swift - Main map view model
import SwiftUI
import Combine

@MainActor
class MapViewModel: ObservableObject {
    @Published var currentMap: WorldModel?
    @Published var mapImage: UIImage?
    @Published var isLoading = false
    @Published var errorMessage: String?
    @Published var zoomLevel: Double = 1.0
    @Published var mapOffset: CGSize = .zero
    
    private let coreEngine: CoreEngine
    private let fileService: FileService
    private var cancellables = Set<AnyCancellable>()
    
    init(coreEngine: CoreEngine, fileService: FileService) {
        self.coreEngine = coreEngine
        self.fileService = fileService
        
        // Observe core engine changes
        coreEngine.$currentWorld
            .assign(to: \.currentMap, on: self)
            .store(in: &cancellables)
        
        coreEngine.$isLoading
            .assign(to: \.isLoading, on: self)
            .store(in: &cancellables)
        
        coreEngine.$errorMessage
            .assign(to: \.errorMessage, on: self)
            .store(in: &cancellables)
    }
    
    func importMap() async {
        do {
            let url = try await fileService.pickFile(types: [.json, .data])
            let world = try await coreEngine.importAzgaarMap(from: url)
            await updateMapImage()
        } catch {
            errorMessage = error.localizedDescription
        }
    }
    
    func exportMap() async {
        guard let world = currentMap else { return }
        
        do {
            let url = try await fileService.saveFile(name: "exported_map.png", type: .png)
            // Export logic here
        } catch {
            errorMessage = error.localizedDescription
        }
    }
    
    private func updateMapImage() async {
        guard let world = currentMap else { return }
        
        do {
            let size = CGSize(width: 1024, height: 1024)
            mapImage = try await coreEngine.renderMap(world: world, size: size)
        } catch {
            errorMessage = error.localizedDescription
        }
    }
}
```

### 2. SwiftUI Views

```swift
// MapView.swift - Main map display view
import SwiftUI

struct MapView: View {
    @StateObject private var viewModel: MapViewModel
    @State private var showingImportSheet = false
    @State private var showingExportSheet = false
    
    init(coreEngine: CoreEngine, fileService: FileService) {
        _viewModel = StateObject(wrappedValue: MapViewModel(
            coreEngine: coreEngine,
            fileService: fileService
        ))
    }
    
    var body: some View {
        NavigationView {
            ZStack {
                // Map Display
                ScrollView([.horizontal, .vertical]) {
                    ZStack {
                        if let mapImage = viewModel.mapImage {
                            Image(uiImage: mapImage)
                                .resizable()
                                .aspectRatio(contentMode: .fit)
                                .scaleEffect(viewModel.zoomLevel)
                                .offset(viewModel.mapOffset)
                        } else {
                            RoundedRectangle(cornerRadius: 12)
                                .fill(Color.gray.opacity(0.3))
                                .frame(width: 400, height: 300)
                                .overlay(
                                    Text("No map loaded")
                                        .foregroundColor(.secondary)
                                )
                        }
                    }
                }
                .gesture(
                    MagnificationGesture()
                        .onChanged { value in
                            viewModel.zoomLevel = value
                        }
                )
                
                // Loading Overlay
                if viewModel.isLoading {
                    ZStack {
                        Color.black.opacity(0.3)
                            .ignoresSafeArea()
                        
                        VStack {
                            ProgressView()
                                .scaleEffect(1.5)
                            Text("Loading map...")
                                .padding(.top)
                        }
                        .padding()
                        .background(Color(.systemBackground))
                        .cornerRadius(12)
                    }
                }
            }
            .navigationTitle("World Map")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItemGroup(placement: .navigationBarTrailing) {
                    Button("Import") {
                        showingImportSheet = true
                    }
                    
                    Button("Export") {
                        showingExportSheet = true
                    }
                    .disabled(viewModel.currentMap == nil)
                }
            }
            .sheet(isPresented: $showingImportSheet) {
                ImportSheet(viewModel: viewModel)
            }
            .sheet(isPresented: $showingExportSheet) {
                ExportSheet(viewModel: viewModel)
            }
            .alert("Error", isPresented: .constant(viewModel.errorMessage != nil)) {
                Button("OK") {
                    viewModel.errorMessage = nil
                }
            } message: {
                Text(viewModel.errorMessage ?? "")
            }
        }
    }
}

// ImportSheet.swift - Import options sheet
struct ImportSheet: View {
    @ObservedObject var viewModel: MapViewModel
    @Environment(\.dismiss) private var dismiss
    
    var body: some View {
        NavigationView {
            VStack(spacing: 20) {
                Text("Import Map")
                    .font(.title2)
                    .fontWeight(.semibold)
                
                VStack(alignment: .leading, spacing: 12) {
                    Text("Supported Formats:")
                        .font(.headline)
                    
                    Label("Azgaar JSON Export", systemImage: "doc.text")
                    Label("World Foundry Map", systemImage: "map")
                    Label("GeoJSON Data", systemImage: "location")
                }
                .frame(maxWidth: .infinity, alignment: .leading)
                .padding()
                .background(Color(.systemGray6))
                .cornerRadius(12)
                
                Spacer()
                
                Button("Choose File") {
                    Task {
                        await viewModel.importMap()
                        dismiss()
                    }
                }
                .buttonStyle(.borderedProminent)
                .controlSize(.large)
            }
            .padding()
            .navigationTitle("Import")
            .navigationBarTitleDisplayMode(.inline)
            .navigationBarBackButtonHidden()
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
            }
        }
    }
}
```

## Performance Optimization

### 1. Memory Management

```swift
// Efficient image handling for large maps
class ImageCache {
    private let cache = NSCache<NSString, UIImage>()
    private let queue = DispatchQueue(label: "image-cache", qos: .utility)
    
    init() {
        cache.countLimit = 50
        cache.totalCostLimit = 100 * 1024 * 1024 // 100MB
    }
    
    func image(for key: String) -> UIImage? {
        return cache.object(forKey: key as NSString)
    }
    
    func setImage(_ image: UIImage, for key: String) {
        let cost = image.pngData()?.count ?? 0
        cache.setObject(image, forKey: key as NSString, cost: cost)
    }
}

// Lazy loading for large datasets
@propertyWrapper
struct LazyLoaded<T> {
    private var _value: T?
    private let loader: () -> T
    
    init(_ loader: @escaping () -> T) {
        self.loader = loader
    }
    
    var wrappedValue: T {
        mutating get {
            if _value == nil {
                _value = loader()
            }
            return _value!
        }
    }
}
```

### 2. Metal Performance

```swift
// Metal-accelerated map rendering
import Metal
import MetalKit

class MetalMapRenderer: NSObject, MTKViewDelegate {
    private let device: MTLDevice
    private let commandQueue: MTLCommandQueue
    private var renderPipelineState: MTLRenderPipelineState?
    
    init?(metalView: MTKView) {
        guard let device = MTLCreateSystemDefaultDevice(),
              let commandQueue = device.makeCommandQueue() else {
            return nil
        }
        
        self.device = device
        self.commandQueue = commandQueue
        
        super.init()
        
        metalView.device = device
        metalView.delegate = self
        
        setupRenderPipeline()
    }
    
    private func setupRenderPipeline() {
        guard let library = device.makeDefaultLibrary() else { return }
        
        let pipelineDescriptor = MTLRenderPipelineDescriptor()
        pipelineDescriptor.vertexFunction = library.makeFunction(name: "vertex_main")
        pipelineDescriptor.fragmentFunction = library.makeFunction(name: "fragment_main")
        pipelineDescriptor.colorAttachments[0].pixelFormat = .bgra8Unorm
        
        do {
            renderPipelineState = try device.makeRenderPipelineState(descriptor: pipelineDescriptor)
        } catch {
            print("Failed to create render pipeline state: \(error)")
        }
    }
    
    func mtkView(_ view: MTKView, drawableSizeWillChange size: CGSize) {
        // Handle size changes
    }
    
    func draw(in view: MTKView) {
        guard let drawable = view.currentDrawable,
              let renderPassDescriptor = view.currentRenderPassDescriptor,
              let commandBuffer = commandQueue.makeCommandBuffer(),
              let renderEncoder = commandBuffer.makeRenderCommandEncoder(descriptor: renderPassDescriptor),
              let pipelineState = renderPipelineState else {
            return
        }
        
        renderEncoder.setRenderPipelineState(pipelineState)
        
        // Render map data
        renderMapData(encoder: renderEncoder)
        
        renderEncoder.endEncoding()
        commandBuffer.present(drawable)
        commandBuffer.commit()
    }
    
    private func renderMapData(encoder: MTLRenderCommandEncoder) {
        // Implement map rendering logic
    }
}
```

## Deployment and Distribution

### 1. App Store Preparation

```bash
# Create App Store archive
xcodebuild -project WorldFoundry.xcodeproj \
    -scheme WorldFoundry \
    -configuration Release \
    -destination generic/platform=iOS \
    archive \
    -archivePath WorldFoundry.xcarchive

# Export for App Store
xcodebuild -exportArchive \
    -archivePath WorldFoundry.xcarchive \
    -exportPath ./AppStore \
    -exportOptionsPlist ExportOptions-AppStore.plist

# Upload to App Store Connect
xcrun altool --upload-app \
    --type ios \
    --file "./AppStore/WorldFoundry.ipa" \
    --username "your-apple-id@example.com" \
    --password "app-specific-password"
```

### 2. TestFlight Distribution

```bash
# Export for TestFlight
xcodebuild -exportArchive \
    -archivePath WorldFoundry.xcarchive \
    -exportPath ./TestFlight \
    -exportOptionsPlist ExportOptions-TestFlight.plist

# Upload to TestFlight
xcrun altool --upload-app \
    --type ios \
    --file "./TestFlight/WorldFoundry.ipa" \
    --username "your-apple-id@example.com" \
    --password "app-specific-password"
```

### 3. Enterprise Distribution

```bash
# Export for enterprise distribution
xcodebuild -exportArchive \
    -archivePath WorldFoundry.xcarchive \
    -exportPath ./Enterprise \
    -exportOptionsPlist ExportOptions-Enterprise.plist
```

## Troubleshooting

### Common Issues and Solutions

#### 1. Rust Library Linking Issues
```
Error: Library not loaded: libworld_foundry.a
```

**Solution:**
```bash
# Ensure library is built for correct architecture
cd core
cargo lipo --release

# Verify library architecture
lipo -info target/universal/release/libworld_foundry.a

# Copy to correct location
cp target/universal/release/libworld_foundry.a ../platforms/ios/WorldFoundryCore/Libraries/
```

#### 2. Code Signing Issues
```
Error: Code signing failed
```

**Solution:**
1. Check Apple Developer account status
2. Verify provisioning profiles are up to date
3. Ensure bundle identifier matches registered app ID
4. Check code signing settings in Xcode

#### 3. Simulator vs Device Issues
```
Error: Architecture mismatch
```

**Solution:**
```bash
# Build separate libraries for simulator and device
cargo build --target x86_64-apple-ios          # Simulator (Intel)
cargo build --target aarch64-apple-ios-sim     # Simulator (Apple Silicon)
cargo build --target aarch64-apple-ios         # Device

# Create universal library
lipo -create \
    target/x86_64-apple-ios/release/libworld_foundry.a \
    target/aarch64-apple-ios-sim/release/libworld_foundry.a \
    target/aarch64-apple-ios/release/libworld_foundry.a \
    -output target/universal/release/libworld_foundry.a
```

#### 4. Memory Issues
```
Issue: App crashes due to memory pressure
```

**Solution:**
```swift
// Implement memory management
class MemoryManager {
    func handleMemoryWarning() {
        // Clear caches
        ImageCache.shared.clearCache()
        
        // Release non-essential resources
        releaseNonEssentialResources()
        
        // Force garbage collection
        autoreleasepool {
            // Perform cleanup
        }
    }
}

// In your view controllers
override func didReceiveMemoryWarning() {
    super.didReceiveMemoryWarning()
    MemoryManager.shared.handleMemoryWarning()
}
```

## Next Steps

After completing the iOS development setup:

1. **Complete the Setup**: Follow the [main development setup guide](DEVELOPMENT_SETUP.md)
2. **Learn Android Development**: [Android Development Guide](ANDROID_DEVELOPMENT.md)
3. **Deployment**: [iOS Deployment Guide](../deployment/IOS_DEPLOYMENT.md)
4. **Debugging**: [iOS Debugging Guide](../debugging/IOS_DEBUGGING.md)

## Additional Resources

- [SwiftUI Documentation](https://developer.apple.com/documentation/swiftui/)
- [iOS App Development](https://developer.apple.com/ios/)
- [Xcode Documentation](https://developer.apple.com/documentation/xcode/)
- [Metal Performance Shaders](https://developer.apple.com/documentation/metalperformanceshaders/)
- [App Store Connect](https://developer.apple.com/app-store-connect/)

---

**Last Updated**: March 2024  
**iOS SDK Version**: iOS 17.0+  
**Xcode Version**: 15.0+  
**Swift Version**: 5.9+