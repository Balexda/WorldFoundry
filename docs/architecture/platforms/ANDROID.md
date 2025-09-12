# Android Platform Architecture

## Overview

The Android implementation of World Foundry leverages Jetpack Compose and modern Android development practices to provide a native Android experience with Material Design 3, adaptive layouts, and deep Android ecosystem integration.

## Architecture Stack

```
┌─────────────────────────────────────────────────────────────┐
│                 Jetpack Compose UI                          │
├─────────────────────────────────────────────────────────────┤
│                    Android Framework                        │
├─────────────────────────────────────────────────────────────┤
│              Platform Abstraction Layer                    │
├─────────────────────────────────────────────────────────────┤
│                 World Foundry Core                         │
│                   (Rust Engine)                            │
└─────────────────────────────────────────────────────────────┘
```

## Technology Stack

### UI Framework: Jetpack Compose
- **Declarative UI** - Modern reactive UI toolkit
- **Material Design 3** - Latest Material Design components
- **Adaptive Layouts** - Phone, tablet, and foldable support
- **Accessibility** - TalkBack and accessibility service integration

### Architecture: Android Architecture Components
- **ViewModel** - UI-related data management
- **LiveData/StateFlow** - Reactive data observation
- **Room Database** - Local data persistence
- **WorkManager** - Background task management

### Core Integration: JNI (Java Native Interface)
- **Rust Library** - Native library integration via JNI
- **Memory Management** - Safe memory handling between JVM and native code
- **Threading** - Coroutines integration with Rust async runtime
- **Error Handling** - Exception translation between Kotlin and Rust

## Project Structure

```
platforms/android/
├── app/
│   ├── build.gradle.kts              # App module build configuration
│   ├── src/
│   │   ├── main/
│   │   │   ├── AndroidManifest.xml   # App manifest
│   │   │   ├── java/com/balexda/worldfoundry/
│   │   │   │   ├── MainActivity.kt   # Main activity
│   │   │   │   ├── WorldFoundryApplication.kt # Application class
│   │   │   │   ├── ui/               # Compose UI components
│   │   │   │   │   ├── theme/        # Material Design theme
│   │   │   │   │   │   ├── Color.kt  # Theme colors
│   │   │   │   │   │   ├── Theme.kt  # App theme
│   │   │   │   │   │   └── Type.kt   # Typography
│   │   │   │   │   ├── screens/      # Screen composables
│   │   │   │   │   │   ├── MapScreen.kt      # Map display screen
│   │   │   │   │   │   ├── ImportScreen.kt   # Import workflow
│   │   │   │   │   │   ├── ExportScreen.kt   # Export options
│   │   │   │   │   │   └── SettingsScreen.kt # App settings
│   │   │   │   │   ├── components/   # Reusable UI components
│   │   │   │   │   │   ├── MapCanvas.kt      # Custom map view
│   │   │   │   │   │   ├── ToolPalette.kt    # Drawing tools
│   │   │   │   │   │   ├── LayerPanel.kt     # Map layers
│   │   │   │   │   │   └── LoadingOverlay.kt # Loading states
│   │   │   │   │   └── navigation/   # Navigation setup
│   │   │   │   │       └── WorldFoundryNavigation.kt
│   │   │   │   ├── viewmodel/        # ViewModels
│   │   │   │   │   ├── MapViewModel.kt       # Map state management
│   │   │   │   │   ├── ImportViewModel.kt    # Import workflow
│   │   │   │   │   └── SettingsViewModel.kt  # App settings
│   │   │   │   ├── data/             # Data layer
│   │   │   │   │   ├── repository/   # Repository pattern
│   │   │   │   │   │   ├── MapRepository.kt  # Map data operations
│   │   │   │   │   │   └── SettingsRepository.kt # Settings persistence
│   │   │   │   │   ├── database/     # Room database
│   │   │   │   │   │   ├── WorldFoundryDatabase.kt # Database setup
│   │   │   │   │   │   ├── entities/ # Database entities
│   │   │   │   │   │   └── dao/      # Data access objects
│   │   │   │   │   └── model/        # Data models
│   │   │   │   │       ├── WorldMap.kt       # Map data structures
│   │   │   │   │       ├── MapCell.kt        # Cell data
│   │   │   │   │       └── AppSettings.kt    # Settings model
│   │   │   │   ├── service/          # Platform services
│   │   │   │   │   ├── FileService.kt        # File operations
│   │   │   │   │   ├── StorageService.kt     # Data persistence
│   │   │   │   │   ├── RenderingService.kt   # Graphics rendering
│   │   │   │   │   └── ShareService.kt       # System sharing
│   │   │   │   ├── core/             # Rust FFI layer
│   │   │   │   │   ├── CoreEngine.kt         # High-level wrapper
│   │   │   │   │   ├── NativeLib.kt          # JNI bindings
│   │   │   │   │   └── DataConverter.kt      # Data conversion
│   │   │   │   └── util/             # Utility classes
│   │   │   │       ├── Extensions.kt         # Kotlin extensions
│   │   │   │       ├── Constants.kt          # App constants
│   │   │   │       └── Logger.kt             # Logging utilities
│   │   │   ├── cpp/                  # Native C++ bridge (optional)
│   │   │   │   ├── native-lib.cpp    # JNI implementation
│   │   │   │   └── CMakeLists.txt    # CMake configuration
│   │   │   ├── jniLibs/              # Native libraries
│   │   │   │   ├── arm64-v8a/        # ARM64 libraries
│   │   │   │   │   └── libworld_foundry.so
│   │   │   │   ├── armeabi-v7a/      # ARM32 libraries
│   │   │   │   │   └── libworld_foundry.so
│   │   │   │   ├── x86_64/           # x86_64 libraries (emulator)
│   │   │   │   │   └── libworld_foundry.so
│   │   │   │   └── x86/              # x86 libraries (emulator)
│   │   │   │       └── libworld_foundry.so
│   │   │   └── res/                  # Android resources
│   │   │       ├── drawable/         # Vector drawables
│   │   │       ├── mipmap/           # App icons
│   │   │       ├── values/           # Strings, colors, dimensions
│   │   │       └── xml/              # XML configurations
│   │   ├── androidTest/              # Instrumented tests
│   │   │   └── java/com/balexda/worldfoundry/
│   │   │       ├── ExampleInstrumentedTest.kt
│   │   │       ├── ui/               # UI tests
│   │   │       └── database/         # Database tests
│   │   └── test/                     # Unit tests
│   │       └── java/com/balexda/worldfoundry/
│   │           ├── ExampleUnitTest.kt
│   │           ├── viewmodel/        # ViewModel tests
│   │           └── repository/       # Repository tests
├── build.gradle.kts                  # Project build configuration
├── gradle.properties                 # Gradle properties
├── settings.gradle.kts               # Project settings
└── gradle/                           # Gradle wrapper
    └── wrapper/
        ├── gradle-wrapper.jar
        └── gradle-wrapper.properties
```

## Core Integration

### JNI Bridge Layer

```kotlin
// CoreEngine.kt - High-level Kotlin wrapper
class CoreEngine {
    companion object {
        init {
            System.loadLibrary("world_foundry")
        }
    }
    
    private var nativeHandle: Long = 0
    
    init {
        nativeHandle = nativeCreate()
    }
    
    fun close() {
        if (nativeHandle != 0L) {
            nativeDestroy(nativeHandle)
            nativeHandle = 0
        }
    }
    
    suspend fun importAzgaarMap(filePath: String): WorldMap = withContext(Dispatchers.IO) {
        val result = nativeImportAzgaar(nativeHandle, filePath)
        DataConverter.convertWorldMap(result)
    }
    
    suspend fun renderMap(map: WorldMap, width: Int, height: Int): Bitmap? = 
        withContext(Dispatchers.Default) {
            val imageData = nativeRenderMap(nativeHandle, map.toNative(), width, height)
            imageData?.let { DataConverter.convertToBitmap(it) }
        }
    
    // Native method declarations
    private external fun nativeCreate(): Long
    private external fun nativeDestroy(handle: Long)
    private external fun nativeImportAzgaar(handle: Long, path: String): NativeWorldMap
    private external fun nativeRenderMap(handle: Long, map: NativeWorldMap, width: Int, height: Int): NativeImageData?
}
```

### Data Conversion Layer

```kotlin
// DataConverter.kt - Kotlin/Rust data conversion
object DataConverter {
    fun convertWorldMap(native: NativeWorldMap): WorldMap {
        return WorldMap(
            id = UUID.randomUUID(),
            name = native.name,
            width = native.width,
            height = native.height,
            cells = convertCells(native.cells)
        )
    }
    
    private fun convertCells(nativeCells: Array<NativeCell>): List<MapCell> {
        return nativeCells.map { nativeCell ->
            MapCell(
                x = nativeCell.x,
                y = nativeCell.y,
                elevation = nativeCell.elevation,
                biome = BiomeType.fromValue(nativeCell.biome),
                culture = nativeCell.culture,
                religion = nativeCell.religion
            )
        }
    }
    
    fun convertToBitmap(imageData: NativeImageData): Bitmap {
        return Bitmap.createBitmap(
            imageData.pixels,
            imageData.width,
            imageData.height,
            Bitmap.Config.ARGB_8888
        )
    }
}

// Native data structures
data class NativeWorldMap(
    val name: String,
    val width: Int,
    val height: Int,
    val cells: Array<NativeCell>
)

data class NativeCell(
    val x: Int,
    val y: Int,
    val elevation: Float,
    val biome: Int,
    val culture: Int,
    val religion: Int
)

data class NativeImageData(
    val pixels: IntArray,
    val width: Int,
    val height: Int
)
```

### JNI Implementation

```cpp
// native-lib.cpp - JNI bridge implementation
#include <jni.h>
#include <string>
#include <memory>
#include "world_foundry.h" // Rust FFI headers

extern "C" {

JNIEXPORT jlong JNICALL
Java_com_balexda_worldfoundry_core_CoreEngine_nativeCreate(JNIEnv *env, jobject thiz) {
    auto* engine = world_foundry_create();
    return reinterpret_cast<jlong>(engine);
}

JNIEXPORT void JNICALL
Java_com_balexda_worldfoundry_core_CoreEngine_nativeDestroy(JNIEnv *env, jobject thiz, jlong handle) {
    auto* engine = reinterpret_cast<void*>(handle);
    world_foundry_destroy(engine);
}

JNIEXPORT jobject JNICALL
Java_com_balexda_worldfoundry_core_CoreEngine_nativeImportAzgaar(JNIEnv *env, jobject thiz, 
                                                                jlong handle, jstring path) {
    auto* engine = reinterpret_cast<void*>(handle);
    const char* pathStr = env->GetStringUTFChars(path, nullptr);
    
    // Call Rust function
    NativeWorldMap result = world_foundry_import_azgaar(engine, pathStr);
    
    env->ReleaseStringUTFChars(path, pathStr);
    
    // Convert to Java object
    jclass mapClass = env->FindClass("com/balexda/worldfoundry/core/NativeWorldMap");
    jmethodID constructor = env->GetMethodID(mapClass, "<init>", "(Ljava/lang/String;II[Lcom/balexda/worldfoundry/core/NativeCell;)V");
    
    // Create Java string for name
    jstring jname = env->NewStringUTF(result.name);
    
    // Convert cells array
    jclass cellClass = env->FindClass("com/balexda/worldfoundry/core/NativeCell");
    jobjectArray cellArray = env->NewObjectArray(result.cell_count, cellClass, nullptr);
    
    for (uint32_t i = 0; i < result.cell_count; i++) {
        jmethodID cellConstructor = env->GetMethodID(cellClass, "<init>", "(IIFIII)V");
        jobject cell = env->NewObject(cellClass, cellConstructor,
            result.cells[i].x,
            result.cells[i].y,
            result.cells[i].elevation,
            result.cells[i].biome,
            result.cells[i].culture,
            result.cells[i].religion
        );
        env->SetObjectArrayElement(cellArray, i, cell);
    }
    
    return env->NewObject(mapClass, constructor, jname, result.width, result.height, cellArray);
}

} // extern "C"
```

## Jetpack Compose UI Architecture

### MVVM with Compose

```kotlin
// MapViewModel.kt - Map state management
class MapViewModel(
    private val coreEngine: CoreEngine,
    private val mapRepository: MapRepository
) : ViewModel() {
    
    private val _uiState = MutableStateFlow(MapUiState())
    val uiState: StateFlow<MapUiState> = _uiState.asStateFlow()
    
    private val _mapImage = MutableStateFlow<Bitmap?>(null)
    val mapImage: StateFlow<Bitmap?> = _mapImage.asStateFlow()
    
    fun importMap(uri: Uri) {
        viewModelScope.launch {
            _uiState.update { it.copy(isLoading = true, error = null) }
            
            try {
                val filePath = getFilePathFromUri(uri)
                val map = coreEngine.importAzgaarMap(filePath)
                
                _uiState.update { 
                    it.copy(
                        currentMap = map,
                        isLoading = false
                    )
                }
                
                // Save to database
                mapRepository.saveMap(map)
                
                // Render initial image
                updateMapImage()
                
            } catch (e: Exception) {
                _uiState.update { 
                    it.copy(
                        isLoading = false,
                        error = e.message
                    )
                }
            }
        }
    }
    
    fun updateZoom(zoomLevel: Float) {
        _uiState.update { it.copy(zoomLevel = zoomLevel) }
        updateMapImage()
    }
    
    private fun updateMapImage() {
        val currentMap = _uiState.value.currentMap ?: return
        val zoomLevel = _uiState.value.zoomLevel
        
        viewModelScope.launch {
            val width = (currentMap.width * zoomLevel).toInt()
            val height = (currentMap.height * zoomLevel).toInt()
            
            val bitmap = coreEngine.renderMap(currentMap, width, height)
            _mapImage.value = bitmap
        }
    }
}

data class MapUiState(
    val currentMap: WorldMap? = null,
    val isLoading: Boolean = false,
    val error: String? = null,
    val zoomLevel: Float = 1.0f,
    val panOffset: Offset = Offset.Zero
)
```

### Compose UI Components

```kotlin
// MapScreen.kt - Main map display screen
@Composable
fun MapScreen(
    viewModel: MapViewModel = hiltViewModel(),
    onNavigateToImport: () -> Unit
) {
    val uiState by viewModel.uiState.collectAsState()
    val mapImage by viewModel.mapImage.collectAsState()
    
    Column(
        modifier = Modifier.fillMaxSize()
    ) {
        // Top app bar
        TopAppBar(
            title = { Text("World Foundry") },
            actions = {
                IconButton(onClick = onNavigateToImport) {
                    Icon(Icons.Default.Add, contentDescription = "Import Map")
                }
                
                IconButton(onClick = { /* Export menu */ }) {
                    Icon(Icons.Default.Share, contentDescription = "Export Map")
                }
            }
        )
        
        // Map content
        Box(
            modifier = Modifier
                .fillMaxSize()
                .weight(1f)
        ) {
            when {
                uiState.isLoading -> {
                    LoadingOverlay(
                        message = "Loading map...",
                        modifier = Modifier.fillMaxSize()
                    )
                }
                
                uiState.error != null -> {
                    ErrorMessage(
                        error = uiState.error,
                        onRetry = { /* Retry logic */ },
                        modifier = Modifier.align(Alignment.Center)
                    )
                }
                
                mapImage != null -> {
                    MapCanvas(
                        bitmap = mapImage,
                        zoomLevel = uiState.zoomLevel,
                        panOffset = uiState.panOffset,
                        onZoomChange = viewModel::updateZoom,
                        onPanChange = { /* Update pan */ },
                        modifier = Modifier.fillMaxSize()
                    )
                }
                
                else -> {
                    EmptyMapState(
                        onImportClick = onNavigateToImport,
                        modifier = Modifier.align(Alignment.Center)
                    )
                }
            }
        }
        
        // Bottom tool bar
        BottomAppBar {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceEvenly
            ) {
                IconButton(onClick = { /* Pan tool */ }) {
                    Icon(Icons.Default.PanTool, contentDescription = "Pan")
                }
                
                IconButton(onClick = { /* Zoom in */ }) {
                    Icon(Icons.Default.ZoomIn, contentDescription = "Zoom In")
                }
                
                IconButton(onClick = { /* Zoom out */ }) {
                    Icon(Icons.Default.ZoomOut, contentDescription = "Zoom Out")
                }
                
                IconButton(onClick = { /* Layers */ }) {
                    Icon(Icons.Default.Layers, contentDescription = "Layers")
                }
            }
        }
    }
}
```

### Custom Map Canvas

```kotlin
// MapCanvas.kt - Interactive map display
@Composable
fun MapCanvas(
    bitmap: Bitmap?,
    zoomLevel: Float,
    panOffset: Offset,
    onZoomChange: (Float) -> Unit,
    onPanChange: (Offset) -> Unit,
    modifier: Modifier = Modifier
) {
    var scale by remember { mutableStateOf(1f) }
    var offset by remember { mutableStateOf(Offset.Zero) }
    
    val transformableState = rememberTransformableState { zoomChange, panChange, _ ->
        scale = (scale * zoomChange).coerceIn(0.1f, 5f)
        offset += panChange
        
        onZoomChange(scale)
        onPanChange(offset)
    }
    
    Canvas(
        modifier = modifier
            .fillMaxSize()
            .transformable(state = transformableState)
    ) {
        bitmap?.let { bmp ->
            val imageBitmap = bmp.asImageBitmap()
            
            drawImage(
                image = imageBitmap,
                topLeft = offset,
                colorFilter = null,
                style = Fill
            )
        }
    }
}
```

## Platform Services

### File System Integration

```kotlin
// FileService.kt - Android file operations
class FileService @Inject constructor(
    private val context: Context
) {
    suspend fun pickFile(mimeTypes: Array<String>): Uri? = suspendCoroutine { continuation ->
        val intent = Intent(Intent.ACTION_GET_CONTENT).apply {
            type = "*/*"
            putExtra(Intent.EXTRA_MIME_TYPES, mimeTypes)
            addCategory(Intent.CATEGORY_OPENABLE)
        }
        
        // This would typically be handled by an Activity result launcher
        // Implementation depends on the calling context
        continuation.resume(null) // Placeholder
    }
    
    suspend fun saveFile(data: ByteArray, fileName: String, mimeType: String): Uri? {
        return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
            saveFileModern(data, fileName, mimeType)
        } else {
            saveFileLegacy(data, fileName)
        }
    }
    
    @RequiresApi(Build.VERSION_CODES.Q)
    private suspend fun saveFileModern(data: ByteArray, fileName: String, mimeType: String): Uri? {
        val resolver = context.contentResolver
        val contentValues = ContentValues().apply {
            put(MediaStore.MediaColumns.DISPLAY_NAME, fileName)
            put(MediaStore.MediaColumns.MIME_TYPE, mimeType)
            put(MediaStore.MediaColumns.RELATIVE_PATH, Environment.DIRECTORY_DOWNLOADS)
        }
        
        val uri = resolver.insert(MediaStore.Downloads.EXTERNAL_CONTENT_URI, contentValues)
        
        return uri?.let { targetUri ->
            try {
                resolver.openOutputStream(targetUri)?.use { outputStream ->
                    outputStream.write(data)
                }
                targetUri
            } catch (e: Exception) {
                resolver.delete(targetUri, null, null)
                null
            }
        }
    }
    
    private suspend fun saveFileLegacy(data: ByteArray, fileName: String): Uri? {
        val downloadsDir = Environment.getExternalStoragePublicDirectory(Environment.DIRECTORY_DOWNLOADS)
        val file = File(downloadsDir, fileName)
        
        return try {
            file.writeBytes(data)
            Uri.fromFile(file)
        } catch (e: Exception) {
            null
        }
    }
}
```

### Room Database Integration

```kotlin
// WorldFoundryDatabase.kt - Room database setup
@Database(
    entities = [MapEntity::class, SettingsEntity::class],
    version = 1,
    exportSchema = false
)
@TypeConverters(Converters::class)
abstract class WorldFoundryDatabase : RoomDatabase() {
    abstract fun mapDao(): MapDao
    abstract fun settingsDao(): SettingsDao
    
    companion object {
        @Volatile
        private var INSTANCE: WorldFoundryDatabase? = null
        
        fun getDatabase(context: Context): WorldFoundryDatabase {
            return INSTANCE ?: synchronized(this) {
                val instance = Room.databaseBuilder(
                    context.applicationContext,
                    WorldFoundryDatabase::class.java,
                    "world_foundry_database"
                ).build()
                INSTANCE = instance
                instance
            }
        }
    }
}

// MapEntity.kt - Database entity
@Entity(tableName = "maps")
data class MapEntity(
    @PrimaryKey val id: String,
    val name: String,
    val width: Int,
    val height: Int,
    val mapData: String, // JSON serialized map data
    val thumbnailPath: String?,
    val dateCreated: Long,
    val dateModified: Long
)

// MapDao.kt - Data access object
@Dao
interface MapDao {
    @Query("SELECT * FROM maps ORDER BY dateModified DESC")
    fun getAllMaps(): Flow<List<MapEntity>>
    
    @Query("SELECT * FROM maps WHERE id = :id")
    suspend fun getMapById(id: String): MapEntity?
    
    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insertMap(map: MapEntity)
    
    @Delete
    suspend fun deleteMap(map: MapEntity)
    
    @Query("DELETE FROM maps WHERE id = :id")
    suspend fun deleteMapById(id: String)
}
```

## Graphics and Rendering

### Canvas-based Rendering

```kotlin
// RenderingService.kt - High-performance map rendering
class RenderingService @Inject constructor() {
    
    suspend fun renderMap(map: WorldMap, width: Int, height: Int): Bitmap = withContext(Dispatchers.Default) {
        val bitmap = Bitmap.createBitmap(width, height, Bitmap.Config.ARGB_8888)
        val canvas = Canvas(bitmap)
        
        // Clear with ocean color
        canvas.drawColor(Color.BLUE)
        
        val paint = Paint().apply {
            isAntiAlias = true
            style = Paint.Style.FILL
        }
        
        val scaleX = width.toFloat() / map.width
        val scaleY = height.toFloat() / map.height
        
        // Render cells
        for (cell in map.cells) {
            paint.color = getBiomeColor(cell.biome)
            
            val left = cell.x * scaleX
            val top = cell.y * scaleY
            val right = left + scaleX
            val bottom = top + scaleY
            
            canvas.drawRect(left, top, right, bottom, paint)
        }
        
        bitmap
    }
    
    private fun getBiomeColor(biome: BiomeType): Int {
        return when (biome) {
            BiomeType.OCEAN -> Color.BLUE
            BiomeType.FOREST -> Color.GREEN
            BiomeType.DESERT -> Color.YELLOW
            BiomeType.MOUNTAIN -> Color.GRAY
            BiomeType.GRASSLAND -> Color.rgb(144, 238, 144)
            BiomeType.TUNDRA -> Color.rgb(220, 220, 220)
            else -> Color.WHITE
        }
    }
    
    suspend fun generateThumbnail(map: WorldMap, size: Int = 256): Bitmap {
        return renderMap(map, size, size)
    }
}
```

## Material Design 3 Integration

### Theme Configuration

```kotlin
// Theme.kt - Material Design 3 theme
@Composable
fun WorldFoundryTheme(
    darkTheme: Boolean = isSystemInDarkTheme(),
    dynamicColor: Boolean = true,
    content: @Composable () -> Unit
) {
    val colorScheme = when {
        dynamicColor && Build.VERSION.SDK_INT >= Build.VERSION_CODES.S -> {
            val context = LocalContext.current
            if (darkTheme) dynamicDarkColorScheme(context) else dynamicLightColorScheme(context)
        }
        darkTheme -> DarkColorScheme
        else -> LightColorScheme
    }
    
    MaterialTheme(
        colorScheme = colorScheme,
        typography = Typography,
        content = content
    )
}

// Color.kt - Theme colors
val LightColorScheme = lightColorScheme(
    primary = Color(0xFF1976D2),
    onPrimary = Color.White,
    primaryContainer = Color(0xFFBBDEFB),
    onPrimaryContainer = Color(0xFF0D47A1),
    secondary = Color(0xFF388E3C),
    onSecondary = Color.White,
    surface = Color(0xFFFFFBFE),
    onSurface = Color(0xFF1C1B1F)
)

val DarkColorScheme = darkColorScheme(
    primary = Color(0xFF90CAF9),
    onPrimary = Color(0xFF0D47A1),
    primaryContainer = Color(0xFF1565C0),
    onPrimaryContainer = Color(0xFFBBDEFB),
    secondary = Color(0xFF81C784),
    onSecondary = Color(0xFF1B5E20),
    surface = Color(0xFF1C1B1F),
    onSurface = Color(0xFFE6E1E5)
)
```

## Deployment

### Build Configuration

```kotlin
// build.gradle.kts (app module)
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
        versionName = "1.0.0"
        
        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        
        ndk {
            abiFilters += listOf("arm64-v8a", "armeabi-v7a", "x86_64", "x86")
        }
    }
    
    buildTypes {
        release {
            isMinifyEnabled = true
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
        }
    }
    
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }
    
    kotlinOptions {
        jvmTarget = "1.8"
    }
    
    buildFeatures {
        compose = true
    }
    
    composeOptions {
        kotlinCompilerExtensionVersion = "1.5.4"
    }
    
    packaging {
        resources {
            excludes += "/META-INF/{AL2.0,LGPL2.1}"
        }
    }
}

dependencies {
    // Compose BOM
    implementation(platform("androidx.compose:compose-bom:2023.10.01"))
    implementation("androidx.compose.ui:ui")
    implementation("androidx.compose.ui:ui-graphics")
    implementation("androidx.compose.ui:ui-tooling-preview")
    implementation("androidx.compose.material3:material3")
    
    // Activity Compose
    implementation("androidx.activity:activity-compose:1.8.0")
    
    // ViewModel
    implementation("androidx.lifecycle:lifecycle-viewmodel-compose:2.6.2")
    
    // Navigation
    implementation("androidx.navigation:navigation-compose:2.7.4")
    
    // Hilt
    implementation("com.google.dagger:hilt-android:2.48")
    implementation("androidx.hilt:hilt-navigation-compose:1.1.0")
    kapt("com.google.dagger:hilt-compiler:2.48")
    
    // Room
    implementation("androidx.room:room-runtime:2.6.0")
    implementation("androidx.room:room-ktx:2.6.0")
    kapt("androidx.room:room-compiler:2.6.0")
    
    // Coroutines
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-android:1.7.3")
    
    // Testing
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.1")
    androidTestImplementation(platform("androidx.compose:compose-bom:2023.10.01"))
    androidTestImplementation("androidx.compose.ui:ui-test-junit4")
    debugImplementation("androidx.compose.ui:ui-tooling")
    debugImplementation("androidx.compose.ui:ui-test-manifest")
}
```

### Android Manifest

```xml
<!-- AndroidManifest.xml -->
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    xmlns:tools="http://schemas.android.com/tools">
    
    <!-- Permissions -->
    <uses-permission android:name="android.permission.READ_EXTERNAL_STORAGE" />
    <uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE"
        android:maxSdkVersion="28" />
    <uses-permission android:name="android.permission.INTERNET" />
    
    <!-- Hardware features -->
    <uses-feature android:name="android.hardware.touchscreen" android:required="false" />
    <uses-feature android:name="android.software.leanback" android:required="false" />
    
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
        tools:targetApi="31">
        
        <activity
            android:name=".MainActivity"
            android:exported="true"
            android:theme="@style/Theme.WorldFoundry">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
            
            <!-- File associations -->
            <intent-filter>
                <action android:name="android.intent.action.VIEW" />
                <category android:name="android.intent.category.DEFAULT" />
                <category android:name="android.intent.category.BROWSABLE" />
                <data android:mimeType="application/json" />
                <data android:pathPattern=".*\\.map" />
            </intent-filter>
        </activity>
        
        <!-- File provider for sharing -->
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

## Performance Optimization

### Memory Management
- **Bitmap Recycling** - Proper bitmap memory management
- **Native Memory** - Careful JNI memory handling
- **Caching Strategy** - Efficient image and data caching

### Background Processing
- **WorkManager** - Long-running map generation tasks
- **Coroutines** - Structured concurrency for UI operations
- **Room Database** - Efficient local data persistence

### UI Performance
- **Compose Performance** - Optimized recomposition
- **LazyColumn** - Efficient list rendering for large datasets
- **State Hoisting** - Proper state management patterns

## Testing Strategy

### Unit Tests
```kotlin
class MapViewModelTest {
    @get:Rule
    val mainDispatcherRule = MainDispatcherRule()
    
    @Mock
    private lateinit var coreEngine: CoreEngine
    
    @Mock
    private lateinit var mapRepository: MapRepository
    
    private lateinit var viewModel: MapViewModel
    
    @Before
    fun setup() {
        MockitoAnnotations.openMocks(this)
        viewModel = MapViewModel(coreEngine, mapRepository)
    }
    
    @Test
    fun `importMap updates ui state correctly`() = runTest {
        // Arrange
        val testMap = WorldMap(/* test data */)
        whenever(coreEngine.importAzgaarMap(any())).thenReturn(testMap)
        
        // Act
        viewModel.importMap(Uri.parse("test://map.json"))
        
        // Assert
        assertEquals(testMap, viewModel.uiState.value.currentMap)
        assertFalse(viewModel.uiState.value.isLoading)
    }
}
```

### UI Tests
```kotlin
@HiltAndroidTest
class MapScreenTest {
    @get:Rule(order = 0)
    val hiltRule = HiltAndroidRule(this)
    
    @get:Rule(order = 1)
    val composeTestRule = createAndroidComposeRule<MainActivity>()
    
    @Test
    fun mapScreen_displaysCorrectly() {
        composeTestRule.setContent {
            WorldFoundryTheme {
                MapScreen(onNavigateToImport = {})
            }
        }
        
        composeTestRule.onNodeWithText("World Foundry").assertIsDisplayed()
        composeTestRule.onNodeWithContentDescription("Import Map").assertIsDisplayed()
    }
}
```

## Future Enhancements

### Android 14+ Features
- **Predictive Back** - Smooth back navigation animations
- **Per-app Language** - Localization improvements
- **Themed App Icons** - Dynamic app icon theming

### Foldable Support
- **Adaptive Layouts** - Optimized UI for foldable devices
- **Multi-window** - Enhanced multitasking support
- **Hinge Awareness** - UI adaptation for device hinges

### Advanced Features
- **Wear OS** - Companion app for smartwatches
- **Android Auto** - In-car map viewing
- **Shortcuts** - Dynamic shortcuts for quick actions

## Development Setup

### Prerequisites
- Android Studio Hedgehog (2023.1.1) or later
- Android SDK 34
- NDK 25.1.8937393 or later
- Kotlin 1.9.10+
- Rust toolchain with Android targets

### Build Instructions
```bash
# Install Android targets for Rust
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
rustup target add i686-linux-android

# Set up Android NDK environment
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/25.1.8937393

# Build Rust core for Android
cd core
cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target x86_64-linux-android --release
cargo build --target i686-linux-android --release

# Copy libraries to Android project
mkdir -p ../platforms/android/app/src/main/jniLibs/{arm64-v8a,armeabi-v7a,x86_64,x86}
cp target/aarch64-linux-android/release/libworld_foundry.so ../platforms/android/app/src/main/jniLibs/arm64-v8a/
cp target/armv7-linux-androideabi/release/libworld_foundry.so ../platforms/android/app/src/main/jniLibs/armeabi-v7a/
cp target/x86_64-linux-android/release/libworld_foundry.so ../platforms/android/app/src/main/jniLibs/x86_64/
cp target/i686-linux-android/release/libworld_foundry.so ../platforms/android/app/src/main/jniLibs/x86/

# Open Android Studio
cd ../platforms/android
# Open project in Android Studio and build
```

This Android platform architecture provides a comprehensive foundation for a native Android experience with modern Jetpack Compose UI, efficient Rust integration, and Material Design 3 theming optimized for phones, tablets, and foldable devices.