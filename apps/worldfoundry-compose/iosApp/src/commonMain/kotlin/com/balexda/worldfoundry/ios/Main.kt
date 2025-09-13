package com.balexda.worldfoundry.ios

import androidx.compose.ui.window.ComposeUIViewController
import com.balexda.worldfoundry.ui.Splash
import platform.UIKit.UIViewController

fun MainViewController(): UIViewController = ComposeUIViewController {
    Splash(appName = "WorldFoundry", version = "0.0.0-splash")
}