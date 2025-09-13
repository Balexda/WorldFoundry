plugins {
    alias(libs.plugins.kotlin.multiplatform)
    alias(libs.plugins.kotlin.compose)
    alias(libs.plugins.compose.multiplatform)
}

kotlin {
    jvm("desktop")
    
    iosX64()
    iosArm64()
    iosSimulatorArm64()

    sourceSets {
        val commonMain by getting {
            dependencies {
                implementation(libs.compose.runtime)
                implementation(libs.compose.ui)
                implementation(libs.compose.foundation)
                implementation(libs.compose.material3)
                implementation(libs.compose.material.icons.extended)
                implementation(libs.compose.resources)
                implementation(libs.kotlinx.coroutines.core)
                
                // World Foundry engine bridge
                implementation(project(":bridge-kotlin"))
            }
        }
        
        val desktopMain by getting {
            dependencies {
                // Desktop-specific dependencies if needed
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