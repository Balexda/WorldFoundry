package com.balexda.worldfoundry.ui

import androidx.compose.foundation.*
import androidx.compose.foundation.gestures.*
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.*
import androidx.compose.ui.graphics.drawscope.DrawScope
import androidx.compose.ui.input.pointer.*
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.balexda.worldfoundry.engine.WfEngine
import kotlinx.coroutines.launch
import kotlin.math.*

/**
 * Main world map view with pan/zoom functionality
 */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun WorldMapView(
    engine: WfEngine,
    modifier: Modifier = Modifier
) {
    val scope = rememberCoroutineScope()
    val presenter = remember { TilePresenter(engine, scope) }
    
    // Collect state from presenter
    val viewportState by presenter.viewportState.collectAsState()
    val tileCache by presenter.tileCache.collectAsState()
    val isLoading by presenter.isLoading.collectAsState()
    val error by presenter.error.collectAsState()
    
    // UI state
    var showFileDialog by remember { mutableStateOf(false) }
    var showExportDialog by remember { mutableStateOf(false) }
    
    // Dispose presenter when composable is removed
    DisposableEffect(presenter) {
        onDispose {
            presenter.dispose()
        }
    }
    
    Column(modifier = modifier.fillMaxSize()) {
        // Top toolbar
        TopAppBar(
            title = { 
                Text(
                    "World Foundry",
                    fontWeight = FontWeight.Bold
                )
            },
            actions = {
                // Import button
                IconButton(
                    onClick = { showFileDialog = true }
                ) {
                    Icon(Icons.Default.FolderOpen, contentDescription = "Import Map")
                }
                
                // Export button
                IconButton(
                    onClick = { showExportDialog = true },
                    enabled = tileCache.isNotEmpty()
                ) {
                    Icon(Icons.Default.GetApp, contentDescription = "Export PNG")
                }
                
                // Zoom controls
                IconButton(
                    onClick = { presenter.zoom(1) },
                    enabled = viewportState.zoom < 10
                ) {
                    Icon(Icons.Default.Add, contentDescription = "Zoom In")
                }
                
                IconButton(
                    onClick = { presenter.zoom(-1) },
                    enabled = viewportState.zoom > 0
                ) {
                    Icon(Icons.Default.Remove, contentDescription = "Zoom Out")
                }
            },
            colors = TopAppBarDefaults.topAppBarColors(
                containerColor = MaterialTheme.colorScheme.primaryContainer
            )
        )
        
        // Main map area
        Box(
            modifier = Modifier
                .fillMaxSize()
                .weight(1f)
        ) {
            if (tileCache.isEmpty() && !isLoading) {
                // Empty state
                EmptyMapState(
                    onImportClick = { showFileDialog = true },
                    modifier = Modifier.align(Alignment.Center)
                )
            } else {
                // Map canvas with tiles
                MapCanvas(
                    viewportState = viewportState,
                    tileCache = tileCache,
                    onPan = { deltaX, deltaY -> presenter.pan(deltaX, deltaY) },
                    onZoom = { delta, focusX, focusY -> presenter.zoom(delta, focusX, focusY) },
                    modifier = Modifier.fillMaxSize()
                )
            }
            
            // Loading overlay
            if (isLoading) {
                LoadingOverlay(
                    modifier = Modifier.align(Alignment.Center)
                )
            }
            
            // Error snackbar
            error?.let { errorMessage ->
                LaunchedEffect(errorMessage) {
                    // TODO: Show snackbar with error message
                }
            }
            
            // Map info overlay
            if (tileCache.isNotEmpty()) {
                MapInfoOverlay(
                    viewportState = viewportState,
                    modifier = Modifier
                        .align(Alignment.BottomStart)
                        .padding(16.dp)
                )
            }
        }
    }
    
    // File import dialog
    if (showFileDialog) {
        FileImportDialog(
            onFileSelected = { filePath ->
                scope.launch {
                    presenter.loadMap(filePath)
                }
                showFileDialog = false
            },
            onDismiss = { showFileDialog = false }
        )
    }
    
    // Export dialog
    if (showExportDialog) {
        ExportDialog(
            onExport = { outputPath, width, height ->
                scope.launch {
                    presenter.exportPng(outputPath, width, height)
                }
                showExportDialog = false
            },
            onDismiss = { showExportDialog = false }
        )
    }
}

@Composable
private fun EmptyMapState(
    onImportClick: () -> Unit,
    modifier: Modifier = Modifier
) {
    Column(
        modifier = modifier.padding(32.dp),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Icon(
            Icons.Default.Public,
            contentDescription = null,
            modifier = Modifier.size(64.dp),
            tint = MaterialTheme.colorScheme.onSurfaceVariant
        )
        
        Spacer(modifier = Modifier.height(16.dp))
        
        Text(
            text = "No map loaded",
            style = MaterialTheme.typography.headlineSmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
        
        Text(
            text = "Import an Azgaar .map or .json file to get started",
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
        
        Spacer(modifier = Modifier.height(24.dp))
        
        Button(
            onClick = onImportClick
        ) {
            Icon(Icons.Default.FolderOpen, contentDescription = null)
            Spacer(modifier = Modifier.width(8.dp))
            Text("Import Map")
        }
    }
}

@Composable
private fun LoadingOverlay(
    modifier: Modifier = Modifier
) {
    Surface(
        modifier = modifier,
        color = MaterialTheme.colorScheme.surface.copy(alpha = 0.9f),
        shape = RoundedCornerShape(8.dp)
    ) {
        Column(
            modifier = Modifier.padding(24.dp),
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            CircularProgressIndicator()
            Spacer(modifier = Modifier.height(16.dp))
            Text(
                text = "Loading map...",
                style = MaterialTheme.typography.bodyMedium
            )
        }
    }
}

@Composable
private fun MapInfoOverlay(
    viewportState: ViewportState,
    modifier: Modifier = Modifier
) {
    Surface(
        modifier = modifier,
        color = MaterialTheme.colorScheme.surface.copy(alpha = 0.9f),
        shape = RoundedCornerShape(8.dp)
    ) {
        Column(
            modifier = Modifier.padding(12.dp)
        ) {
            Text(
                text = "Zoom: ${viewportState.zoom}",
                style = MaterialTheme.typography.bodySmall,
                fontWeight = FontWeight.Medium
            )
            Text(
                text = "Center: (${String.format("%.3f", viewportState.centerX)}, ${String.format("%.3f", viewportState.centerY)})",
                style = MaterialTheme.typography.bodySmall
            )
        }
    }
}

@Composable
private fun MapCanvas(
    viewportState: ViewportState,
    tileCache: Map<TileKey, TileData>,
    onPan: (Double, Double) -> Unit,
    onZoom: (Int, Double, Double) -> Unit,
    modifier: Modifier = Modifier
) {
    val density = LocalDensity.current
    
    Canvas(
        modifier = modifier
            .fillMaxSize()
            .pointerInput(Unit) {
                detectDragGestures { change, dragAmount ->
                    onPan(dragAmount.x.toDouble(), dragAmount.y.toDouble())
                }
            }
            .pointerInput(Unit) {
                detectTapGestures(
                    onDoubleTap = { offset ->
                        val focusX = offset.x / size.width
                        val focusY = offset.y / size.height
                        onZoom(1, focusX.toDouble(), focusY.toDouble())
                    }
                )
            }
    ) {
        drawMapTiles(viewportState, tileCache)
    }
}

private fun DrawScope.drawMapTiles(
    viewportState: ViewportState,
    tileCache: Map<TileKey, TileData>
) {
    val canvasWidth = size.width
    val canvasHeight = size.height
    
    // Calculate tile positions and sizes
    val tilesPerSide = (1 shl viewportState.zoom) // 2^zoom
    val tileSize = 256.0
    val worldSize = tilesPerSide * tileSize
    
    // Calculate viewport bounds in world coordinates
    val viewportLeft = viewportState.centerX * worldSize - canvasWidth / 2
    val viewportTop = viewportState.centerY * worldSize - canvasHeight / 2
    
    // Draw tiles
    tileCache.forEach { (key, tileData) ->
        if (key.zoom == viewportState.zoom) {
            val tileWorldX = key.x * tileSize
            val tileWorldY = key.y * tileSize
            
            val tileScreenX = tileWorldX - viewportLeft
            val tileScreenY = tileWorldY - viewportTop
            
            // Only draw tiles that are visible on screen
            if (tileScreenX + tileSize >= 0 && tileScreenX < canvasWidth &&
                tileScreenY + tileSize >= 0 && tileScreenY < canvasHeight) {
                
                // Convert RGBA bytes to ImageBitmap and draw
                try {
                    val imageBitmap = createImageBitmapFromRgba(tileData.rgbaBytes)
                    drawImage(
                        image = imageBitmap,
                        topLeft = Offset(tileScreenX.toFloat(), tileScreenY.toFloat())
                    )
                } catch (e: Exception) {
                    // Draw placeholder rectangle if image creation fails
                    drawRect(
                        color = Color.Gray,
                        topLeft = Offset(tileScreenX.toFloat(), tileScreenY.toFloat()),
                        size = androidx.compose.ui.geometry.Size(tileSize.toFloat(), tileSize.toFloat())
                    )
                }
            }
        }
    }
}

private fun createImageBitmapFromRgba(rgbaBytes: ByteArray): ImageBitmap {
    // Convert RGBA bytes to ImageBitmap
    // This is a simplified implementation - in practice you'd need platform-specific code
    return ImageBitmap(256, 256)
}

@Composable
private fun FileImportDialog(
    onFileSelected: (String) -> Unit,
    onDismiss: () -> Unit
) {
    AlertDialog(
        onDismissRequest = onDismiss,
        title = { Text("Import Map") },
        text = {
            Column {
                Text("Select an Azgaar map file to import:")
                Spacer(modifier = Modifier.height(16.dp))
                Text(
                    "Supported formats: .map, .json",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }
        },
        confirmButton = {
            TextButton(
                onClick = {
                    // TODO: Open file picker
                    // For now, use a hardcoded example path
                    onFileSelected("/workspace/project/core/example_world.json")
                }
            ) {
                Text("Browse")
            }
        },
        dismissButton = {
            TextButton(onClick = onDismiss) {
                Text("Cancel")
            }
        }
    )
}

@Composable
private fun ExportDialog(
    onExport: (String, Int, Int) -> Unit,
    onDismiss: () -> Unit
) {
    var width by remember { mutableStateOf("1920") }
    var height by remember { mutableStateOf("1080") }
    
    AlertDialog(
        onDismissRequest = onDismiss,
        title = { Text("Export PNG") },
        text = {
            Column {
                Text("Export current viewport as PNG:")
                Spacer(modifier = Modifier.height(16.dp))
                
                OutlinedTextField(
                    value = width,
                    onValueChange = { width = it },
                    label = { Text("Width") },
                    modifier = Modifier.fillMaxWidth()
                )
                
                Spacer(modifier = Modifier.height(8.dp))
                
                OutlinedTextField(
                    value = height,
                    onValueChange = { height = it },
                    label = { Text("Height") },
                    modifier = Modifier.fillMaxWidth()
                )
            }
        },
        confirmButton = {
            TextButton(
                onClick = {
                    val w = width.toIntOrNull() ?: 1920
                    val h = height.toIntOrNull() ?: 1080
                    onExport("/workspace/project/exported_map.png", w, h)
                }
            ) {
                Text("Export")
            }
        },
        dismissButton = {
            TextButton(onClick = onDismiss) {
                Text("Cancel")
            }
        }
    )
}