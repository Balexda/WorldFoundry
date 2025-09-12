# iOS Platform Architecture

## Overview

The iOS implementation of World Foundry leverages SwiftUI and modern iOS frameworks to provide a native iOS experience optimized for both iPhone and iPad, with support for Apple Pencil, multitasking, and iOS-specific features.

## Architecture Stack

```
┌─────────────────────────────────────────────────────────────┐
│                    SwiftUI Application                     │
├─────────────────────────────────────────────────────────────┤
│                      UIKit Bridge                          │
├─────────────────────────────────────────────────────────────┤
│              Platform Abstraction Layer                    │
├─────────────────────────────────────────────────────────────┤
│                 World Foundry Core                         │
│                   (Rust Engine)                            │
└─────────────────────────────────────────────────────────────┘
```

## Technology Stack

### UI Framework: SwiftUI
- **Declarative UI** - Modern reactive UI framework
- **Cross-Device** - Adaptive layouts for iPhone and iPad
- **Accessibility** - Built-in VoiceOver and accessibility support
- **Animations** - Smooth, performant animations and transitions

### Platform Integration: iOS SDK
- **Files App** - Integration with iOS file system
- **Document Picker** - Native file import/export
- **Share Sheet** - System sharing capabilities
- **Multitasking** - iPad Split View and Slide Over support

### Core Integration: Swift/Rust FFI
- **Swift Package Manager** - Rust library integration
- **Memory Safety** - Safe interop between Swift and Rust
- **Async/Await** - Modern concurrency patterns
- **Error Handling** - Swift Result types with Rust error handling

## Project Structure

```
platforms/ios/
├── WorldFoundry.xcodeproj/         # Xcode project
├── WorldFoundry/
│   ├── WorldFoundryApp.swift       # App entry point
│   ├── ContentView.swift           # Main content view
│   ├── Views/                      # SwiftUI views
│   │   ├── MapView.swift          # Map display and interaction
│   │   ├── ImportView.swift       # Azgaar import interface
│   │   ├── ExportView.swift       # Export options
│   │   ├── SettingsView.swift     # App settings
│   │   └── Components/            # Reusable UI components
│   │       ├── MapCanvas.swift    # Custom map rendering
│   │       ├── ToolPalette.swift  # Drawing tools
│   │       └── LayerPanel.swift   # Map layer controls
│   ├── ViewModels/                # MVVM view models
│   │   ├── MapViewModel.swift     # Map state management
│   │   ├── ImportViewModel.swift  # Import workflow
│   │   └── AppViewModel.swift     # Global app state
│   ├── Services/                  # Platform services
│   │   ├── FileService.swift      # File operations
│   │   ├── StorageService.swift   # Core Data persistence
│   │   ├── RenderingService.swift # Metal rendering
│   │   └── ShareService.swift     # System sharing
│   ├── Models/                    # Swift data models
│   │   ├── WorldMap.swift         # Map data structures
│   │   ├── AppSettings.swift      # User preferences
│   │   └── ImportState.swift      # Import progress
│   ├── Core/                      # Rust FFI layer
│   │   ├── CoreEngine.swift       # High-level wrapper
│   │   ├── FFI.swift             # Low-level bindings
│   │   └── DataBridge.swift      # Data conversion
│   ├── Resources/                 # App resources
│   │   ├── Assets.xcassets       # Images and colors
│   │   ├── Localizable.strings   # Localization
│   │   └── Info.plist           # App configuration
│   └── Extensions/               # Swift extensions
│       ├── Color+Theme.swift     # Theme colors
│       ├── View+Modifiers.swift  # Custom modifiers
│       └── CGPoint+Math.swift    # Geometry utilities
├── WorldFoundryCore/             # Rust core framework
│   ├── Package.swift            # Swift package manifest
│   ├── Sources/                 # Swift wrapper sources
│   └── Libraries/               # Compiled Rust libraries
│       ├── libworld_foundry_ios.a      # iOS device
│       └── libworld_foundry_simulator.a # iOS simulator
├── WorldFoundryTests/           # Unit tests
│   ├── ViewModelTests.swift     # View model tests
│   ├── ServiceTests.swift       # Service layer tests
│   └── CoreTests.swift          # Core integration tests
└── WorldFoundryUITests/         # UI automation tests
    ├── MapInteractionTests.swift # Map UI tests
    └── ImportFlowTests.swift    # Import workflow tests
```

## Core Integration

### Swift/Rust FFI Bridge

```swift
// CoreEngine.swift - High-level Swift wrapper
@MainActor
class CoreEngine: ObservableObject {
    private var handle: OpaquePointer?
    
    init() {
        handle = world_foundry_create()
    }
    
    deinit {
        if let handle = handle {
            world_foundry_destroy(handle)
        }
    }
    
    func importAzgaarMap(from url: URL) async throws -> WorldMap {
        return try await withCheckedThrowingContinuation { continuation in
            Task.detached {
                do {
                    let path = url.path
                    let result = world_foundry_import_azgaar(self.handle, path)
                    let map = try WorldMap.from(native: result)
                    continuation.resume(returning: map)
                } catch {
                    continuation.resume(throwing: error)
                }
            }
        }
    }
    
    func renderMap(_ map: WorldMap, size: CGSize) async -> CGImage? {
        return await withCheckedContinuation { continuation in
            Task.detached {
                let imageData = world_foundry_render_map(
                    self.handle, 
                    map.nativeHandle,
                    Int32(size.width),
                    Int32(size.height)
                )
                
                let image = self.createCGImage(from: imageData)
                continuation.resume(returning: image)
            }
        }
    }
}
```

### Data Bridge Layer

```swift
// DataBridge.swift - Swift/Rust data conversion
struct WorldMap {
    let id: UUID
    let name: String
    let width: UInt32
    let height: UInt32
    let cells: [MapCell]
    
    internal var nativeHandle: OpaquePointer?
    
    static func from(native: NativeWorldMap) throws -> WorldMap {
        guard let namePtr = native.name else {
            throw CoreError.invalidData("Map name is null")
        }
        
        let name = String(cString: namePtr)
        let cells = try convertCells(native.cells, count: native.cell_count)
        
        return WorldMap(
            id: UUID(),
            name: name,
            width: native.width,
            height: native.height,
            cells: cells
        )
    }
    
    private static func convertCells(_ cellsPtr: UnsafePointer<NativeCell>?, 
                                   count: UInt32) throws -> [MapCell] {
        guard let cellsPtr = cellsPtr else {
            throw CoreError.invalidData("Cells pointer is null")
        }
        
        let buffer = UnsafeBufferPointer(start: cellsPtr, count: Int(count))
        return buffer.map { nativeCell in
            MapCell(
                x: nativeCell.x,
                y: nativeCell.y,
                elevation: nativeCell.elevation,
                biome: BiomeType(rawValue: nativeCell.biome) ?? .ocean,
                culture: nativeCell.culture,
                religion: nativeCell.religion
            )
        }
    }
}
```

### FFI Declarations

```swift
// FFI.swift - Low-level C bindings
import Foundation

// Core engine functions
@_silgen_name("world_foundry_create")
func world_foundry_create() -> OpaquePointer?

@_silgen_name("world_foundry_destroy")
func world_foundry_destroy(_ handle: OpaquePointer?)

@_silgen_name("world_foundry_import_azgaar")
func world_foundry_import_azgaar(_ handle: OpaquePointer?, 
                                _ path: UnsafePointer<CChar>) -> NativeWorldMap

@_silgen_name("world_foundry_render_map")
func world_foundry_render_map(_ handle: OpaquePointer?,
                             _ map: OpaquePointer?,
                             _ width: Int32,
                             _ height: Int32) -> NativeImageData

// Native data structures
struct NativeWorldMap {
    let name: UnsafePointer<CChar>?
    let width: UInt32
    let height: UInt32
    let cells: UnsafePointer<NativeCell>?
    let cell_count: UInt32
}

struct NativeCell {
    let x: UInt32
    let y: UInt32
    let elevation: Float
    let biome: UInt8
    let culture: UInt32
    let religion: UInt32
}

struct NativeImageData {
    let data: UnsafePointer<UInt8>?
    let width: UInt32
    let height: UInt32
    let bytes_per_row: UInt32
}
```

## SwiftUI Architecture

### MVVM Pattern with Combine

```swift
// MapViewModel.swift - Map state management
import SwiftUI
import Combine

@MainActor
class MapViewModel: ObservableObject {
    @Published var currentMap: WorldMap?
    @Published var mapImage: CGImage?
    @Published var isLoading = false
    @Published var errorMessage: String?
    @Published var zoomLevel: Double = 1.0
    @Published var panOffset: CGSize = .zero
    
    private let coreEngine: CoreEngine
    private let renderingService: RenderingService
    private var cancellables = Set<AnyCancellable>()
    
    init(coreEngine: CoreEngine, renderingService: RenderingService) {
        self.coreEngine = coreEngine
        self.renderingService = renderingService
        
        setupBindings()
    }
    
    private func setupBindings() {
        // Re-render when map or zoom changes
        Publishers.CombineLatest($currentMap, $zoomLevel)
            .debounce(for: .milliseconds(100), scheduler: RunLoop.main)
            .sink { [weak self] map, zoom in
                Task { await self?.updateMapImage() }
            }
            .store(in: &cancellables)
    }
    
    func importMap(from url: URL) async {
        isLoading = true
        errorMessage = nil
        
        do {
            let map = try await coreEngine.importAzgaarMap(from: url)
            currentMap = map
        } catch {
            errorMessage = error.localizedDescription
        }
        
        isLoading = false
    }
    
    private func updateMapImage() async {
        guard let map = currentMap else { return }
        
        let size = CGSize(
            width: Double(map.width) * zoomLevel,
            height: Double(map.height) * zoomLevel
        )
        
        mapImage = await renderingService.renderMap(map, size: size)
    }
}
```

### Adaptive UI Layout

```swift
// ContentView.swift - Main app interface
struct ContentView: View {
    @StateObject private var mapViewModel: MapViewModel
    @State private var selectedTool: MapTool = .pan
    @State private var showingImportSheet = false
    
    var body: some View {
        NavigationSplitView {
            // Sidebar for iPad, bottom bar for iPhone
            ToolPalette(selectedTool: $selectedTool)
                .navigationTitle("Tools")
        } detail: {
            MapView(viewModel: mapViewModel, selectedTool: selectedTool)
                .navigationTitle("World Foundry")
                .navigationBarTitleDisplayMode(.inline)
                .toolbar {
                    ToolbarItemGroup(placement: .navigationBarTrailing) {
                        Button("Import") {
                            showingImportSheet = true
                        }
                        
                        Menu("Export") {
                            Button("PNG Image") { exportPNG() }
                            Button("JSON Data") { exportJSON() }
                            Button("PDF Document") { exportPDF() }
                        }
                    }
                }
        }
        .sheet(isPresented: $showingImportSheet) {
            ImportView(viewModel: mapViewModel)
        }
    }
}
```

### Custom Map Canvas

```swift
// MapCanvas.swift - Interactive map rendering
struct MapCanvas: View {
    @ObservedObject var viewModel: MapViewModel
    @State private var dragOffset: CGSize = .zero
    @State private var lastDragValue: CGSize = .zero
    
    var body: some View {
        GeometryReader { geometry in
            ZStack {
                // Map image
                if let image = viewModel.mapImage {
                    Image(decorative: image, scale: 1.0)
                        .resizable()
                        .aspectRatio(contentMode: .fit)
                        .offset(
                            x: viewModel.panOffset.width + dragOffset.width,
                            y: viewModel.panOffset.height + dragOffset.height
                        )
                        .scaleEffect(viewModel.zoomLevel)
                        .clipped()
                }
                
                // Loading overlay
                if viewModel.isLoading {
                    ProgressView("Rendering map...")
                        .frame(maxWidth: .infinity, maxHeight: .infinity)
                        .background(Color.black.opacity(0.3))
                }
            }
        }
        .gesture(
            SimultaneousGesture(
                // Pan gesture
                DragGesture()
                    .onChanged { value in
                        dragOffset = CGSize(
                            width: value.translation.width - lastDragValue.width,
                            height: value.translation.height - lastDragValue.height
                        )
                    }
                    .onEnded { value in
                        viewModel.panOffset.width += dragOffset.width
                        viewModel.panOffset.height += dragOffset.height
                        dragOffset = .zero
                        lastDragValue = .zero
                    },
                
                // Zoom gesture
                MagnificationGesture()
                    .onChanged { value in
                        viewModel.zoomLevel = max(0.1, min(5.0, value))
                    }
            )
        )
    }
}
```

## Platform Services

### File System Integration

```swift
// FileService.swift - iOS file operations
import UniformTypeIdentifiers

class FileService: ObservableObject {
    func importFile(types: [UTType]) async -> URL? {
        return await withCheckedContinuation { continuation in
            let picker = UIDocumentPickerViewController(
                forOpeningContentTypes: types,
                asCopy: true
            )
            
            picker.delegate = DocumentPickerDelegate { url in
                continuation.resume(returning: url)
            }
            
            if let windowScene = UIApplication.shared.connectedScenes.first as? UIWindowScene,
               let window = windowScene.windows.first {
                window.rootViewController?.present(picker, animated: true)
            }
        }
    }
    
    func exportFile(data: Data, name: String, type: UTType) async -> Bool {
        let tempURL = FileManager.default.temporaryDirectory
            .appendingPathComponent(name)
        
        do {
            try data.write(to: tempURL)
            
            let activityVC = UIActivityViewController(
                activityItems: [tempURL],
                applicationActivities: nil
            )
            
            if let windowScene = UIApplication.shared.connectedScenes.first as? UIWindowScene,
               let window = windowScene.windows.first {
                
                // iPad popover support
                if let popover = activityVC.popoverPresentationController {
                    popover.sourceView = window.rootViewController?.view
                    popover.sourceRect = CGRect(x: window.bounds.midX, y: window.bounds.midY, width: 0, height: 0)
                }
                
                await window.rootViewController?.present(activityVC, animated: true)
                return true
            }
        } catch {
            print("Export error: \(error)")
        }
        
        return false
    }
}
```

### Core Data Storage

```swift
// StorageService.swift - Persistent data storage
import CoreData

class StorageService: ObservableObject {
    lazy var persistentContainer: NSPersistentContainer = {
        let container = NSPersistentContainer(name: "WorldFoundry")
        container.loadPersistentStores { _, error in
            if let error = error {
                fatalError("Core Data error: \(error)")
            }
        }
        return container
    }()
    
    var context: NSManagedObjectContext {
        persistentContainer.viewContext
    }
    
    func saveContext() {
        if context.hasChanges {
            do {
                try context.save()
            } catch {
                print("Save error: \(error)")
            }
        }
    }
    
    func saveMap(_ map: WorldMap) {
        let entity = MapEntity(context: context)
        entity.id = map.id
        entity.name = map.name
        entity.width = Int32(map.width)
        entity.height = Int32(map.height)
        entity.dateCreated = Date()
        
        // Serialize map data
        if let data = try? JSONEncoder().encode(map) {
            entity.mapData = data
        }
        
        saveContext()
    }
    
    func loadMaps() -> [WorldMap] {
        let request: NSFetchRequest<MapEntity> = MapEntity.fetchRequest()
        request.sortDescriptors = [NSSortDescriptor(keyPath: \MapEntity.dateCreated, ascending: false)]
        
        do {
            let entities = try context.fetch(request)
            return entities.compactMap { entity in
                guard let data = entity.mapData else { return nil }
                return try? JSONDecoder().decode(WorldMap.self, from: data)
            }
        } catch {
            print("Load error: \(error)")
            return []
        }
    }
}
```

## Graphics and Rendering

### Metal Integration

```swift
// RenderingService.swift - High-performance rendering
import Metal
import MetalKit

class RenderingService: ObservableObject {
    private let device: MTLDevice
    private let commandQueue: MTLCommandQueue
    private let renderPipeline: MTLRenderPipelineState
    
    init() throws {
        guard let device = MTLCreateSystemDefaultDevice() else {
            throw RenderingError.metalNotSupported
        }
        
        self.device = device
        
        guard let commandQueue = device.makeCommandQueue() else {
            throw RenderingError.commandQueueCreationFailed
        }
        
        self.commandQueue = commandQueue
        
        // Create render pipeline
        let library = device.makeDefaultLibrary()
        let vertexFunction = library?.makeFunction(name: "vertex_main")
        let fragmentFunction = library?.makeFunction(name: "fragment_main")
        
        let pipelineDescriptor = MTLRenderPipelineDescriptor()
        pipelineDescriptor.vertexFunction = vertexFunction
        pipelineDescriptor.fragmentFunction = fragmentFunction
        pipelineDescriptor.colorAttachments[0].pixelFormat = .bgra8Unorm
        
        self.renderPipeline = try device.makeRenderPipelineState(descriptor: pipelineDescriptor)
    }
    
    func renderMap(_ map: WorldMap, size: CGSize) async -> CGImage? {
        return await withCheckedContinuation { continuation in
            let textureDescriptor = MTLTextureDescriptor.texture2DDescriptor(
                pixelFormat: .bgra8Unorm,
                width: Int(size.width),
                height: Int(size.height),
                mipmapped: false
            )
            textureDescriptor.usage = [.renderTarget, .shaderRead]
            
            guard let texture = device.makeTexture(descriptor: textureDescriptor) else {
                continuation.resume(returning: nil)
                return
            }
            
            let renderPassDescriptor = MTLRenderPassDescriptor()
            renderPassDescriptor.colorAttachments[0].texture = texture
            renderPassDescriptor.colorAttachments[0].loadAction = .clear
            renderPassDescriptor.colorAttachments[0].clearColor = MTLClearColor(red: 0, green: 0, blue: 1, alpha: 1) // Ocean blue
            
            guard let commandBuffer = commandQueue.makeCommandBuffer(),
                  let renderEncoder = commandBuffer.makeRenderCommandEncoder(descriptor: renderPassDescriptor) else {
                continuation.resume(returning: nil)
                return
            }
            
            renderEncoder.setRenderPipelineState(renderPipeline)
            
            // Render map cells
            for cell in map.cells {
                let vertices = createCellVertices(cell: cell, mapSize: (map.width, map.height), renderSize: size)
                let vertexBuffer = device.makeBuffer(bytes: vertices, length: vertices.count * MemoryLayout<Vertex>.stride)
                
                renderEncoder.setVertexBuffer(vertexBuffer, offset: 0, index: 0)
                renderEncoder.drawPrimitives(type: .triangle, vertexStart: 0, vertexCount: 6)
            }
            
            renderEncoder.endEncoding()
            
            commandBuffer.addCompletedHandler { _ in
                let image = self.createCGImage(from: texture)
                continuation.resume(returning: image)
            }
            
            commandBuffer.commit()
        }
    }
}
```

## iPad-Specific Features

### Multitasking Support

```swift
// SceneDelegate.swift - Scene configuration
class SceneDelegate: UIResponder, UIWindowSceneDelegate {
    func scene(_ scene: UIScene, willConnectTo session: UISceneSession, options connectionOptions: UIScene.ConnectionOptions) {
        guard let windowScene = scene as? UIWindowScene else { return }
        
        // Configure for multitasking
        if UIDevice.current.userInterfaceIdiom == .pad {
            windowScene.sizeRestrictions?.minimumSize = CGSize(width: 768, height: 1024)
            windowScene.sizeRestrictions?.maximumSize = CGSize(width: 1366, height: 1024)
        }
    }
}
```

### Apple Pencil Integration

```swift
// PencilDrawingView.swift - Apple Pencil support
import PencilKit

struct PencilDrawingView: UIViewRepresentable {
    @Binding var canvasView: PKCanvasView
    
    func makeUIView(context: Context) -> PKCanvasView {
        canvasView.drawingPolicy = .anyInput
        canvasView.tool = PKInkingTool(.pen, color: .black, width: 2)
        
        // Enable Apple Pencil features
        if #available(iOS 14.0, *) {
            canvasView.drawingGestureRecognizer.isEnabled = true
        }
        
        return canvasView
    }
    
    func updateUIView(_ uiView: PKCanvasView, context: Context) {
        // Update drawing tools based on selected map tool
    }
}
```

## Deployment

### App Store Configuration

```xml
<!-- Info.plist - App configuration -->
<dict>
    <key>CFBundleDisplayName</key>
    <string>World Foundry</string>
    <key>CFBundleIdentifier</key>
    <string>com.balexda.worldfoundry</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    
    <!-- Document types -->
    <key>CFBundleDocumentTypes</key>
    <array>
        <dict>
            <key>CFBundleTypeName</key>
            <string>Azgaar Map File</string>
            <key>CFBundleTypeRole</key>
            <string>Viewer</string>
            <key>LSItemContentTypes</key>
            <array>
                <string>com.azgaar.map</string>
            </array>
        </dict>
    </array>
    
    <!-- Exported UTI -->
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
    
    <!-- Required device capabilities -->
    <key>UIRequiredDeviceCapabilities</key>
    <array>
        <string>metal</string>
    </array>
    
    <!-- Supported interface orientations -->
    <key>UISupportedInterfaceOrientations</key>
    <array>
        <string>UIInterfaceOrientationPortrait</string>
        <string>UIInterfaceOrientationLandscapeLeft</string>
        <string>UIInterfaceOrientationLandscapeRight</string>
    </array>
</dict>
```

### Build Configuration

```swift
// Package.swift - Swift package for Rust core
// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "WorldFoundryCore",
    platforms: [
        .iOS(.v15),
        .macOS(.v12)
    ],
    products: [
        .library(name: "WorldFoundryCore", targets: ["WorldFoundryCore"])
    ],
    targets: [
        .target(
            name: "WorldFoundryCore",
            dependencies: ["WorldFoundryRust"]
        ),
        .binaryTarget(
            name: "WorldFoundryRust",
            path: "Libraries/libworld_foundry.xcframework"
        )
    ]
)
```

## Performance Optimization

### Memory Management
- **ARC Integration** - Proper Swift/Rust memory coordination
- **Image Caching** - Efficient texture caching for map rendering
- **Background Processing** - Core engine operations on background queues

### Battery Optimization
- **Metal Performance** - GPU-accelerated rendering when beneficial
- **Adaptive Quality** - Reduce rendering quality on battery power
- **Background App Refresh** - Minimal background processing

### Responsive UI
- **Main Actor** - UI updates on main thread
- **Async/Await** - Non-blocking operations
- **Progressive Loading** - Incremental map loading for large files

## Testing Strategy

### Unit Tests
```swift
class MapViewModelTests: XCTestCase {
    func testImportValidMap() async throws {
        let viewModel = MapViewModel(
            coreEngine: MockCoreEngine(),
            renderingService: MockRenderingService()
        )
        
        let testURL = Bundle(for: type(of: self)).url(forResource: "test", withExtension: "map")!
        
        await viewModel.importMap(from: testURL)
        
        XCTAssertNotNil(viewModel.currentMap)
        XCTAssertNil(viewModel.errorMessage)
    }
}
```

### UI Tests
```swift
class MapInteractionUITests: XCTestCase {
    func testMapPanAndZoom() throws {
        let app = XCUIApplication()
        app.launch()
        
        let mapView = app.otherElements["MapCanvas"]
        XCTAssertTrue(mapView.exists)
        
        // Test pan gesture
        mapView.swipeRight()
        
        // Test zoom gesture
        mapView.pinch(withScale: 2.0, velocity: 1.0)
        
        // Verify map responded to gestures
        XCTAssertTrue(mapView.exists)
    }
}
```

## Future Enhancements

### iOS 17+ Features
- **Interactive Widgets** - Map preview widgets
- **Shortcuts Integration** - Siri shortcuts for common tasks
- **Live Activities** - Map generation progress

### visionOS Support
- **Spatial Computing** - 3D map visualization
- **Hand Tracking** - Natural interaction with maps
- **Immersive Environments** - Full 3D world exploration

### Advanced Features
- **CloudKit Sync** - Cross-device map synchronization
- **Collaborative Editing** - Real-time map collaboration
- **AR Integration** - Augmented reality map overlay

## Development Setup

### Prerequisites
- Xcode 15.0 or later
- iOS 15.0+ deployment target
- Swift 5.9+
- Rust toolchain with iOS targets

### Build Instructions
```bash
# Install iOS targets for Rust
rustup target add aarch64-apple-ios
rustup target add x86_64-apple-ios

# Build Rust core for iOS
cd core
cargo build --target aarch64-apple-ios --release
cargo build --target x86_64-apple-ios --release

# Create XCFramework
xcodebuild -create-xcframework \
  -library target/aarch64-apple-ios/release/libworld_foundry.a \
  -library target/x86_64-apple-ios/release/libworld_foundry.a \
  -output ../platforms/ios/Libraries/libworld_foundry.xcframework

# Open Xcode project
cd ../platforms/ios
open WorldFoundry.xcodeproj

# Build and run
# Use Xcode to build and deploy to device or simulator
```

This iOS platform architecture provides a comprehensive foundation for a native iOS experience with modern SwiftUI patterns, efficient Rust integration, and platform-specific optimizations for both iPhone and iPad.