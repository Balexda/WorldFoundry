package com.balexda.worldfoundry.ui

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import com.balexda.worldfoundry.engine.WfEngine
import kotlinx.coroutines.delay

/**
 * Main application composable
 */
@Composable
fun App() {
    val engine = remember { WfEngine() }
    var showSplash by remember { mutableStateOf(true) }
    
    // Show splash screen for a few seconds
    LaunchedEffect(Unit) {
        delay(3000) // 3 seconds
        showSplash = false
    }
    
    MaterialTheme {
        Surface(
            modifier = Modifier.fillMaxSize(),
            color = MaterialTheme.colorScheme.background
        ) {
            if (showSplash) {
                SplashScreen()
            } else {
                WorldMapView(
                    engine = engine,
                    modifier = Modifier.fillMaxSize()
                )
            }
        }
    }
}

/**
 * Preview function for development
 */
@Composable
fun AppPreview() {
    MaterialTheme {
        SplashScreen()
    }
}