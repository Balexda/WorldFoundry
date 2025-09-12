# Windows Platform Architecture

## Overview

The Windows implementation of World Foundry leverages WinUI 3 and the Windows App SDK to provide a native Windows experience with modern UI patterns and deep OS integration.

## Architecture Stack

```
┌─────────────────────────────────────────────────────────────┐
│                    WinUI 3 Application                     │
├─────────────────────────────────────────────────────────────┤
│                  Windows App SDK                           │
├─────────────────────────────────────────────────────────────┤
│              Platform Abstraction Layer                    │
├─────────────────────────────────────────────────────────────┤
│                 World Foundry Core                         │
│                   (Rust Engine)                            │
└─────────────────────────────────────────────────────────────┘
```

## Technology Stack

### UI Framework: WinUI 3
- **Modern XAML** - Declarative UI with data binding
- **Fluent Design** - Native Windows 11 design language
- **Performance** - Hardware-accelerated rendering
- **Accessibility** - Built-in screen reader and keyboard navigation support

### Runtime: Windows App SDK
- **Deployment** - Self-contained or framework-dependent
- **APIs** - Access to modern Windows APIs
- **Packaging** - MSIX packaging for Microsoft Store distribution
- **Updates** - Automatic update mechanisms

### Core Integration: C# Interop
- **P/Invoke** - Direct calls to Rust core engine
- **Memory Management** - Safe marshaling between managed and native code
- **Threading** - Async/await patterns with Rust async runtime
- **Error Handling** - Exception translation between C# and Rust

## Project Structure

```
platforms/windows/
├── WorldFoundry.sln                 # Visual Studio solution
├── WorldFoundry/
│   ├── WorldFoundry.csproj         # Main application project
│   ├── App.xaml                    # Application definition
│   ├── App.xaml.cs                 # Application code-behind
│   ├── MainWindow.xaml             # Main window UI
│   ├── MainWindow.xaml.cs          # Main window logic
│   ├── Views/                      # XAML views and pages
│   │   ├── MapView.xaml           # Map rendering view
│   │   ├── ImportView.xaml        # Azgaar import interface
│   │   ├── ExportView.xaml        # Export options
│   │   └── SettingsView.xaml      # Application settings
│   ├── ViewModels/                # MVVM view models
│   │   ├── MainViewModel.cs       # Main window view model
│   │   ├── MapViewModel.cs        # Map display logic
│   │   └── ImportViewModel.cs     # Import workflow logic
│   ├── Services/                  # Platform services
│   │   ├── FileService.cs         # File system operations
│   │   ├── StorageService.cs      # Local data storage
│   │   └── RenderingService.cs    # Graphics rendering
│   ├── Interop/                   # Rust interop layer
│   │   ├── CoreEngine.cs          # Core engine wrapper
│   │   ├── NativeMethods.cs       # P/Invoke declarations
│   │   └── DataStructures.cs      # Shared data types
│   └── Assets/                    # Application resources
│       ├── Icons/                 # Application icons
│       ├── Images/                # UI images
│       └── Styles/                # XAML styles
├── WorldFoundry.Core/             # Rust core library wrapper
│   ├── WorldFoundry.Core.csproj   # C# wrapper project
│   ├── CoreEngine.cs              # High-level API wrapper
│   └── Native/                    # Native library binaries
│       ├── world_foundry.dll      # Rust core engine (Windows)
│       └── world_foundry.lib      # Import library
└── WorldFoundry.Tests/            # Unit and integration tests
    ├── WorldFoundry.Tests.csproj  # Test project
    ├── ViewModelTests/            # View model unit tests
    ├── ServiceTests/              # Service layer tests
    └── IntegrationTests/          # End-to-end tests
```

## Core Integration

### Rust Core Binding

The Windows platform communicates with the Rust core through a C-compatible FFI layer:

```csharp
// CoreEngine.cs - High-level C# wrapper
public class CoreEngine : IDisposable
{
    private IntPtr _handle;
    
    public CoreEngine()
    {
        _handle = NativeMethods.world_foundry_create();
    }
    
    public async Task<WorldMap> ImportAzgaarMapAsync(string filePath)
    {
        return await Task.Run(() =>
        {
            var result = NativeMethods.world_foundry_import_azgaar(
                _handle, filePath);
            return WorldMap.FromNative(result);
        });
    }
    
    public void Dispose()
    {
        if (_handle != IntPtr.Zero)
        {
            NativeMethods.world_foundry_destroy(_handle);
            _handle = IntPtr.Zero;
        }
    }
}
```

### Data Marshaling

```csharp
// DataStructures.cs - Shared data types
[StructLayout(LayoutKind.Sequential)]
public struct NativeWorldMap
{
    public IntPtr name;
    public uint width;
    public uint height;
    public IntPtr cells;
    public uint cell_count;
}

public class WorldMap
{
    public string Name { get; set; }
    public uint Width { get; set; }
    public uint Height { get; set; }
    public List<Cell> Cells { get; set; }
    
    public static WorldMap FromNative(NativeWorldMap native)
    {
        return new WorldMap
        {
            Name = Marshal.PtrToStringUTF8(native.name),
            Width = native.width,
            Height = native.height,
            Cells = MarshalCellArray(native.cells, native.cell_count)
        };
    }
}
```

## UI Architecture

### MVVM Pattern

The Windows app follows the Model-View-ViewModel (MVVM) pattern:

```csharp
// MapViewModel.cs - Map display logic
public class MapViewModel : INotifyPropertyChanged
{
    private readonly CoreEngine _coreEngine;
    private readonly RenderingService _renderingService;
    private WorldMap _currentMap;
    
    public WorldMap CurrentMap
    {
        get => _currentMap;
        set
        {
            _currentMap = value;
            OnPropertyChanged();
            UpdateMapDisplay();
        }
    }
    
    public ICommand ImportMapCommand { get; }
    public ICommand ExportMapCommand { get; }
    public ICommand ZoomInCommand { get; }
    public ICommand ZoomOutCommand { get; }
    
    private async void UpdateMapDisplay()
    {
        if (_currentMap != null)
        {
            var bitmap = await _renderingService.RenderMapAsync(_currentMap);
            MapImage = bitmap;
        }
    }
}
```

### Data Binding

```xml
<!-- MapView.xaml - Map rendering view -->
<UserControl x:Class="WorldFoundry.Views.MapView">
    <Grid>
        <ScrollViewer ZoomMode="Enabled" 
                      HorizontalScrollMode="Enabled"
                      VerticalScrollMode="Enabled">
            <Image Source="{Binding MapImage}"
                   Stretch="None"/>
        </ScrollViewer>
        
        <CommandBar VerticalAlignment="Bottom">
            <AppBarButton Icon="Import" 
                         Label="Import"
                         Command="{Binding ImportMapCommand}"/>
            <AppBarButton Icon="Save" 
                         Label="Export"
                         Command="{Binding ExportMapCommand}"/>
            <AppBarSeparator/>
            <AppBarButton Icon="ZoomIn" 
                         Command="{Binding ZoomInCommand}"/>
            <AppBarButton Icon="ZoomOut" 
                         Command="{Binding ZoomOutCommand}"/>
        </CommandBar>
    </Grid>
</UserControl>
```

## Platform Services

### File System Integration

```csharp
// FileService.cs - File system operations
public class FileService : IFileService
{
    public async Task<string> PickFileAsync(string[] extensions)
    {
        var picker = new FileOpenPicker();
        picker.ViewMode = PickerViewMode.List;
        
        foreach (var ext in extensions)
        {
            picker.FileTypeFilter.Add(ext);
        }
        
        var file = await picker.PickSingleFileAsync();
        return file?.Path;
    }
    
    public async Task<string> SaveFileAsync(string suggestedName, 
                                           string[] extensions)
    {
        var picker = new FileSavePicker();
        picker.SuggestedFileName = suggestedName;
        
        foreach (var ext in extensions)
        {
            picker.FileTypeChoices.Add($"{ext} files", new[] { ext });
        }
        
        var file = await picker.PickSaveFileAsync();
        return file?.Path;
    }
}
```

### Storage Service

```csharp
// StorageService.cs - Local data storage
public class StorageService : IStorageService
{
    private readonly string _appDataPath;
    
    public StorageService()
    {
        _appDataPath = Path.Combine(
            Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData),
            "WorldFoundry");
        Directory.CreateDirectory(_appDataPath);
    }
    
    public async Task SaveSettingsAsync(AppSettings settings)
    {
        var json = JsonSerializer.Serialize(settings);
        var path = Path.Combine(_appDataPath, "settings.json");
        await File.WriteAllTextAsync(path, json);
    }
    
    public async Task<AppSettings> LoadSettingsAsync()
    {
        var path = Path.Combine(_appDataPath, "settings.json");
        if (!File.Exists(path))
            return new AppSettings();
            
        var json = await File.ReadAllTextAsync(path);
        return JsonSerializer.Deserialize<AppSettings>(json);
    }
}
```

## Graphics and Rendering

### Win2D Integration

For high-performance graphics rendering, the Windows platform uses Win2D:

```csharp
// RenderingService.cs - Graphics rendering
public class RenderingService : IRenderingService
{
    private CanvasDevice _canvasDevice;
    
    public RenderingService()
    {
        _canvasDevice = CanvasDevice.GetSharedDevice();
    }
    
    public async Task<SoftwareBitmap> RenderMapAsync(WorldMap map)
    {
        var renderTarget = new CanvasRenderTarget(
            _canvasDevice, map.Width, map.Height, 96);
            
        using (var session = renderTarget.CreateDrawingSession())
        {
            session.Clear(Colors.Blue); // Ocean
            
            foreach (var cell in map.Cells)
            {
                var color = GetBiomeColor(cell.Biome);
                var rect = new Rect(cell.X, cell.Y, 1, 1);
                session.FillRectangle(rect, color);
            }
        }
        
        return await SoftwareBitmap.CreateCopyFromSurfaceAsync(renderTarget);
    }
    
    private Color GetBiomeColor(BiomeType biome)
    {
        return biome switch
        {
            BiomeType.Ocean => Colors.Blue,
            BiomeType.Forest => Colors.Green,
            BiomeType.Desert => Colors.Yellow,
            BiomeType.Mountain => Colors.Gray,
            _ => Colors.White
        };
    }
}
```

## Deployment

### MSIX Packaging

```xml
<!-- Package.appxmanifest - Application manifest -->
<Package xmlns="http://schemas.microsoft.com/appx/manifest/foundation/windows10">
  <Identity Name="WorldFoundry"
            Publisher="CN=Balexda"
            Version="1.0.0.0" />
            
  <Properties>
    <DisplayName>World Foundry</DisplayName>
    <PublisherDisplayName>Balexda</PublisherDisplayName>
    <Logo>Assets\StoreLogo.png</Logo>
  </Properties>
  
  <Dependencies>
    <TargetDeviceFamily Name="Windows.Universal" 
                        MinVersion="10.0.17763.0" 
                        MaxVersionTested="10.0.22621.0" />
    <PackageDependency Name="Microsoft.WindowsAppRuntime.1.4"
                       MinVersion="4000.934.1904.0"
                       Publisher="CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond, S=Washington, C=US" />
  </Dependencies>
  
  <Applications>
    <Application Id="App" Executable="WorldFoundry.exe" EntryPoint="$targetentrypoint$">
      <uap:VisualElements DisplayName="World Foundry"
                          Square150x150Logo="Assets\Square150x150Logo.png"
                          Square44x44Logo="Assets\Square44x44Logo.png"
                          BackgroundColor="transparent">
        <uap:DefaultTile Wide310x150Logo="Assets\Wide310x150Logo.png" />
      </uap:VisualElements>
    </Application>
  </Applications>
</Package>
```

### Build Configuration

```xml
<!-- WorldFoundry.csproj - Project configuration -->
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>WinExe</OutputType>
    <TargetFramework>net8.0-windows10.0.19041.0</TargetFramework>
    <UseWinUI>true</UseWinUI>
    <EnableMsixTooling>true</EnableMsixTooling>
    <WindowsAppSDKSelfContained>true</WindowsAppSDKSelfContained>
  </PropertyGroup>
  
  <ItemGroup>
    <PackageReference Include="Microsoft.WindowsAppSDK" Version="1.4.231008000" />
    <PackageReference Include="Microsoft.Windows.SDK.BuildTools" Version="10.0.22621.2428" />
    <PackageReference Include="Win2D.WinUI" Version="1.0.4" />
  </ItemGroup>
  
  <ItemGroup>
    <Content Include="Native\world_foundry.dll">
      <CopyToOutputDirectory>PreserveNewest</CopyToOutputDirectory>
    </Content>
  </ItemGroup>
</Project>
```

## Performance Considerations

### Memory Management
- **Native Interop** - Careful management of native memory allocations
- **Image Caching** - Efficient bitmap caching for map tiles
- **Async Operations** - Non-blocking UI with async/await patterns

### Threading
- **UI Thread** - Keep UI responsive with background processing
- **Rust Runtime** - Coordinate with Rust async runtime
- **Cancellation** - Proper cancellation token usage

### Graphics Optimization
- **Hardware Acceleration** - Leverage GPU for rendering when available
- **Level of Detail** - Adaptive rendering based on zoom level
- **Viewport Culling** - Only render visible map regions

## Testing Strategy

### Unit Tests
```csharp
[TestClass]
public class MapViewModelTests
{
    [TestMethod]
    public async Task ImportMap_ValidFile_UpdatesCurrentMap()
    {
        // Arrange
        var mockEngine = new Mock<ICoreEngine>();
        var mockFileService = new Mock<IFileService>();
        var viewModel = new MapViewModel(mockEngine.Object, mockFileService.Object);
        
        // Act
        await viewModel.ImportMapCommand.ExecuteAsync("test.map");
        
        // Assert
        Assert.IsNotNull(viewModel.CurrentMap);
    }
}
```

### Integration Tests
```csharp
[TestClass]
public class CoreIntegrationTests
{
    [TestMethod]
    public void CoreEngine_ImportAzgaar_ReturnsValidMap()
    {
        // Arrange
        using var engine = new CoreEngine();
        var testFile = "TestData/sample.map";
        
        // Act
        var map = engine.ImportAzgaarMap(testFile);
        
        // Assert
        Assert.IsNotNull(map);
        Assert.IsTrue(map.Width > 0);
        Assert.IsTrue(map.Height > 0);
    }
}
```

## Future Enhancements

### Windows 11 Features
- **Snap Layouts** - Multi-window support for complex workflows
- **Widgets** - Quick map preview widgets
- **Share Contract** - Easy sharing of generated maps

### Microsoft Store
- **Store Deployment** - MSIX packaging for store distribution
- **In-App Purchases** - Premium features and content packs
- **Store Reviews** - User feedback integration

### Enterprise Features
- **Group Policy** - Enterprise deployment configuration
- **SSO Integration** - Active Directory authentication
- **Network Storage** - SharePoint and OneDrive integration

## Development Setup

### Prerequisites
- Visual Studio 2022 (17.8 or later)
- Windows 11 SDK (10.0.22621.0 or later)
- .NET 8.0 SDK
- Windows App SDK 1.4
- Rust toolchain (for core engine development)

### Build Instructions
```bash
# Clone repository
git clone https://github.com/Balexda/WorldFoundry.git
cd WorldFoundry/platforms/windows

# Restore dependencies
dotnet restore

# Build core engine (if modified)
cd ../../core
cargo build --release
cp target/release/world_foundry.dll ../platforms/windows/WorldFoundry/Native/

# Build Windows app
cd ../platforms/windows
dotnet build --configuration Release

# Run tests
dotnet test

# Package for distribution
dotnet publish --configuration Release --output ./publish
```

This Windows platform architecture provides a solid foundation for a native Windows experience while maintaining clean separation from the core engine and enabling future enhancements.