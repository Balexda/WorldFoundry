# Windows Development Guide

This guide covers setting up and developing the Windows platform implementation of World Foundry using WinUI 3 and the Windows App SDK.

## Prerequisites

### System Requirements
- **OS**: Windows 10 version 1903 (build 18362) or later
- **Recommended**: Windows 11 version 22H2 or later
- **RAM**: 8GB minimum, 16GB recommended
- **Storage**: 10GB free space for development tools
- **GPU**: DirectX 11 compatible graphics card

### Required Software

#### 1. Visual Studio 2022
Download and install Visual Studio 2022 Community (free) or higher:
- **Download**: https://visualstudio.microsoft.com/downloads/

**Required Workloads:**
- ✅ **.NET desktop development**
- ✅ **Universal Windows Platform development**
- ✅ **Desktop development with C++**

**Individual Components to Add:**
- Windows 11 SDK (10.0.22621.0 or later)
- MSVC v143 - VS 2022 C++ x64/x86 build tools
- CMake tools for Visual Studio
- Git for Windows

#### 2. Windows App SDK
The Windows App SDK will be installed automatically with the project templates, but you can also install it manually:
- **Download**: https://docs.microsoft.com/en-us/windows/apps/windows-app-sdk/

#### 3. Additional Tools
```powershell
# Install Windows Terminal (recommended)
winget install Microsoft.WindowsTerminal

# Install PowerShell 7
winget install Microsoft.PowerShell

# Install Git (if not installed with Visual Studio)
winget install Git.Git

# Install Windows Subsystem for Linux (optional, for cross-platform testing)
wsl --install
```

## Development Environment Setup

### 1. Clone and Setup Repository

```powershell
# Clone the repository
git clone https://github.com/Balexda/WorldFoundry.git
cd WorldFoundry

# Navigate to Windows platform
cd platforms\windows
```

### 2. Rust Core Engine Setup

```powershell
# Navigate to core engine
cd ..\..\core

# Install Windows-specific Rust target
rustup target add x86_64-pc-windows-msvc

# Build the core engine
cargo build --release --target x86_64-pc-windows-msvc

# Copy the built library to Windows platform
Copy-Item "target\x86_64-pc-windows-msvc\release\world_foundry.dll" "..\platforms\windows\WorldFoundry\Native\"
Copy-Item "target\x86_64-pc-windows-msvc\release\world_foundry.lib" "..\platforms\windows\WorldFoundry\Native\"
```

### 3. Windows Platform Setup

```powershell
# Navigate back to Windows platform
cd ..\platforms\windows

# Restore NuGet packages
dotnet restore

# Build the solution
dotnet build

# Or open in Visual Studio
start WorldFoundry.sln
```

## Project Structure Deep Dive

### Solution Structure
```
platforms/windows/
├── WorldFoundry.sln                 # Visual Studio solution file
├── WorldFoundry/                    # Main WinUI 3 application
│   ├── WorldFoundry.csproj         # Project file
│   ├── Package.appxmanifest        # App manifest for packaging
│   ├── app.manifest                # Application manifest
│   ├── App.xaml                    # Application definition
│   ├── App.xaml.cs                 # Application code-behind
│   ├── MainWindow.xaml             # Main window XAML
│   ├── MainWindow.xaml.cs          # Main window code-behind
│   └── [Other source files...]
├── WorldFoundry.Core/              # Rust core wrapper library
│   ├── WorldFoundry.Core.csproj    # Wrapper project
│   ├── CoreEngine.cs               # High-level C# API
│   └── Native/                     # Native Rust libraries
│       ├── world_foundry.dll       # Rust core engine
│       └── world_foundry.lib       # Import library
└── WorldFoundry.Tests/             # Unit and integration tests
    ├── WorldFoundry.Tests.csproj   # Test project
    └── [Test files...]
```

### Key Configuration Files

#### WorldFoundry.csproj
```xml
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>WinExe</OutputType>
    <TargetFramework>net8.0-windows10.0.19041.0</TargetFramework>
    <TargetPlatformMinVersion>10.0.17763.0</TargetPlatformMinVersion>
    <UseWinUI>true</UseWinUI>
    <EnableMsixTooling>true</EnableMsixTooling>
    <WindowsAppSDKSelfContained>true</WindowsAppSDKSelfContained>
    <WindowsPackageType>None</WindowsPackageType>
    <PublishProfile>win-$(Platform).pubxml</PublishProfile>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>enable</Nullable>
    <UseWindowsForms>false</UseWindowsForms>
    <UseWPF>false</UseWPF>
    <ApplicationManifest>app.manifest</ApplicationManifest>
  </PropertyGroup>

  <ItemGroup>
    <PackageReference Include="Microsoft.WindowsAppSDK" Version="1.4.231008000" />
    <PackageReference Include="Microsoft.Windows.SDK.BuildTools" Version="10.0.22621.2428" />
    <PackageReference Include="Win2D.WinUI" Version="1.0.4" />
    <PackageReference Include="CommunityToolkit.Mvvm" Version="8.2.2" />
    <PackageReference Include="Microsoft.Extensions.Hosting" Version="8.0.0" />
    <PackageReference Include="Microsoft.Extensions.DependencyInjection" Version="8.0.0" />
  </ItemGroup>

  <ItemGroup>
    <Content Include="Native\world_foundry.dll">
      <CopyToOutputDirectory>PreserveNewest</CopyToOutputDirectory>
    </Content>
  </ItemGroup>

  <ItemGroup>
    <ProjectReference Include="..\WorldFoundry.Core\WorldFoundry.Core.csproj" />
  </ItemGroup>
</Project>
```

## Development Workflow

### 1. Daily Development Setup

```powershell
# Start Visual Studio with the solution
start WorldFoundry.sln

# Or use command line development
# Terminal 1: Watch for Rust changes
cd core
cargo watch -x "build --release --target x86_64-pc-windows-msvc"

# Terminal 2: Watch for C# changes
cd platforms\windows
dotnet watch run
```

### 2. Building the Application

#### Debug Build
```powershell
# Build debug version
dotnet build --configuration Debug

# Run debug version
dotnet run --configuration Debug

# Or use Visual Studio: Build → Build Solution (Ctrl+Shift+B)
```

#### Release Build
```powershell
# Build release version
dotnet build --configuration Release

# Create self-contained deployment
dotnet publish --configuration Release --runtime win-x64 --self-contained true

# Create MSIX package (for Microsoft Store)
dotnet publish --configuration Release --runtime win-x64 /p:PublishProfile=win-x64.pubxml
```

### 3. Debugging

#### Visual Studio Debugging
1. Set breakpoints in C# code
2. Press F5 or Debug → Start Debugging
3. Use Debug → Windows → Output to view debug output

#### Rust Core Debugging
```powershell
# Build Rust core with debug symbols
cd core
cargo build --target x86_64-pc-windows-msvc

# Copy debug version to Windows platform
Copy-Item "target\x86_64-pc-windows-msvc\debug\world_foundry.dll" "..\platforms\windows\WorldFoundry\Native\"

# Use Visual Studio's mixed-mode debugging
# Debug → Attach to Process → Select WorldFoundry.exe
# Enable native code debugging in project properties
```

#### Logging and Diagnostics
```csharp
// Add to App.xaml.cs for comprehensive logging
public partial class App : Application
{
    protected override void OnLaunched(LaunchActivatedEventArgs args)
    {
        // Enable debug output
        #if DEBUG
        this.DebugSettings.EnableFrameRateCounter = true;
        this.DebugSettings.IsBindingTracingEnabled = true;
        this.DebugSettings.IsOverdrawHeatMapEnabled = true;
        #endif

        base.OnLaunched(args);
    }
}
```

### 4. Testing

#### Unit Tests
```powershell
# Run all tests
dotnet test

# Run tests with coverage
dotnet test --collect:"XPlat Code Coverage"

# Run specific test category
dotnet test --filter Category=Integration
```

#### UI Tests
```powershell
# Install WinAppDriver for UI automation testing
winget install Microsoft.WinAppDriver

# Run UI tests (if implemented)
dotnet test --filter Category=UI
```

## Core Engine Integration

### 1. P/Invoke Declarations

```csharp
// NativeMethods.cs - P/Invoke declarations for Rust core
using System.Runtime.InteropServices;

internal static class NativeMethods
{
    private const string DllName = "world_foundry.dll";

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr world_foundry_create();

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern void world_foundry_destroy(IntPtr handle);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern int world_foundry_import_azgaar(
        IntPtr handle, 
        [MarshalAs(UnmanagedType.LPStr)] string path);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern NativeImageData world_foundry_render_map(
        IntPtr handle, 
        IntPtr map, 
        int width, 
        int height);

    [StructLayout(LayoutKind.Sequential)]
    internal struct NativeImageData
    {
        public IntPtr data;
        public uint width;
        public uint height;
        public uint bytes_per_pixel;
    }
}
```

### 2. High-Level C# Wrapper

```csharp
// CoreEngine.cs - High-level wrapper for Rust core
public sealed class CoreEngine : IDisposable
{
    private IntPtr _handle;
    private bool _disposed;

    public CoreEngine()
    {
        _handle = NativeMethods.world_foundry_create();
        if (_handle == IntPtr.Zero)
        {
            throw new InvalidOperationException("Failed to create core engine");
        }
    }

    public async Task<WorldMap> ImportAzgaarMapAsync(string filePath)
    {
        ThrowIfDisposed();
        
        return await Task.Run(() =>
        {
            var result = NativeMethods.world_foundry_import_azgaar(_handle, filePath);
            if (result != 0)
            {
                throw new InvalidOperationException($"Failed to import map: {result}");
            }
            
            // Convert native data to managed objects
            return ConvertNativeMap(result);
        });
    }

    public async Task<SoftwareBitmap> RenderMapAsync(WorldMap map, int width, int height)
    {
        ThrowIfDisposed();
        
        return await Task.Run(() =>
        {
            var nativeMap = ConvertToNativeMap(map);
            var imageData = NativeMethods.world_foundry_render_map(_handle, nativeMap, width, height);
            
            return ConvertToSoftwareBitmap(imageData);
        });
    }

    private void ThrowIfDisposed()
    {
        if (_disposed)
        {
            throw new ObjectDisposedException(nameof(CoreEngine));
        }
    }

    public void Dispose()
    {
        if (!_disposed && _handle != IntPtr.Zero)
        {
            NativeMethods.world_foundry_destroy(_handle);
            _handle = IntPtr.Zero;
            _disposed = true;
        }
    }
}
```

## UI Development with WinUI 3

### 1. MVVM Pattern Implementation

```csharp
// ViewModels/MapViewModel.cs - Main map view model
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;

public partial class MapViewModel : ObservableObject
{
    private readonly CoreEngine _coreEngine;
    private readonly IFileService _fileService;

    [ObservableProperty]
    private WorldMap? _currentMap;

    [ObservableProperty]
    private SoftwareBitmap? _mapImage;

    [ObservableProperty]
    private bool _isLoading;

    [ObservableProperty]
    private string? _errorMessage;

    public MapViewModel(CoreEngine coreEngine, IFileService fileService)
    {
        _coreEngine = coreEngine;
        _fileService = fileService;
    }

    [RelayCommand]
    private async Task ImportMapAsync()
    {
        try
        {
            IsLoading = true;
            ErrorMessage = null;

            var file = await _fileService.PickFileAsync(new[] { ".map", ".json" });
            if (file != null)
            {
                CurrentMap = await _coreEngine.ImportAzgaarMapAsync(file.Path);
                await UpdateMapImageAsync();
            }
        }
        catch (Exception ex)
        {
            ErrorMessage = ex.Message;
        }
        finally
        {
            IsLoading = false;
        }
    }

    [RelayCommand]
    private async Task ExportMapAsync()
    {
        if (CurrentMap == null) return;

        try
        {
            var file = await _fileService.SaveFileAsync("exported_map.png", new[] { ".png" });
            if (file != null)
            {
                // Export logic here
            }
        }
        catch (Exception ex)
        {
            ErrorMessage = ex.Message;
        }
    }

    private async Task UpdateMapImageAsync()
    {
        if (CurrentMap != null)
        {
            MapImage = await _coreEngine.RenderMapAsync(CurrentMap, 1024, 1024);
        }
    }
}
```

### 2. XAML UI Design

```xml
<!-- Views/MapView.xaml - Main map display -->
<UserControl x:Class="WorldFoundry.Views.MapView">
    <Grid>
        <Grid.RowDefinitions>
            <RowDefinition Height="Auto"/>
            <RowDefinition Height="*"/>
            <RowDefinition Height="Auto"/>
        </Grid.RowDefinitions>

        <!-- Command Bar -->
        <CommandBar Grid.Row="0" DefaultLabelPosition="Right">
            <AppBarButton Icon="Import" 
                         Label="Import Map"
                         Command="{x:Bind ViewModel.ImportMapCommand}"/>
            <AppBarButton Icon="Save" 
                         Label="Export Map"
                         Command="{x:Bind ViewModel.ExportMapCommand}"/>
            <AppBarSeparator/>
            <AppBarButton Icon="ZoomIn" Label="Zoom In"/>
            <AppBarButton Icon="ZoomOut" Label="Zoom Out"/>
        </CommandBar>

        <!-- Map Display -->
        <ScrollViewer Grid.Row="1" 
                      ZoomMode="Enabled"
                      HorizontalScrollMode="Enabled"
                      VerticalScrollMode="Enabled"
                      HorizontalScrollBarVisibility="Auto"
                      VerticalScrollBarVisibility="Auto">
            
            <Grid>
                <!-- Map Image -->
                <Image x:Name="MapImage"
                       Source="{x:Bind ViewModel.MapImage, Mode=OneWay}"
                       Stretch="None"
                       HorizontalAlignment="Center"
                       VerticalAlignment="Center"/>

                <!-- Loading Overlay -->
                <Border Background="{ThemeResource SystemControlAcrylicWindowBrush}"
                        Visibility="{x:Bind ViewModel.IsLoading, Mode=OneWay}">
                    <StackPanel HorizontalAlignment="Center" 
                               VerticalAlignment="Center">
                        <ProgressRing IsActive="True" Width="50" Height="50"/>
                        <TextBlock Text="Loading map..." 
                                  Margin="0,10,0,0"
                                  HorizontalAlignment="Center"/>
                    </StackPanel>
                </Border>

                <!-- Error Display -->
                <InfoBar Grid.Row="1"
                         IsOpen="{x:Bind ViewModel.ErrorMessage, Converter={StaticResource StringToBoolConverter}, Mode=OneWay}"
                         Severity="Error"
                         Title="Error"
                         Message="{x:Bind ViewModel.ErrorMessage, Mode=OneWay}"
                         VerticalAlignment="Top"/>
            </Grid>
        </ScrollViewer>

        <!-- Status Bar -->
        <Border Grid.Row="2" 
                Background="{ThemeResource SystemControlBackgroundChromeMediumBrush}"
                BorderBrush="{ThemeResource SystemControlForegroundBaseMediumLowBrush}"
                BorderThickness="0,1,0,0">
            <Grid Padding="12,6">
                <Grid.ColumnDefinitions>
                    <ColumnDefinition Width="*"/>
                    <ColumnDefinition Width="Auto"/>
                </Grid.ColumnDefinitions>
                
                <TextBlock Grid.Column="0"
                          Text="{x:Bind ViewModel.CurrentMap.Name, Mode=OneWay, FallbackValue='No map loaded'}"
                          VerticalAlignment="Center"/>
                
                <TextBlock Grid.Column="1"
                          Text="{x:Bind ViewModel.CurrentMap.Dimensions, Mode=OneWay, FallbackValue=''}"
                          VerticalAlignment="Center"
                          Foreground="{ThemeResource SystemControlForegroundBaseMediumBrush}"/>
            </Grid>
        </Border>
    </Grid>
</UserControl>
```

## Performance Optimization

### 1. Graphics Performance

```csharp
// Use Win2D for high-performance graphics rendering
using Win2D.UI.Xaml;
using Win2D;

public sealed partial class MapCanvas : UserControl
{
    private CanvasDevice _canvasDevice;
    private WorldMap _worldMap;

    public MapCanvas()
    {
        this.InitializeComponent();
        _canvasDevice = CanvasDevice.GetSharedDevice();
    }

    private void CanvasControl_Draw(CanvasControl sender, CanvasDrawEventArgs args)
    {
        var session = args.DrawingSession;
        
        if (_worldMap != null)
        {
            DrawMap(session, _worldMap);
        }
    }

    private void DrawMap(CanvasDrawingSession session, WorldMap map)
    {
        // High-performance map rendering using Win2D
        session.Clear(Colors.Blue); // Ocean background

        foreach (var cell in map.Cells)
        {
            var color = GetBiomeColor(cell.Biome);
            var rect = new Rect(cell.X, cell.Y, 1, 1);
            session.FillRectangle(rect, color);
        }
    }
}
```

### 2. Memory Management

```csharp
// Implement proper disposal patterns
public sealed class MapService : IDisposable
{
    private readonly CoreEngine _coreEngine;
    private readonly SemaphoreSlim _semaphore;
    private bool _disposed;

    public MapService()
    {
        _coreEngine = new CoreEngine();
        _semaphore = new SemaphoreSlim(1, 1);
    }

    public async Task<WorldMap> LoadMapAsync(string path)
    {
        await _semaphore.WaitAsync();
        try
        {
            ThrowIfDisposed();
            return await _coreEngine.ImportAzgaarMapAsync(path);
        }
        finally
        {
            _semaphore.Release();
        }
    }

    private void ThrowIfDisposed()
    {
        if (_disposed)
        {
            throw new ObjectDisposedException(nameof(MapService));
        }
    }

    public void Dispose()
    {
        if (!_disposed)
        {
            _coreEngine?.Dispose();
            _semaphore?.Dispose();
            _disposed = true;
        }
    }
}
```

## Packaging and Distribution

### 1. MSIX Packaging

```xml
<!-- Package.appxmanifest - Application manifest -->
<?xml version="1.0" encoding="utf-8"?>
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
                       Publisher="CN=Microsoft Corporation" />
  </Dependencies>

  <Applications>
    <Application Id="App" Executable="WorldFoundry.exe" EntryPoint="$targetentrypoint$">
      <uap:VisualElements DisplayName="World Foundry"
                          Square150x150Logo="Assets\Square150x150Logo.png"
                          Square44x44Logo="Assets\Square44x44Logo.png"
                          BackgroundColor="transparent">
        <uap:DefaultTile Wide310x150Logo="Assets\Wide310x150Logo.png" />
      </uap:VisualElements>
      
      <Extensions>
        <uap:Extension Category="windows.fileTypeAssociation">
          <uap:FileTypeAssociation Name="azgaarmap">
            <uap:SupportedFileTypes>
              <uap:FileType>.map</uap:FileType>
            </uap:SupportedFileTypes>
          </uap:FileTypeAssociation>
        </uap:Extension>
      </Extensions>
    </Application>
  </Applications>
</Package>
```

### 2. Build Scripts

```powershell
# Build-Release.ps1 - Automated release build script
param(
    [string]$Version = "1.0.0",
    [string]$Configuration = "Release"
)

Write-Host "Building World Foundry Windows Release v$Version" -ForegroundColor Green

# Build Rust core
Write-Host "Building Rust core..." -ForegroundColor Yellow
Set-Location "..\..\core"
cargo build --release --target x86_64-pc-windows-msvc

# Copy native libraries
Write-Host "Copying native libraries..." -ForegroundColor Yellow
Copy-Item "target\x86_64-pc-windows-msvc\release\world_foundry.dll" "..\platforms\windows\WorldFoundry\Native\"
Copy-Item "target\x86_64-pc-windows-msvc\release\world_foundry.lib" "..\platforms\windows\WorldFoundry\Native\"

# Build Windows application
Write-Host "Building Windows application..." -ForegroundColor Yellow
Set-Location "..\platforms\windows"
dotnet restore
dotnet build --configuration $Configuration --verbosity minimal

# Create MSIX package
Write-Host "Creating MSIX package..." -ForegroundColor Yellow
dotnet publish --configuration $Configuration --runtime win-x64 --self-contained true

# Run tests
Write-Host "Running tests..." -ForegroundColor Yellow
dotnet test --configuration $Configuration --verbosity minimal

Write-Host "Build completed successfully!" -ForegroundColor Green
```

## Troubleshooting

### Common Issues and Solutions

#### 1. Native Library Not Found
```
Error: DllNotFoundException: Unable to load DLL 'world_foundry.dll'
```

**Solution:**
```powershell
# Ensure the DLL is in the output directory
Copy-Item "core\target\x86_64-pc-windows-msvc\release\world_foundry.dll" "platforms\windows\WorldFoundry\bin\Debug\net8.0-windows10.0.19041.0\"

# Or add to project file:
# <Content Include="Native\world_foundry.dll">
#   <CopyToOutputDirectory>PreserveNewest</CopyToOutputDirectory>
# </Content>
```

#### 2. WinUI 3 Designer Issues
```
Error: The designer could not be loaded
```

**Solution:**
```powershell
# Clear Visual Studio cache
Remove-Item -Recurse -Force "$env:LOCALAPPDATA\Microsoft\VisualStudio\17.0_*\ComponentModelCache"

# Restart Visual Studio
# Rebuild solution
```

#### 3. MSIX Packaging Errors
```
Error: App manifest validation error
```

**Solution:**
- Verify all required assets are present in Assets folder
- Check Package.appxmanifest syntax
- Ensure version numbers are properly formatted
- Validate publisher certificate

#### 4. Performance Issues
```
Issue: Slow map rendering or UI freezing
```

**Solution:**
```csharp
// Use async/await for long-running operations
private async Task LoadMapAsync()
{
    await Task.Run(async () =>
    {
        // Heavy work on background thread
        var map = await _coreEngine.ImportAzgaarMapAsync(path);
        
        // Update UI on main thread
        await Dispatcher.BeginInvoke(() =>
        {
            CurrentMap = map;
        });
    });
}
```

## Next Steps

1. **Complete the Setup**: Follow the [main development setup guide](DEVELOPMENT_SETUP.md)
2. **Learn iOS Development**: [iOS Development Guide](IOS_DEVELOPMENT.md)
3. **Learn Android Development**: [Android Development Guide](ANDROID_DEVELOPMENT.md)
4. **Deployment**: [Windows Deployment Guide](../deployment/WINDOWS_DEPLOYMENT.md)
5. **Debugging**: [Windows Debugging Guide](../debugging/WINDOWS_DEBUGGING.md)

## Additional Resources

- [WinUI 3 Documentation](https://docs.microsoft.com/en-us/windows/apps/winui/winui3/)
- [Windows App SDK Documentation](https://docs.microsoft.com/en-us/windows/apps/windows-app-sdk/)
- [MSIX Packaging Documentation](https://docs.microsoft.com/en-us/windows/msix/)
- [Win2D Documentation](https://microsoft.github.io/Win2D/WinUI3/html/Introduction.htm)
- [Community Toolkit MVVM](https://docs.microsoft.com/en-us/dotnet/communitytoolkit/mvvm/)