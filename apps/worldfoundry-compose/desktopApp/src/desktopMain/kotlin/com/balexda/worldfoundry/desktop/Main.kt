package com.balexda.worldfoundry.desktop

import androidx.compose.ui.window.Window
import androidx.compose.ui.window.application
import com.balexda.worldfoundry.ui.Splash

fun main() = application {
    Window(
        onCloseRequest = ::exitApplication, 
        title = "WorldFoundry"
    ) {
        Splash(appName = "WorldFoundry", version = "0.0.0-splash")
    }
}