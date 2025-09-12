# Android Development Guide

This guide covers setting up and developing the Android platform implementation of World Foundry using Jetpack Compose and the Android SDK.

## Prerequisites

### System Requirements
- **OS**: Windows 10/11, macOS 10.14+, or Linux (Ubuntu 18.04+ recommended)
- **RAM**: 8GB minimum, 16GB recommended
- **Storage**: 20GB free space for Android Studio and SDKs
- **CPU**: Intel i5 or equivalent, 64-bit processor required

### Required Software

#### 1. Java Development Kit (JDK)
```bash
# Install JDK 17 (required for Android development)

# Ubuntu/Debian:
sudo apt update
sudo apt install openjdk-17-jdk

# macOS (using Homebrew):
brew install openjdk@17

# Windows:
# Download from Oracle or use OpenJDK distribution
# https://adoptium.net/temurin/releases/

# Verify installation
java -version
javac -version
```

#### 2. Android Studio
Download and install Android Studio:
- **Download**: https://developer.android.com/studio
- **Version**: Android Studio Hedgehog (2023.1.1) or later
- **Size**: ~1GB download, ~3GB installed

**Installation Steps:**
1. Download Android Studio installer
2. Run installer and follow setup wizard
3. Install Android SDK (API level 34 recommended)
4. Install Android NDK (version 25.1.8937393 or later)
5. Install CMake and LLDB for native debugging

#### 3. Android SDK Components
After installing Android Studio:
1. Open Android Studio
2. Go to Tools → SDK Manager
3. Install required components:
   - **SDK Platforms**: Android 14 (API level 34)
   - **SDK Tools**: 
     - Android NDK (Side by side)
     - CMake
     - LLDB
     - Android Emulator
     - Android SDK Platform-Tools

## Development Environment Setup

### 1. Environment Variables

```bash
# Add to ~/.bashrc, ~/.zshrc, or equivalent

# Android SDK
export ANDROID_HOME=$HOME/Android/Sdk  # Linux/macOS
# export ANDROID_HOME=%LOCALAPPDATA%\Android\Sdk  # Windows

# Android NDK
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/25.1.8937393

# Java
export JAVA_HOME=/usr/lib/jvm/java-17-openjdk-amd64  # Linux
# export JAVA_HOME=/Library/Java/JavaVirtualMachines/openjdk-17.jdk/Contents/Home  # macOS

# Path additions
export PATH=$PATH:$ANDROID_HOME/tools:$ANDROID_HOME/platform-tools:$ANDROID_HOME/cmdline-tools/latest/bin

# Reload environment
source ~/.bashrc  # or ~/.zshrc
```

### 2. Clone and Setup Repository

```bash
# Clone the repository
git clone https://github.com/Balexda/WorldFoundry.git
cd WorldFoundry

# Navigate to Android platform
cd platforms/android
```

### 3. Rust Core Engine Setup

```bash
# Navigate to core engine
cd ../../core

# Install Android-specific Rust targets
rustup target add aarch64-linux-android      # ARM64 (most common)
rustup target add armv7-linux-androideabi    # ARM32 (older devices)
rustup target add x86_64-linux-android       # x86_64 (emulator)
rustup target add i686-linux-android         # x86 (older emulator)

# Install cargo-ndk for Android builds
cargo install cargo-ndk

# Build the core engine for Android
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 -t x86 \
    -o ../platforms/android/app/src/main/jniLibs \
    build --release
```

### 4. Android Project Setup

```bash
# Navigate to Android platform
cd ../platforms/android

# Sync Gradle dependencies
./gradlew build

# Or open in Android Studio
# File → Open → Select platforms/android directory
```

## Project Structure Deep Dive

### Android Project Structure
```
platforms/android/
├── app/                            # Main Android application module
│   ├── build.gradle.kts           # App-level Gradle build script
│   ├── proguard-rules.pro         # ProGuard configuration
│   └── src/
│       ├── main/
│       │   ├── AndroidManifest.xml # App manifest
│       │   ├── java/com/balexda/worldfoundry/
│       │   │   ├── MainActivity.kt # Main activity
│       │   │   ├── WorldFoundryApplication.kt # Application class
│       │   │   ├── ui/            # UI components
│       │   │   │   ├── theme/     # Material Design theme
│       │   │   │   ├── screens/   # Compose screens
│       │   │   │   ├── components/ # Reusable UI components
│       │   │   │   └── navigation/ # Navigation setup
│       │   │   ├── viewmodel/     # ViewModels for MVVM
│       │   │   │   ├── MapViewModel.kt
│       │   │   │   ├── CultureViewModel.kt
│       │   │   │   └── SettingsViewModel.kt
│       │   │   ├── model/         # Data models
│       │   │   │   ├── WorldModel.kt
│       │   │   │   ├── CultureModel.kt
│       │   │   │   └── MapModel.kt
│       │   │   ├── service/       # Business logic services
│       │   │   │   ├── CoreEngineService.kt
│       │   │   │   ├── FileService.kt
│       │   │   │   └── ExportService.kt
│       │   │   ├── repository/    # Data repositories
│       │   │   │   ├── WorldRepository.kt
│       │   │   │   └── SettingsRepository.kt
│       │   │   ├── di/            # Dependency injection
│       │   │   │   └── AppModule.kt
│       │   │   └── util/          # Utility classes
│       │   │       ├── Extensions.kt
│       │   │       └── Constants.kt
│       │   ├── jniLibs/           # Native libraries
│       │   │   ├── arm64-v8a/     # ARM64 libraries
│       │   │   │   └── libworld_foundry.so
│       │   │   ├── armeabi-v7a/   # ARM32 libraries
│       │   │   ├── x86_64/        # x86_64 libraries
│       │   │   └── x86/           # x86 libraries
│       │   ├── res/               # Android resources
│       │   │   ├── drawable/      # Images and vector drawables
│       │   │   ├── values/        # Strings, colors, dimensions
│       │   │   ├── values-night/  # Dark theme resources
│       │   │   └── xml/           # XML configurations
│       │   └── assets/            # Raw assets
│       ├── test/                  # Unit tests
│       │   └── java/com/balexda/worldfoundry/
│       │       ├── ExampleUnitTest.kt
│       │       ├── ViewModelTest.kt
│       │       └── RepositoryTest.kt
│       └── androidTest/           # Instrumented tests
│           └── java/com/balexda/worldfoundry/
│               ├── ExampleInstrumentedTest.kt
│               └── UITest.kt
├── core/                          # Core module (if using multi-module)
│   ├── build.gradle.kts
│   └── src/main/java/com/balexda/worldfoundry/core/
├── build.gradle.kts               # Project-level Gradle build script
├── gradle.properties             # Gradle properties
├── settings.gradle.kts            # Gradle settings
├── local.properties              # Local SDK paths (not in version control)
└── gradle/                       # Gradle wrapper
    └── wrapper/
        ├── gradle-wrapper.jar
        └── gradle-wrapper.properties
```

### Key Configuration Files

#### build.gradle.kts (App Level)
```kotlin
plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("kotlin-kapt")
    id("dagger.hilt.android.plugin")
    id("kotlin-parcelize")
}

android {
    namespace = "com.balexda.worldfoundry"
    compileSdk = 34

    defaultConfig {
        applicationId = "com.balexda.worldfoundry"
        minSdk = 24
        targetSdk = 34
        versionCode = 1
        versionName = "1.0"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        vectorDrawables {
            useSupportLibrary = true
        }

        ndk {
            abiFilters += listOf("arm64-v8a", "armeabi-v7a", "x86_64", "x86")
        }
    }

    buildTypes {
        debug {
            isDebuggable = true
            applicationIdSuffix = ".debug"
            versionNameSuffix = "-debug"
            
            buildConfigField("boolean", "ENABLE_LOGGING", "true")
            buildConfigField("String", "API_BASE_URL", "\"https://debug-api.worldfoundry.com\"")
            
            ndk {
                debugSymbolLevel = "FULL"
            }
        }
        
        release {
            isMinifyEnabled = true
            isShrinkResources = true
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
            
            buildConfigField("boolean", "ENABLE_LOGGING", "false")
            buildConfigField("String", "API_BASE_URL", "\"https://api.worldfoundry.com\"")
            
            signingConfig = signingConfigs.getByName("debug") // Use proper signing for release
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
        freeCompilerArgs += listOf(
            "-opt-in=androidx.compose.material3.ExperimentalMaterial3Api",
            "-opt-in=androidx.compose.foundation.ExperimentalFoundationApi"
        )
    }

    buildFeatures {
        compose = true
        buildConfig = true
    }

    composeOptions {
        kotlinCompilerExtensionVersion = "1.5.8"
    }

    packaging {
        resources {
            excludes += "/META-INF/{AL2.0,LGPL2.1}"
        }
    }
}

dependencies {
    // Compose BOM
    implementation(platform("androidx.compose:compose-bom:2024.02.00"))
    
    // Core Android
    implementation("androidx.core:core-ktx:1.12.0")
    implementation("androidx.lifecycle:lifecycle-runtime-ktx:2.7.0")
    implementation("androidx.activity:activity-compose:1.8.2")
    
    // Compose
    implementation("androidx.compose.ui:ui")
    implementation("androidx.compose.ui:ui-graphics")
    implementation("androidx.compose.ui:ui-tooling-preview")
    implementation("androidx.compose.material3:material3")
    implementation("androidx.compose.material:material-icons-extended")
    
    // Navigation
    implementation("androidx.navigation:navigation-compose:2.7.6")
    
    // ViewModel
    implementation("androidx.lifecycle:lifecycle-viewmodel-compose:2.7.0")
    
    // Dependency Injection
    implementation("com.google.dagger:hilt-android:2.48.1")
    implementation("androidx.hilt:hilt-navigation-compose:1.1.0")
    kapt("com.google.dagger:hilt-compiler:2.48.1")
    
    // Coroutines
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-android:1.7.3")
    
    // JSON
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.6.2")
    
    // File handling
    implementation("androidx.documentfile:documentfile:1.0.1")
    
    // Image loading
    implementation("io.coil-kt:coil-compose:2.5.0")
    
    // Testing
    testImplementation("junit:junit:4.13.2")
    testImplementation("org.mockito:mockito-core:5.8.0")
    testImplementation("org.jetbrains.kotlinx:kotlinx-coroutines-test:1.7.3")
    
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.1")
    androidTestImplementation("androidx.compose.ui:ui-test-junit4")
    
    debugImplementation("androidx.compose.ui:ui-tooling")
    debugImplementation("androidx.compose.ui:ui-test-manifest")
}
```

#### AndroidManifest.xml
```xml
<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    xmlns:tools="http://schemas.android.com/tools">

    <!-- Permissions -->
    <uses-permission android:name="android.permission.INTERNET" />
    <uses-permission android:name="android.permission.READ_EXTERNAL_STORAGE" />
    <uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE" 
        android:maxSdkVersion="28" />
    <uses-permission android:name="android.permission.MANAGE_EXTERNAL_STORAGE"
        tools:ignore="ScopedStorage" />

    <!-- Hardware features -->
    <uses-feature
        android:name="android.hardware.touchscreen"
        android:required="false" />
    <uses-feature
        android:name="android.hardware.screen.landscape"
        android:required="false" />

    <application
        android:name=".WorldFoundryApplication"
        android:allowBackup="true"
        android:dataExtractionRules="@xml/data_extraction_rules"
        android:fullBackupContent="@xml/backup_rules"
        android:icon="@mipmap/ic_launcher"
        android:label="@string/app_name"
        android:roundIcon="@mipmap/ic_launcher_round"
        android:supportsRtl="true"
        android:theme="@style/Theme.WorldFoundry"
        android:hardwareAccelerated="true"
        tools:targetApi="31">
        
        <activity
            android:name=".MainActivity"
            android:exported="true"
            android:launchMode="singleTop"
            android:theme="@style/Theme.WorldFoundry"
            android:screenOrientation="unspecified"
            android:configChanges="orientation|screenSize|keyboardHidden">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
            
            <!-- File associations -->
            <intent-filter>
                <action android:name="android.intent.action.VIEW" />
                <category android:name="android.intent.category.DEFAULT" />
                <category android:name="android.intent.category.BROWSABLE" />
                <data android:scheme="file" />
                <data android:mimeType="application/json" />
                <data android:pathPattern=".*\\.wfmap" />
            </intent-filter>
        </activity>

        <!-- File provider for sharing files -->
        <provider
            android:name="androidx.core.content.FileProvider"
            android:authorities="${applicationId}.fileprovider"
            android:exported="false"
            android:grantUriPermissions="true">
            <meta-data
                android:name="android.support.FILE_PROVIDER_PATHS"
                android:resource="@xml/file_paths" />
        </provider>
    </application>
</manifest>
```

## Development Workflow

### 1. Daily Development Setup

```bash
# Open Android Studio with the project
# File → Open → Select platforms/android

# Or use command line development
# Terminal 1: Watch for Rust changes
cd core
cargo watch -x "ndk -t arm64-v8a -t armeabi-v7a -t x86_64 -t x86 -o ../platforms/android/app/src/main/jniLibs build --release"

# Terminal 2: Watch for Android changes
cd platforms/android
./gradlew build --continuous
```

### 2. Building the Application

#### Debug Build
```bash
# Build debug APK
./gradlew assembleDebug

# Install on connected device/emulator
./gradlew installDebug

# Build and install in one command
./gradlew installDebug

# Run specific variant
./gradlew installDebugArm64
```

#### Release Build
```bash
# Build release APK
./gradlew assembleRelease

# Build Android App Bundle (for Play Store)
./gradlew bundleRelease

# Sign and align APK
./gradlew assembleRelease
zipalign -v -p 4 app/build/outputs/apk/release/app-release-unsigned.apk app-release-aligned.apk
apksigner sign --ks my-release-key.keystore --out app-release-signed.apk app-release-aligned.apk
```

### 3. Debugging

#### Android Studio Debugging
1. Set breakpoints in Kotlin code
2. Click Debug button or press Shift+F9
3. Use Logcat for log output
4. Use Layout Inspector for UI debugging

#### Native Code Debugging
```bash
# Build with debug symbols
cd core
cargo ndk -t arm64-v8a -o ../platforms/android/app/src/main/jniLibs build

# Enable native debugging in Android Studio
# Run → Edit Configurations → Debugger → Debug type: Dual (Java + Native)
```

#### Logging and Diagnostics
```kotlin
// Add to WorldFoundryApplication.kt for comprehensive logging
class WorldFoundryApplication : Application() {
    override fun onCreate() {
        super.onCreate()
        
        if (BuildConfig.DEBUG) {
            setupDebugLogging()
            enableStrictMode()
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

### 4. Testing

#### Unit Tests
```bash
# Run all unit tests
./gradlew test

# Run tests for specific variant
./gradlew testDebugUnitTest

# Run tests with coverage
./gradlew testDebugUnitTestCoverage
```

#### Instrumented Tests
```bash
# Run all instrumented tests
./gradlew connectedAndroidTest

# Run specific test class
./gradlew connectedAndroidTest -Pandroid.testInstrumentationRunnerArguments.class=com.balexda.worldfoundry.ExampleInstrumentedTest
```

## Core Engine Integration

### 1. JNI Bridge Implementation

```kotlin
// NativeBridge.kt - JNI bridge for Rust core
class NativeBridge {
    companion object {
        init {
            System.loadLibrary("world_foundry")
        }
        
        // Native method declarations
        @JvmStatic
        external fun createEngine(): Long
        
        @JvmStatic
        external fun destroyEngine(handle: Long)
        
        @JvmStatic
        external fun importAzgaarMap(handle: Long, path: String): Int
        
        @JvmStatic
        external fun renderMap(handle: Long, mapHandle: Long, width: Int, height: Int): ByteArray?
        
        @JvmStatic
        external fun getLastError(handle: Long): String?
    }
}

// Data classes for native interop
data class NativeImageData(
    val data: ByteArray,
    val width: Int,
    val height: Int,
    val bytesPerPixel: Int
) {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false
        
        other as NativeImageData
        
        if (!data.contentEquals(other.data)) return false
        if (width != other.width) return false
        if (height != other.height) return false
        if (bytesPerPixel != other.bytesPerPixel) return false
        
        return true
    }
    
    override fun hashCode(): Int {
        var result = data.contentHashCode()
        result = 31 * result + width
        result = 31 * result + height
        result = 31 * result + bytesPerPixel
        return result
    }
}
```

### 2. High-Level Service Layer

```kotlin
// CoreEngineService.kt - High-level service for core engine
@Singleton
class CoreEngineService @Inject constructor() {
    private var engineHandle: Long = 0
    private val mutex = Mutex()
    
    suspend fun initialize(): Result<Unit> = withContext(Dispatchers.IO) {
        mutex.withLock {
            try {
                engineHandle = NativeBridge.createEngine()
                if (engineHandle == 0L) {
                    Result.failure(CoreEngineException("Failed to create core engine"))
                } else {
                    Result.success(Unit)
                }
            } catch (e: Exception) {
                Result.failure(CoreEngineException("Engine initialization failed", e))
            }
        }
    }
    
    suspend fun importAzgaarMap(filePath: String): Result<WorldModel> = withContext(Dispatchers.IO) {
        mutex.withLock {
            try {
                val result = NativeBridge.importAzgaarMap(engineHandle, filePath)
                if (result == 0) {
                    // Convert native data to Kotlin model
                    val world = convertNativeWorld(result)
                    Result.success(world)
                } else {
                    val error = NativeBridge.getLastError(engineHandle) ?: "Unknown error"
                    Result.failure(CoreEngineException("Import failed: $error"))
                }
            } catch (e: Exception) {
                Result.failure(CoreEngineException("Import operation failed", e))
            }
        }
    }
    
    suspend fun renderMap(world: WorldModel, width: Int, height: Int): Result<Bitmap> = withContext(Dispatchers.IO) {
        mutex.withLock {
            try {
                val nativeWorld = convertToNativeWorld(world)
                val imageData = NativeBridge.renderMap(engineHandle, nativeWorld, width, height)
                
                if (imageData != null) {
                    val bitmap = convertToBitmap(imageData, width, height)
                    Result.success(bitmap)
                } else {
                    val error = NativeBridge.getLastError(engineHandle) ?: "Render failed"
                    Result.failure(CoreEngineException("Render failed: $error"))
                }
            } catch (e: Exception) {
                Result.failure(CoreEngineException("Render operation failed", e))
            }
        }
    }
    
    private fun convertToBitmap(data: ByteArray, width: Int, height: Int): Bitmap {
        val bitmap = Bitmap.createBitmap(width, height, Bitmap.Config.ARGB_8888)
        val buffer = ByteBuffer.wrap(data)
        bitmap.copyPixelsFromBuffer(buffer)
        return bitmap
    }
    
    fun cleanup() {
        if (engineHandle != 0L) {
            NativeBridge.destroyEngine(engineHandle)
            engineHandle = 0
        }
    }
}

class CoreEngineException(message: String, cause: Throwable? = null) : Exception(message, cause)
```

## UI Development with Jetpack Compose

### 1. MVVM Pattern Implementation

```kotlin
// MapViewModel.kt - Main map view model
@HiltViewModel
class MapViewModel @Inject constructor(
    private val coreEngineService: CoreEngineService,
    private val fileService: FileService
) : ViewModel() {
    
    private val _uiState = MutableStateFlow(MapUiState())
    val uiState: StateFlow<MapUiState> = _uiState.asStateFlow()
    
    private val _events = Channel<MapEvent>()
    val events = _events.receiveAsFlow()
    
    init {
        viewModelScope.launch {
            coreEngineService.initialize()
        }
    }
    
    fun importMap() {
        viewModelScope.launch {
            _uiState.update { it.copy(isLoading = true, errorMessage = null) }
            
            try {
                val uri = fileService.pickFile(listOf("application/json", "application/octet-stream"))
                if (uri != null) {
                    val filePath = fileService.getPath(uri)
                    val result = coreEngineService.importAzgaarMap(filePath)
                    
                    result.fold(
                        onSuccess = { world ->
                            _uiState.update { 
                                it.copy(
                                    currentWorld = world,
                                    isLoading = false
                                )
                            }
                            updateMapImage()
                        },
                        onFailure = { error ->
                            _uiState.update { 
                                it.copy(
                                    isLoading = false,
                                    errorMessage = error.message
                                )
                            }
                        }
                    )
                }
            } catch (e: Exception) {
                _uiState.update { 
                    it.copy(
                        isLoading = false,
                        errorMessage = e.message
                    )
                }
            }
        }
    }
    
    fun exportMap() {
        viewModelScope.launch {
            val world = _uiState.value.currentWorld ?: return@launch
            
            try {
                val uri = fileService.saveFile("exported_map.png", "image/png")
                if (uri != null) {
                    // Export logic here
                    _events.send(MapEvent.ExportSuccess)
                }
            } catch (e: Exception) {
                _uiState.update { it.copy(errorMessage = e.message) }
            }
        }
    }
    
    private fun updateMapImage() {
        viewModelScope.launch {
            val world = _uiState.value.currentWorld ?: return@launch
            
            val result = coreEngineService.renderMap(world, 1024, 1024)
            result.fold(
                onSuccess = { bitmap ->
                    _uiState.update { it.copy(mapImage = bitmap) }
                },
                onFailure = { error ->
                    _uiState.update { it.copy(errorMessage = error.message) }
                }
            )
        }
    }
    
    fun clearError() {
        _uiState.update { it.copy(errorMessage = null) }
    }
    
    override fun onCleared() {
        super.onCleared()
        coreEngineService.cleanup()
    }
}

data class MapUiState(
    val currentWorld: WorldModel? = null,
    val mapImage: Bitmap? = null,
    val isLoading: Boolean = false,
    val errorMessage: String? = null,
    val zoomLevel: Float = 1f,
    val offsetX: Float = 0f,
    val offsetY: Float = 0f
)

sealed class MapEvent {
    object ExportSuccess : MapEvent()
    object ImportSuccess : MapEvent()
}
```

### 2. Compose UI Implementation

```kotlin
// MapScreen.kt - Main map display screen
@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun MapScreen(
    viewModel: MapViewModel = hiltViewModel(),
    onNavigateToSettings: () -> Unit
) {
    val uiState by viewModel.uiState.collectAsState()
    val context = LocalContext.current
    
    LaunchedEffect(Unit) {
        viewModel.events.collect { event ->
            when (event) {
                is MapEvent.ExportSuccess -> {
                    Toast.makeText(context, "Map exported successfully", Toast.LENGTH_SHORT).show()
                }
                is MapEvent.ImportSuccess -> {
                    Toast.makeText(context, "Map imported successfully", Toast.LENGTH_SHORT).show()
                }
            }
        }
    }
    
    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("World Map") },
                actions = {
                    IconButton(onClick = { viewModel.importMap() }) {
                        Icon(Icons.Default.FileOpen, contentDescription = "Import Map")
                    }
                    IconButton(
                        onClick = { viewModel.exportMap() },
                        enabled = uiState.currentWorld != null
                    ) {
                        Icon(Icons.Default.FileDownload, contentDescription = "Export Map")
                    }
                    IconButton(onClick = onNavigateToSettings) {
                        Icon(Icons.Default.Settings, contentDescription = "Settings")
                    }
                }
            )
        }
    ) { paddingValues ->
        Box(
            modifier = Modifier
                .fillMaxSize()
                .padding(paddingValues)
        ) {
            // Map Display
            MapDisplay(
                bitmap = uiState.mapImage,
                modifier = Modifier.fillMaxSize()
            )
            
            // Loading Overlay
            if (uiState.isLoading) {
                Box(
                    modifier = Modifier
                        .fillMaxSize()
                        .background(Color.Black.copy(alpha = 0.5f)),
                    contentAlignment = Alignment.Center
                ) {
                    Card(
                        modifier = Modifier.padding(32.dp)
                    ) {
                        Column(
                            modifier = Modifier.padding(24.dp),
                            horizontalAlignment = Alignment.CenterHorizontally
                        ) {
                            CircularProgressIndicator()
                            Spacer(modifier = Modifier.height(16.dp))
                            Text("Loading map...")
                        }
                    }
                }
            }
            
            // Error Snackbar
            uiState.errorMessage?.let { error ->
                LaunchedEffect(error) {
                    // Show snackbar or error dialog
                }
            }
        }
    }
}

@Composable
fun MapDisplay(
    bitmap: Bitmap?,
    modifier: Modifier = Modifier
) {
    var scale by remember { mutableFloatStateOf(1f) }
    var offset by remember { mutableStateOf(Offset.Zero) }
    
    Box(
        modifier = modifier
            .pointerInput(Unit) {
                detectTransformGestures { _, pan, zoom, _ ->
                    scale = (scale * zoom).coerceIn(0.5f, 3f)
                    offset += pan
                }
            },
        contentAlignment = Alignment.Center
    ) {
        if (bitmap != null) {
            Image(
                bitmap = bitmap.asImageBitmap(),
                contentDescription = "World Map",
                modifier = Modifier
                    .graphicsLayer(
                        scaleX = scale,
                        scaleY = scale,
                        translationX = offset.x,
                        translationY = offset.y
                    )
            )
        } else {
            Card(
                modifier = Modifier
                    .size(300.dp, 200.dp),
                colors = CardDefaults.cardColors(
                    containerColor = MaterialTheme.colorScheme.surfaceVariant
                )
            ) {
                Box(
                    modifier = Modifier.fillMaxSize(),
                    contentAlignment = Alignment.Center
                ) {
                    Text(
                        text = "No map loaded",
                        style = MaterialTheme.typography.bodyLarge,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                }
            }
        }
    }
}
```

## Performance Optimization

### 1. Memory Management

```kotlin
// Efficient bitmap handling for large maps
class BitmapCache {
    private val cache = LruCache<String, Bitmap>(
        (Runtime.getRuntime().maxMemory() / 8).toInt()
    )
    
    fun get(key: String): Bitmap? = cache.get(key)
    
    fun put(key: String, bitmap: Bitmap) {
        cache.put(key, bitmap)
    }
    
    fun clear() {
        cache.evictAll()
    }
}

// Memory-efficient image loading
@Composable
fun rememberBitmapFromBytes(
    data: ByteArray?,
    width: Int,
    height: Int
): Bitmap? {
    return remember(data, width, height) {
        data?.let {
            BitmapFactory.decodeByteArray(it, 0, it.size)?.let { bitmap ->
                if (bitmap.width != width || bitmap.height != height) {
                    Bitmap.createScaledBitmap(bitmap, width, height, true).also {
                        if (it != bitmap) bitmap.recycle()
                    }
                } else {
                    bitmap
                }
            }
        }
    }
}
```

### 2. Vulkan/OpenGL Acceleration

```kotlin
// GPU-accelerated rendering using Vulkan API
class VulkanMapRenderer {
    private external fun initVulkan(): Long
    private external fun destroyVulkan(handle: Long)
    private external fun renderMapVulkan(handle: Long, mapData: ByteArray, width: Int, height: Int): ByteArray?
    
    companion object {
        init {
            System.loadLibrary("vulkan_renderer")
        }
    }
    
    private var vulkanHandle: Long = 0
    
    fun initialize(): Boolean {
        vulkanHandle = initVulkan()
        return vulkanHandle != 0L
    }
    
    fun renderMap(mapData: ByteArray, width: Int, height: Int): Bitmap? {
        val result = renderMapVulkan(vulkanHandle, mapData, width, height)
        return result?.let { data ->
            val bitmap = Bitmap.createBitmap(width, height, Bitmap.Config.ARGB_8888)
            val buffer = ByteBuffer.wrap(data)
            bitmap.copyPixelsFromBuffer(buffer)
            bitmap
        }
    }
    
    fun cleanup() {
        if (vulkanHandle != 0L) {
            destroyVulkan(vulkanHandle)
            vulkanHandle = 0
        }
    }
}
```

## Deployment and Distribution

### 1. Google Play Store Preparation

```bash
# Generate signed APK
./gradlew assembleRelease

# Generate Android App Bundle (recommended for Play Store)
./gradlew bundleRelease

# Upload to Play Console
# Use Android Studio: Build → Generate Signed Bundle/APK
# Or use Play Console upload interface
```

### 2. Alternative Distribution

```bash
# Firebase App Distribution
./gradlew assembleDebug
firebase appdistribution:distribute \
    app/build/outputs/apk/debug/app-debug.apk \
    --app 1:123456789:android:abcdef \
    --groups "testers" \
    --release-notes "Debug build with latest features"

# Direct APK distribution
./gradlew assembleRelease
# Distribute APK file directly
```

## Troubleshooting

### Common Issues and Solutions

#### 1. Native Library Loading Issues
```
Error: java.lang.UnsatisfiedLinkError: No implementation found for long createEngine()
```

**Solution:**
```bash
# Ensure libraries are built for correct architectures
cd core
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 -t x86 \
    -o ../platforms/android/app/src/main/jniLibs \
    build --release

# Verify library architecture
file platforms/android/app/src/main/jniLibs/arm64-v8a/libworld_foundry.so
```

#### 2. Gradle Build Issues
```
Error: Could not resolve dependencies
```

**Solution:**
```bash
# Clean and rebuild
./gradlew clean
./gradlew build

# Update Gradle wrapper
./gradlew wrapper --gradle-version 8.2

# Sync project in Android Studio
# File → Sync Project with Gradle Files
```

#### 3. Memory Issues
```
Issue: OutOfMemoryError during map rendering
```

**Solution:**
```kotlin
// Add to AndroidManifest.xml
<application
    android:largeHeap="true"
    android:hardwareAccelerated="true">

// Implement memory management
class MemoryManager {
    fun handleLowMemory() {
        // Clear caches
        BitmapCache.getInstance().clear()
        
        // Force garbage collection
        System.gc()
        
        // Release non-essential resources
        releaseNonEssentialResources()
    }
}
```

#### 4. Performance Issues
```
Issue: Slow UI rendering or ANRs
```

**Solution:**
```kotlin
// Use background threads for heavy operations
viewModelScope.launch(Dispatchers.IO) {
    // Heavy computation
    val result = heavyComputation()
    
    // Update UI on main thread
    withContext(Dispatchers.Main) {
        updateUI(result)
    }
}

// Enable hardware acceleration
// Add to AndroidManifest.xml
<activity android:hardwareAccelerated="true">
```

## Next Steps

After completing the Android development setup:

1. **Complete the Setup**: Follow the [main development setup guide](DEVELOPMENT_SETUP.md)
2. **Learn iOS Development**: [iOS Development Guide](IOS_DEVELOPMENT.md)
3. **Deployment**: [Android Deployment Guide](../deployment/ANDROID_DEPLOYMENT.md)
4. **Debugging**: [Android Debugging Guide](../debugging/ANDROID_DEBUGGING.md)

## Additional Resources

- [Android Developer Documentation](https://developer.android.com/)
- [Jetpack Compose Documentation](https://developer.android.com/jetpack/compose)
- [Android NDK Documentation](https://developer.android.com/ndk)
- [Kotlin Documentation](https://kotlinlang.org/docs/)
- [Material Design 3](https://m3.material.io/)

---

**Last Updated**: March 2024  
**Android SDK Version**: API 34 (Android 14)  
**Kotlin Version**: 1.9.22  
**Compose Version**: 2024.02.00