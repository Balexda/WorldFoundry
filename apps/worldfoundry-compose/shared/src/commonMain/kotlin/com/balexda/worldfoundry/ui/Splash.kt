package com.balexda.worldfoundry.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import kotlinx.coroutines.delay

@Composable
fun Splash(appName: String, version: String) {
    SplashScreen()
}

@Composable
fun SplashScreen() {
    // Animated loading state
    var loadingProgress by remember { mutableStateOf(0f) }
    
    LaunchedEffect(Unit) {
        while (loadingProgress < 1f) {
            delay(50)
            loadingProgress += 0.02f
        }
    }
    
    // Fantasy-themed gradient background inspired by the provided image
    val gradientBrush = Brush.verticalGradient(
        colors = listOf(
            Color(0xFF1A237E), // Deep blue
            Color(0xFF283593), // Medium blue
            Color(0xFF3949AB), // Lighter blue
            Color(0xFF1A237E)  // Back to deep blue
        )
    )
    
    Surface(
        modifier = Modifier.fillMaxSize(),
        color = Color.Transparent
    ) {
        Box(
            modifier = Modifier
                .fillMaxSize()
                .background(gradientBrush)
        ) {
            Column(
                modifier = Modifier
                    .fillMaxSize()
                    .padding(32.dp),
                horizontalAlignment = Alignment.CenterHorizontally,
                verticalArrangement = Arrangement.Center
            ) {
                // Main title with fantasy styling matching the image
                Text(
                    text = "WORLD",
                    fontSize = 48.sp,
                    fontWeight = FontWeight.Bold,
                    color = Color(0xFFF5F5DC), // Beige/cream color like in the image
                    letterSpacing = 4.sp
                )
                
                Text(
                    text = "FOUNDRY",
                    fontSize = 48.sp,
                    fontWeight = FontWeight.Bold,
                    color = Color(0xFFF5F5DC), // Beige/cream color like in the image
                    letterSpacing = 4.sp
                )
                
                Spacer(modifier = Modifier.height(32.dp))
                
                // Magical world representation (placeholder for the character and world map)
                Box(
                    modifier = Modifier
                        .size(120.dp)
                        .clip(RoundedCornerShape(60.dp))
                        .background(
                            Brush.radialGradient(
                                colors = listOf(
                                    Color(0xFF4FC3F7), // Light blue
                                    Color(0xFF29B6F6), // Medium blue
                                    Color(0xFF0288D1)  // Dark blue
                                )
                            )
                        ),
                    contentAlignment = Alignment.Center
                ) {
                    // Placeholder for the anime character and world map from the image
                    Text(
                        text = "🌍✨",
                        fontSize = 36.sp
                    )
                }
                
                Spacer(modifier = Modifier.height(32.dp))
                
                // Subtitle matching the image
                Text(
                    text = "Forge your fantasy worlds with\nnative performance",
                    fontSize = 18.sp,
                    color = Color(0xFFE8EAF6), // Light blue-grey
                    textAlign = TextAlign.Center,
                    lineHeight = 24.sp
                )
                
                Spacer(modifier = Modifier.height(48.dp))
                
                // Animated progress bar with magical styling
                Column(
                    horizontalAlignment = Alignment.CenterHorizontally
                ) {
                    LinearProgressIndicator(
                        progress = { loadingProgress },
                        modifier = Modifier
                            .width(200.dp)
                            .height(4.dp)
                            .clip(RoundedCornerShape(2.dp)),
                        color = Color(0xFFFFD700), // Gold color for magical feel
                        trackColor = Color(0xFF3949AB).copy(alpha = 0.3f)
                    )
                    
                    Spacer(modifier = Modifier.height(16.dp))
                    
                    Text(
                        text = "Loading magical worlds...",
                        fontSize = 14.sp,
                        color = Color(0xFFE8EAF6).copy(alpha = 0.8f)
                    )
                }
            }
            
            // Version info at bottom
            Text(
                text = "v1.0.0-alpha",
                fontSize = 12.sp,
                color = Color(0xFFE8EAF6).copy(alpha = 0.6f),
                modifier = Modifier
                    .align(Alignment.BottomCenter)
                    .padding(16.dp)
            )
        }
    }
}