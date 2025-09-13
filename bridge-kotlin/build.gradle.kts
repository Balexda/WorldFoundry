plugins {
    alias(libs.plugins.kotlin.multiplatform)
}

kotlin {
    jvm("desktop")
    
    iosX64()
    iosArm64()
    iosSimulatorArm64()

    sourceSets {
        val commonMain by getting {
            dependencies {
                // Common dependencies
            }
        }
        
        val desktopMain by getting {
            dependencies {
                // Desktop-specific dependencies for FFI
            }
        }
        
        val iosMain by creating {
            dependsOn(commonMain)
        }
        
        val iosX64Main by getting { 
            dependsOn(iosMain) 
        }
        
        val iosArm64Main by getting { 
            dependsOn(iosMain) 
        }
        
        val iosSimulatorArm64Main by getting { 
            dependsOn(iosMain) 
        }
    }
}