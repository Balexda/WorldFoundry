package com.balexda.worldfoundry.ui

import com.balexda.worldfoundry.engine.*
import kotlinx.coroutines.*
import kotlinx.coroutines.flow.*

/**
 * Presenter for tile-based world map rendering with pan/zoom support.
 * Manages tile loading, caching, and viewport state.
 */
class TilePresenter(
    private val engine: WfEngine,
    private val scope: CoroutineScope = CoroutineScope(Dispatchers.Default + SupervisorJob())
) {
    
    // Viewport state
    private val _viewportState = MutableStateFlow(ViewportState())
    val viewportState: StateFlow<ViewportState> = _viewportState.asStateFlow()
    
    // Tile cache
    private val _tileCache = MutableStateFlow<Map<TileKey, TileData>>(emptyMap())
    val tileCache: StateFlow<Map<TileKey, TileData>> = _tileCache.asStateFlow()
    
    // Loading state
    private val _isLoading = MutableStateFlow(false)
    val isLoading: StateFlow<Boolean> = _isLoading.asStateFlow()
    
    // Error state
    private val _error = MutableStateFlow<String?>(null)
    val error: StateFlow<String?> = _error.asStateFlow()
    
    // Engine resources
    private var context: WfContext? = null
    private var project: WfProject? = null
    private var renderer: WfRenderer? = null
    
    /**
     * Load a world map from file
     */
    suspend fun loadMap(filePath: String) {
        withContext(Dispatchers.Default) {
            try {
                _isLoading.value = true
                _error.value = null
                
                // Clean up existing resources
                cleanup()
                
                // Create new engine resources
                context = engine.createContext()
                if (context == null) {
                    throw Exception("Failed to create engine context")
                }
                
                project = engine.createProject(context!!, filePath)
                if (project == null) {
                    throw Exception("Failed to load project from: $filePath")
                }
                
                renderer = engine.createRenderer(project!!)
                if (renderer == null) {
                    throw Exception("Failed to create renderer")
                }
                
                // Reset viewport to show full map
                _viewportState.value = ViewportState(
                    centerX = 0.5,
                    centerY = 0.5,
                    zoom = 0,
                    viewportWidth = 1024,
                    viewportHeight = 768
                )
                
                // Clear tile cache
                _tileCache.value = emptyMap()
                
                // Load initial tiles
                loadVisibleTiles()
                
            } catch (e: Exception) {
                _error.value = e.message ?: "Unknown error loading map"
                cleanup()
            } finally {
                _isLoading.value = false
            }
        }
    }
    
    /**
     * Update viewport (pan/zoom)
     */
    fun updateViewport(
        centerX: Double? = null,
        centerY: Double? = null,
        zoom: Int? = null,
        viewportWidth: Int? = null,
        viewportHeight: Int? = null
    ) {
        val current = _viewportState.value
        val newState = current.copy(
            centerX = centerX ?: current.centerX,
            centerY = centerY ?: current.centerY,
            zoom = zoom ?: current.zoom,
            viewportWidth = viewportWidth ?: current.viewportWidth,
            viewportHeight = viewportHeight ?: current.viewportHeight
        )
        
        _viewportState.value = newState
        
        // Load tiles for new viewport
        scope.launch {
            loadVisibleTiles()
        }
    }
    
    /**
     * Pan the viewport by pixel offset
     */
    fun pan(deltaX: Double, deltaY: Double) {
        val current = _viewportState.value
        val scale = Math.pow(2.0, current.zoom.toDouble())
        val worldSize = 256.0 * scale // Assuming 256px tiles
        
        val newCenterX = (current.centerX - deltaX / worldSize).coerceIn(0.0, 1.0)
        val newCenterY = (current.centerY - deltaY / worldSize).coerceIn(0.0, 1.0)
        
        updateViewport(centerX = newCenterX, centerY = newCenterY)
    }
    
    /**
     * Zoom in/out around a point
     */
    fun zoom(delta: Int, focusX: Double = 0.5, focusY: Double = 0.5) {
        val current = _viewportState.value
        val newZoom = (current.zoom + delta).coerceIn(0, 10)
        
        if (newZoom != current.zoom) {
            // Adjust center to zoom around focus point
            val zoomFactor = Math.pow(2.0, delta.toDouble())
            val newCenterX = current.centerX + (focusX - 0.5) * (1.0 - 1.0 / zoomFactor)
            val newCenterY = current.centerY + (focusY - 0.5) * (1.0 - 1.0 / zoomFactor)
            
            updateViewport(
                centerX = newCenterX.coerceIn(0.0, 1.0),
                centerY = newCenterY.coerceIn(0.0, 1.0),
                zoom = newZoom
            )
        }
    }
    
    /**
     * Export current viewport as PNG
     */
    suspend fun exportPng(outputPath: String, width: Int = 1920, height: Int = 1080): Boolean {
        return withContext(Dispatchers.IO) {
            try {
                project?.let { proj ->
                    engine.exportPng(proj, outputPath, width, height)
                } ?: false
            } catch (e: Exception) {
                _error.value = "Export failed: ${e.message}"
                false
            }
        }
    }
    
    /**
     * Load tiles visible in current viewport
     */
    private suspend fun loadVisibleTiles() {
        val renderer = this.renderer ?: return
        val viewport = _viewportState.value
        
        withContext(Dispatchers.Default) {
            try {
                val visibleTiles = calculateVisibleTiles(viewport)
                val currentCache = _tileCache.value.toMutableMap()
                
                // Load missing tiles
                val loadJobs = visibleTiles.mapNotNull { tileKey ->
                    if (!currentCache.containsKey(tileKey)) {
                        async {
                            loadTile(renderer, tileKey)
                        }
                    } else null
                }
                
                // Wait for all tiles to load
                val newTiles = loadJobs.awaitAll().filterNotNull()
                
                // Update cache
                newTiles.forEach { tile ->
                    currentCache[tile.key] = tile
                }
                
                // Remove old tiles (keep cache size reasonable)
                if (currentCache.size > 100) {
                    val tilesToRemove = currentCache.keys.take(currentCache.size - 100)
                    tilesToRemove.forEach { currentCache.remove(it) }
                }
                
                _tileCache.value = currentCache
                
            } catch (e: Exception) {
                _error.value = "Failed to load tiles: ${e.message}"
            }
        }
    }
    
    /**
     * Calculate which tiles are visible in the viewport
     */
    private fun calculateVisibleTiles(viewport: ViewportState): List<TileKey> {
        val tiles = mutableListOf<TileKey>()
        val tilesPerSide = Math.pow(2.0, viewport.zoom.toDouble()).toInt()
        val tileSize = 256.0
        
        // Calculate viewport bounds in tile coordinates
        val viewportLeft = viewport.centerX - (viewport.viewportWidth / 2.0) / (tilesPerSide * tileSize)
        val viewportRight = viewport.centerX + (viewport.viewportWidth / 2.0) / (tilesPerSide * tileSize)
        val viewportTop = viewport.centerY - (viewport.viewportHeight / 2.0) / (tilesPerSide * tileSize)
        val viewportBottom = viewport.centerY + (viewport.viewportHeight / 2.0) / (tilesPerSide * tileSize)
        
        // Find tiles that intersect with viewport
        val minTileX = (viewportLeft * tilesPerSide).toInt().coerceAtLeast(0)
        val maxTileX = (viewportRight * tilesPerSide).toInt().coerceAtMost(tilesPerSide - 1)
        val minTileY = (viewportTop * tilesPerSide).toInt().coerceAtLeast(0)
        val maxTileY = (viewportBottom * tilesPerSide).toInt().coerceAtMost(tilesPerSide - 1)
        
        for (x in minTileX..maxTileX) {
            for (y in minTileY..maxTileY) {
                tiles.add(TileKey(x, y, viewport.zoom))
            }
        }
        
        return tiles
    }
    
    /**
     * Load a single tile from the engine
     */
    private suspend fun loadTile(renderer: WfRenderer, key: TileKey): TileData? {
        return withContext(Dispatchers.Default) {
            try {
                val tileBytes = engine.renderTile(renderer, key.x, key.y, key.zoom)
                if (tileBytes != null) {
                    TileData(key, tileBytes)
                } else {
                    null
                }
            } catch (e: Exception) {
                null
            }
        }
    }
    
    /**
     * Clean up engine resources
     */
    private fun cleanup() {
        renderer?.let { engine.destroyRenderer(it) }
        project?.let { engine.destroyProject(it) }
        context?.let { engine.destroyContext(it) }
        
        renderer = null
        project = null
        context = null
    }
    
    /**
     * Clean up resources when presenter is no longer needed
     */
    fun dispose() {
        scope.cancel()
        cleanup()
    }
}

/**
 * Viewport state for pan/zoom
 */
data class ViewportState(
    val centerX: Double = 0.5,      // Center X in world coordinates (0.0 to 1.0)
    val centerY: Double = 0.5,      // Center Y in world coordinates (0.0 to 1.0)
    val zoom: Int = 0,              // Zoom level (0 = most zoomed out)
    val viewportWidth: Int = 1024,  // Viewport width in pixels
    val viewportHeight: Int = 768   // Viewport height in pixels
)

/**
 * Tile identifier
 */
data class TileKey(
    val x: Int,
    val y: Int,
    val zoom: Int
)

/**
 * Tile data with RGBA bytes
 */
data class TileData(
    val key: TileKey,
    val rgbaBytes: ByteArray // 256x256x4 RGBA bytes
) {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (other !is TileData) return false
        if (key != other.key) return false
        if (!rgbaBytes.contentEquals(other.rgbaBytes)) return false
        return true
    }
    
    override fun hashCode(): Int {
        var result = key.hashCode()
        result = 31 * result + rgbaBytes.contentHashCode()
        return result
    }
}