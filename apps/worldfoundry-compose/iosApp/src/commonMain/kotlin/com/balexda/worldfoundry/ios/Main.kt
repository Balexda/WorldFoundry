package com.balexda.worldfoundry.ios

import androidx.compose.ui.window.ComposeUIViewController
import com.balexda.worldfoundry.ui.App
import platform.UIKit.UIViewController

fun MainViewController(): UIViewController = ComposeUIViewController {
    App()
}