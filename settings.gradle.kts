pluginManagement {
    repositories {
        google()
        gradlePluginPortal()
        mavenCentral()
    }
}

dependencyResolutionManagement {
    repositories {
        google()
        mavenCentral()
    }
}

rootProject.name = "WorldFoundry"

// Include KMP modules
include(":apps:worldfoundry-compose:shared")
include(":apps:worldfoundry-compose:desktopApp")
include(":apps:worldfoundry-compose:iosApp")
include(":bridge-kotlin")