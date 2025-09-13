package com.balexda.worldfoundry.desktop

import androidx.compose.ui.window.Window
import androidx.compose.ui.window.application
import androidx.compose.ui.window.WindowState
import androidx.compose.ui.unit.dp
import com.balexda.worldfoundry.ui.App

fun main() = application {
    Window(
        onCloseRequest = ::exitApplication, 
        title = "World Foundry",
        state = WindowState(width = 1200.dp, height = 800.dp)
    ) {
        App()
    }
}